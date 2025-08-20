use sp1_sdk::{include_elf, HashableKey, Prover, ProverClient};
use std::fs::read;
use std::path::PathBuf;

fn load_program(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let program_path = PathBuf::from(path);
    read(program_path)
}

fn main() {
    let temp = load_program("/Users/bhargav/Projects/units/target/elf-compilation/riscv32im-succinct-zkvm-elf/release/program").unwrap();
    let UNITS_ELF = temp.as_slice();

    let prover = ProverClient::builder().cpu().build();
    let (_, vk) = prover.setup(UNITS_ELF);
    println!("{}", vk.bytes32());
}
