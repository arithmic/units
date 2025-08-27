//! NFT Token ZK Proof System
//!
//! This implements the 6-step NFT flow with zero-knowledge proof generation.

use borsh::to_vec;
use clap::Parser;
use sp1_sdk::{EnvProver, SP1Stdin};
use execution_engine::common::{TransactionInput, TransactionOutput};
use tokens::nft_token::MyNFTTokenData;
use execution_engine::types::Address;

/// The arguments for the command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Execute the program without generating proof
    Execute,
    /// Generate and verify a ZK proof
    Prove,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Build the program.
    sp1_build::build_program("../program");
    let elf_path = "../target/elf-compilation/riscv32im-succinct-zkvm-elf/release/program";

    // Load the ELF.
    let elf = std::fs::read(elf_path).expect("Failed to read ELF file");

    // Parse the command line arguments.
    let args = Args::parse();

    // Setup the prover client.
    let client = EnvProver::default();

    // Route to appropriate command
    match args.command {
        Commands::Execute => execute_simple(&client, &elf),
        Commands::Prove => prove_simple(&client, &elf),
    }
}

// Simple execute command
fn execute_simple(client: &EnvProver, elf: &[u8]) {
    println!("=== Simple Execute Mode ===");

    let (stdin, _nft_data) = create_mint_transaction();

    // Execute the program
    let (output, report) = client.execute(elf, &stdin).run().unwrap();
    println!("Program executed successfully.");
    println!(
        "Total instruction count: {}",
        report.total_instruction_count()
    );
    println!("Output: {:?}", output);

    // Try to deserialize and display result
    // SP1 commit uses bincode serialization
    match bincode::deserialize::<TransactionOutput>(&output.as_slice()) {
        Ok(token_output) => {
            println!("Execution result: Success = {}", token_output.success);
            if let Some(error) = token_output.error {
                println!("Error: {}", error);
            }
            if let Some(receipt) = token_output.receipt {
                println!("Receipt: {:?}", receipt);
            }
        }
        Err(e) => {
            println!("Failed to deserialize with bincode: {}", e);
            println!("Output bytes length: {}", output.as_slice().len());
            // Print first few bytes for debugging
            let bytes = output.as_slice();
            if bytes.len() > 0 {
                println!("First 20 bytes: {:?}", &bytes[..std::cmp::min(20, bytes.len())]);
            }
        }
    }
}

// Simple prove command
fn prove_simple(client: &EnvProver, elf: &[u8]) {
    println!("=== Simple Prove Mode ===");

    let (stdin, _nft_data) = create_mint_transaction();

    // Generate the proof
    println!("Generating ZK proof...");
    let (pk, vk) = client.setup(elf);
    let proof = client.prove(&pk, &stdin).run().unwrap();
    println!("Successfully generated proof!");

    // Save the proof
    save_proof_to_file(&proof, "simple_proof.bin");
    println!("Proof saved to simple_proof.bin");

    // Verify the proof
    client.verify(&proof, &vk).expect("failed to verify proof");
    println!("Successfully verified proof!");
}

// Helper function to create sample input data
fn create_mint_transaction() -> (SP1Stdin, MyNFTTokenData) {
    // Setup admin and recipient addresses
    let admin_address: Address = [1u8; 32];
    let recipient: Address = [2u8; 32];

    // Create NFT token data
    let token_id = [42u8; 32];
    let unique_identifier = [123u8; 32];
    let collectible_hash = [255u8; 32];
    let image_data = vec![0xDE, 0xAD, 0xBE, 0xEF];

    let nft_data = MyNFTTokenData {
        token_id,
        unique_identifier,
        collectible_hash,
        owner_id: recipient,
        collectible_image_data: image_data,
    };

    // Prepare input for SP1 program (mint operation)
    let token_name = [84u8; 32]; // "T" repeated as token identifier
    let mint_input_data = to_vec(&nft_data).expect("Failed to serialize NFT data");

    let token_input = TransactionInput {
        token_name,
        function_name: "mint".to_string(),
        signer: admin_address,
        pre_state: vec![],
        timestamp: 1638400000,
        block_id: 1,
        transaction_hash: [0u8; 32],
        token_id: "NFTToken".to_string(),
        nonce: 0,
        input_data: mint_input_data,
    };

    let input_bytes = to_vec(&token_input).expect("Failed to serialize token input");

    let mut stdin = SP1Stdin::new();
    stdin.write(&input_bytes);

    (stdin, nft_data)
}

// Helper function to save proof to file
fn save_proof_to_file(proof: &sp1_sdk::SP1ProofWithPublicValues, filename: &str) {
    use std::fs::File;
    use std::io::Write;

    // Serialize the proof using bincode
    if let Ok(proof_bytes) = bincode::serialize(proof) {
        let proof_size = proof_bytes.len();
        if let Ok(mut file) = File::create(filename) {
            if file.write_all(&proof_bytes).is_ok() {
                println!("Proof (size: {} bytes) saved to {}", proof_size, filename);
            } else {
                println!("Failed to write proof data to {}", filename);
            }
        } else {
            println!("Failed to create file {}", filename);
        }
    } else {
        println!("Failed to serialize proof");
    }
}