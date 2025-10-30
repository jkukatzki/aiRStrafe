#!/bin/bash

# Build script for compiling movement logic to WASM
# This script generates the WASM files and JavaScript bindings

set -e

echo "Building movement logic for WASM..."

# Navigate to the movement-logic directory
cd "$(dirname "$0")"

# Install wasm-pack if not already installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WASM package
echo "Compiling to WASM..."
wasm-pack build --target web --out-dir pkg --out-name movement_logic

# Check if build was successful
if [ -f "pkg/movement_logic.js" ] && [ -f "pkg/movement_logic_bg.wasm" ]; then
    echo "✅ WASM build successful!"
    echo "Generated files:"
    echo "  - pkg/movement_logic.js (JavaScript bindings)"
    echo "  - pkg/movement_logic_bg.wasm (WebAssembly module)"
    echo "  - pkg/movement_logic.d.ts (TypeScript definitions)"
else
    echo "❌ WASM build failed!"
    exit 1
fi

echo ""
echo "You can now import the movement logic in your frontend like this:"
echo ""
echo "import init, { Vector3, airAccelerate, gravityForce, playerMove } from './pkg/movement_logic.js';"
echo ""
echo "// Initialize the WASM module"
echo "await init();"
echo ""
echo "// Create vectors"
echo "let velocity = new Vector3(0, 0, 0);"
echo "let wishDirection = new Vector3(1, 0, 0);"
echo "let gravity = new Vector3(0, -9.8, 0);"
echo ""
echo "// Get air acceleration and add to velocity"
echo "let acceleration = airAccelerate(velocity, wishDirection, 100, 10, 30, 0.016);"
echo "velocity.x += acceleration.x;"
echo "velocity.y += acceleration.y;"
echo "velocity.z += acceleration.z;"
echo ""
echo "// Apply gravity force"
echo "let gravityAccel = gravityForceAcceleration(gravity, 0.016);"
echo "velocity.x += gravityAccel.x;"
echo "velocity.y += gravityAccel.y;"
echo "velocity.z += gravityAccel.z;"
echo ""
echo "⚠️  NOTE: This build uses wasm-pack, which is being archived in Sept 2025."
echo "   Consider migrating to alternative tools or maintaining a local fork."
