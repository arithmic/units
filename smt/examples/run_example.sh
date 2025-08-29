#!/bin/bash

# Sparse Merkle Tree Usage Example Runner
# This script runs the comprehensive usage example for the sparse merkle tree implementation

set -e

echo "=== Sparse Merkle Tree Usage Example ==="
echo "This example demonstrates various operations with sparse merkle trees using SHA256 hashing."
echo ""

# Check if we're in the correct directory
if [ ! -f "Cargo.toml" ]; then
    echo "Error: Please run this script from the smt directory"
    echo "Usage: cd units/smt && ./examples/run_example.sh"
    exit 1
fi

# Check if the example exists
if [ ! -f "examples/usage_example.rs" ]; then
    echo "Error: usage_example.rs not found in examples directory"
    exit 1
fi

echo "Building and running the usage example..."
echo ""

# Run the example
cargo run --example usage_example

echo ""
echo "=== Running Tests ==="
echo ""

# Run the tests
cargo test --example usage_example

echo ""
echo "=== Example completed successfully! ==="
echo ""
echo "For more information, see examples/README.md"
