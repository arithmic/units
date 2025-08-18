use crate::inclusion_proof::InclusionProof;
use crate::traits::{SignatureVerifier, StateManager, TokenContract};
use crate::utils::{
    apply_writes, compute_tx_hash, encode_token_fields, generate_owner_key,
    get_balance_from_pre_state, get_token_from_pre_state,
};
use crate::{
    Address, ExecutionContext, KeyValue, MetadataHash, TokenError, TokenResult, TransactionReceipt,
};

pub struct DummyToken1<V: SignatureVerifier, S: StateManager> {
    issuer_id: Address,
    token_type: u8,
    verifier: V,
    state_manager: S,
}

impl<V: SignatureVerifier, S: StateManager> DummyToken1<V, S> {
    pub fn new(issuer_id: Address, token_type: u8, verifier: V, state_manager: S) -> Self {
        Self {
            issuer_id,
            token_type,
            verifier,
            state_manager,
        }
    }
}

impl<V: SignatureVerifier, S: StateManager> TokenContract for DummyToken1<V, S> {
    fn mint(
        &self,
        ctx: &ExecutionContext,
        to: Address,
        amount: u64,
        metadata_hash: MetadataHash,
    ) -> TokenResult<TransactionReceipt> {
        // V::verify_signature(&ctx.signer, ctx.message, ctx.signature)?;
        self.verifier
            .verify_signature(&ctx.signer, ctx.message, ctx.signature)?;
        if ctx.signer != self.issuer_id {
            return Err(TokenError::Unauthorized);
        }

        // Use state manager to get current root for validation
        let _current_root = self.state_manager.get_current_root();

        let owner_key = generate_owner_key(&to);

        let current_balance = get_balance_from_pre_state(ctx.pre_state, &owner_key)?;
        if current_balance != 0 {
            return Err(TokenError::StateMismatch);
        }

        let new_balance_value = encode_token_fields(amount, self.token_type, &metadata_hash)?;
        let write = KeyValue {
            key: owner_key,
            value: new_balance_value,
        };

        let writes = alloc::vec![write];
        Ok(TransactionReceipt {
            transaction_hash: compute_tx_hash(ctx, &writes)?,
            writes: writes.clone(),
            pre_state: ctx.pre_state.clone(),
            post_state: apply_writes(ctx.pre_state, &writes),
            new_state_root: [0u8; 32],
        })
    }

    fn burn(&self, ctx: &ExecutionContext, from: Address) -> TokenResult<TransactionReceipt> {
        // V::verify_signature(&ctx.signer, ctx.message, ctx.signature)?;
        self.verifier
            .verify_signature(&ctx.signer, ctx.message, ctx.signature)?;

        // Use state manager to get current root for validation
        let _current_root = self.state_manager.get_current_root();

        let owner_key = generate_owner_key(&from);

        let current_token = get_token_from_pre_state(ctx.pre_state, &owner_key)?;
        if current_token.is_none() {
            return Err(TokenError::TokenNotFound);
        }

        let write = KeyValue {
            key: owner_key,
            value: [0u8; 32],
        };

        let writes = alloc::vec![write];
        Ok(TransactionReceipt {
            transaction_hash: compute_tx_hash(ctx, &writes)?,
            writes: writes.clone(),
            pre_state: ctx.pre_state.clone(),
            post_state: apply_writes(ctx.pre_state, &writes),
            new_state_root: [0u8; 32],
        })
    }

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
        F: Fn(&ExecutionContext, &[KeyValue], Option<&InclusionProof>) -> TokenResult<()>,
    {
        // V::verify_signature(&ctx.signer, ctx.message, ctx.signature)?;
        self.verifier
            .verify_signature(&ctx.signer, ctx.message, ctx.signature)?;

        // Use state manager to get current root for validation
        let _current_root = self.state_manager.get_current_root();

        custom_check(ctx, checks, inclusion_proof.as_ref())?;

        let from_key = generate_owner_key(&from);
        let to_key = generate_owner_key(&to);

        let from_token =
            get_token_from_pre_state(ctx.pre_state, &from_key)?.ok_or(TokenError::TokenNotFound)?;
        let _to_current = get_balance_from_pre_state(ctx.pre_state, &to_key)?;

        let from_write = KeyValue {
            key: from_key,
            value: [0u8; 32],
        };
        let to_write = KeyValue {
            key: to_key,
            value: encode_token_fields(
                from_token.amount,
                from_token.token_type,
                &from_token.metadata_hash,
            )?,
        };

        let writes = alloc::vec![from_write, to_write];
        Ok(TransactionReceipt {
            transaction_hash: compute_tx_hash(ctx, &writes)?,
            writes: writes.clone(),
            pre_state: ctx.pre_state.clone(),
            post_state: apply_writes(ctx.pre_state, &writes),
            new_state_root: [0u8; 32],
        })
    }
}
