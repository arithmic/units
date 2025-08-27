//! NFT Token ZK Proof System
//!
//! This implements the 6-step NFT flow with zero-knowledge proof generation.

use borsh::to_vec;
use clap::Parser;
use sp1_sdk::{EnvProver, SP1ProofWithPublicValues, SP1Stdin};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use unitsdesign::common::{
    LedgerEntry, TokenValidationResult, TransactionInput, TransactionLog, TransactionOutput,
    ZKProofMetadata,
};
use unitsdesign::nft_token::MyNFTTokenData;
use unitsdesign::types::Address;

/// The arguments for the command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Execute the program without generating proof
    Execute,
    /// Generate and verify a ZK proof
    Prove,
    /// Run the full NFT flow with all 6 logical steps
    Nftflow,
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

    // Setup the prover client.
    let client = EnvProver::default();

    // Route to appropriate command
    match args.command {
        Commands::Execute => execute_simple(&client, &elf),
        Commands::Prove => prove_simple(&client, &elf),
        Commands::Nftflow => execute_nft_flow(&client, &elf),
    }
}

// Simple execute command
fn execute_simple(client: &EnvProver, elf: &[u8]) {
    println!("=== Simple Execute Mode ===");

    let (stdin, _nft_data) = create_mint_transaction();

    // Execute the program
    let (output, report) = client.execute(elf, &stdin).run().unwrap();
    println!("Program executed successfully.");
    println!(
        "Total instruction count: {}",
        report.total_instruction_count()
    );
    println!("Output: {:?}", output);

    // Try to deserialize and display result
    if let Ok(token_output) = borsh::from_slice::<TransactionOutput>(&output.as_slice()) {
        println!("Execution result: Success = {}", token_output.success);
        if let Some(error) = token_output.error {
            println!("Error: {}", error);
        }
    }
}

// Simple prove command
fn prove_simple(client: &EnvProver, elf: &[u8]) {
    println!("=== Simple Prove Mode ===");

    let (stdin, _nft_data) = create_mint_transaction();

    // Generate the proof
    println!("Generating ZK proof...");
    let (pk, vk) = client.setup(elf);
    let proof = client.prove(&pk, &stdin).run().unwrap();
    println!("Successfully generated proof!");

    // Save the proof
    save_proof_to_file(&proof, "simple_proof.bin");
    println!("Proof saved to simple_proof.bin");

    // Verify the proof
    client.verify(&proof, &vk).expect("failed to verify proof");
    println!("Successfully verified proof!");
}

