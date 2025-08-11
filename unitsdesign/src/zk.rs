use crate::{KeyValue, TokenResult};
use alloc::vec::Vec;

pub struct ZKExecutionContext {
    pub risc_v_cycles: u32,
    pub memory_limit: u32,
}

pub const PROOF_SYSTEM: &str = "SparseMerkleTree";

impl ZKExecutionContext {
    pub fn new() -> Self {
        Self {
            risc_v_cycles: 1 << 20,
            memory_limit: 1 << 20,
        }
    }
    
    pub fn generate_execution_proof(
        &self,
        _pre_state: &[KeyValue],
        _post_state: &[KeyValue],
        _execution_trace: &[u8],
    ) -> TokenResult<Vec<u8>> {
        Ok(alloc::vec![0u8; 32])
    }
}