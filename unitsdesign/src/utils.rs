use crate::types::{
    Address, ExecutionContext, KeyValue, MetadataHash, StateKey, StateValue, TokenInfo, TokenResult
};

pub fn generate_owner_key(owner: &Address) -> StateKey {
    let mut key = [0u8; 32];
    key[..32].copy_from_slice(&owner[..32]);
    key
}

pub fn encode_token_fields(
    amount: u64,
    token_type: u8,
    metadata_hash: &MetadataHash,
) -> TokenResult<StateValue> {
    let mut value = [0u8; 32];
    value[..8].copy_from_slice(&amount.to_le_bytes());
    value[8] = token_type;
    value[9..32].copy_from_slice(&metadata_hash[..23]);
    Ok(value)
}

pub fn get_balance_from_pre_state_vec(pre_state: &[KeyValue], key: &StateKey) -> TokenResult<u64> {
    for kv in pre_state {
        if kv.key == *key {
            return Ok(u64::from_le_bytes([
                kv.value[0], kv.value[1], kv.value[2], kv.value[3],
                kv.value[4], kv.value[5], kv.value[6], kv.value[7]
            ]));
        }
    }
    Ok(0)
}

pub fn get_token_from_pre_state_vec(
    pre_state: &[KeyValue],
    key: &StateKey,
) -> TokenResult<Option<TokenInfo>> {
    for kv in pre_state {
        if kv.key == *key {
            let amount = u64::from_le_bytes([
                kv.value[0], kv.value[1], kv.value[2], kv.value[3],
                kv.value[4], kv.value[5], kv.value[6], kv.value[7]
            ]);
            if amount > 0 {
                let token_type = kv.value[8];
                let mut metadata_hash = [0u8; 32];
                metadata_hash[..23].copy_from_slice(&kv.value[9..32]);
                
                return Ok(Some(TokenInfo {
                    metadata_hash,
                    amount,
                    token_type,
                }));
            }
        }
    }
    Ok(None)
}

pub fn compute_tx_hash(_ctx: &ExecutionContext, _writes: &[KeyValue]) -> TokenResult<[u8; 32]> {
    Ok([0u8; 32])
}

pub fn get_value_from_pre_state_vec(pre_state: &[KeyValue], key: &StateKey) -> TokenResult<StateValue> {
    for kv in pre_state {
        if kv.key == *key {
            return Ok(kv.value);
        }
    }
    Ok([0u8; 32])
}

// Helper functions for nonce and key management
pub fn get_nonce_from_pre_state(signer: Address, pre_state: &[KeyValue]) -> u64 {
    let nonce_key = hash_nonce_key(signer);
    for kv in pre_state {
        if kv.key == nonce_key {
            // Safely convert slice to array
            let mut array = [0u8; 8];
            let bytes = &kv.value[0..8];
            array.copy_from_slice(bytes);
            return u64::from_le_bytes(array);
        }
    }
    0 // Default nonce for new accounts
}

pub fn hash_nonce_key(signer: Address) -> [u8; 32] {
    // Simple hash function to derive the nonce key for the signer
    // In practice, you'd use a proper hash function like SHA-256 or Keccak-256
    let mut key = [0u8; 32];
    key[0] = 0xFF; // Nonce key prefix
    for i in 0..31 {
        key[i + 1] = signer[i];
    }
    key
}
