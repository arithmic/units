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
use sp1_sdk::{ProverClient, SP1Stdin};

/// The arguments for the command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    execute: bool,

    #[arg(long)]
    prove: bool,
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

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let stdin = SP1Stdin::new();

    if args.execute {
        // Execute the program.
        println!("Executing program...");
        let (output, report) = client.execute(&elf, &stdin).run().unwrap();
        println!("Program executed successfully.");
        println!("Output: {:?}", output);
        println!("Total instruction count: {}", report.total_instruction_count());
    } else {
        // Generate the proof.
        println!("Generating proof...");
        let (pk, vk) = client.setup(&elf);
        let proof = client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof");
        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
