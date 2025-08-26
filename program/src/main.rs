#![no_main]

extern crate alloc;

sp1_zkvm::entrypoint!(main);

use alloc::{vec, vec::Vec, string::ToString};
use unitsdesign::nft_token::{NFTToken, MyNFTTokenData};
use unitsdesign::traits::TokenContract;
use unitsdesign::types::{Address, ExecutionContext, KeyValue, TransactionReceipt};

#[derive(Debug)]
struct TransactionLog {
    tx_id: [u8; 32],
    token_id: [u8; 32],
    from: Address,
    to: Address,
    timestamp: u64,
    receipt: TransactionReceipt,
}

#[derive(Debug)]
struct ZKProof {
    proof_data: Vec<u8>,
    public_inputs: Vec<u8>,
}

#[derive(Debug)]
struct LedgerRecord {
    block_id: u64,
    tx_index: u32,
    merkle_proof: Vec<u8>,
}

fn validate(token_data: &MyNFTTokenData) -> bool {
    !token_data.token_id.is_empty() && 
    !token_data.unique_identifier.is_empty() && 
    !token_data.collectible_hash.is_empty()
}

fn initiate_transfer(
    token: &MyNFTTokenData, 
    from: Address, 
    to: Address,
    receipt: TransactionReceipt,
    tx_hash: [u8; 32]
) -> TransactionLog {
    TransactionLog {
        tx_id: tx_hash,
        token_id: token.token_id,
        from,
        to,
        timestamp: 1638400000, // Current timestamp
        receipt,
    }
}

fn generate_zk_proof(tx_log: &TransactionLog) -> ZKProof {
    // Generate ZK proof from transaction log
    let mut proof_data = Vec::new();
    proof_data.extend_from_slice(&tx_log.tx_id);
    proof_data.extend_from_slice(&tx_log.token_id);
    proof_data.extend_from_slice(&tx_log.from);
    proof_data.extend_from_slice(&tx_log.to);
    proof_data.extend_from_slice(&tx_log.timestamp.to_le_bytes());
    
    ZKProof {
        proof_data: proof_data.clone(),
        public_inputs: proof_data,
    }
}

fn save_tx_log(tx_log: &TransactionLog, proof: &ZKProof) -> bool {
    // Simulate saving to database
    println!("Saving transaction log with ID: {:?}", tx_log.tx_id);
    println!("Proof data size: {}", proof.proof_data.len());
    true
}

fn commit_transfer(token_data: &mut MyNFTTokenData, new_owner: Address) {
    // Update token state
    token_data.owner_id = new_owner;
    println!("Token ownership committed to: {:?}", new_owner);
}

fn save_proof_in_public_ledger(proof: &ZKProof, tx_log: &TransactionLog) -> LedgerRecord {
    // Simulate blockchain write
    let record = LedgerRecord {
        block_id: 12345,
        tx_index: 1,
        merkle_proof: proof.proof_data.clone(),
    };
    
    println!("Saved proof to public ledger - Block: {}, Index: {}", 
             record.block_id, record.tx_index);
    record
}

fn update_tx_log_with_ledger_metadata(tx_id: [u8; 32], ledger_record: &LedgerRecord) {
    // Update transaction log with ledger metadata
    println!("Updated tx {:?} with ledger metadata - Block: {}, Index: {}", 
             tx_id, ledger_record.block_id, ledger_record.tx_index);
}

fn main() {
    // NFT Token Flow Implementation
    
    // Setup admin and recipient addresses
    let admin_address: Address = [1u8; 32];
    let recipient: Address = [2u8; 32];
    let new_owner: Address = [3u8; 32];
    
    // Initialize NFT token contract
    let nft_contract = NFTToken::new(admin_address);
    
    // Create mint input data
    let token_id = [42u8; 32];
    let unique_identifier = [123u8; 32];
    let collectible_hash = [255u8; 32];
    let image_data = vec![0xDE, 0xAD, 0xBE, 0xEF]; // Sample image data
    let image_len = image_data.len() as u32;
    
    let mut mint_input = Vec::new();
    mint_input.extend_from_slice(&token_id);
    mint_input.extend_from_slice(&unique_identifier);
    mint_input.extend_from_slice(&collectible_hash);
    mint_input.extend_from_slice(&recipient);
    mint_input.extend_from_slice(&image_len.to_le_bytes());
    mint_input.extend_from_slice(&image_data);
    
    println!("Mint input size: {}", mint_input.len());
    
    let pre_state: Vec<KeyValue> = vec![];
    let ctx = ExecutionContext {
        signer: admin_address,
        pre_state: &pre_state,
        timestamp: 1638400000,
        block_id: 1,
        transaction_hash: [0u8; 32],
        token_id: "NFTToken".to_string(),
        nonce: 0,
    };
    
    // Step 1: Execute mint operation
    println!("=== Step 1: Minting NFT ===");
    let mint_result = nft_contract.execute(&ctx, "mint", &mint_input);
    if let Err(e) = &mint_result {
        println!("Mint operation failed with error: {:?}", e);
    }
    assert!(mint_result.is_ok(), "Mint operation failed");
    let mint_receipt = mint_result.unwrap();
    
    // Create NFT data for validation
    let mut nft_data = MyNFTTokenData {
        token_id,
        unique_identifier,
        collectible_hash,
        owner_id: recipient,
        collectible_image_data: image_data,
    };
    
    // Step 2: Validate token
    println!("=== Step 2: Validating Token ===");
    let is_valid = validate(&nft_data);
    assert!(is_valid, "Token validation failed");
    println!("Token validation: PASSED");
    
    // Step 3: Initiate transfer
    println!("=== Step 3: Initiating Transfer ===");
    let tx_log = initiate_transfer(&nft_data, recipient, new_owner, mint_receipt, ctx.transaction_hash);
    println!("Transfer initiated from {:?} to {:?}", recipient, new_owner);
    
    // Step 4: Generate ZK Proof
    println!("=== Step 4: Generating ZK Proof ===");
    let zk_proof = generate_zk_proof(&tx_log);
    println!("ZK proof generated with {} bytes", zk_proof.proof_data.len());
    
    // Step 5: Save transaction log
    println!("=== Step 5: Saving Transaction Log ===");
    let save_success = save_tx_log(&tx_log, &zk_proof);
    assert!(save_success, "Failed to save transaction log");
    
    // Step 6: Commit transfer
    println!("=== Step 6: Committing Transfer ===");
    commit_transfer(&mut nft_data, new_owner);
    
    // Step 7: Save proof in public ledger
    println!("=== Step 7: Saving Proof to Public Ledger ===");
    let ledger_record = save_proof_in_public_ledger(&zk_proof, &tx_log);
    
    // Step 8: Update transaction log with ledger metadata
    println!("=== Step 8: Updating Transaction Log with Ledger Metadata ===");
    update_tx_log_with_ledger_metadata(tx_log.tx_id, &ledger_record);
    
    println!("=== NFT Transfer Flow Completed Successfully ===");
    println!("Final token owner: {:?}", nft_data.owner_id);
    println!("Ledger record: Block {}, Index {}", ledger_record.block_id, ledger_record.tx_index);
}
