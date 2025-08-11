use std::collections::BTreeMap;
use units::{
    aadhaar_token::AadhaarToken, examples::MockStateManager, signature::Ed25519Verifier,
    traits::TokenContract, Address, ExecutionContext,
};

#[test]
fn test_token_mint() {
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
    let result = token.mint(&ctx, recipient, 100, metadata_hash);
    println!("{:?}", result);

    assert!(result.is_ok());

    let receipt = result.unwrap();
    assert_eq!(receipt.writes.len(), 1);
    assert_eq!(receipt.transaction_hash, [0u8; 32]);
}
