use sp1_sdk::{include_elf, HashableKey, Prover, ProverClient};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const UNITS_ELF: &[u8] = include_elf!("program");

fn main() {
    let prover = ProverClient::builder().cpu().build();
    let (_, vk) = prover.setup(UNITS_ELF);
    println!("{}", vk.bytes32());
}
