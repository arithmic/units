//! Common types shared across the zk-proof system
//! 
//! This module contains transaction input/output types and other shared structures
//! used by both the SP1 program and host application.

extern crate alloc;

use alloc::{string::String, vec::Vec};
use crate::types::{Address, TransactionReceipt, KeyValue};
use borsh::{BorshSerialize, BorshDeserialize};

#[cfg(feature = "std")]
use {std::collections::HashMap, serde::{Serialize, Deserialize}};

#[cfg(not(feature = "std"))]
use serde::{Serialize, Deserialize};

/// Input data structure for token transactions
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInput {
    pub token_name: [u8; 32],
    pub function_name: String,
    pub signer: Address,
    pub pre_state: Vec<KeyValue>,
    pub timestamp: u64,
    pub block_id: u64,
    pub transaction_hash: [u8; 32],
    pub token_id: String,
    pub nonce: u64,
    pub input_data: Vec<u8>,
}

/// Output data structure for token transactions
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Serialize, Deserialize)]
pub struct TransactionOutput {
    pub success: bool,
    pub receipt: Option<TransactionReceipt>,
    pub error: Option<String>,
}

/// Transaction log entry for tracking NFT operations
#[cfg(feature = "std")]
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
#[cfg(feature = "std")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZKProofMetadata {
    pub proof_hash: String,
    pub verification_key_hash: String,
    pub public_inputs: Vec<String>,
    pub proof_size: usize,
    pub generation_time: u64,
}

/// Ledger entry for blockchain storage
#[cfg(feature = "std")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LedgerEntry {
    pub block_id: u64,
    pub transaction_index: u64,
    pub proof_hash: String,
    pub timestamp: u64,
    pub verification_status: String,
}

/// Token validation result
#[cfg(feature = "std")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenValidationResult {
    pub is_valid: bool,
    pub validation_errors: Vec<String>,
    pub token_metadata: HashMap<String, String>,
}