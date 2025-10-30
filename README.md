# Movement Logic

A Rust crate that provides Source Engine-like air acceleration physics and ground movement with surface projection, compiled for both server-side Rust and client-side WebAssembly (WASM).

## Features

- **Source Engine Air Acceleration**: Faithful implementation of Source Engine's air strafing mechanics
- **Ground Movement with Surface Projection**: Walk properly on slopes and uneven terrain
- **Movement Speed Modifiers**: Sprint and crouch functionality
- **Dual Target Support**: Works both in native Rust (for server) and WebAssembly (for client)
- **Vector3 Implementation**: Complete 3D vector operations with WASM bindings
- **Type-Safe**: Full Rust type safety with optional TypeScript definitions for WASM
- **Well-Tested**: Comprehensive unit tests for all functionality

## Quick Start

### Server-Side Usage (Rust)

```rust
use movement_logic::{Vector3, air_accelerate_native, player_move_native, RayCollisionHit};

// Air acceleration example
let mut velocity = Vector3::new(0.0, 0.0, 0.0);
let wish_direction = Vector3::new(1.0, 0.0, 0.0); // normalized
let wish_speed = 100.0;
let air_accelerate = 10.0;
let max_air_wish_speed = 30.0;
let delta_time = 0.016; // 60 FPS

air_accelerate_native(
    &mut velocity,
    &wish_direction,
    wish_speed,
    air_accelerate,
    max_air_wish_speed,
    delta_time,
);

// Ground movement example
let input_direction = Vector3::new(1.0, 0.0, 0.0);
let ground_normal = Vector3::new(0.0, 1.0, 0.0);
let ground_hit = Some(RayCollisionHit::new_native(ground_normal, 1.0));

let movement = player_move_native(
    &input_direction,
    delta_time,
    false, // not sprinting
    false, // not crouching
    ground_hit,
);
```

### Client-Side Usage (WASM + JavaScript/TypeScript)

1. Build the WASM module:
```bash
./build_wasm.sh
```

2. Use in your JavaScript/TypeScript:
```typescript
import init, { Vector3, airAccelerate, playerMove, RayCollisionHit } from './pkg/movement_logic.js';

// Initialize WASM module
await init();

// Air acceleration
let velocity = new Vector3(0, 0, 0);
let wishDirection = new Vector3(1, 0, 0);
airAccelerate(velocity, wishDirection, 100, 10, 30, 0.016);

// Ground movement
let inputDirection = new Vector3(1, 0, 0);
let groundHit = new RayCollisionHit(0, 1, 0, 1.0); // normal + distance
let movement = playerMove(inputDirection, 0.016, false, false, groundHit);
```

## Movement Physics

### Air Acceleration

This implementation follows Source Engine's air acceleration algorithm:

1. **Speed Clamping**: Wish speed is clamped to maximum air wish speed
2. **Current Speed Calculation**: Project current velocity onto wish direction
3. **Add Speed Calculation**: Determine how much speed to add
4. **Acceleration Limiting**: Limit acceleration based on air accelerate value
5. **Velocity Update**: Add calculated acceleration to current velocity

### Ground Movement with Surface Projection

The `playerMove` function handles:

1. **Surface Projection**: Projects movement direction onto ground plane for proper slope walking
2. **Length Preservation**: Maintains original input magnitude after projection
3. **Speed Modifiers**: Applies sprint (1.5x) and crouch (0.67x) multipliers
4. **Air Movement**: Falls back to direct movement when no ground is detected

#### Parameters

**Air Acceleration:**
- `wish_speed`: Desired movement speed (typically based on player input)
- `air_accelerate`: Air acceleration multiplier (typically 10.0 in Source games)
- `max_air_wish_speed`: Maximum wish speed while airborne (typically 30.0)
- `delta_time`: Time elapsed since last update (in seconds)

**Player Movement:**
- `direction`: Input movement direction vector
- `delta_time`: Time elapsed since last update (in seconds)
- `sprint`: Whether player is sprinting (1.5x speed)
- `crouch`: Whether player is crouching (0.67x speed)
- `down_ray_hit`: Optional ground collision with normal and distance

## Vector3 Operations

The `Vector3` struct provides:

- Basic arithmetic (add, subtract, multiply by scalar)
- Dot product calculation
- Magnitude and normalization
- Component access (x, y, z)
- Plane projection for surface walking
- Length operations (get/set)
- Clone and copy operations
- WASM-compatible bindings

## RayCollisionHit

Represents collision information for ground detection:

- `normal`: Surface normal vector at collision point
- `distance`: Distance from ray origin to collision point
- Used for surface projection in `playerMove`

## Building

### Native Rust
```bash
cargo build
cargo test
```

### WebAssembly
```bash
# Install wasm-pack if needed
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build WASM package
./build_wasm.sh
```

## Integration with Server

The crate integrates seamlessly with the server's `DbVector3` type through utility functions:

```rust
use movement_logic::Vector3;
use crate::movement_utils::MovementUtils;

let mut db_velocity = DbVector3::new(0.0, 0.0, 0.0);
let db_direction = DbVector3::new(1.0, 0.0, 0.0);

MovementUtils::apply_air_acceleration(
    &mut db_velocity,
    &db_direction,
    100.0, 10.0, 30.0, 0.016
);
```

## Files

- `src/lib.rs`: Main library implementation
- `build_wasm.sh`: WASM build script
- `example_usage.ts`: TypeScript usage examples
- `Cargo.toml`: Rust package configuration

## Generated WASM Files

After running `build_wasm.sh`:

- `pkg/movement_logic.js`: JavaScript bindings
- `pkg/movement_logic_bg.wasm`: WebAssembly binary
- `pkg/movement_logic.d.ts`: TypeScript definitions
- `pkg/package.json`: NPM package definition

## Physics Constants

Typical Source Engine values:

- Air Accelerate: `10.0`
- Max Air Wish Speed: `30.0`
- Ground Accelerate: `10.0`
- Friction: `4.0`
- Stop Speed: `100.0`

## Testing

Run the test suite:
```bash
cargo test
```

Tests cover:
- Basic air acceleration functionality
- Speed clamping behavior
- Vector operations
- Integration with server types

## Performance

- **Native Rust**: Zero-cost abstractions, optimal performance
- **WASM**: Near-native performance in web browsers
- **Memory**: Minimal allocation, stack-based operations
- **Deterministic**: Identical results across platforms

## License

This project uses the same license as the parent project.
