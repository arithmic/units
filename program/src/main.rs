#![no_main]

extern crate alloc;

sp1_zkvm::entrypoint!(main);

use alloc::vec;
use unitsdesign::example_token::MyToken;
use unitsdesign::traits::TokenContract;
use unitsdesign::types::{Address, ExecutionContext, KeyValue};

fn main() {
    // Hardcoded values for a single test instance.
    let signer: Address = [1u8; 32];
    let pre_state: Vec<KeyValue> = vec![];
    let input_data: Vec<u8> = vec![];

    let ctx = ExecutionContext {
        signer,
        pre_state: &pre_state,
        timestamp: 1638400000,
        block_id: 1,
        transaction_hash: [0u8; 32],
        token_id: "MyToken".to_string(),
        nonce: 0,
    };

    let token = MyToken;
    let result = token.execute(&ctx, "mint", &input_data);

    println!("Execution result: {:?}", result);
    assert!(result.is_ok(), "Transaction execution failed");

    let receipt = result.unwrap();
    println!("Transaction executed with {} writes", receipt.writes.len());
}
