extern crate alloc;
use alloc::{string::ToString, vec, vec::Vec};

use borsh::{from_slice, BorshDeserialize, BorshSerialize};

use execution_engine::traits::TokenContract;
use execution_engine::types::{
    Address, ExecutionContext, KeyValue, TokenError, TokenResult, TransactionReceipt,
};
use execution_engine::utils::{get_nonce_from_pre_state, hash_nonce_key};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct MyNFTTokenData {
    pub unique_identifier: [u8; 32],
    pub collectible_hash: [u8; 32],
    pub owner_id: Address,
    pub collectible_image_data: Vec<u8>,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct MintInput {
    pub unique_identifier: [u8; 32],
    pub collectible_hash: [u8; 32],
    pub owner_id: Address,
    pub collectible_image_data: Vec<u8>,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct TransferInput {
    pub unique_identifier: [u8; 32],
    pub new_owner: Address,
}



pub struct NFTToken {
    pub admin_address: Address,
    pub token_name: [u8; 32], // Fixed identifier for the token
}

impl NFTToken {
    pub fn new(admin_address: Address, token_name: [u8; 32]) -> Self {
        Self {
            admin_address,
            token_name,
        }
    }

    fn get_nft_key(unique_identifier: &[u8; 32]) -> [u8; 32] {
        let mut key = [0u8; 32];
        key[..32].copy_from_slice(unique_identifier);
        key
    }

    fn find_nft_owner(unique_identifier: &[u8; 32], pre_state: &[KeyValue]) -> Option<Address> {
        let nft_key = Self::get_nft_key(unique_identifier);

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

        // Deserialize NFT token data directly using borsh
        let mint_input = from_slice::<MintInput>(input).map_err(|_| TokenError::InvalidInput)?;

        if Self::find_nft_owner(&mint_input.unique_identifier, ctx.pre_state).is_some() {
            return Err(TokenError::Custom("Unique identifier already exists".to_string()));
        }

        // Store only the owner in the state (32 bytes)
        let mut value = [0u8; 32];
        value.copy_from_slice(&mint_input.owner_id);

        let nft_write = KeyValue {
            key: Self::get_nft_key(&mint_input.unique_identifier),
            value,
        };

        Ok(TransactionReceipt {
            writes: vec![nft_write],
        })
    }

    fn transfer(&self, ctx: &ExecutionContext, input: &[u8]) -> TokenResult<TransactionReceipt> {
        let transfer_input =
            from_slice::<TransferInput>(input).map_err(|_| TokenError::InvalidInput)?;

        let current_owner =
            Self::find_nft_owner(&transfer_input.unique_identifier, ctx.pre_state)
                .ok_or(TokenError::Custom("Unique identifier not found".to_string()))?;

        if current_owner != ctx.signer {
            return Err(TokenError::Unauthorized);
        }

        // Store the new owner in the state
        let mut value = [0u8; 32];
        value.copy_from_slice(&transfer_input.new_owner);

        let nft_write = KeyValue {
            key: Self::get_nft_key(&transfer_input.unique_identifier),
            value,
        };

        Ok(TransactionReceipt {
            writes: vec![nft_write],
        })
    }
}

