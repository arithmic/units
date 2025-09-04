//! NFT-specific types for the flow example
//! 
//! This module contains NFT transaction types that were moved from execution-engine/common.rs

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use execution_engine::types::Address;

/// Transaction log entry for tracking NFT operations
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionLog {
    pub transaction_id: String,
    pub token_id: [u8; 32],
    pub from_address: Address,
    pub to_address: Address,
    pub timestamp: u64,
    pub operation: String,
    pub status: String,
    pub proof_hash: Option<String>,
    pub ledger_block_id: Option<u64>,
    pub ledger_index: Option<u64>,
}

/// ZK Proof metadata for tracking proof information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZKProofMetadata {
    pub proof_hash: String,
    pub verification_key_hash: String,
    pub public_inputs: Vec<String>,
    pub proof_size: usize,
    pub generation_time: u64,
}

/// Ledger entry for blockchain storage
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LedgerEntry {
    pub block_id: u64,
    pub transaction_index: u64,
    pub proof_hash: String,
    pub timestamp: u64,
    pub verification_status: String,
}

/// Token validation result
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenValidationResult {
    pub is_valid: bool,
    pub validation_errors: Vec<String>,
    pub token_metadata: HashMap<String, String>,
}