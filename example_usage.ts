// Example usage of movement logic WASM module in TypeScript/JavaScript
// This file demonstrates how to use the compiled WASM air acceleration, player movement, and gravity functions

import init, { Vector3, airAccelerate, playerMove, gravityForce, gravityForceAcceleration, RayCollisionHit } from './pkg/movement_logic.js';

/**
 * Movement controller class that uses the WASM air acceleration
 */
export class MovementController {
    private wasmInitialized = false;
    
    /**
     * Initialize the WASM module
     */
    async init() {
        if (!this.wasmInitialized) {
            await init();
            this.wasmInitialized = true;
        }
    }
    
    /**
     * Apply Source Engine-like air acceleration to a player's velocity
     * @param velocity Current velocity vector (will be modified)
     * @param wishDirection Desired movement direction (should be normalized)
     * @param wishSpeed Desired movement speed
     * @param airAccelerateValue Air acceleration setting (typically 10.0)
     * @param maxAirWishSpeed Maximum air wish speed (typically 30.0)
     * @param deltaTime Time since last update in seconds
     */
    applyAirAcceleration(
        velocity: Vector3,
        wishDirection: Vector3,
        wishSpeed: number,
        airAccelerateValue: number = 10.0,
        maxAirWishSpeed: number = 30.0,
        deltaTime: number = 0.016 // ~60 FPS
    ) {
        if (!this.wasmInitialized) {
            throw new Error('WASM module not initialized. Call init() first.');
        }
        
        airAccelerate(
            velocity,
            wishDirection,
            wishSpeed,
            airAccelerateValue,
            maxAirWishSpeed,
            deltaTime
        );
    }
    
    /**
     * Apply player movement with ground projection and speed modifiers
     * @param direction Input movement direction
     * @param deltaTime Time since last update in seconds
     * @param sprint Whether the player is sprinting
     * @param crouch Whether the player is crouching
     * @param groundHit Optional ground collision information
     * @returns The final movement vector to apply to position
     */
    applyPlayerMovement(
        direction: Vector3,
        deltaTime: number,
        sprint: boolean = false,
        crouch: boolean = false,
        groundHit?: RayCollisionHit | null
    ): Vector3 {
        if (!this.wasmInitialized) {
            throw new Error('WASM module not initialized. Call init() first.');
        }
        
        return playerMove(direction, deltaTime, sprint, crouch, groundHit);
    }
    
    /**
     * Apply gravity force to a gravity force vector (modifies in place)
     * @param gravityForceVector Current gravity force vector (will be modified)
     * @param gravityVector Gravity acceleration vector (typically (0, -9.8, 0))
     * @param deltaTime Time since last update in seconds
     * @returns The updated gravity force vector
     */
    applyGravityForce(
        gravityForceVector: Vector3,
        gravityVector: Vector3,
        deltaTime: number
    ): Vector3 {
        if (!this.wasmInitialized) {
            throw new Error('WASM module not initialized. Call init() first.');
        }
        
        return gravityForce(gravityForceVector, gravityVector, deltaTime);
    }
    
    /**
     * Calculate gravity force acceleration without modifying input
     * @param gravityVector Gravity acceleration vector (typically (0, -9.8, 0))
     * @param deltaTime Time since last update in seconds
     * @returns The gravity acceleration vector to add to velocity
     */
    calculateGravityAcceleration(
        gravityVector: Vector3,
        deltaTime: number
    ): Vector3 {
        if (!this.wasmInitialized) {
            throw new Error('WASM module not initialized. Call init() first.');
        }
        
        return gravityForceAcceleration(gravityVector, deltaTime);
    }
    
    /**
     * Calculate wish direction based on input keys
     * This is a typical implementation for WASD movement
     */
    calculateWishDirection(
        forward: boolean,
        backward: boolean,
        left: boolean,
        right: boolean,
        cameraForward: Vector3,
        cameraRight: Vector3
    ): Vector3 {
        let wishDirection = new Vector3(0, 0, 0);
        
        if (forward) {
            wishDirection.x += cameraForward.x;
            wishDirection.y += cameraForward.y;
            wishDirection.z += cameraForward.z;
        }
        if (backward) {
            // Note: We subtract here, but in WASM we need to add the negative
            const negativeForward = {x: cameraForward.x * -1, y: cameraForward.y * -1, z: cameraForward.z * -1};
            wishDirection.x += negativeForward.x;
            wishDirection.y += negativeForward.y;
            wishDirection.z += negativeForward.z;
        }
        if (right) {
            wishDirection.x += cameraRight.x;
            wishDirection.y += cameraRight.y;
            wishDirection.z += cameraRight.z;
        }
        if (left) {
            const negativeRight = {x: cameraRight.x * -1, y: cameraRight.y * -1, z: cameraRight.z * -1};
            wishDirection.x += negativeRight.x;
            wishDirection.y += negativeRight.y;
            wishDirection.z += negativeRight.z;
        }
        return {x: wishDirection.x, y: wishDirection.y, z: wishDirection.z};
        // Normalize the wish direction
        //new THREE.Vector3(wishDirection.x, wishDirection.y, wishDirection.z).normalize();
    }
}

