#!/bin/bash

# Build script for Dragon's Labyrinth WebAssembly deployment
echo "Building Dragon's Labyrinth for WebAssembly..."

# Install WASM target if not already installed
rustup target add wasm32-unknown-unknown

# Install trunk for WASM building
cargo install trunk

# Create index.html for WASM deployment
cat > index.html << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Dragon's Labyrinth</title>
    <style>
        body {
            margin: 0;
            padding: 0;
            background: black;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            font-family: Arial, sans-serif;
        }
        canvas {
            max-width: 100vw;
            max-height: 100vh;
        }
        #loading {
            color: white;
            text-align: center;
        }
    </style>
</head>
<body>
    <div id="loading">
        <h1>Dragon's Labyrinth</h1>
        <p>Loading horror experience...</p>
    </div>
    <canvas id="bevy"></canvas>
    <script type="module">
        import init from './dragons_labyrinth.js';
        init().then(() => {
            document.getElementById('loading').style.display = 'none';
        });
    </script>
</body>
</html>
EOF

# Build for WASM with optimizations
cargo build --release --target wasm32-unknown-unknown

# Generate WASM bindings
wasm-bindgen --out-dir ./pkg --web target/wasm32-unknown-unknown/release/dragons_labyrinth.wasm

echo "Build complete! Open index.html in a web server to run the game."
echo "For development, run: python3 -m http.server 8000"