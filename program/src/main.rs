#![no_main]

extern crate alloc;

sp1_zkvm::entrypoint!(main);

use alloc::string::ToString;
use alloc::vec::Vec;
use execution_engine::common::{TransactionInput, TransactionOutput};
use execution_engine::traits::TokenContract;
use execution_engine::types::{ExecutionContext, TokenError};
use tokens::aadhaar_token::AadhaarToken;
use tokens::example_token::MyToken;
use tokens::nft_token::NFTToken;
use tokens::loan_token::LoanToken;
use tokens::loan_pool_token::LoanPoolToken;

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
    // Commit the result
    sp1_zkvm::io::commit(&result);
}

fn route_token_call(input: &TransactionInput, ctx: &ExecutionContext) -> TransactionOutput {
    let result = match input.token_name.as_str() {
        "NFT" => {
            let admin_address = [1u8; 32]; // For now, hardcoded
            let mut token_name_bytes = [0u8; 32];
            let name_bytes = input.token_name.as_bytes();
            let copy_len = name_bytes.len().min(32);
            token_name_bytes[..copy_len].copy_from_slice(&name_bytes[..copy_len]);
            let nft_contract = NFTToken::new(admin_address, token_name_bytes);
            nft_contract.execute(ctx, &input.function_name, &input.input_data)
        }
        "MyToken" => {
            let my_token_contract = MyToken;
            my_token_contract.execute(ctx, &input.function_name, &input.input_data)
        }
        "Aadhaar" => {
            let aadhaar_contract = AadhaarToken::new([0u8; 32], [0u8; 32]);
            aadhaar_contract.execute(ctx, &input.function_name, &input.input_data)
        }
        "LoanToken" => {
            let admin_address = [1u8; 32]; // For now, hardcoded
            let loan_contract = LoanToken::new(admin_address);
            loan_contract.execute(ctx, &input.function_name, &input.input_data)
        }
        "LoanPoolToken" => {
            let admin_address = [1u8; 32]; // For now, hardcoded
            let pool_contract = LoanPoolToken::new(admin_address);
            pool_contract.execute(ctx, &input.function_name, &input.input_data)
        }
        _ => Err(TokenError::Custom("Unknown token".to_string())),
    };

    match result {
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
