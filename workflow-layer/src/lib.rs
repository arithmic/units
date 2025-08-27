//! Workflow Layer - UNITS Core Architecture
//!
//! This crate implements the Workflow Layer component of the UNITS Core design.
//! The Workflow Layer orchestrates transaction processing between the Application Layer and Kernel Modules.
//!
//! ## UNITS Core Architecture
//!
//! The Workflow Layer sits between the Application Layer and Kernel Modules:
//! - **Application Layer** → submits Instructions with JWT + signatures  
//! - **Workflow Layer** → handles identity/policy, planning, async execution, and receipts
//! - **Kernel Modules** → perform stateless verification and return MutationPlans
//!
//! ## Workflow Layer Responsibilities
//!
//! - **Identity & Policy**: JWT/OIDC validation, ACL/ABAC evaluation
//! - **Planning**: Build readset/writeset, prefetch data for kernel execution  
//! - **Async Executor**: Submit kernel steps, apply MutationPlans under locks
//! - **Receipts/Audit**: Persist TransactionReceipts with policy snapshots
//! - **State Management**: CAS operations with per-key versioning and WAL

pub mod types;
pub mod zk_proof;

use execution_engine::types::KeyValue;
use zk_proof::ProofResult;

// Re-export types for convenience
pub use tokens::{AadhaarToken, MyNFTTokenData};
pub use types::{LedgerEntry, TokenValidationResult, TransactionLog, ZKProofMetadata};
pub use zk_proof::{execute_transaction, generate_zk_proof, save_proof_to_file};

/// Save transaction log and proof to database
/// Accepts the output of execute_transaction for database storage
pub fn save_transaction_log_to_database(transfer_output: &Vec<KeyValue>) -> Result<(), String> {
    // TODO: Implement database save functionality
    println!("     Saving {} state changes to database", transfer_output.len());
    Ok(())
}

/// Commit global state updates
/// Accepts the output of execute_transaction to save global state
pub fn commit_global_state(transfer_output: &Vec<KeyValue>) -> Result<(), String> {
    // TODO: Implement global state commit functionality  
    println!("     Committing {} state updates", transfer_output.len());
    Ok(())
}

/// Submit ZK proof to public ledger/blockchain
/// Accepts ProofResult to publish to the public ledger
pub fn submit_proof_to_public_ledger(proof_result: &ProofResult) -> Result<(), String> {
    // TODO: Implement public ledger submission functionality
    println!("     Submitting proof {} to ledger", &proof_result.metadata.proof_hash[..12]);
    Ok(())
}
