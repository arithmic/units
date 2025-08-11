use crate::{
    aadhaar_token::AadhaarToken,
    inclusion_proof::InclusionProof,
    signature::Ed25519Verifier,
    traits::{StateManager, TokenContract},
    Address, ExecutionContext, KeyValue, StateCommitment, StateKey, TokenResult,
};
use alloc::{collections::BTreeMap, vec::Vec};

pub struct MockStateManager;

impl StateManager for MockStateManager {
    fn get_current_root(&self) -> StateCommitment {
        StateCommitment { root: [0u8; 32] }
    }

    fn generate_inclusion_proof(&self, _key: &StateKey) -> TokenResult<InclusionProof> {
        Ok(InclusionProof {
            siblings: Vec::new(),
            leaf_hash: [0u8; 32],
            root: [0u8; 32],
            key: [0u8; 32],
            value: [0u8; 32],
        })
    }

    fn update_state(&mut self, _writes: &[KeyValue]) -> TokenResult<StateCommitment> {
        Ok(StateCommitment { root: [0u8; 32] })
    }

    fn verify_state_transition(
        &self,
        _pre_state: &StateCommitment,
        _post_state: &StateCommitment,
        _writes: &[KeyValue],
    ) -> TokenResult<bool> {
        Ok(true)
    }
}

pub fn example_token_mint() -> TokenResult<()> {
    let issuer_id: Address = [1u8; 32];
    let recipient: Address = [2u8; 32];

    let token = AadhaarToken::new(issuer_id, 1, Ed25519Verifier, MockStateManager);

    let signature = [0u8; 64];
    let message = b"mint transaction";
    let pre_state = BTreeMap::new();

    let ctx = ExecutionContext {
        signer: issuer_id,
        signature: &signature,
        message,
        pre_state: &pre_state,
        input: &[],
        timestamp: 1234567890,
    };

    let metadata_hash = [3u8; 32];
    let _receipt = token.mint(&ctx, recipient, 100, metadata_hash)?;

    Ok(())
}