/**
 * Example usage in a game loop
 */
export async function exampleUsage() {
    const movementController = new MovementController();
    await movementController.init();
    
    // Player state
    let playerVelocity = new Vector3(0, 0, 0);
    
    // Camera vectors (these would come from your camera system)
    const cameraForward = new Vector3(1, 0, 0);
    const cameraRight = new Vector3(0, 0, 1);
    
    // Input state (these would come from your input system)
    const input = {
        forward: false,
        backward: false,
        left: true,  // Player is pressing A (strafe left)
        right: false
    };
    
    // Calculate wish direction based on input
    const wishDirection = movementController.calculateWishDirection(
        input.forward,
        input.backward,
        input.left,
        input.right,
        cameraForward,
        cameraRight
    );
    
    console.log(`Player velocity after air strafe: x=${playerVelocity.x}, y=${playerVelocity.y}, z=${playerVelocity.z}`);
}

/**
 * Example usage of playerMove function for ground movement
 */
export async function examplePlayerMovement() {
    const movementController = new MovementController();
    await movementController.init();
    
    // Input direction (e.g., from WASD keys)
    const inputDirection = new Vector3(1, 0, 0); // Moving forward
    
    // Ground collision information (from physics engine)
    const groundNormal = new Vector3(0, 1, 0); // Flat ground pointing up
    const groundHit = new RayCollisionHit(0, 1, 0, 1.0); // Normal + distance
    
    // Apply player movement with different modifiers
    console.log("=== Player Movement Examples ===");
    
    // Normal walking
    const normalMovement = movementController.applyPlayerMovement(
        inputDirection,
        0.016,  // 60 FPS
        false,  // not sprinting
        false,  // not crouching
        groundHit
    );
    console.log(`Normal walk: x=${normalMovement.x.toFixed(3)}, y=${normalMovement.y.toFixed(3)}, z=${normalMovement.z.toFixed(3)}`);
    
    // Sprinting
    const sprintMovement = movementController.applyPlayerMovement(
        inputDirection,
        0.016,  // 60 FPS
        true,   // sprinting
        false,  // not crouching
        groundHit
    );
    console.log(`Sprint: x=${sprintMovement.x.toFixed(3)}, y=${sprintMovement.y.toFixed(3)}, z=${sprintMovement.z.toFixed(3)}`);
    
    // Crouching
    const crouchMovement = movementController.applyPlayerMovement(
        inputDirection,
        0.016,  // 60 FPS
        false,  // not sprinting
        true,   // crouching
        groundHit
    );
    console.log(`Crouch: x=${crouchMovement.x.toFixed(3)}, y=${crouchMovement.y.toFixed(3)}, z=${crouchMovement.z.toFixed(3)}`);
    
    // Movement on a slope
    //const slopeNormal = new Vector3(0.3, 0.9, 0).normalized(); // 30-degree slope
    //const slopeHit = new RayCollisionHit(slopeNormal.x, slopeNormal.y, slopeNormal.z, 1.0);
    
    // const slopeMovement = movementController.applyPlayerMovement(
    //     inputDirection,
    //     0.016,  // 60 FPS
    //     false,  // not sprinting
    //     false,  // not crouching
    //     slopeHit
    // );
    // console.log(`Slope walk: x=${slopeMovement.x.toFixed(3)}, y=${slopeMovement.y.toFixed(3)}, z=${slopeMovement.z.toFixed(3)}`);
    
    // Air movement (no ground)
    const airMovement = movementController.applyPlayerMovement(
        inputDirection,
        0.016,  // 60 FPS
        false,  // not sprinting
        false,  // not crouching
        null    // no ground
    );
    console.log(`Air movement: x=${airMovement.x.toFixed(3)}, y=${airMovement.y.toFixed(3)}, z=${airMovement.z.toFixed(3)}`);
}

/**
 * Advanced example with realistic physics parameters
 */
export class SourceEngineMovement {
    private movementController = new MovementController();
    
    // Source Engine movement constants
    private readonly AIR_ACCELERATE = 10.0;
    private readonly MAX_AIR_WISH_SPEED = 30.0;
    private readonly GROUND_ACCELERATE = 10.0;
    private readonly MAX_GROUND_SPEED = 320.0;
    private readonly FRICTION = 4.0;
    private readonly STOP_SPEED = 100.0;
    
    async init() {
        await this.movementController.init();
    }
    
    /**
     * Complete movement update including ground friction and air movement
     */
    updateMovement(
        velocity: Vector3,
        wishDirection: Vector3,
        wishSpeed: number,
        onGround: boolean,
        deltaTime: number
    ) {
        if (onGround) {
            this.applyGroundFriction(velocity, deltaTime);
            this.applyGroundAcceleration(velocity, wishDirection, wishSpeed, deltaTime);
        } else {
            this.movementController.applyAirAcceleration(
                velocity,
                wishDirection,
                wishSpeed,
                this.AIR_ACCELERATE,
                this.MAX_AIR_WISH_SPEED,
                deltaTime
            );
        }
    }
    
