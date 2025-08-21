#!/bin/bash
# Build script for Dragon's Labyrinth with Bevy

echo "Building Dragon's Labyrinth..."
cd "$(dirname "$0")"

# Build the game with available Rust version
cargo build --release

echo "Build complete!"