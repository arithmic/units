use crate::inclusion_proof::InclusionProof;
use crate::{
    Address, ExecutionContext, KeyValue, MetadataHash, StateCommitment, StateKey, TokenResult,
    TransactionReceipt,
};

pub trait TokenContract {
    fn mint(
        &self,
        ctx: &ExecutionContext,
        to: Address,
        amount: u64,
        metadata_hash: MetadataHash,
    ) -> TokenResult<TransactionReceipt>;

    fn burn(&self, ctx: &ExecutionContext, from: Address) -> TokenResult<TransactionReceipt>;

    fn transfer_with_checks<F>(
        &self,
        ctx: &ExecutionContext,
        from: Address,
        to: Address,
        checks: &[KeyValue],
        inclusion_proof: Option<InclusionProof>,
        custom_check: F,
    ) -> TokenResult<TransactionReceipt>
    where
        F: Fn(&ExecutionContext, &[KeyValue], Option<&InclusionProof>) -> TokenResult<()>;
}

pub trait SignatureVerifier {
    fn verify_signature(
        &self,
        signer: &Address,
        message: &[u8],
        signature: &[u8],
    ) -> TokenResult<()>;
}

pub trait StateManager {
    fn get_current_root(&self) -> StateCommitment;
    fn generate_inclusion_proof(&self, key: &StateKey) -> TokenResult<InclusionProof>;
    fn update_state(&mut self, writes: &[KeyValue]) -> TokenResult<StateCommitment>;
    fn verify_state_transition(
        &self,
        pre_state: &StateCommitment,
        post_state: &StateCommitment,
        writes: &[KeyValue],
    ) -> TokenResult<bool>;
}

