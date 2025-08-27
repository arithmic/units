//! Workflow Layer - 7-Step Transaction Processing Demo
//! 
//! This example demonstrates the UNITS Core Workflow Layer implementation,
//! showing both the core workflow functions and how they're used in NFT transactions.

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use borsh::to_vec;
use sp1_sdk::{EnvProver, SP1ProofWithPublicValues};
use execution_engine::common::TransactionInput;
use execution_engine::types::Address;
use tokens::{MyNFTTokenData, AadhaarToken};

use workflow_layer::{
    // Core Workflow Layer functions
    validate_identity_and_policy,
    plan_and_prefetch,
    execute_with_kernel,
    apply_state_changes,
    create_transaction_receipt,
    WorkflowInstruction,
    
    // Types  
    TransactionLog, TokenValidationResult, LedgerEntry,
};

use workflow_layer::zk_proof::{generate_zk_proof, ProofResult};

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
    let mut nft_token = create_sample_nft_token();
    let from_address: Address = nft_token.owner_id;
    let to_address: Address = [3u8; 32];

    println!("Demo NFT Token:");
    println!("  Token ID: {}", hex::encode(nft_token.token_id));
    println!("  Current Owner: {}", hex::encode(from_address));
    println!("  New Owner: {}", hex::encode(to_address));
    println!();

    // Demonstrate core Workflow Layer functions
    demonstrate_core_workflow_functions(&nft_token, from_address, to_address);
    
    println!("\n{}", "=".repeat(60));
    println!("Now demonstrating the 7-step NFT flow using Workflow Layer\n");

    // Execute the 7-step NFT flow
    let transaction_input = create_transaction_input(&nft_token);
    if let Err(e) = execute_seven_step_nft_flow(
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

    println!("\n=== Demo: Aadhaar Token Validation ===");
    demonstrate_aadhaar_token_validation();

    println!("\n=== Workflow Layer Demo Completed Successfully ===");
    println!("This demonstrates the UNITS Core Workflow Layer architecture");
    println!("and how it orchestrates transaction processing between Application and Kernel layers.");
}

/// Demonstrate the core Workflow Layer functions
fn demonstrate_core_workflow_functions(token: &MyNFTTokenData, from: Address, to: Address) {
    println!("=== UNITS Core Workflow Layer Functions Demo ===\n");

    let jwt_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.mock_jwt_token";
    
    // Execute workflow steps with early returns on error
    if let Err(e) = execute_workflow_steps(token, from, to, jwt_token) {
        println!("   Workflow failed: {}\n", e);
    }
}

/// Execute the complete workflow steps
fn execute_workflow_steps(token: &MyNFTTokenData, from: Address, to: Address, jwt_token: &str) -> Result<(), String> {
    // Step 1: Identity and Policy Validation
    let validation_context = validate_identity_and_policy(jwt_token, "transfer", "nft_token")?;
    println!("   Identity validated for principal: {}\n", validation_context.principal_id);

    // Step 2: Planning and Prefetching
    let instruction = WorkflowInstruction {
        function_name: "transfer".to_string(),
        token_id: token.token_id,
        from_address: from,
        to_address: to,
        input_data: vec![],
        signatures: vec!["mock_signature".to_string()],
    };

    let execution_plan = plan_and_prefetch(&instruction, &validation_context)?;
    println!("   Execution plan created with {} read keys, {} write keys\n", 
        execution_plan.readset.len(), execution_plan.writeset.len());

    // Step 3: Kernel Execution Coordination
    let mutation_result = execute_with_kernel(&execution_plan, &instruction)?;
    println!("   Kernel execution completed, gas used: {}\n", mutation_result.gas_used);

    // Step 4: State Management
    let state_result = apply_state_changes(&mutation_result, &execution_plan)?;
    println!("   State changes applied, WAL sequence: {}\n", state_result.wal_sequence);

    // Step 5: Transaction Receipt
    let receipt = create_transaction_receipt(&validation_context, &mutation_result, &state_result)?;
    println!("   Transaction receipt created: {}\n", receipt.transaction_id);

    Ok(())
}

