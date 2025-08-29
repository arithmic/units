# Sparse Merkle Tree Examples

This directory contains comprehensive examples demonstrating the usage of the sparse merkle tree implementation.

## Running the Examples

To run the main usage example:

```bash
# From the smt directory
cargo run --example usage_example
```

To run the tests for the examples:

```bash
# From the smt directory
cargo test --example usage_example
```

## Example Overview

The `usage_example.rs` file demonstrates six different use cases:

### 1. Basic Operations
- Creating a new sparse merkle tree
- Inserting key-value pairs using H256 types
- Retrieving values by key
- Deleting entries (by setting value to zero)
- Checking if the tree is empty

### 2. Custom Account Type
- Implementing the `Value` trait for custom data structures
- Creating an `Account` struct with balance, nonce, and username
- Storing and retrieving account data
- Updating account information

### 3. Word Storage
- Using string-based values with the `Word` type
- Inserting a sentence word by word
- Index-based key generation
- Retrieving stored words

### 4. Batch Operations
- Performing bulk updates with `update_all()`
- Efficient insertion of multiple key-value pairs
- Verification of batch operations

### 5. Merkle Proofs
- Generating merkle proofs for specific keys
- Verifying proofs by recomputing the root
- Testing proof validation with incorrect values
- Understanding proof structure and size

### 6. Tree Persistence
- Extracting the underlying store from a tree
- Reconstructing a tree from an existing store
- Verifying data integrity after reconstruction
- Understanding tree state management

## Key Concepts Demonstrated

### Custom Value Types
The examples show how to implement the `Value` trait for custom data structures:

```rust
impl Value for CustomType {
    fn to_h256(&self) -> H256 {
        // Convert your data to H256 hash
    }
    
    fn zero() -> Self {
        // Return the zero/empty value
    }
}
```

### Tree Type Aliases
Common type aliases used throughout the examples:

```rust
type AccountSMT = SparseMerkleTree<Sha256Hasher, Account, DefaultStore<Account>>;
type WordSMT = SparseMerkleTree<Sha256Hasher, Word, DefaultStore<Word>>;
type SimpleSMT = SparseMerkleTree<Sha256Hasher, H256, DefaultStore<H256>>;
```

### Key Generation
Simple deterministic key generation:

```rust
fn generate_key(data: &[u8]) -> H256 {
    let mut hash_bytes = [0u8; 32];
    for (i, &byte) in data.iter().enumerate() {
        hash_bytes[i % 32] ^= byte;
    }
    H256::from(hash_bytes)
}
```

## Dependencies

The examples use the following features and dependencies:

- `sha256::Sha256Hasher` - Hash function implementation
- `default_store::DefaultStore` - In-memory storage backend
- Standard library collections for demonstration purposes

## Performance Notes

- Batch operations (`update_all`) are more efficient than individual updates for multiple insertions
- The default store is an in-memory implementation suitable for testing and small datasets
- For production use, consider implementing custom storage backends that persist to disk
- Merkle proofs provide logarithmic verification time relative to tree size

## Error Handling

All examples demonstrate proper error handling using the `Result<(), Error>` pattern. Common errors include:

- Storage operation failures
- Invalid merkle proof computations
- Incorrect number of leaves in proof verification

## Testing

The examples include comprehensive tests that verify:

- All example functions execute successfully
- Custom `Value` trait implementations work correctly
- Zero values are handled properly
- Hash computations are deterministic

Run tests with:

```bash
cargo test --example usage_example -- --nocapture
```

The `--nocapture` flag allows you to see the println! output during test execution.

## Quick Start

For a quick demonstration, you can use the provided script:

```bash
cd units/smt
./examples/run_example.sh
```

This script will:
1. Build and run the comprehensive usage example
2. Execute all tests to verify functionality
3. Display helpful output and explanations

## Summary

This example provides a complete demonstration of sparse merkle tree capabilities, including:

- **Basic CRUD operations** - Insert, retrieve, update, and delete key-value pairs
- **Custom data types** - Implement the `Value` trait for your own structures
- **Batch operations** - Efficiently handle multiple updates at once
- **Cryptographic proofs** - Generate and verify merkle proofs for data integrity
- **State management** - Persist and reconstruct tree state from storage
- **Error handling** - Proper error management and recovery

The implementation uses SHA256 hashing for deterministic results and includes comprehensive tests to ensure reliability. This makes it suitable for production use cases requiring verifiable data structures and cryptographic proofs.