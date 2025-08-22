# Units ZK Proof System

This repository contains a zero-knowledge proof system built with SP1.

## Building and Running

### 1. Build the Program

First, build the program crate using SP1's prove toolchain:

```bash
# Navigate
cd program
cargo prove build
```

This compiles the program for the RISC-V target and generates the necessary ELF file.

### 2. Run the ZK Proof System

Navigate to the zk-proof directory and run either execution or proving:

```bash
cd zk-proof
```

#### Execute (faster, for testing):
```bash
cargo r -r -- --execute
```

#### Prove (generates ZK proof):
```bash
cargo r -r -- --prove
```

## Project Structure

- `program/` - The SP1 program that gets compiled to RISC-V
- `zk-proof/` - The host application that executes or proves the program
- `unitsdesign/` - Core token and state management logic

## Requirements

- SP1 toolchain installed
- Rust with RISC-V target support