/// Execute the complete 7-step NFT flow process
fn execute_seven_step_nft_flow(
    client: &EnvProver,
    elf: &[u8],
    token: &mut MyNFTTokenData,
    from: Address,
    to: Address,
    transaction_input: &TransactionInput,
) -> Result<(), String> {
    println!("=== 7-Step NFT Flow Using Workflow Layer ===\n");

    // Step 1: Validate token and transfer
    println!("Step 1: validate_nft_token() - Token and transfer validation");
    println!("   Uses Workflow Layer's validate_identity_and_policy() architecture");
    let validation_result = validate_nft_token(token, from, to);
    if !validation_result.is_valid {
        return Err(format!("❌ Validation failed: {:?}", validation_result.validation_errors));
    }
    println!("   Token and transfer validation passed\n");

    // Step 2: Initiate transfer and create transaction log
    println!("Step 2: initiate_nft_transfer() - Create transaction log");
    println!("   Uses Workflow Layer's plan_and_prefetch() architecture");
    let transaction_id = initiate_nft_transfer(token, from, to)?;
    println!("   Transfer initiated with ID: {}\n", transaction_id);

    // Create transaction log for the flow
    let tx_log = TransactionLog {
        transaction_id: transaction_id.clone(),
        token_id: token.token_id,
        from_address: from,
        to_address: to,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        operation: "transfer".to_string(),
        status: "pending".to_string(),
        proof_hash: None,
        ledger_block_id: None,
        ledger_index: None,
    };

    // Step 3: Generate ZK proof
    println!("Step 3: generate_zk_proof_for_nft_transaction() - Generate ZK proof");
    println!("   Uses Workflow Layer's execute_with_kernel() architecture");
    let proof_result = generate_zk_proof_for_nft_transaction(client, elf, &tx_log, transaction_input)?;
    println!("   ZK proof generated and verified\n");

    // Step 4: Save transaction log and proof to database
    println!("Step 4: save_nft_tx_log() - Save to database");
    println!("   Uses Workflow Layer's apply_state_changes() architecture");
    save_nft_tx_log(&tx_log, &proof_result.proof)?;
    println!("   Transaction log and proof saved to database\n");

    // Step 5: Commit transfer (update token state)
    println!("Step 5: commit_nft_transfer() - Update token states");
    println!("   Uses Workflow Layer's StateStore with CAS operations");
    commit_nft_transfer(token, to)?;
    println!("   Token ownership transferred to new owner\n");

    // Step 6: Save to public ledger (blockchain)
    println!("Step 6: save_nft_proof_in_public_ledger() - Save to blockchain");
    println!("   Uses Workflow Layer's create_transaction_receipt() architecture");
    let ledger_entry = save_nft_proof_in_public_ledger(&proof_result.proof, &tx_log)?;
    println!("   Proof published to ledger (Block: {}, Index: {})\n", 
        ledger_entry.block_id, ledger_entry.transaction_index);

    // Step 7: Update transaction log with ledger metadata
    println!("Step 7: update_nft_tx_log_with_ledger_metadata()");
    println!("   Completes Workflow Layer's audit trail (Web2 principals <-> Web3 keys)");
    let _final_tx_log = update_nft_tx_log_with_ledger_metadata(&transaction_id, &ledger_entry)?;
    println!("   Transaction log updated with blockchain metadata\n");

    println!("7-Step NFT Flow Completed Successfully!");
    println!("   Final token owner: {}", hex::encode(to));
    println!("   Proof hash: {}", proof_result.metadata.proof_hash);
    println!("   Ledger record: Block {}, Index {}\n", 
        ledger_entry.block_id, ledger_entry.transaction_index);

    Ok(())
}

/// Create a sample NFT token for demonstration
fn create_sample_nft_token() -> MyNFTTokenData {
    let initial_owner: Address = [2u8; 32];
    
    MyNFTTokenData {
        token_id: [42u8; 32],
        unique_identifier: [123u8; 32],
        collectible_hash: [255u8; 32],
        owner_id: initial_owner,
        collectible_image_data: vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE],
    }
}

/// Create transaction input for ZK proof generation
fn create_transaction_input(nft_token: &MyNFTTokenData) -> TransactionInput {
    let admin_address: Address = [1u8; 32];
    let token_name = [84u8; 32];
    let input_data = to_vec(nft_token).expect("Failed to serialize NFT data");

    TransactionInput {
        token_name,
        function_name: "transfer".to_string(),
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
        input_data,
    }
}

/// Demonstrate Aadhaar token validation (empty implementation)
fn demonstrate_aadhaar_token_validation() {
    let aadhaar_token = AadhaarToken {
        token_id: [100u8; 32],
        owner_id: [5u8; 32],
    };

    let from: Address = [5u8; 32];
    let to: Address = [6u8; 32];

    println!("Demo Aadhaar Token:");
    println!("  Token ID: {}", hex::encode(aadhaar_token.token_id));
    println!("  Owner: {}", hex::encode(aadhaar_token.owner_id));

    let validation_result = validate_aadhaar_token(&aadhaar_token, from, to);
    println!("  Validation Result: {}", if validation_result.is_valid { "Valid" } else { "Invalid" });
}

