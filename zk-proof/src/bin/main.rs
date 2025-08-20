//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use clap::Parser;
use serde::de::Unexpected::Str;
use serde::{Deserialize, Serialize};
use sp1_build::build_program_with_args;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use std::collections::BTreeMap;
use std::fs::{read, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use unitsdesign::{
    Address, ExecutionContext, KeyValue, MetadataHash, TokenError, TokenResult, TransactionReceipt,
};

/// The arguments for the command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    execute: bool,

    #[arg(long)]
    prove: bool,
}

#[derive(Serialize, Deserialize)]
struct TransactionDetails<'a> {
    pub token_id: &'a str,
    pub fn_name: &'a str,
    pub args: Vec<u8>,
}

fn load_program(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let program_path = PathBuf::from(path);
    read(program_path)
}

fn create_binary_and_prove(transaction_details: &TransactionDetails) {
    let mut file = File::create("../program/src/main.rs").unwrap();

    if transaction_details.token_id == "AadhaarToken" && transaction_details.fn_name == "mint" {
        let mut issuer_id = transaction_details.args[0..32]
            .iter()
            .map(|x| format!("{x}u8"))
            .collect::<Vec<_>>()
            .join(", ");
        issuer_id = format!("[{}]", issuer_id);

        let mut recepient = transaction_details.args[32..64]
            .iter()
            .map(|x| format!("{x}u8"))
            .collect::<Vec<_>>()
            .join(", ");
        recepient = format!("[{}]", recepient);

        let mut signature = transaction_details.args[64..128]
            .iter()
            .map(|x| format!("{x}u8"))
            .collect::<Vec<_>>()
            .join(", ");
        signature = format!("[{}]", signature);

        let ts = u32::from_be_bytes(transaction_details.args[128..132].try_into().unwrap());
        let timestamp = format!("{ts}");

        let mut metadata_hash = transaction_details.args[132..164]
            .iter()
            .map(|x| format!("{x}u8"))
            .collect::<Vec<_>>()
            .join(", ");
        metadata_hash = format!("[{}]", metadata_hash);

        let amt = u32::from_be_bytes(transaction_details.args[164..168].try_into().unwrap());
        let amount = format!("{amt}");

        let program = format!(
            r#"#![no_main]
sp1_zkvm::entrypoint!(mint_token);

use unitsdesign::examples::MockStateManager;
use unitsdesign::signature::Ed25519Verifier;
use unitsdesign::traits::TokenContract;
use unitsdesign::{{aadhaar_token::AadhaarToken, Address, ExecutionContext}};
use std::collections::BTreeMap;

fn mint_token() {{
    let issuer_id: Address = {issuer_id};
    let recipient: Address = {recipient};

    let token = AadhaarToken::new(issuer_id, 1, Ed25519Verifier, MockStateManager);

    let signature = {signature};
    let message = b"Mint transaction.";
    let pre_state = BTreeMap::new();

    let ctx = ExecutionContext {{
        signer: issuer_id,
        signature: &signature,
        message,
        pre_state: &pre_state,
        input: &[],
        timestamp: {timestamp},
    }};

    let metadata_hash = {metadata_hash};
    let result = token.mint(&ctx, recipient, {amount}, metadata_hash);
    println!("{{:?}}", result);

    assert!(result.is_ok());

    let receipt = result.unwrap();
    assert_eq!(receipt.writes.len(), 1);
    assert_eq!(receipt.transaction_hash, [0u8; 32]);
}}"#,
            issuer_id = issuer_id,
            recipient = recepient,
            signature = signature,
            timestamp = timestamp,
            metadata_hash = metadata_hash,
            amount = amount,
        );

        file.write_all(program.as_bytes()).unwrap();
    } else if transaction_details.token_id == "DummyToken1" && transaction_details.fn_name == "mint"
    {
        let mut issuer_id = transaction_details.args[0..32]
            .iter()
            .map(|x| format!("{x}u8"))
            .collect::<Vec<_>>()
            .join(", ");
        issuer_id = format!("[{}]", issuer_id);

        let mut recepient = transaction_details.args[32..64]
            .iter()
            .map(|x| format!("{x}u8"))
            .collect::<Vec<_>>()
            .join(", ");
        recepient = format!("[{}]", recepient);

        let mut signature = transaction_details.args[64..128]
            .iter()
            .map(|x| format!("{x}u8"))
            .collect::<Vec<_>>()
            .join(", ");
        signature = format!("[{}]", signature);

        let ts = u32::from_be_bytes(transaction_details.args[128..132].try_into().unwrap());
        let timestamp = format!("{ts}");

        let mut metadata_hash = transaction_details.args[132..164]
            .iter()
            .map(|x| format!("{x}u8"))
            .collect::<Vec<_>>()
            .join(", ");
        metadata_hash = format!("[{}]", metadata_hash);

        let amt = u32::from_be_bytes(transaction_details.args[164..168].try_into().unwrap());
        let amount = format!("{amt}");

        let program = format!(
            r#"#![no_main]
sp1_zkvm::entrypoint!(mint_token);

use unitsdesign::examples::MockStateManager;
use unitsdesign::signature::Ed25519Verifier;
use unitsdesign::traits::TokenContract;
use unitsdesign::{{dummy_token_1::DummyToken1, Address, ExecutionContext}};
use std::collections::BTreeMap;

fn mint_token() {{
    let issuer_id: Address = {issuer_id};
    let recipient: Address = {recipient};

    let token = DummyToken1::new(issuer_id, 1, Ed25519Verifier, MockStateManager);

    let signature = {signature};
    let message = b"Mint transaction.";
    let pre_state = BTreeMap::new();

    let ctx = ExecutionContext {{
        signer: issuer_id,
        signature: &signature,
        message,
        pre_state: &pre_state,
        input: &[],
        timestamp: {timestamp},
    }};

    let metadata_hash = {metadata_hash};
    let result = token.mint(&ctx, recipient, {amount}, metadata_hash);
    println!("{{:?}}", result);

    assert!(result.is_ok());

    let receipt = result.unwrap();
    assert_eq!(receipt.writes.len(), 1);
    assert_eq!(receipt.transaction_hash, [0u8; 32]);
}}"#,
            issuer_id = issuer_id,
            recipient = recepient,
            signature = signature,
            timestamp = timestamp,
            metadata_hash = metadata_hash,
            amount = amount,
        );

        file.write_all(program.as_bytes()).unwrap();
    } else if transaction_details.token_id == "DummyToken2" && transaction_details.fn_name == "mint"
    {
        let mut issuer_id = transaction_details.args[0..32]
            .iter()
            .map(|x| format!("{x}u8"))
            .collect::<Vec<_>>()
            .join(", ");
        issuer_id = format!("[{}]", issuer_id);

        let mut recepient = transaction_details.args[32..64]
            .iter()
            .map(|x| format!("{x}u8"))
            .collect::<Vec<_>>()
            .join(", ");
        recepient = format!("[{}]", recepient);

        let mut signature = transaction_details.args[64..128]
            .iter()
            .map(|x| format!("{x}u8"))
            .collect::<Vec<_>>()
            .join(", ");
        signature = format!("[{}]", signature);

        let ts = u32::from_be_bytes(transaction_details.args[128..132].try_into().unwrap());
        let timestamp = format!("{ts}");

        let mut metadata_hash = transaction_details.args[132..164]
            .iter()
            .map(|x| format!("{x}u8"))
            .collect::<Vec<_>>()
            .join(", ");
        metadata_hash = format!("[{}]", metadata_hash);

        let amt = u32::from_be_bytes(transaction_details.args[164..168].try_into().unwrap());
        let amount = format!("{amt}");

        let program = format!(
            r#"#![no_main]
sp1_zkvm::entrypoint!(mint_token);

use unitsdesign::examples::MockStateManager;
use unitsdesign::signature::Ed25519Verifier;
use unitsdesign::traits::TokenContract;
use unitsdesign::{{dummy_token_2::DummyToken2, Address, ExecutionContext}};
use std::collections::BTreeMap;

fn mint_token() {{
    let issuer_id: Address = {issuer_id};
    let recipient: Address = {recipient};

    let token = DummyToken2::new(issuer_id, 1, Ed25519Verifier, MockStateManager);

    let signature = {signature};
    let message = b"Mint transaction.";
    let pre_state = BTreeMap::new();

    let ctx = ExecutionContext {{
        signer: issuer_id,
        signature: &signature,
        message,
        pre_state: &pre_state,
        input: &[],
        timestamp: {timestamp},
    }};

    let metadata_hash = {metadata_hash};
    let result = token.mint(&ctx, recipient, {amount}, metadata_hash);
    println!("{{:?}}", result);

    assert!(result.is_ok());

    let receipt = result.unwrap();
    assert_eq!(receipt.writes.len(), 1);
    assert_eq!(receipt.transaction_hash, [0u8; 32]);
}}"#,
            issuer_id = issuer_id,
            recipient = recepient,
            signature = signature,
            timestamp = timestamp,
            metadata_hash = metadata_hash,
            amount = amount,
        );

        file.write_all(program.as_bytes()).unwrap();
    }

    build_program_with_args("../program", Default::default());

    let temp = load_program("/Users/bhargav/Projects/units/target/elf-compilation/riscv32im-succinct-zkvm-elf/release/program").unwrap();
    let UNITS_ELF = temp.as_slice();

    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    // stdin.write(&args.n);

    // println!("n: {}", args.n);

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(UNITS_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(UNITS_ELF);

        // Generate the proof
        println!("Generating proof...");
        let proof = client
            .prove(&pk, &stdin)
            .compressed()
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}

fn main() {
    let issuer_id: Address = [1u8; 32];
    let recipient: Address = [2u8; 32];
    let signature = [0u8; 64];
    let timestamp: u32 = 1638400000;
    let metadata_hash = [3u8; 32];
    let amount: u32 = 100;

    let serialised_transaction_1: Vec<u8> = [
        &issuer_id[..],
        &recipient[..],
        &signature[..],
        &timestamp.to_be_bytes(),
        &metadata_hash[..],
        &amount.to_be_bytes(),
    ]
    .concat();

    let transaction_1_details = TransactionDetails {
        token_id: "AadhaarToken",
        fn_name: "mint",
        args: serialised_transaction_1,
    };

    let serialised_transaction_2: Vec<u8> = [
        &issuer_id[..],
        &recipient[..],
        &signature[..],
        &timestamp.to_be_bytes(),
        &metadata_hash[..],
        &amount.to_be_bytes(),
    ]
    .concat();

    let transaction_2_details = TransactionDetails {
        token_id: "DummyToken1",
        fn_name: "mint",
        args: serialised_transaction_2,
    };

    let serialised_transaction_3: Vec<u8> = [
        &issuer_id[..],
        &recipient[..],
        &signature[..],
        &timestamp.to_be_bytes(),
        &metadata_hash[..],
        &amount.to_be_bytes(),
    ]
    .concat();

    let transaction_3_details = TransactionDetails {
        token_id: "DummyToken2",
        fn_name: "mint",
        args: serialised_transaction_3,
    };

    create_binary_and_prove(&transaction_3_details);
}