// Full NFT flow command - 6 logical steps
fn execute_nft_flow(client: &EnvProver, elf: &[u8]) {
    println!("=== NFT Token Flow Implementation (6 Steps) ===");
    let (stdin, nft_data) = create_mint_transaction();
    let recipient: Address = [2u8; 32];
    let new_owner: Address = [3u8; 32];
    let transaction_id = format!("txn_{}", hex::encode(&nft_data.token_id[..8]));

    // Step 1: Validate transaction input
    println!("=== Step 1: Validating Transaction Input ===");
    let validation_result = validate_transaction_input(&nft_data, recipient, new_owner);
    println!(
        "Input validation: {}",
        if validation_result.is_valid {
            "PASSED"
        } else {
            "FAILED"
        }
    );
    if !validation_result.validation_errors.is_empty() {
        println!(
            "Validation errors: {:?}",
            validation_result.validation_errors
        );
        return; // Exit if validation fails
    }

    // Step 2: Initiate transfer (execute the transaction)
    println!("=== Step 2: Initiating Transfer & Executing Transaction ===");
    let transfer_result = initiate_transfer(&nft_data, recipient, new_owner);
    if !transfer_result {
        println!("Transfer initiation failed!");
        return;
    }
    println!("Transfer initiated and executed successfully");

    // Step 3: Generate ZK proof
    println!("=== Step 3: Generating ZK Proof ===");
    let (pk, vk) = client.setup(elf);
    let proof = client.prove(&pk, &stdin).run().unwrap();
    println!("Successfully generated ZK proof!");

    // Verify the proof
    client.verify(&proof, &vk).expect("Failed to verify proof");
    println!("Successfully verified ZK proof!");

    // Step 4: Save to database (mocked - transaction log and proof)
    println!("=== Step 4: Saving Transaction & Proof to Database (Mocked) ===");
    save_proof_to_file(&proof, "nft_flow_proof.bin");
    println!("Proof saved to database (file: nft_flow_proof.bin)");

    let proof_metadata = generate_zk_proof_metadata(&proof);
    let tx_log = save_transaction_log(
        &transaction_id,
        &nft_data,
        recipient,
        new_owner,
        "transfer",
        &proof_metadata.proof_hash,
    );
    println!(
        "Transaction log saved to database with ID: {}",
        tx_log.transaction_id
    );

    // Step 5: Update global state AST (SMT) using transaction receipt (mocked)
    println!("=== Step 5: Updating Global State AST/SMT (Mocked) ===");
    let state_update_result = update_global_state_smt(&nft_data, new_owner, &proof_metadata);
    println!(
        "Global state updated: {}",
        if state_update_result {
            "SUCCESS"
        } else {
            "FAILED"
        }
    );

    // Step 6: Update transaction log with ledger metadata (mocked)
    println!("=== Step 6: Publishing to Public Ledger & Updating Transaction Log (Mocked) ===");
    let ledger_entry = save_proof_to_ledger(&proof_metadata.proof_hash);
    println!(
        "Proof published to public ledger - Block: {}, Index: {}",
        ledger_entry.block_id, ledger_entry.transaction_index
    );

    let _updated_log = update_transaction_log_with_ledger(&transaction_id, &ledger_entry);
    println!("Transaction log updated with ledger metadata");

    println!("=== NFT Transfer Flow Completed Successfully ===");
    println!("Final token owner: {:?}", new_owner);
    println!("Proof hash: {}", proof_metadata.proof_hash);
    println!(
        "Ledger record: Block {}, Index {}",
        ledger_entry.block_id, ledger_entry.transaction_index
    );
}

// Step 1: Transaction Input Validation
fn validate_transaction_input(
    nft_data: &MyNFTTokenData,
    from: Address,
    to: Address,
) -> TokenValidationResult {
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

    // Validate transfer addresses
    if from == to {
        validation_errors.push("Cannot transfer to the same address".to_string());
    }

    if nft_data.owner_id != from {
        validation_errors.push("Transfer must be initiated by the token owner".to_string());
    }

    // Add metadata
    token_metadata.insert("token_id_hex".to_string(), hex::encode(nft_data.token_id));
    token_metadata.insert(
        "unique_id_hex".to_string(),
        hex::encode(nft_data.unique_identifier),
    );
    token_metadata.insert(
        "collectible_hash_hex".to_string(),
        hex::encode(nft_data.collectible_hash),
    );
    token_metadata.insert(
        "owner_address_hex".to_string(),
        hex::encode(nft_data.owner_id),
    );
    token_metadata.insert("from_address_hex".to_string(), hex::encode(from));
    token_metadata.insert("to_address_hex".to_string(), hex::encode(to));
    token_metadata.insert(
        "image_data_size".to_string(),
        nft_data.collectible_image_data.len().to_string(),
    );

    TokenValidationResult {
        is_valid: validation_errors.is_empty(),
        validation_errors,
        token_metadata,
    }
}

// Step 2: Initiate Transfer (Execute Transaction)
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

