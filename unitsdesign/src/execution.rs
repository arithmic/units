use crate::{Address, ExecutionContext, KeyValue, MetadataHash, TokenResult, TransactionReceipt};
use crate::inclusion_proof::InclusionProof;
use crate::traits::TokenContract;
use crate::zk::ZKExecutionContext;
use alloc::vec::Vec;

pub struct ExecutionEnvironment<T: TokenContract> {
    contract: T,
    zk_context: ZKExecutionContext,
}

impl<T: TokenContract> ExecutionEnvironment<T> {
    pub fn new(contract: T) -> Self {
        Self {
            contract,
            zk_context: ZKExecutionContext::new(),
        }
    }

    pub fn execute_transaction(
        &self,
        transaction_type: TransactionType,
        ctx: ExecutionContext,
    ) -> TokenResult<TransactionReceipt> {
        // Track ZK context cycles for proof generation
        let _cycles = self.zk_context.risc_v_cycles;
        
        let result = match transaction_type {
            TransactionType::Mint { to, metadata_hash, amount } => {
                self.contract.mint(&ctx, to, amount, metadata_hash)
            }
            TransactionType::Burn { from } => {
                self.contract.burn(&ctx, from)
            }
            TransactionType::TransferWithChecks { from, to, checks, proof, custom_check } => {
                self.contract.transfer_with_checks(&ctx, from, to, &checks, proof, custom_check)
            }
        };
        
        // Generate ZK proof if transaction succeeded
        if let Ok(ref receipt) = result {
            let _proof = self.zk_context.generate_execution_proof(
                &receipt.writes,
                &receipt.writes,
                &[]
            )?;
        }
        
        result
    }
}

#[derive(Clone, Debug)]
pub enum TransactionType {
    Mint {
        to: Address,
        metadata_hash: MetadataHash,
        amount: u64,
    },
    Burn {
        from: Address,
    },
    TransferWithChecks {
        from: Address,
        to: Address,
        checks: Vec<KeyValue>,
        proof: Option<InclusionProof>,
        custom_check: fn(&ExecutionContext, &[KeyValue], Option<&InclusionProof>) -> TokenResult<()>,
    },
}