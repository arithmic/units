//! Common types shared across the zk-proof system
//! 
//! This module contains transaction input/output types and other shared structures
//! used by both the SP1 program and host application.

extern crate alloc;

use alloc::{string::String, vec::Vec};
use crate::types::{Address, TransactionReceipt, KeyValue};
use borsh::{BorshSerialize, BorshDeserialize};
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

