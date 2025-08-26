#![no_main]

extern crate alloc;

sp1_zkvm::entrypoint!(main);

use alloc::{string::String, vec::Vec};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};
use unitsdesign::nft_token::NFTToken;
use unitsdesign::traits::TokenContract;
use unitsdesign::types::{Address, ExecutionContext, KeyValue, TransactionReceipt};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct TokenInput {
    token_name: [u8; 32],
    function_name: String,
    signer: Address,
    pre_state: Vec<KeyValue>,
    timestamp: u64,
    block_id: u64,
    transaction_hash: [u8; 32],
    token_id: String,
    nonce: u64,
    input_data: Vec<u8>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
struct TokenOutput {
    success: bool,
    receipt: Option<TransactionReceipt>,
    error: Option<String>,
}

fn main() {
    // Read input from stdin
    let input_bytes = sp1_zkvm::io::read::<Vec<u8>>();
    
    // Deserialize the input
    let token_input = match borsh::from_slice::<TokenInput>(&input_bytes) {
        Ok(input) => input,
        Err(e) => {
            let output = TokenOutput {
                success: false,
                receipt: None,
                error: Some(format!("Failed to deserialize input: {}", e)),
            };
            sp1_zkvm::io::commit(&output);
            return;
        }
    };
    
    // Create execution context
    let ctx = ExecutionContext {
        signer: token_input.signer,
        pre_state: &token_input.pre_state,
        timestamp: token_input.timestamp,
        block_id: token_input.block_id,
        transaction_hash: token_input.transaction_hash,
        token_id: token_input.token_id.clone(),
        nonce: token_input.nonce,
    };
    
    // Route to appropriate token contract based on token_name
    let result = route_token_call(&token_input, &ctx);
    
    // Commit the result
    sp1_zkvm::io::commit(&result);
}

fn route_token_call(input: &TokenInput, ctx: &ExecutionContext) -> TokenOutput {
    // For now, we'll assume all token names route to NFT
    // In the future, you could add other token types here
    
    // Create NFT contract (admin address would be passed in input in real implementation)
    let admin_address = [1u8; 32]; // For now, hardcoded
    let nft_contract = NFTToken::new(admin_address, input.token_name);
    
    // Execute the function
    match nft_contract.execute(ctx, &input.function_name, &input.input_data) {
        Ok(receipt) => TokenOutput {
            success: true,
            receipt: Some(receipt),
            error: None,
        },
        Err(e) => TokenOutput {
            success: false,
            receipt: None,
            error: Some(format!("{:?}", e)),
        },
    }
}