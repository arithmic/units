extern crate alloc;
use alloc::vec;

use execution_engine::traits::TokenContract;
use execution_engine::types::{
    ExecutionContext, KeyValue, TokenError, TokenResult, TransactionReceipt,
};
use execution_engine::utils::{get_nonce_from_pre_state, hash_nonce_key};

pub struct MyToken;

impl TokenContract for MyToken {
    fn execute(
        &self,
        ctx: &ExecutionContext,
        function: &str,
        input: &[u8],
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
            "mint" => self.mint(ctx, input),
            "burn" => self.burn(ctx, input),
            "transfer" => self.transfer(ctx, input),
            "approve" => self.approve(ctx, input),
            _ => Err(TokenError::FunctionNotFound),
        }?;

        // Always include the nonce increment in the writes
        receipt.writes.push(nonce_write);
        Ok(receipt)
    }
}

impl MyToken {
    // Internal function to handle minting logic
    fn mint(&self, _ctx: &ExecutionContext, _input: &[u8]) -> TokenResult<TransactionReceipt> {
        // 1. Deserialize input bytes to get minting parameters (e.g., recipient, amount).
        // 2. Perform authorization checks (e.g., is ctx.signer the issuer?).
        // 3. Check pre_state to ensure recipient doesn't already have a token.
        // 4. Construct the KeyValue writes for the new token.
        // 5. Return the TransactionReceipt with the writes.
        Ok(TransactionReceipt { writes: vec![] })
    }

    // Internal function for burning
    fn burn(&self, _ctx: &ExecutionContext, _input: &[u8]) -> TokenResult<TransactionReceipt> {
        Ok(TransactionReceipt { writes: vec![] })
    }

    // Internal function for transferring
    fn transfer(&self, _ctx: &ExecutionContext, _input: &[u8]) -> TokenResult<TransactionReceipt> {
        Ok(TransactionReceipt { writes: vec![] })
    }

    // Internal function for a custom action
    fn approve(&self, _ctx: &ExecutionContext, _input: &[u8]) -> TokenResult<TransactionReceipt> {
        Ok(TransactionReceipt { writes: vec![] })
    }
}
