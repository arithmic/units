extern crate alloc;
use alloc::{vec, vec::Vec};

use borsh::{BorshSerialize, BorshDeserialize, from_slice, to_vec};

use crate::traits::TokenContract;
use crate::types::{
    Address, ExecutionContext, KeyValue, TokenError, TokenResult, TransactionReceipt,
};
use crate::utils::{get_nonce_from_pre_state, hash_nonce_key};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct MyNFTTokenData {
    pub token_id: [u8; 32],
    pub unique_identifier: [u8; 32],
    pub collectible_hash: [u8; 32],
    pub owner_id: Address,
    pub collectible_image_data: Vec<u8>,
}

pub struct NFTToken {
    pub admin_address: Address,
}

impl NFTToken {
    pub fn new(admin_address: Address) -> Self {
        Self { admin_address }
    }

    fn get_nft_key(token_id: &[u8; 32]) -> [u8; 32] {
        let mut key = [0u8; 32];
        key[..32].copy_from_slice(token_id);
        key
    }

    fn find_nft_owner(
        token_id: &[u8; 32],
        pre_state: &[KeyValue],
    ) -> Option<Address> {
        let nft_key = Self::get_nft_key(token_id);
        
        for kv in pre_state {
            if kv.key == nft_key {
                let mut owner = [0u8; 32];
                owner.copy_from_slice(&kv.value);
                return Some(owner);
            }
        }
        None
    }
}

impl TokenContract for NFTToken {
    fn execute(
        &self,
        ctx: &ExecutionContext,
        function: &str,
        input: &[u8],
    ) -> TokenResult<TransactionReceipt> {
        let current_nonce = get_nonce_from_pre_state(ctx.signer, ctx.pre_state);
        if ctx.nonce != current_nonce {
            return Err(TokenError::InvalidNonce);
        }

        let nonce_key = hash_nonce_key(ctx.signer);
        let mut nonce_bytes = [0u8; 32];
        nonce_bytes[0..8].copy_from_slice(&(current_nonce + 1).to_le_bytes());
        let nonce_write = KeyValue {
            key: nonce_key,
            value: nonce_bytes,
        };

        let mut receipt = match function {
            "mint" => self.mint(ctx, input),
            "transfer" => self.transfer(ctx, input),
            _ => Err(TokenError::FunctionNotFound),
        }?;

        receipt.writes.push(nonce_write);
        Ok(receipt)
    }
}

impl NFTToken {
    fn mint(&self, ctx: &ExecutionContext, input: &[u8]) -> TokenResult<TransactionReceipt> {
        if ctx.signer != self.admin_address {
            return Err(TokenError::Unauthorized);
        }

        if input.len() < 132 {
            return Err(TokenError::InvalidInput);
        }

        let mut token_id = [0u8; 32];
        let mut unique_identifier = [0u8; 32];
        let mut collectible_hash = [0u8; 32];
        let mut recipient = [0u8; 32];
        
        token_id.copy_from_slice(&input[0..32]);
        unique_identifier.copy_from_slice(&input[32..64]);
        collectible_hash.copy_from_slice(&input[64..96]);
        recipient.copy_from_slice(&input[96..128]);

        let image_data_len = u32::from_le_bytes([input[128], input[129], input[130], input[131]]) as usize;
        
        if input.len() < 132 + image_data_len {
            return Err(TokenError::InvalidInput);
        }

        let collectible_image_data = input[132..132 + image_data_len].to_vec();

        if Self::find_nft_owner(&token_id, ctx.pre_state).is_some() {
            return Err(TokenError::Custom(1)); // Token already exists
        }

        let nft_data = MyNFTTokenData {
            token_id,
            unique_identifier,
            collectible_hash,
            owner_id: recipient,
            collectible_image_data,
        };

        // Store only the owner in the state (32 bytes)
        let mut value = [0u8; 32];
        value.copy_from_slice(&recipient);

        let nft_write = KeyValue {
            key: Self::get_nft_key(&token_id),
            value,
        };

        Ok(TransactionReceipt {
            writes: vec![nft_write],
        })
    }

    fn transfer(&self, ctx: &ExecutionContext, input: &[u8]) -> TokenResult<TransactionReceipt> {
        if input.len() < 64 {
            return Err(TokenError::InvalidInput);
        }

        let mut token_id = [0u8; 32];
        let mut new_owner = [0u8; 32];
        
        token_id.copy_from_slice(&input[0..32]);
        new_owner.copy_from_slice(&input[32..64]);

        let current_owner = Self::find_nft_owner(&token_id, ctx.pre_state)
            .ok_or(TokenError::Custom(2))?; // Token not found

        if current_owner != ctx.signer {
            return Err(TokenError::Unauthorized);
        }

        // Store the new owner in the state
        let mut value = [0u8; 32];
        value.copy_from_slice(&new_owner);

        let nft_write = KeyValue {
            key: Self::get_nft_key(&token_id),
            value,
        };

        Ok(TransactionReceipt {
            writes: vec![nft_write],
        })
    }
}