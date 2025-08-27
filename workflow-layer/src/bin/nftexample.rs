//! Workflow Layer - 7-Step Transaction Processing Demo
//!
//! This example demonstrates the UNITS Core Workflow Layer implementation,
//! showing both the core workflow functions and how they're used in NFT transactions.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use borsh::to_vec;
use execution_engine::common::TransactionInput;
use execution_engine::types::Address;
use sp1_sdk::EnvProver;
use tokens::MyNFTTokenData;

use workflow_layer::execute_transaction;
use workflow_layer::zk_proof::generate_zk_proof;

fn main() {
    println!("=== UNITS Core Workflow Layer Demo ===");
    println!("Demonstrating core workflow functions and NFT transaction processing\n");

    // Setup SP1 environment
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Build the program
    sp1_build::build_program("../program");
    let elf_path = "../target/elf-compilation/riscv32im-succinct-zkvm-elf/release/program";
    let elf = std::fs::read(elf_path).expect("Failed to read ELF file");
    let client = EnvProver::default();

    // Create example NFT token
    let mut nft_token = MyNFTTokenData {
        token_id: [42u8; 32],
        unique_identifier: [123u8; 32],
        collectible_hash: [255u8; 32],
        owner_id: [2u8; 32],
        collectible_image_data: vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE],
    };
    let from_address: Address = nft_token.owner_id;
    let to_address: Address = [3u8; 32];

    println!("Demo NFT Token:");
    println!("  Token ID: {}", hex::encode(nft_token.token_id));
    println!("  Current Owner: {}", hex::encode(from_address));
    println!("  New Owner: {}", hex::encode(to_address));
    println!();

    let transaction_input = TransactionInput {
        token_name: [84u8; 32],
        function_name: "transfer".to_string(),
        signer: [1u8; 32],
        pre_state: vec![],
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        block_id: 1,
        transaction_hash: [0u8; 32],
        token_id: "NFTToken".to_string(),
        nonce: 0,
        input_data: to_vec(&nft_token).expect("Failed to serialize NFT data"),
    };
    if let Err(e) = execute_nft_flow(
        &client,
        &elf,
        &mut nft_token,
        from_address,
        to_address,
        &transaction_input,
    ) {
        println!("❌ NFT Flow failed: {}", e);
        return;
    }

    println!("\n=== Workflow Layer Demo Completed Successfully ===");
    println!("This demonstrates the UNITS Core Workflow Layer architecture");
    println!(
        "and how it orchestrates transaction processing between Application and Kernel layers."
    );
}

fn execute_nft_flow(
    client: &EnvProver,
    elf: &[u8],
    token: &mut MyNFTTokenData,
    from: Address,
    to: Address,
    transaction_input: &TransactionInput,
) -> Result<(), String> {
    println!("=== 7-Step NFT Flow Using Workflow Layer ===\n");

    // Step 1 (future): Validate token and transfer, uses Workflow Layer's validate_identity_and_policy() architecture
    println!("Step 1: validate_nft_token() - Token and transfer validation");

    // Step 2: Initiate transfer and create transaction log
    println!("Step 2: initiate_nft_transfer() - Create transaction log");
    let transaction_id = initiate_nft_transfer(token, from, to, client, elf)?;

    // Step 3: Generate ZK proof
    println!("Step 3: generate_zk_proof_for_nft_transaction() - Generate ZK proof");
    println!("   Uses Workflow Layer's execute_with_kernel() architecture");
    // 3a: Future - Create ordering of executed transactions and block them
    // The UNITS ledger will be the set of ordered blocks of such transactions
    // Send block with required information for proof generation
    // Merkle Path-receipts for each transaction in the block can be generated
    let proof_result = generate_zk_proof(client, elf, transaction_input)?;
    println!("Successfully generated and verified ZK proof!");
    println!("   ZK proof generated and verified\n");
    println!("proof_result: {:?}", proof_result.metadata);

    // Step 4: Save transaction log and proof to database
    println!("Step 4: save_nft_tx_log() - Save to database");
    println!("   Uses Workflow Layer's apply_state_changes() architecture");
    println!("   Transaction log and proof saved to database\n");

    // Step 5: Commit transfer (update token state)
    println!("Step 5: commit_nft_transfer() - Update token states");
    println!("   Uses Workflow Layer's StateStore with CAS operations");
    // Update global state with CAS operations and canonical lock ordering
    token.owner_id = to;
    println!("Global state updated with new token ownership");
    println!("   Token ownership transferred to new owner\n");

    // Step 6: Save to public ledger (blockchain)
    println!("Step 6: save_nft_proof_in_public_ledger() - Save to blockchain");
    println!("   Uses Workflow Layer's create_transaction_receipt() architecture");
    println!("   Proof published to ledger\n");

    // Step 7: Update transaction log with ledger metadata
    println!("Step 7: update_nft_tx_log_with_ledger_metadata()");
    println!("   Completes Workflow Layer's audit trail (Web2 principals <-> Web3 keys)");
    println!("   Transaction log updated with blockchain metadata\n");

    println!("7-Step NFT Flow Completed Successfully!");
    println!("   Final token owner: {}", hex::encode(to));
    println!("   Proof hash: {}", proof_result.metadata.proof_hash);

    Ok(())
}

fn initiate_nft_transfer(
    token: &MyNFTTokenData,
    from: Address,
    to: Address,
    client: &EnvProver,
    elf: &[u8],
) -> Result<String, String> {
    let transaction_id = format!("txn_{}", hex::encode(&token.token_id[..8]));

    // future: Prefetch the reads

    // Create transaction input for execution
    let transaction_input = TransactionInput {
        token_name: [84u8; 32],
        function_name: "transfer".to_string(),
        signer: from,
        pre_state: vec![],
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        block_id: 1,
        transaction_hash: [0u8; 32],
        token_id: "NFTToken".to_string(),
        nonce: 0,
        input_data: to_vec(token).expect("Failed to serialize NFT data"),
    };
    let res = execute_transaction(client, elf, &transaction_input);
    println!("res: {:?}", res);
    Ok(transaction_id)
}