// Step 3: Generate ZK Proof Metadata
fn generate_zk_proof_metadata(proof: &SP1ProofWithPublicValues) -> ZKProofMetadata {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};

    // Serialize the proof to get its bytes
    let proof_bytes = bincode::serialize(proof).unwrap_or_else(|_| vec![]);
    let proof_size = proof_bytes.len();

    // Generate hash of the actual proof
    let mut hasher = DefaultHasher::new();
    proof_bytes.hash(&mut hasher);
    let proof_hash = format!("proof_{:x}", hasher.finish());

    // Hash the verification key (we'd need to pass this in real implementation)
    let mut vk_hasher = DefaultHasher::new();
    "verification_key".hash(&mut vk_hasher);
    let vk_hash = format!("vk_{:x}", vk_hasher.finish());

    // Extract public inputs from the proof's public values
    let public_values = &proof.public_values;
    let public_inputs = vec![
        format!("public_values_size_{}", public_values.as_slice().len()),
        format!(
            "public_values_hash_{}",
            hex::encode(&public_values.as_slice()[..8.min(public_values.as_slice().len())])
        ),
    ];

    ZKProofMetadata {
        proof_hash,
        verification_key_hash: vk_hash,
        public_inputs,
        proof_size,
        generation_time: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
    }
}

// Step 4: Save Transaction Log to Database
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
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
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

// Step 5: Update Global State AST (SMT) - Mocked
fn update_global_state_smt(
    nft_data: &MyNFTTokenData,
    new_owner: Address,
    proof_metadata: &ZKProofMetadata,
) -> bool {
    // In a real implementation, this would:
    // 1. Update the Sparse Merkle Tree (SMT) with new state
    // 2. Compute new state root
    // 3. Update the global state with transaction receipt key-value pairs
    // 4. Persist the updated state tree

    // Mock implementation: simulate state update
    let state_update = format!(
        "{{\"operation\":\"state_update\",\"token_id\":\"{}\",\"new_owner\":\"{}\",\"proof_hash\":\"{}\",\"timestamp\":{},\"state_root\":\"mock_state_root_{}\"}}",
        hex::encode(nft_data.token_id),
        hex::encode(new_owner),
        proof_metadata.proof_hash,
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        hex::encode(&new_owner[..4]) // Mock state root based on owner
    );

    if let Ok(mut file) = File::create("global_state_smt.json") {
        let _ = file.write_all(state_update.as_bytes());
        println!("Global state SMT updated and saved to global_state_smt.json");
        return true;
    }

    false
}

// Step 6: Save Proof to Public Ledger
fn save_proof_to_ledger(proof_hash: &str) -> LedgerEntry {
    use std::time::{SystemTime, UNIX_EPOCH};

    // Mock ledger entry (in real implementation, this would interact with actual blockchain)
    let block_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        % 1000000;
    let transaction_index = 1; // Mock index

    let ledger_entry = LedgerEntry {
        block_id,
        transaction_index,
        proof_hash: proof_hash.to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
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

// Step 6 (continued): Update Transaction Log with Ledger Metadata
fn update_transaction_log_with_ledger(
    transaction_id: &str,
    ledger_entry: &LedgerEntry,
) -> TransactionLog {
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

// Helper function to create sample input data
fn create_mint_transaction() -> (SP1Stdin, MyNFTTokenData) {
    // Setup admin and recipient addresses
    let admin_address: Address = [1u8; 32];
    let recipient: Address = [2u8; 32];

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

    // Prepare input for SP1 program (mint operation)
    let token_name = [84u8; 32]; // "T" repeated as token identifier
    let mint_input_data = to_vec(&nft_data).expect("Failed to serialize NFT data");

    let token_input = TransactionInput {
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

    (stdin, nft_data)
}

// Helper function to save proof to file
fn save_proof_to_file(proof: &SP1ProofWithPublicValues, filename: &str) {
    use std::fs::File;
    use std::io::Write;

    // Serialize the proof using bincode
    if let Ok(proof_bytes) = bincode::serialize(proof) {
        let proof_size = proof_bytes.len();
        if let Ok(mut file) = File::create(filename) {
            if file.write_all(&proof_bytes).is_ok() {
                println!("Proof (size: {} bytes) saved to {}", proof_size, filename);
            } else {
                println!("Failed to write proof data to {}", filename);
            }
        } else {
            println!("Failed to create file {}", filename);
        }
    } else {
        println!("Failed to serialize proof");
    }
}
