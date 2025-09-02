#!/bin/bash

# Setup script for Horror RPG development environment
echo "Setting up Horror RPG development environment..."

# Export proper PATH for cargo and rustup
export PATH="$HOME/.cargo/bin:$PATH"
export RUSTUP_HOME="$HOME/.rustup"
export CARGO_HOME="$HOME/.cargo"

# Ensure stable toolchain is default
rustup default stable 2>/dev/null || echo "Rustup already configured"

# Add WASM target if not already installed
rustup target add wasm32-unknown-unknown 2>/dev/null || echo "WASM target already installed"

# Create a modified .replit that sources our environment
if [ ! -f ".replit.backup" ]; then
    cp .replit .replit.backup
fi

cat > .replit << 'EOF'
modules = ["rust"]
[agent]
expertMode = true

[nix]
channel = "stable-25_05"
packages = ["trunk", "cargo-edit", "rustup", "wasm-pack"]

[workflows]
runButton = "Project"

[[workflows.workflow]]
name = "Project"
mode = "parallel"
author = "agent"

[[workflows.workflow.tasks]]
task = "workflow.run"
args = "Horror RPG Server"

[[workflows.workflow]]
name = "Horror RPG Server"
author = "agent"

[[workflows.workflow.tasks]]
task = "shell.exec"
args = "source ./setup_env.sh && trunk serve --address 0.0.0.0 --port 5000"
waitForPort = 5000

[workflows.workflow.metadata]
outputType = "webview"

[[ports]]
localPort = 5000
externalPort = 80
EOF

echo "Environment setup complete!"
echo "WASM target: $(rustup target list --installed | grep wasm32-unknown-unknown || echo 'Not installed')"
echo "Cargo version: $(cargo --version 2>/dev/null || echo 'Not available')"
echo "Rustc version: $(rustc --version 2>/dev/null || echo 'Not available')"