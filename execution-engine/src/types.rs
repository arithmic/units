extern crate alloc;
use alloc::{string::String, vec::Vec};
use borsh::{BorshSerialize, BorshDeserialize};
use serde::{Serialize, Deserialize};

// Core data structures based on the LLD

pub type Address = [u8; 32];
pub type StateKey = [u8; 32];
pub type StateValue = [u8; 32];
pub type MetadataHash = [u8; 32];

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct KeyValue {
    pub key: StateKey,
    pub value: StateValue,
}

#[derive(Clone, Debug)]
pub struct Transaction {
    pub transaction_hash: [u8; 32],
    pub token_id: String,
    pub function: String,
    pub input: Vec<u8>,
    pub nonce: u64,
    pub signature: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub block_id: u64,
    pub transactions: Vec<Transaction>,
    pub transaction_merkle_root: [u8; 32],
    pub pre_state_root: [u8; 32],
    pub post_state_root: [u8; 32],
    pub execution_proof: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct ExecutionContext<'a> {
    pub signer: Address,
    pub pre_state: &'a [KeyValue],
    pub timestamp: u64,
    pub block_id: u64,
    pub transaction_hash: [u8; 32],
    pub token_id: String,
    pub nonce: u64,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum TokenError {
    InvalidInput,
    FunctionNotFound,
    Unauthorized,
    StateMismatch,
    InvalidNonce,
    Custom(u8),
}

pub type TokenResult<T> = Result<T, TokenError>;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct TransactionReceipt {
    pub writes: Vec<KeyValue>,
}

#[derive(Clone, Debug)]
pub struct StateCommitment {
    pub root: [u8; 32],
}

pub struct TokenInfo {
    pub amount: u64,
    pub token_type: u8,
    pub metadata_hash: MetadataHash,
}
