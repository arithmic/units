extern crate alloc;

use alloc::vec;
use borsh::{BorshDeserialize, BorshSerialize};
use execution_engine::{
    traits::TokenContract,
    types::{
        Address, ExecutionContext, KeyValue, TokenError, TokenResult, TransactionReceipt,
    },
    utils::{get_nonce_from_pre_state, hash_nonce_key},
};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct AadhaarToken {
    pub token_id: [u8; 32],
    pub owner_id: Address,
}

impl AadhaarToken {
    pub fn new(token_id: [u8; 32], owner_id: Address) -> Self {
        Self { token_id, owner_id }
    }

    // Internal function to handle minting logic
    fn mint(&self, _ctx: &ExecutionContext) -> TokenResult<TransactionReceipt> {
        Ok(TransactionReceipt { writes: vec![] })
    }

    // Internal function for burning
    fn burn(&self, _ctx: &ExecutionContext) -> TokenResult<TransactionReceipt> {
        Ok(TransactionReceipt { writes: vec![] })
    }

    // Internal function for transferring
    fn transfer(&self, _ctx: &ExecutionContext) -> TokenResult<TransactionReceipt> {
        Ok(TransactionReceipt { writes: vec![] })
    }

    // Internal function for a custom action
    fn approve(&self, _ctx: &ExecutionContext) -> TokenResult<TransactionReceipt> {
        Ok(TransactionReceipt { writes: vec![] })
    }
}

impl TokenContract for AadhaarToken {
    fn execute(
        &self,
        ctx: &ExecutionContext,
        function: &str,
        _input: &[u8],
    ) -> TokenResult<TransactionReceipt> {
        // Nonce check (provable, must be first)
        let current_nonce = get_nonce_from_pre_state(ctx.signer, ctx.pre_state);
        if ctx.nonce != current_nonce {
            return Err(TokenError::InvalidNonce);
        }

        // Prepare the nonce increment write
        let nonce_key = hash_nonce_key(ctx.signer);
        let mut nonce_bytes = [0u8; 32];
        nonce_bytes[0..8].copy_from_slice(&(current_nonce + 1).to_le_bytes());
        let nonce_write = KeyValue { key: nonce_key, value: nonce_bytes };

        let mut receipt = match function {
            "mint" => self.mint(ctx),
            "burn" => self.burn(ctx),
            "transfer" => self.transfer(ctx),
            "approve" => self.approve(ctx),
            _ => Err(TokenError::FunctionNotFound),
        }?;

        // Always include the nonce increment in the writes
        receipt.writes.push(nonce_write);
        Ok(receipt)
    }
}
