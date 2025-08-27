#![no_main]

extern crate alloc;

sp1_zkvm::entrypoint!(main);

use alloc::vec::Vec;
use execution_engine::common::{TransactionInput, TransactionOutput};
use execution_engine::traits::TokenContract;
use execution_engine::types::ExecutionContext;
use tokens::nft_token::NFTToken;

fn main() {
    // Read input from stdin
    let input_bytes = sp1_zkvm::io::read::<Vec<u8>>();

    // Deserialize the input
    let txn_input = match borsh::from_slice::<TransactionInput>(&input_bytes) {
        Ok(input) => input,
        Err(_e) => {
            let output = TransactionOutput {
                success: false,
                receipt: None,
                error: Some("Failed to deserialize input".to_string()),
            };
            sp1_zkvm::io::commit(&output);
            return;
        }
    };

    // Create execution context
    let ctx = ExecutionContext {
        signer: txn_input.signer,
        pre_state: &txn_input.pre_state,
        timestamp: txn_input.timestamp,
        block_id: txn_input.block_id,
        transaction_hash: txn_input.transaction_hash,
        token_id: txn_input.token_id.clone(),
        nonce: txn_input.nonce,
    };

    // Route to appropriate token contract based on token_name
    let result: TransactionOutput = route_token_call(&txn_input, &ctx);
    println!("Transaction result: {:?}", result);
    // Commit the result
    sp1_zkvm::io::commit(&result);
}

fn route_token_call(input: &TransactionInput, ctx: &ExecutionContext) -> TransactionOutput {
    // For now there is a single nft token, we'll assume all token names route to NFT
    // In the future, you could add other token types here

    // Create NFT contract (admin address would be passed in input in real implementation)
    let admin_address = [1u8; 32]; // For now, hardcoded
    let nft_contract = NFTToken::new(admin_address, input.token_name);

    // Execute the function
    match nft_contract.execute(ctx, &input.function_name, &input.input_data) {
        Ok(receipt) => TransactionOutput {
            success: true,
            receipt: Some(receipt),
            error: None,
        },
        Err(e) => TransactionOutput {
            success: false,
            receipt: None,
            error: Some(format!("Token execution error: {:?}", e)),
        },
    }
}