    private applyGroundFriction(velocity: Vector3, deltaTime: number) {
        // const speed = velocity.magnitude();
        // if (speed < 0.1) return;
        
        // const drop = Math.max(this.STOP_SPEED, speed) * this.FRICTION * deltaTime;
        // const newSpeed = Math.max(0, speed - drop);
        
        // if (newSpeed !== speed) {
        //     const scale = newSpeed / speed;
        //     velocity.x = velocity.x * scale;
        //     velocity.y = velocity.y * scale;
        //     velocity.z = velocity.z * scale;
        // }
    }
    
    private applyGroundAcceleration(
        velocity: Vector3,
        wishDirection: Vector3,
        wishSpeed: number,
        deltaTime: number
    ) {
        // const currentSpeed = velocity.dot(wishDirection);
        // const addSpeed = Math.min(wishSpeed, this.MAX_GROUND_SPEED) - currentSpeed;
        
        // if (addSpeed <= 0) return;
        
        // const accelSpeed = Math.min(addSpeed, this.GROUND_ACCELERATE * deltaTime * wishSpeed);
        // const acceleration = wishDirection.multiplyScalar(accelSpeed);
        
        // velocity.add(acceleration);
    }
}

/**
 * Example usage of gravity force functions
 */
export async function exampleGravityForce() {
    const movementController = new MovementController();
    await movementController.init();
    
    console.log("=== Gravity Force Examples ===");
    
    // Standard Earth gravity
    const gravity = new Vector3(0, -9.8, 0);
    const deltaTime = 0.016; // 60 FPS
    
    // Example 1: Using gravity force with accumulation (modifies in place)
    let gravityForceVector = new Vector3(0, 0, 0);
    
    console.log("Initial gravity force:", `x=${gravityForceVector.x}, y=${gravityForceVector.y}, z=${gravityForceVector.z}`);
    
    // Apply gravity for 5 frames to see accumulation
    for (let frame = 0; frame < 5; frame++) {
        const result = movementController.applyGravityForce(gravityForceVector, gravity, deltaTime);
        console.log(`Frame ${frame + 1} - Gravity force: x=${result.x.toFixed(6)}, y=${result.y.toFixed(6)}, z=${result.z.toFixed(6)}`);
    }
    
    // Example 2: Using gravity force acceleration (non-mutating)
    console.log("\n--- Using Gravity Acceleration (non-mutating) ---");
    
    let velocity = new Vector3(5, 2, -3); // Some initial velocity
    console.log("Initial velocity:", `x=${velocity.x}, y=${velocity.y}, z=${velocity.z}`);
    
    // Apply gravity acceleration for several frames
    for (let frame = 0; frame < 10; frame++) {
        const gravityAccel = movementController.calculateGravityAcceleration(gravity, deltaTime);
        
        // Add acceleration to velocity (you would do this in your physics system)
        velocity.x += gravityAccel.x;
        velocity.y += gravityAccel.y;
        velocity.z += gravityAccel.z;
        
        if (frame % 2 === 0) { // Log every other frame
            console.log(`Frame ${frame + 1} - Velocity: x=${velocity.x.toFixed(3)}, y=${velocity.y.toFixed(3)}, z=${velocity.z.toFixed(3)}, Gravity accel: y=${gravityAccel.y.toFixed(6)}`);
        }
    }
    
    // Example 3: Different gravity environments
    console.log("\n--- Different Gravity Environments ---");
    
    const moonGravity = new Vector3(0, -1.62, 0); // Moon gravity
    const marsGravity = new Vector3(0, -3.71, 0); // Mars gravity
    
    const environments = [
        { name: "Earth", gravity: new Vector3(0, -9.8, 0) },
        { name: "Moon", gravity: moonGravity },
        { name: "Mars", gravity: marsGravity },
        { name: "Jupiter", gravity: new Vector3(0, -24.79, 0) }
    ];
    
    environments.forEach(env => {
        const accel = movementController.calculateGravityAcceleration(env.gravity, deltaTime);
        console.log(`${env.name} gravity acceleration per frame: y=${accel.y.toFixed(6)}`);
    });
    
    // Example 4: Gravity in different directions (for special game mechanics)
    console.log("\n--- Non-standard Gravity Directions ---");
    
    const sidewaysGravity = new Vector3(-9.8, 0, 0); // Gravity pulling to the left
    const diagonalGravity = new Vector3(-4.9, -8.5, 0); // Gravity pulling down-left
    
    [sidewaysGravity, diagonalGravity].forEach((grav, i) => {
        const accel = movementController.calculateGravityAcceleration(grav, deltaTime);
        const name = i === 0 ? "Sideways" : "Diagonal";
        console.log(`${name} gravity acceleration: x=${accel.x.toFixed(6)}, y=${accel.y.toFixed(6)}, z=${accel.z.toFixed(6)}`);
    });
}

// Run examples when this module is imported
// exampleUsage();
// examplePlayerMovement();
// exampleGravityForce();