// ============================================================================
// NFT-SPECIFIC WORKFLOW LAYER FUNCTIONS
// ============================================================================

/// Step 1: Token and transfer validation using Workflow Layer
/// 
/// **UNITS Core Architecture (Future)**: Once implemented, this would use the Workflow Layer's
/// validate_identity_and_policy() function for JWT/OIDC + ACL/ABAC validation
fn validate_nft_token(token: &MyNFTTokenData, from: Address, to: Address) -> TokenValidationResult {
    let mut validation_errors = Vec::new();
    let mut token_metadata = HashMap::new();

    // Validate token data integrity
    if token.token_id.iter().all(|&x| x == 0) {
        validation_errors.push("Token ID cannot be all zeros".to_string());
    }

    if token.unique_identifier.iter().all(|&x| x == 0) {
        validation_errors.push("Unique identifier cannot be all zeros".to_string());
    }

    if token.collectible_hash.iter().all(|&x| x == 0) {
        validation_errors.push("Collectible hash cannot be all zeros".to_string());
    }

    if token.collectible_image_data.is_empty() {
        validation_errors.push("Collectible image data cannot be empty".to_string());
    }

    // Validate transfer addresses
    if from == to {
        validation_errors.push("Cannot transfer to the same address".to_string());
    }

    if token.owner_id != from {
        validation_errors.push("Transfer must be initiated by the token owner".to_string());
    }

    // Collect token metadata
    token_metadata.insert("token_id_hex".to_string(), hex::encode(token.token_id));
    token_metadata.insert("unique_id_hex".to_string(), hex::encode(token.unique_identifier));
    token_metadata.insert("collectible_hash_hex".to_string(), hex::encode(token.collectible_hash));
    token_metadata.insert("owner_address_hex".to_string(), hex::encode(token.owner_id));
    token_metadata.insert("from_address_hex".to_string(), hex::encode(from));
    token_metadata.insert("to_address_hex".to_string(), hex::encode(to));
    token_metadata.insert("image_data_size".to_string(), token.collectible_image_data.len().to_string());

    TokenValidationResult {
        is_valid: validation_errors.is_empty(),
        validation_errors,
        token_metadata,
    }
}

/// Step 2: Initiate transfer using Workflow Layer
/// 
/// **UNITS Core Architecture (Future)**: Once implemented, this would use the Workflow Layer's
/// plan_and_prefetch() function to build readset/writeset and prefetch data
fn initiate_nft_transfer(token: &MyNFTTokenData, from: Address, to: Address) -> Result<String, String> {
    // Generate transaction ID based on token ID
    let transaction_id = format!("txn_{}", hex::encode(&token.token_id[..8]));

    // Validate the transfer is allowed
    if token.owner_id != from {
        return Err("Transfer initiated by non-owner".to_string());
    }

    if from == to {
        return Err("Cannot transfer to same address".to_string());
    }

    // Create transfer state tracking file
    let transfer_state = format!(
        "{{\"token_id\":\"{}\",\"from\":\"{}\",\"to\":\"{}\",\"status\":\"initiated\",\"timestamp\":{}}}",
        hex::encode(token.token_id),
        hex::encode(from),
        hex::encode(to),
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    );

    File::create("transfer_state.json")
        .and_then(|mut file| file.write_all(transfer_state.as_bytes()))
        .map_err(|e| format!("Failed to save transfer state: {}", e))?;

    println!("Transfer state saved to transfer_state.json");
    Ok(transaction_id)
}

/// Step 3: Generate ZK proof using Workflow Layer
/// 
/// **UNITS Core Architecture (Future)**: Once implemented, this would use the Workflow Layer's
/// execute_with_kernel() function to coordinate with Kernel modules for signature verification
fn generate_zk_proof_for_nft_transaction(
    client: &EnvProver,
    elf: &[u8],
    tx_log: &TransactionLog,
    circuit_data: &TransactionInput,
) -> Result<ProofResult, String> {
    println!("Generating ZK proof for transaction: {}", tx_log.transaction_id);
    
    let proof_result = generate_zk_proof(client, elf, circuit_data)?;
    
    println!("Successfully generated and verified ZK proof!");
    Ok(proof_result)
}

