pub use crate::types::{
    Address, ExecutionContext, KeyValue, MetadataHash, StateCommitment, StateKey, TokenResult,
    TransactionReceipt,
};

pub trait TokenContract {
    /// The main entry point for all token interactions.
    /// This function routes calls to the appropriate internal logic based on the `function` name.
    fn execute(
        &self,
        ctx: &ExecutionContext,
        function: &str,
        input: &[u8],
    ) -> TokenResult<TransactionReceipt>;
}

pub trait SignatureVerifier {
    fn verify_signature(
        &self,
        signer: &Address,
        message: &[u8],
        signature: &[u8],
    ) -> TokenResult<()>;
}



