pub mod aadhaar_token;
pub mod dummy_token_1;
pub mod dummy_token_2;
pub mod examples;
pub mod execution;
pub mod inclusion_proof;
pub mod signature;
pub mod traits;
pub mod utils;
pub mod zk;

extern crate alloc;
use alloc::{collections::BTreeMap, vec::Vec};

use crate::{
    aadhaar_token::AadhaarToken, examples::MockStateManager, signature::Ed25519Verifier,
    traits::TokenContract,
};

pub type Address = [u8; 32];
pub type StateKey = [u8; 32];
pub type StateValue = [u8; 32];
pub type MetadataHash = [u8; 32];

#[derive(Clone, Debug)]
pub struct KeyValue {
    pub key: StateKey,
    pub value: StateValue,
}

#[derive(Clone, Debug)]
pub struct StateCommitment {
    pub root: [u8; 32],
}

#[derive(Clone, Debug)]
pub struct ExecutionContext<'a> {
    pub signer: Address,
    pub signature: &'a [u8],
    pub message: &'a [u8],
    pub pre_state: &'a BTreeMap<StateKey, StateValue>,
    pub input: &'a [u8],
    pub timestamp: u64,
}

#[derive(Clone, Debug)]
pub enum TokenError {
    InvalidSignature,
    TokenNotFound,
    InsufficientBalance,
    Unauthorized,
    StateMismatch,
    InvalidProof,
    Overflow,
    Custom(u8),
}

pub type TokenResult<T> = core::result::Result<T, TokenError>;

#[derive(Clone, Debug)]
pub struct TransactionReceipt {
    pub transaction_hash: [u8; 32],
    pub writes: Vec<KeyValue>,
    pub pre_state: BTreeMap<StateKey, StateValue>,
    pub post_state: BTreeMap<StateKey, StateValue>,
    pub new_state_root: [u8; 32],
}

pub struct TokenInfo {
    pub amount: u64,
    pub token_type: u8,
    pub metadata_hash: MetadataHash,
}