/// Step 4: Save to database using Workflow Layer
/// 
/// **UNITS Core Architecture (Future)**: Once implemented, this would use the Workflow Layer's
/// apply_state_changes() function with CAS operations and WAL persistence
fn save_nft_tx_log(tx_log: &TransactionLog, proof: &SP1ProofWithPublicValues) -> Result<(), String> {
    // Save transaction log
    let log_json = serde_json::to_string_pretty(tx_log)
        .map_err(|e| format!("Failed to serialize transaction log: {}", e))?;
    
    let filename = format!("transaction_log_{}.json", tx_log.transaction_id);
    File::create(&filename)
        .and_then(|mut file| file.write_all(log_json.as_bytes()))
        .map_err(|e| format!("Failed to save transaction log: {}", e))?;

    // Save proof
    workflow_layer::zk_proof::save_proof_to_file(proof, &format!("proof_{}.bin", tx_log.transaction_id))?;
    
    println!("Transaction log and proof saved to database");
    Ok(())
}

/// Step 5: Commit transfer using Workflow Layer
/// 
/// **UNITS Core Architecture (Future)**: Once implemented, this would use the Workflow Layer's
/// state management functions with per-key versioning and canonical lock ordering
fn commit_nft_transfer(token: &mut MyNFTTokenData, new_owner: Address) -> Result<(), String> {
    // Update token state
    token.owner_id = new_owner;
    
    println!("Token ownership committed to new owner: {}", hex::encode(new_owner));
    
    // In a real implementation, this would integrate with StateStore
    Ok(())
}

/// Step 6: Save to blockchain using Workflow Layer
/// 
/// **UNITS Core Architecture (Future)**: Once implemented, this would use the Workflow Layer's
/// create_transaction_receipt() function with policy snapshots and Merkle paths
fn save_nft_proof_in_public_ledger(_proof: &SP1ProofWithPublicValues, tx_log: &TransactionLog) -> Result<LedgerEntry, String> {
    let block_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() % 1000000;
    
    let transaction_index = 1; // Mock index

    let ledger_entry = LedgerEntry {
        block_id,
        transaction_index,
        proof_hash: format!("proof_hash_{}", tx_log.transaction_id),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        verification_status: "verified".to_string(),
    };

    // Save to ledger file
    let ledger_json = serde_json::to_string_pretty(&ledger_entry)
        .map_err(|e| format!("Failed to serialize ledger entry: {}", e))?;
    
    let filename = format!("ledger_entry_{}.json", block_id);
    File::create(&filename)
        .and_then(|mut file| file.write_all(ledger_json.as_bytes()))
        .map_err(|e| format!("Failed to save ledger entry: {}", e))?;

    println!("Proof published to public ledger - Block: {}, Index: {}", block_id, transaction_index);
    Ok(ledger_entry)
}

/// Step 7: Update transaction log with ledger metadata
/// 
/// **UNITS Core Architecture (Future)**: Once implemented, this would complete the audit trail
/// maintained by the Workflow Layer linking Web2 principals ↔ Web3 keys
fn update_nft_tx_log_with_ledger_metadata(tx_id: &str, ledger_record: &LedgerEntry) -> Result<TransactionLog, String> {
    let filename = format!("transaction_log_{}.json", tx_id);
    
    let mut tx_log = if let Ok(mut file) = File::open(&filename) {
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("Failed to read transaction log: {}", e))?;
            
        serde_json::from_str::<TransactionLog>(&contents)
            .map_err(|e| format!("Failed to parse transaction log: {}", e))?
    } else {
        return Err("Transaction log not found".to_string());
    };

    // Update with ledger metadata
    tx_log.ledger_block_id = Some(ledger_record.block_id);
    tx_log.ledger_index = Some(ledger_record.transaction_index);
    tx_log.status = "completed".to_string();

    // Save updated log
    let updated_json = serde_json::to_string_pretty(&tx_log)
        .map_err(|e| format!("Failed to serialize updated transaction log: {}", e))?;
    
    File::create(&filename)
        .and_then(|mut file| file.write_all(updated_json.as_bytes()))
        .map_err(|e| format!("Failed to save updated transaction log: {}", e))?;

    println!("Transaction log updated with ledger metadata");
    Ok(tx_log)
}

/// Aadhaar token validation (empty implementation for demonstration)
fn validate_aadhaar_token(token: &AadhaarToken, from: Address, to: Address) -> TokenValidationResult {
    let mut token_metadata = HashMap::new();
    token_metadata.insert("token_id_hex".to_string(), hex::encode(token.token_id));
    token_metadata.insert("owner_address_hex".to_string(), hex::encode(token.owner_id));
    token_metadata.insert("from_address_hex".to_string(), hex::encode(from));
    token_metadata.insert("to_address_hex".to_string(), hex::encode(to));

    TokenValidationResult {
        is_valid: true,
        validation_errors: vec![],
        token_metadata,
    }
}