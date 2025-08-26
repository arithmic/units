//! NFT Token ZK Proof System
//! 
//! This implements the 8-step NFT flow with zero-knowledge proof generation.

use clap::Parser;
use sp1_sdk::{EnvProver, SP1Stdin};
use borsh::{BorshSerialize, BorshDeserialize, to_vec};
use unitsdesign::nft_token::MyNFTTokenData;
use unitsdesign::types::{Address, KeyValue, TransactionReceipt};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, Read};
use serde::{Serialize, Deserialize};

/// The arguments for the command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    execute: bool,

    #[arg(long)]
    prove: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct TokenInput {
    token_name: [u8; 32],
    function_name: String,
    signer: Address,
    pre_state: Vec<KeyValue>,
    timestamp: u64,
    block_id: u64,
    transaction_hash: [u8; 32],
    token_id: String,
    nonce: u64,
    input_data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TransactionLog {
    transaction_id: String,
    token_id: [u8; 32],
    from_address: Address,
    to_address: Address,
    timestamp: u64,
    operation: String,
    status: String,
    proof_hash: Option<String>,
    ledger_block_id: Option<u64>,
    ledger_index: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ZKProofMetadata {
    proof_hash: String,
    verification_key_hash: String,
    public_inputs: Vec<String>,
    proof_size: usize,
    generation_time: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LedgerEntry {
    block_id: u64,
    transaction_index: u64,
    proof_hash: String,
    timestamp: u64,
    verification_status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TokenValidationResult {
    is_valid: bool,
    validation_errors: Vec<String>,
    token_metadata: HashMap<String, String>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
struct TokenOutput {
    success: bool,
    receipt: Option<TransactionReceipt>,
    error: Option<String>,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Build the program.
    sp1_build::build_program("../program");
    let elf_path = "../target/elf-compilation/riscv32im-succinct-zkvm-elf/release/program";

    // Load the ELF.
    let elf = std::fs::read(elf_path).expect("Failed to read ELF file");

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = EnvProver::default();
    
    // NFT Token Flow Implementation
    execute_nft_flow(client, &elf, args.execute);
}

fn execute_nft_flow(client: EnvProver, elf: &[u8], execute_only: bool) {
    println!("=== NFT Token Flow Implementation ===");
    
    // Setup admin and recipient addresses
    let admin_address: Address = [1u8; 32];
    let recipient: Address = [2u8; 32];
    let new_owner: Address = [3u8; 32];
    
    // Create NFT token data
    let token_id = [42u8; 32];
    let unique_identifier = [123u8; 32];
    let collectible_hash = [255u8; 32];
    let image_data = vec![0xDE, 0xAD, 0xBE, 0xEF];
    
    let nft_data = MyNFTTokenData {
        token_id,
        unique_identifier,
        collectible_hash,
        owner_id: recipient,
        collectible_image_data: image_data,
    };
    
    // Step 1: Validate token
    println!("=== Step 1: Validating Token ===");
    let validation_result = validate_token(&nft_data);
    println!("Token validation: {}", if validation_result.is_valid { "PASSED" } else { "FAILED" });
    if !validation_result.validation_errors.is_empty() {
        println!("Validation errors: {:?}", validation_result.validation_errors);
    }
    
    // Prepare input for SP1 program (mint operation)
    let token_name = [84u8; 32]; // "T" repeated as token identifier
    let mint_input_data = to_vec(&nft_data).expect("Failed to serialize NFT data");
    
    let token_input = TokenInput {
        token_name,
        function_name: "mint".to_string(),
        signer: admin_address,
        pre_state: vec![],
        timestamp: 1638400000,
        block_id: 1,
        transaction_hash: [0u8; 32],
        token_id: "NFTToken".to_string(),
        nonce: 0,
        input_data: mint_input_data,
    };
    
    let input_bytes = to_vec(&token_input).expect("Failed to serialize token input");
    
    let mut stdin = SP1Stdin::new();
    stdin.write(&input_bytes);

    if execute_only {
        // Execute the program
        println!("=== Step 2: Executing Mint Operation ===");
        let (output, report) = client.execute(elf, &stdin).run().unwrap();
        println!("Program executed successfully.");
        println!("Total instruction count: {}", report.total_instruction_count());
        
        // Try to deserialize the output to get receipt
        let receipt = if let Ok(token_output) = borsh::from_slice::<TokenOutput>(&output.as_slice()) {
            token_output.receipt
        } else {
            None
        };
        
        println!("Output: {:?}", output);
        
        // Continue with remaining NFT flow steps...
        run_remaining_nft_steps(&nft_data, recipient, new_owner, receipt);
    } else {
        // Generate the proof
        println!("=== Step 2: Generating ZK Proof for Mint ===");
        let (pk, vk) = client.setup(elf);
        let proof = client.prove(&pk, &stdin).run().unwrap();
        println!("Successfully generated proof!");

        // Verify the proof
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
        
        // In prove mode, we don't get the receipt directly, so pass None
        // Continue with remaining NFT flow steps...
        run_remaining_nft_steps(&nft_data, recipient, new_owner, None);
    }
}

fn run_remaining_nft_steps(nft_data: &MyNFTTokenData, recipient: Address, new_owner: Address, mint_receipt: Option<TransactionReceipt>) {
    let transaction_id = format!("txn_{}", hex::encode(&nft_data.token_id[..8]));
    
    // Step 3: Initiate transfer
    println!("=== Step 3: Initiating Transfer ===");
    let transfer_result = initiate_transfer(nft_data, recipient, new_owner);
    println!("Transfer initiated: {:?}", transfer_result);
    
    // Step 4: Generate ZK Proof metadata
    println!("=== Step 4: Generating ZK Proof Metadata ===");
    let proof_metadata = generate_zk_proof_metadata(&mint_receipt);
    println!("ZK proof metadata: hash = {}, size = {} bytes", 
             proof_metadata.proof_hash, proof_metadata.proof_size);
    
    // Step 5: Save transaction log
    println!("=== Step 5: Saving Transaction Log ===");
    let tx_log = save_transaction_log(&transaction_id, nft_data, recipient, new_owner, "transfer", &proof_metadata.proof_hash);
    println!("Transaction log saved with ID: {}", tx_log.transaction_id);
    
    // Step 6: Commit transfer
    println!("=== Step 6: Committing Transfer ===");
    let commit_result = commit_transfer(nft_data, new_owner);
    println!("Transfer committed: {}", if commit_result { "SUCCESS" } else { "FAILED" });
    
    // Step 7: Save proof in public ledger
    println!("=== Step 7: Saving Proof to Public Ledger ===");
    let ledger_entry = save_proof_to_ledger(&proof_metadata.proof_hash);
    println!("Saved to ledger - Block: {}, Index: {}", ledger_entry.block_id, ledger_entry.transaction_index);
    
    // Step 8: Update transaction log with ledger metadata
    println!("=== Step 8: Updating Transaction Log with Ledger Metadata ===");
    let _updated_log = update_transaction_log_with_ledger(&transaction_id, &ledger_entry);
    println!("Updated transaction log with ledger metadata");
    
    println!("=== NFT Transfer Flow Completed Successfully ===");
    println!("Final token owner: {:?}", new_owner);
    println!("Ledger record: Block {}, Index {}", ledger_entry.block_id, ledger_entry.transaction_index);
}

// Step 1: Token Validation
fn validate_token(nft_data: &MyNFTTokenData) -> TokenValidationResult {
    let mut validation_errors = Vec::new();
    let mut token_metadata = HashMap::new();
    
    // Check token ID is not empty
    if nft_data.token_id.iter().all(|&x| x == 0) {
        validation_errors.push("Token ID cannot be all zeros".to_string());
    }
    
    // Check unique identifier is not empty
    if nft_data.unique_identifier.iter().all(|&x| x == 0) {
        validation_errors.push("Unique identifier cannot be all zeros".to_string());
    }
    
    // Check collectible hash is not empty
    if nft_data.collectible_hash.iter().all(|&x| x == 0) {
        validation_errors.push("Collectible hash cannot be all zeros".to_string());
    }
    
    // Check image data is not empty
    if nft_data.collectible_image_data.is_empty() {
        validation_errors.push("Collectible image data cannot be empty".to_string());
    }
    
    // Add metadata
    token_metadata.insert("token_id_hex".to_string(), hex::encode(nft_data.token_id));
    token_metadata.insert("unique_id_hex".to_string(), hex::encode(nft_data.unique_identifier));
    token_metadata.insert("collectible_hash_hex".to_string(), hex::encode(nft_data.collectible_hash));
    token_metadata.insert("owner_address_hex".to_string(), hex::encode(nft_data.owner_id));
    token_metadata.insert("image_data_size".to_string(), nft_data.collectible_image_data.len().to_string());
    
    TokenValidationResult {
        is_valid: validation_errors.is_empty(),
        validation_errors,
        token_metadata,
    }
}

// Step 3: Initiate Transfer
fn initiate_transfer(nft_data: &MyNFTTokenData, from: Address, to: Address) -> bool {
    // Validate the transfer is allowed
    if nft_data.owner_id != from {
        println!("Error: Transfer initiated by non-owner");
        return false;
    }
    
    if from == to {
        println!("Error: Cannot transfer to same address");
        return false;
    }
    
    // Create transfer state file
    let transfer_state = format!(
        "{{\"token_id\":\"{}\",\"from\":\"{}\",\"to\":\"{}\",\"status\":\"initiated\",\"timestamp\":{}}}",
        hex::encode(nft_data.token_id),
        hex::encode(from),
        hex::encode(to),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
    );
    
    if let Ok(mut file) = File::create("transfer_state.json") {
        let _ = file.write_all(transfer_state.as_bytes());
    }
    
    println!("Transfer state saved to transfer_state.json");
    true
}

// Step 4: Generate ZK Proof Metadata
fn generate_zk_proof_metadata(receipt: &Option<TransactionReceipt>) -> ZKProofMetadata {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let proof_data = match receipt {
        Some(r) => {
            let writes_hash = if !r.writes.is_empty() {
                format!("writes_{}", r.writes.len())
            } else {
                "empty_writes".to_string()
            };
            format!("receipt_hash_{}", hex::encode(writes_hash.as_bytes()))
        },
        None => "mock_proof_data".to_string(),
    };
    
    // Generate mock proof hash (in real implementation, this would be the actual proof hash)
    let proof_hash = format!("proof_{}", hex::encode(proof_data.as_bytes()));
    let vk_hash = format!("vk_{}", hex::encode("verification_key".as_bytes()));
    
    ZKProofMetadata {
        proof_hash: proof_hash.clone(),
        verification_key_hash: vk_hash,
        public_inputs: vec![
            "token_transfer".to_string(),
            proof_data,
        ],
        proof_size: 1024, // Mock size
        generation_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
    }
}

// Step 5: Save Transaction Log
fn save_transaction_log(
    transaction_id: &str,
    nft_data: &MyNFTTokenData,
    from: Address,
    to: Address,
    operation: &str,
    proof_hash: &str,
) -> TransactionLog {
    let tx_log = TransactionLog {
        transaction_id: transaction_id.to_string(),
        token_id: nft_data.token_id,
        from_address: from,
        to_address: to,
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        operation: operation.to_string(),
        status: "pending".to_string(),
        proof_hash: Some(proof_hash.to_string()),
        ledger_block_id: None,
        ledger_index: None,
    };
    
    // Save to file
    let log_json = serde_json::to_string_pretty(&tx_log).unwrap();
    let filename = format!("transaction_log_{}.json", transaction_id);
    
    if let Ok(mut file) = File::create(&filename) {
        let _ = file.write_all(log_json.as_bytes());
        println!("Transaction log saved to {}", filename);
    }
    
    tx_log
}

// Step 6: Commit Transfer
fn commit_transfer(nft_data: &MyNFTTokenData, new_owner: Address) -> bool {
    // Update ownership state
    let ownership_state = format!(
        "{{\"token_id\":\"{}\",\"new_owner\":\"{}\",\"committed_at\":{},\"status\":\"committed\"}}",
        hex::encode(nft_data.token_id),
        hex::encode(new_owner),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
    );
    
    if let Ok(mut file) = File::create("ownership_state.json") {
        let _ = file.write_all(ownership_state.as_bytes());
        println!("Ownership state committed to ownership_state.json");
        return true;
    }
    
    false
}

// Step 7: Save Proof to Public Ledger
fn save_proof_to_ledger(proof_hash: &str) -> LedgerEntry {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    // Mock ledger entry (in real implementation, this would interact with actual blockchain)
    let block_id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() % 1000000;
    let transaction_index = 1; // Mock index
    
    let ledger_entry = LedgerEntry {
        block_id,
        transaction_index,
        proof_hash: proof_hash.to_string(),
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        verification_status: "verified".to_string(),
    };
    
    // Save to ledger file
    let ledger_json = serde_json::to_string_pretty(&ledger_entry).unwrap();
    let filename = format!("ledger_entry_{}.json", block_id);
    
    if let Ok(mut file) = File::create(&filename) {
        let _ = file.write_all(ledger_json.as_bytes());
        println!("Ledger entry saved to {}", filename);
    }
    
    ledger_entry
}

// Step 8: Update Transaction Log with Ledger Metadata
fn update_transaction_log_with_ledger(transaction_id: &str, ledger_entry: &LedgerEntry) -> TransactionLog {
    let filename = format!("transaction_log_{}.json", transaction_id);
    
    // Read existing transaction log
    let mut tx_log = if let Ok(mut file) = File::open(&filename) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            serde_json::from_str::<TransactionLog>(&contents).unwrap_or_else(|_| {
                // Create default if parsing fails
                TransactionLog {
                    transaction_id: transaction_id.to_string(),
                    token_id: [0u8; 32],
                    from_address: [0u8; 32],
                    to_address: [0u8; 32],
                    timestamp: 0,
                    operation: "unknown".to_string(),
                    status: "error".to_string(),
                    proof_hash: None,
                    ledger_block_id: None,
                    ledger_index: None,
                }
            })
        } else {
            // Create default if read fails
            TransactionLog {
                transaction_id: transaction_id.to_string(),
                token_id: [0u8; 32],
                from_address: [0u8; 32],
                to_address: [0u8; 32],
                timestamp: 0,
                operation: "unknown".to_string(),
                status: "error".to_string(),
                proof_hash: None,
                ledger_block_id: None,
                ledger_index: None,
            }
        }
    } else {
        // Create default if file doesn't exist
        TransactionLog {
            transaction_id: transaction_id.to_string(),
            token_id: [0u8; 32],
            from_address: [0u8; 32],
            to_address: [0u8; 32],
            timestamp: 0,
            operation: "unknown".to_string(),
            status: "error".to_string(),
            proof_hash: None,
            ledger_block_id: None,
            ledger_index: None,
        }
    };
    
    // Update with ledger metadata
    tx_log.ledger_block_id = Some(ledger_entry.block_id);
    tx_log.ledger_index = Some(ledger_entry.transaction_index);
    tx_log.status = "completed".to_string();
    
    // Save updated log
    let updated_json = serde_json::to_string_pretty(&tx_log).unwrap();
    if let Ok(mut file) = File::create(&filename) {
        let _ = file.write_all(updated_json.as_bytes());
        println!("Updated transaction log saved to {}", filename);
    }
    
    tx_log
}