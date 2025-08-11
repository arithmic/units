# RISC-V 32-bit Simple Example with M Extension

A minimal Rust program demonstrating compilation to RISC-V 32-bit architecture with M extension support.

## Features

- Fibonacci calculation (iterative approach)
- Multiplication using RISC-V M extension (`MUL` instruction)
- Division using RISC-V M extension (`DIV` instruction)
- Bare metal `no_std` implementation
- Optimized for `riscv32im-unknown-none-elf` target

## Prerequisites

```bash
# Install RISC-V target
rustup target add riscv32im-unknown-none-elf

# Install required tools for binary analysis
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

## Building

```bash
# Build the project
cargo build --release

# View disassembly 
cargo objdump --release -- -d

# Generate binary file
cargo objcopy --release -- -O binary risc-v-simple.bin

# View binary size
cargo size --release
```

## Generated Assembly

The optimizer produces highly efficient code. For Fibonacci(10) = 55 (0x37):

```assembly
000110d4 <_start>:
   110d4: 10000537     	lui	a0, 0x10000      # Load base address 0x10000000
   110d8: 03700593     	li	a1, 0x37         # Load 55 (fibonacci result)
   110dc: 00b52023     	sw	a1, 0x0(a0)      # Store to 0x10000000
   110e0: 06e00593     	li	a1, 0x6e         # Load 110 (55 * 2)
   110e4: 00b52223     	sw	a1, 0x4(a0)      # Store to 0x10000004
   110e8: 02400593     	li	a1, 0x24         # Load 36 (110 / 3)
   110ec: 00b52423     	sw	a1, 0x8(a0)      # Store to 0x10000008
   110f0: 0000006f     	j	0x110f0          # Infinite loop
```

## Verification

The program calculates:
- Fibonacci(10) = 55
- 55 × 2 = 110  
- 110 ÷ 3 = 36

Values are stored at:
- `0x10000000`: 55 (0x37)
- `0x10000004`: 110 (0x6E) 
- `0x10000008`: 36 (0x24)
