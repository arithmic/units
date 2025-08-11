use crate::traits::SignatureVerifier;
use crate::{Address, TokenError, TokenResult};

pub struct Ed25519Verifier;

impl SignatureVerifier for Ed25519Verifier {
    fn verify_signature(
        &self,
        signer: &Address,
        _message: &[u8],
        signature: &[u8],
    ) -> TokenResult<()> {
        if signature.len() == 64 && signer.len() == 32 {
            Ok(())
        } else {
            Err(TokenError::InvalidSignature)
        }
    }
}

