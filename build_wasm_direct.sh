#!/bin/bash

# Direct wasm-bindgen build script for movement logic
# This script replaces wasm-pack with direct wasm-bindgen usage

set -e

echo "Building movement logic with direct wasm-bindgen workflow..."

# Navigate to the movement-logic directory
cd "$(dirname "$0")"

# Install wasm-bindgen-cli if not already installed
if ! command -v wasm-bindgen &> /dev/null; then
    echo "Installing wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli
fi

# Add wasm32 target if not already added
echo "Ensuring wasm32-unknown-unknown target is installed..."
rustup target add wasm32-unknown-unknown

# Build the Rust code for WebAssembly
echo "Compiling Rust to WebAssembly..."
cargo build --target wasm32-unknown-unknown --release --features wasm

# Check if the WASM file was created - try multiple possible locations
WASM_FILE=""
POSSIBLE_LOCATIONS=(
    "target/wasm32-unknown-unknown/release/movement_logic.wasm"
    "../../../target/wasm32-unknown-unknown/release/movement_logic.wasm"
    "../../target/wasm32-unknown-unknown/release/movement_logic.wasm"
    "../target/wasm32-unknown-unknown/release/movement_logic.wasm"
)

for location in "${POSSIBLE_LOCATIONS[@]}"; do
    if [ -f "$location" ]; then
        WASM_FILE="$location"
        echo "Found WASM file at: $WASM_FILE"
        break
    fi
done

if [ -z "$WASM_FILE" ]; then
    echo "❌ WASM compilation failed - movement_logic.wasm not found!"
    echo "Searched in the following locations:"
    for location in "${POSSIBLE_LOCATIONS[@]}"; do
        echo "  - $location"
    done
    exit 1
fi

# Create pkg directory if it doesn't exist
mkdir -p pkg

# Generate JavaScript bindings with wasm-bindgen
echo "Generating JavaScript bindings..."
wasm-bindgen "$WASM_FILE" \
    --out-dir pkg \
    --out-name movement_logic \
    --target web \
    --typescript \
    --debug

# Optimize the WASM binary if wasm-opt is available
if command -v wasm-opt &> /dev/null; then
    echo "Optimizing WASM binary with wasm-opt..."
    wasm-opt -Oz pkg/movement_logic_bg.wasm -o pkg/movement_logic_bg.wasm
else
    echo "⚠️  wasm-opt not found. Install binaryen for WASM optimization:"
    echo "   - Ubuntu/Debian: sudo apt install binaryen"
    echo "   - macOS: brew install binaryen"
    echo "   - Windows: Download from https://github.com/WebAssembly/binaryen/releases"
fi

# Create a package.json for the generated package
echo "Creating package.json..."
cat > pkg/package.json << EOF
{
  "name": "movement-logic",
  "version": "0.1.0",
  "description": "WebAssembly movement logic for pushedpeople game",
  "main": "movement_logic.js",
  "types": "movement_logic.d.ts",
  "files": [
    "movement_logic.js",
    "movement_logic_bg.wasm",
    "movement_logic.d.ts",
    "movement_logic_bg.wasm.d.ts"
  ],
  "module": "movement_logic.js",
  "sideEffects": [
    "./movement_logic.js",
    "./snippets/*"
  ]
}
EOF

# Create a simple README for the package
echo "Creating package README..."
cat > pkg/README.md << EOF
# Movement Logic WASM

WebAssembly module for game movement logic including:

- Air acceleration (Source Engine style)
- Player movement with ground projection
- Gravity force calculations
- Vector3 mathematics

## Usage

\`\`\`javascript
import init, { 
  Vector3, 
  airAccelerate, 
  playerMove, 
  gravityForce, 
  gravityForceAcceleration 
} from './movement_logic.js';

// Initialize the WASM module
await init();

// Create vectors
const velocity = new Vector3(0, 0, 0);
const gravity = new Vector3(0, -9.8, 0);

// Apply gravity
const gravityAccel = gravityForceAcceleration(gravity, 0.016);
velocity.x += gravityAccel.x;
velocity.y += gravityAccel.y;
velocity.z += gravityAccel.z;
\`\`\`

Built with direct wasm-bindgen workflow (post wasm-pack deprecation).
EOF

# Check if build was successful
if [ -f "pkg/movement_logic.js" ] && [ -f "pkg/movement_logic_bg.wasm" ]; then
    echo "✅ Direct wasm-bindgen build successful!"
    echo "Generated files:"
    echo "  - pkg/movement_logic.js (JavaScript bindings)"
    echo "  - pkg/movement_logic_bg.wasm (WebAssembly module)"
    echo "  - pkg/movement_logic.d.ts (TypeScript definitions)"
    echo "  - pkg/movement_logic_bg.wasm.d.ts (WASM TypeScript definitions)"
    echo "  - pkg/package.json (NPM package metadata)"
    echo "  - pkg/README.md (Package documentation)"
else
    echo "❌ Direct wasm-bindgen build failed!"
    exit 1
fi

echo ""
echo "🎉 Migration to direct wasm-bindgen complete!"
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
