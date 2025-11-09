#!/bin/bash

echo "Running mutation testing on business logic service layer..."
echo "This will test the quality of our test suite by introducing mutations"
echo "and verifying that tests catch them."
echo ""

# Check if cargo-mutants is installed
if ! command -v cargo-mutants &> /dev/null; then
    echo "cargo-mutants not found. Installing..."
    cargo install cargo-mutants
fi

# Run mutation testing on the service layer
# Focus on the brewing_service module to test business logic
cargo mutants --features database --test-threads 1 \
    --file src/services/brewing_service.rs \
    --file src/validation.rs \
    --file src/models/mod.rs

echo ""
echo "Mutation testing complete!"
echo "Review the results above to ensure tests catch actual bugs."
