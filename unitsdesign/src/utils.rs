use crate::{
    Address, ExecutionContext, KeyValue, MetadataHash, StateKey, StateValue,
    TokenInfo, TokenResult,
};
use alloc::{vec::Vec, collections::BTreeMap};

pub fn btreemap_to_vec<K: Clone, V: Clone>(map: &BTreeMap<K, V>) -> Vec<(K, V)> {
    let mut vec = Vec::new();
    for (key, value) in map {
        vec.push((key.clone(), value.clone()));
    }
    vec
}

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

pub fn get_balance_from_pre_state(pre_state: &BTreeMap<StateKey, StateValue>, key: &StateKey) -> TokenResult<u64> {
    if let Some(value) = pre_state.get(key) {
        return Ok(u64::from_le_bytes([
            value[0], value[1], value[2], value[3],
            value[4], value[5], value[6], value[7]
        ]));
    }
    Ok(0)
}

pub fn get_token_from_pre_state(
    pre_state: &BTreeMap<StateKey, StateValue>,
    key: &StateKey,
) -> TokenResult<Option<TokenInfo>> {
    if let Some(value) = pre_state.get(key) {
        let amount = u64::from_le_bytes([
            value[0], value[1], value[2], value[3],
            value[4], value[5], value[6], value[7]
        ]);
        if amount > 0 {
            let token_type = value[8];
            let mut metadata_hash = [0u8; 32];
            metadata_hash[..23].copy_from_slice(&value[9..32]);
            
            return Ok(Some(TokenInfo {
                metadata_hash,
                amount,
                token_type,
            }));
        }
    }
    Ok(None)
}

pub fn apply_writes(pre_state: &BTreeMap<StateKey, StateValue>, writes: &[KeyValue]) -> BTreeMap<StateKey, StateValue> {
    let mut post_state = pre_state.clone();
    
    for write in writes {
        post_state.insert(write.key, write.value);
    }
    
    post_state
}

pub fn compute_tx_hash(_ctx: &ExecutionContext, _writes: &[KeyValue]) -> TokenResult<[u8; 32]> {
    Ok([0u8; 32])
}

pub fn get_value_from_pre_state(pre_state: &BTreeMap<StateKey, StateValue>, key: &StateKey) -> TokenResult<StateValue> {
    Ok(pre_state.get(key).copied().unwrap_or([0u8; 32]))
}