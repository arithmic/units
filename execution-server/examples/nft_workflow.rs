//! Workflow Layer - 7-Step Transaction Processing Demo
//!
//! This example demonstrates the UNITS Core Workflow Layer implementation,
//! showing both the core workflow functions and how they're used in NFT transactions.

use std::time::{SystemTime, UNIX_EPOCH};

use borsh::to_vec;
use execution_engine::{
    common::{TransactionInput, TransactionOutput},
    types::{Address, KeyValue},
};
use sp1_sdk::{EnvProver, SP1Stdin};
use tokens::{MintInput, MyNFTTokenData, TransferInput};
use execution_server::{
    zk_proof::generate_zk_proof,
    save_transaction_log_to_database,
    commit_global_state,
    submit_proof_to_public_ledger,
    GlobalStateSMT,
};

fn main() {
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
        unique_identifier: [123u8; 32],
        collectible_hash: [255u8; 32],
        owner_id: [2u8; 32],
        collectible_image_data: vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE],
    };
    let from_address: Address = nft_token.owner_id;
    let to_address: Address = [3u8; 32];

    // Create an instance of the global state SMT
    let mut global_state_smt = GlobalStateSMT::default();
    println!("🚀 Initial SMT root: {:?}", global_state_smt.root());

    println!("NFT Transfer: {} -> {}", hex::encode(&from_address[..4]), hex::encode(&to_address[..4]));

    if let Err(e) = execute_nft_flow(&client, &elf, &mut nft_token, from_address, to_address, &mut global_state_smt) {
        println!("❌ Failed: {}", e);
        return;
    }

    println!("✅ Workflow Layer Demo Complete");
}

fn execute_nft_flow(
    client: &EnvProver,
    elf: &[u8],
    token: &mut MyNFTTokenData,
    from: Address,
    to: Address,
    smt: &mut GlobalStateSMT,
) -> Result<(), String> {

    // Step 1: Mint NFT
    println!("1. Minting NFT...");
    let admin_address = [1u8; 32];
    let mint_input = MintInput {
        unique_identifier: token.unique_identifier,
        collectible_hash: token.collectible_hash,
        owner_id: from,
        collectible_image_data: token.collectible_image_data.clone(),
    };
    let mint_transaction_input = TransactionInput {
        token_name: "NFT".to_string(),
        function_name: "mint".to_string(),
        signer: admin_address,
        pre_state: vec![],
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        block_id: 1,
        transaction_hash: [0u8; 32],
        token_id: "NFTToken".to_string(),
        nonce: 0,
        input_data: to_vec(&mint_input).expect("Failed to serialize mint input"),
    };

    let mint_writes = execute_transaction(client, elf, &mint_transaction_input)?;
    println!("   ✅ Minted");

    // Step 2: Execute transfer
    println!("2. Executing transfer...");
    let transfer_input = TransferInput {
        unique_identifier: token.unique_identifier,
        new_owner: to,
    };
    let transfer_transaction_input = TransactionInput {
        token_name: "NFT".to_string(),
        function_name: "transfer".to_string(),
        signer: from,
        pre_state: mint_writes,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        block_id: 1,
        transaction_hash: [0u8; 32],
        token_id: "NFTToken".to_string(),
        nonce: 0,
        input_data: to_vec(&transfer_input).expect("Failed to serialize transfer input"),
    };

    let transfer_writes = execute_transaction(client, elf, &transfer_transaction_input)?;
    println!("   ✅ Executed");

    // Step 3: Generate ZK proof
    println!("3. Generating ZK proof...");
    let proof_result = generate_zk_proof(client, elf, &transfer_transaction_input)?;
    println!("   ✅ Proof generated ({} bytes)", proof_result.metadata.proof_size);

    // Step 4: Save to database
    println!("4. Saving to database...");
    save_transaction_log_to_database(&transfer_writes)?;
    println!("   ✅ Saved");

    // Step 5: Commit state
    println!("5. Committing state...");
    commit_global_state(smt, &transfer_writes)?;
    token.owner_id = to;
    println!("   ✅ Committed");

    // Step 6: Submit to ledger
    println!("6. Submitting to ledger...");
    submit_proof_to_public_ledger(&proof_result)?;
    println!("   ✅ Submitted");

    println!("✅ Transfer complete: {}", proof_result.metadata.proof_hash);

    Ok(())
}

fn execute_transaction(
    client: &EnvProver,
    elf: &[u8],
    transaction_input: &TransactionInput,
) -> Result<Vec<KeyValue>, String> {
    let input_bytes = borsh::to_vec(transaction_input)
        .map_err(|e| format!("Failed to serialize input: {}", e))?;

    let mut stdin = SP1Stdin::new();
    stdin.write(&input_bytes);

    let (output, _report) = client
        .execute(elf, &stdin)
        .run()
        .map_err(|e| format!("Failed to execute transaction: {}", e))?;

    let output: TransactionOutput = bincode::deserialize(&output.as_slice())
        .map_err(|e| format!("Failed to deserialize output: {}", e))?;

    if !output.success {
        return Err(format!(
            "Transaction failed: {}",
            output.error.unwrap_or_default()
        ));
    }

    Ok(output.receipt.unwrap().writes)
}

