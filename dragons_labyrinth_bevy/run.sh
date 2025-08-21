#!/bin/bash

# PERMANENT FIX: ALWAYS USE CUSTOM RUST PATH
export PATH="$HOME/.cargo/bin:$PATH"

echo "=== Dragon's Labyrinth Build & Run ==="
echo "Using Rust from: $(which rustc)"
echo "Rust version: $(rustc --version)"
echo "Cargo version: $(cargo --version)"
echo "======================================"

# Build the game
cargo build --release

# Run the game
cargo run --release