#!/bin/bash
# Build script for Dragon's Labyrinth with Bevy

# PERMANENT FIX: ALWAYS USE CUSTOM RUST PATH
export PATH="$HOME/.cargo/bin:$PATH"

echo "Building Dragon's Labyrinth..."
cd "$(dirname "$0")"

# Build the game with custom Rust version
cargo build --release

echo "Build complete!"