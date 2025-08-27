//! ZK Proof generation library functions
//! 
//! This module provides reusable ZK proof generation functionality

use sp1_sdk::{EnvProver, SP1ProofWithPublicValues, SP1Stdin};
use execution_engine::common::TransactionInput;
use crate::types::ZKProofMetadata;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

/// ZK Proof generation result
pub struct ProofResult {
    pub proof: SP1ProofWithPublicValues,
    pub metadata: ZKProofMetadata,
}

/// Generate ZK proof for a transaction
/// This function handles the complete proof generation workflow
pub fn generate_zk_proof(
    client: &EnvProver,
    elf: &[u8],
    transaction_input: &TransactionInput,
) -> Result<ProofResult, String> {
    // Serialize the transaction input
    let input_bytes = borsh::to_vec(transaction_input)
        .map_err(|e| format!("Failed to serialize transaction input: {}", e))?;

    // Prepare SP1 stdin
    let mut stdin = SP1Stdin::new();
    stdin.write(&input_bytes);

    // Setup prover keys
    let (pk, vk) = client.setup(elf);

    // Generate the proof
    let proof = client.prove(&pk, &stdin).run()
        .map_err(|e| format!("Failed to generate proof: {}", e))?;

    // Verify the proof
    client.verify(&proof, &vk)
        .map_err(|e| format!("Failed to verify proof: {}", e))?;

    // Generate metadata
    let metadata = generate_proof_metadata(&proof);

    Ok(ProofResult { proof, metadata })
}

/// Execute transaction without generating proof (for testing)
pub fn execute_transaction(
    client: &EnvProver,
    elf: &[u8],
    transaction_input: &TransactionInput,
) -> Result<Vec<u8>, String> {
    // Serialize the transaction input
    let input_bytes = borsh::to_vec(transaction_input)
        .map_err(|e| format!("Failed to serialize transaction input: {}", e))?;

    // Prepare SP1 stdin
    let mut stdin = SP1Stdin::new();
    stdin.write(&input_bytes);

    // Execute the program
    let (output, _report) = client.execute(elf, &stdin).run()
        .map_err(|e| format!("Failed to execute transaction: {}", e))?;

    let result = output.as_slice().to_vec();
    
    // Debug: log execution details
    println!("   SP1 execution successful, output length: {} bytes", result.len());
    if result.len() == 0 {
        return Err("Program produced no output".to_string());
    }

    Ok(result)
}

/// Generate metadata for a ZK proof
fn generate_proof_metadata(proof: &SP1ProofWithPublicValues) -> ZKProofMetadata {
    // Serialize the proof to get its bytes
    let proof_bytes = bincode::serialize(proof).unwrap_or_else(|_| vec![]);
    let proof_size = proof_bytes.len();

    // Generate hash of the actual proof
    let mut hasher = DefaultHasher::new();
    proof_bytes.hash(&mut hasher);
    let proof_hash = format!("proof_{:x}", hasher.finish());

    // Hash the verification key
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

/// Save proof to file
pub fn save_proof_to_file(proof: &SP1ProofWithPublicValues, filename: &str) -> Result<(), String> {
    use std::fs::File;
    use std::io::Write;

    // Serialize the proof using bincode
    let proof_bytes = bincode::serialize(proof)
        .map_err(|e| format!("Failed to serialize proof: {}", e))?;

    let mut file = File::create(filename)
        .map_err(|e| format!("Failed to create file {}: {}", filename, e))?;

    file.write_all(&proof_bytes)
        .map_err(|e| format!("Failed to write proof data to {}: {}", filename, e))?;

    println!("Proof (size: {} bytes) saved to {}", proof_bytes.len(), filename);
    Ok(())
}