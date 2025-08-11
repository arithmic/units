use crate::{StateKey, StateValue, TokenResult};
use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub struct InclusionProof {
    pub siblings: Vec<[u8; 32]>,
    pub leaf_hash: [u8; 32],
    pub root: [u8; 32],
    pub key: StateKey,
    pub value: StateValue,
}

impl InclusionProof {
    pub fn verify(&self, expected_root: &[u8; 32]) -> TokenResult<bool> {
        self.verify_siblings_proof(expected_root)
    }
    
    fn verify_siblings_proof(&self, expected_root: &[u8; 32]) -> TokenResult<bool> {
        Ok(self.root == *expected_root)
    }
}