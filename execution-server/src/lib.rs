pub mod types;
pub mod zk_proof;

use execution_engine::types::KeyValue;
use sparse_merkle_tree::{
    default_store::DefaultStore, sha256::Sha256Hasher, SparseMerkleTree, H256,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use zk_proof::ProofResult;

// Define the SMT type for our global state
pub type GlobalStateSMT = SparseMerkleTree<Sha256Hasher, H256, DefaultStore<H256>>;

// Re-export types for convenience
pub use tokens::{AadhaarToken, MyNFTTokenData};
pub use types::{LedgerEntry, TokenValidationResult, TransactionLog, ZKProofMetadata};
pub use zk_proof::{execute_transaction, generate_zk_proof, save_proof_to_file};

/// Save transaction log and proof to database
/// Accepts the output of execute_transaction for database storage
pub fn save_transaction_log_to_database(writes: &Vec<KeyValue>) -> Result<(), String> {
    // TODO: Implement database save functionality
    println!("Saving {} state changes to database", writes.len());
    Ok(())
}

/// Commit global state updates to the SMT
/// Accepts a mutable reference to the SMT and the key-value writes
pub fn commit_global_state(smt: &mut GlobalStateSMT, writes: &Vec<KeyValue>) -> Result<(), String> {
    println!("Committing {} state updates to SMT...", writes.len());

    // 1. Prepare key-value pairs for the SMT
    let smt_updates: Vec<(H256, H256)> = writes
        .iter()
        .map(|kv| {
            // The key is already a [u8; 32], which can be converted to H256
            let key = H256::from(kv.key);

            // The value can be of variable length, so we hash it to get a fixed-size H256 value
            let mut hasher = DefaultHasher::new();
            kv.value.hash(&mut hasher);
            let value_hash_u64 = hasher.finish();
            let mut value_bytes = [0u8; 32];
            value_bytes[..8].copy_from_slice(&value_hash_u64.to_le_bytes());
            let value = H256::from(value_bytes);

            (key, value)
        })
        .collect();

    // 2. Perform a batch update on the tree
    smt.update_all(smt_updates)
        .map_err(|e| format!("Failed to update SMT: {}", e))?;

    println!("✅ SMT commit complete. New root: {:?}", smt.root());
    Ok(())
}

/// Submit ZK proof to public ledger/blockchain
/// Accepts ProofResult to publish to the public ledger
pub fn submit_proof_to_public_ledger(proof_result: &ProofResult) -> Result<(), String> {
    // TODO: Implement public ledger submission functionality
    println!(
        "Submitting proof {} to ledger",
        &proof_result.metadata.proof_hash[..12]
    );
    Ok(())
}
