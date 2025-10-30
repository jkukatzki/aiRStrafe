#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

// This is like the `extern` block in C.
#[cfg(feature = "wasm")]
#[wasm_bindgen]
extern "C" {
    // Bind the `console.log` function from the browser
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// A 3D vector struct that can be used both in Rust and exported to JavaScript/WASM
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl Vector3 {
    #[wasm_bindgen(constructor)]
    pub fn new_wasm(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    #[wasm_bindgen(getter = x)]
    pub fn get_x(&self) -> f32 {
        self.x
    }

    #[wasm_bindgen(getter = y)]
    pub fn get_y(&self) -> f32 {
        self.y
    }

    #[wasm_bindgen(getter = z)]
    pub fn get_z(&self) -> f32 {
        self.z
    }

    #[wasm_bindgen(setter = x)]
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    #[wasm_bindgen(setter = y)]
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    #[wasm_bindgen(setter = z)]
    pub fn set_z(&mut self, z: f32) {
        self.z = z;
    }
}

/// Represents a ray collision intersection (simplified version for WASM compatibility)
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone, Copy)]
pub struct RayCollisionHit {
    /// The normal vector at the intersection point
    normal: Vector3,
    /// Distance from ray origin to intersection point
    distance: f32,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl RayCollisionHit {
    #[wasm_bindgen(constructor)]
    pub fn new(normal_x: f32, normal_y: f32, normal_z: f32, distance: f32) -> RayCollisionHit {
        RayCollisionHit {
            normal: Vector3::new(normal_x, normal_y, normal_z),
            distance,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn normal(&self) -> Vector3 {
        self.normal
    }

    #[wasm_bindgen(getter)]
    pub fn distance(&self) -> f32 {
        self.distance
    }
}

/// Additional methods for RayCollisionHit (available for WASM too)
#[cfg(feature = "wasm")]
impl RayCollisionHit {
    /// Create a new RayCollisionHit (Rust-native version)
    pub fn new_native(normal: Vector3, distance: f32) -> RayCollisionHit {
        RayCollisionHit { normal, distance }
    }
    
    /// Get the normal vector (Rust-native)
    pub fn normal_native(&self) -> &Vector3 {
        &self.normal
    }
    
    /// Get the distance (Rust-native)
    pub fn distance_native(&self) -> f32 {
        self.distance
    }
}

/// Rust-native implementation for RayCollisionHit
#[cfg(not(feature = "wasm"))]
impl RayCollisionHit {
    /// Create a new RayCollisionHit (native constructor)
    pub fn new(normal_x: f32, normal_y: f32, normal_z: f32, distance: f32) -> RayCollisionHit {
        RayCollisionHit {
            normal: Vector3::new(normal_x, normal_y, normal_z),
            distance,
        }
    }

    /// Create a new RayCollisionHit (Rust-native version)
    pub fn new_native(normal: Vector3, distance: f32) -> RayCollisionHit {
        RayCollisionHit { normal, distance }
    }
    
    /// Get the normal vector (Rust-native)
    pub fn normal_native(&self) -> &Vector3 {
        &self.normal
    }
    
    /// Get the distance (Rust-native)
    pub fn distance_native(&self) -> f32 {
        self.distance
    }
}

/// Player movement function that handles ground projection and movement modifiers
/// 
/// This function processes player input direction, projects it onto the ground surface
/// if the player is on the ground, and applies the provided speed modifier.
/// 
/// # Arguments
/// * `direction` - The input movement direction vector (will be modified)
/// * `delta_time` - Time since last update in seconds
/// * `speed_multiplier` - Speed multiplier to apply (1.0 = normal speed, 1.5 = 50% faster, 0.67 = 33% slower, etc.)
/// * `down_ray_hit` - Optional ground collision information
/// 
/// # Returns
/// The final movement vector to apply to the player's position
#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = playerMove)]
pub fn player_move(
    direction: &Vector3,
    delta_time: f32,
    speed_multiplier: f32,
    down_ray_hit: Option<RayCollisionHit>,
) -> Vector3 {
    player_move_core(direction, delta_time, speed_multiplier, down_ray_hit)
}

/// Core player movement function used by both WASM and native versions
pub fn player_move_core(
    direction: &Vector3,
    delta_time: f32,
    speed_multiplier: f32,
    down_ray_hit: Option<RayCollisionHit>,
) -> Vector3 {
    let mut final_direction = direction.clone();
    
    // If we hit the ground, project the movement direction onto the ground plane
    if let Some(hit) = down_ray_hit {
        let projected = direction.project_on_plane(hit.normal_native());
        
        // Check to avoid issues with zero-length projected vectors
        if projected.length_sq() > 0.0 {
            // Preserve the original direction's magnitude
            let original_length = direction.length();
            final_direction = projected;
            final_direction.set_length(original_length);
        }
        // If projection results in zero vector, keep original direction
    }
    
    // Apply delta time and speed modifier
    final_direction.multiply_scalar(delta_time * speed_multiplier)
}

/// Rust-native version of player_move for server use
pub fn player_move_native(
    direction: &Vector3,
    delta_time: f32,
    speed_multiplier: f32,
    down_ray_hit: Option<RayCollisionHit>,
) -> Vector3 {
    player_move_core(direction, delta_time, speed_multiplier, down_ray_hit)
}

/// Apply gravity force to a gravity force vector
/// 
/// This function applies gravity acceleration using squared delta time and a downscale factor.
/// The gravity force vector is modified in place.
/// 
/// # Arguments
/// * `gravity_influence` - The current gravity force vector (will be modified)
/// * `gravity` - The gravity acceleration vector
/// * `delta` - Time since last update in seconds
/// 
/// # Returns
/// The updated gravity force vector
#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = gravityInfluence)]
pub fn gravity_influence(
    gravity_influence: &mut Vector3,
    gravity: &Vector3,
    delta: f32,
) -> Vector3 {
    gravity_influence_core(gravity_influence, gravity, delta)
}

/// Core gravity force function used by both WASM and native versions
pub fn gravity_influence_core(
    gravity_influence: &mut Vector3,
    gravity: &Vector3,
    delta: f32,
) -> Vector3 {
    const GRAVITY_DOWN_SCALE: f32 = 0.0875;
    let delta_sqrd = delta * delta;
    let scale_factor = delta_sqrd * GRAVITY_DOWN_SCALE;
    
    gravity_influence.x += gravity.x * scale_factor;
    gravity_influence.y += gravity.y * scale_factor;
    gravity_influence.z += gravity.z * scale_factor;
    
    *gravity_influence
}

/// Calculate gravity force acceleration without modifying the input
/// 
/// This function calculates the gravity force acceleration to add without
/// modifying the input gravity force vector. Useful when you want to get
/// the acceleration to add separately.
/// 
/// # Arguments
/// * `gravity` - The gravity acceleration vector
/// * `delta` - Time since last update in seconds
/// 
/// # Returns
/// The gravity force acceleration vector to add
#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = gravityForceAcceleration)]
pub fn gravity_influence_acceleration(
    gravity: &Vector3,
    delta: f32,
) -> Vector3 {
    const GRAVITY_DOWN_SCALE: f32 = 0.0875;
    let delta_sqrd = delta * delta;
    let scale_factor = delta_sqrd * GRAVITY_DOWN_SCALE;
    
    Vector3::new(
        gravity.x * scale_factor,
        gravity.y * scale_factor,
        gravity.z * scale_factor,
    )
}

/// Native version for non-WASM targets
pub fn gravity_influence_acceleration_native(
    gravity: &Vector3,
    delta: f32,
) -> Vector3 {
    const GRAVITY_DOWN_SCALE: f32 = 0.0875;
    let delta_sqrd = delta * delta;
    let scale_factor = delta_sqrd * GRAVITY_DOWN_SCALE;
    
    Vector3::new(
        gravity.x * scale_factor,
        gravity.y * scale_factor,
        gravity.z * scale_factor,
    )
}
/// Air acceleration function that returns the acceleration vector to add
/// 
/// This function calculates the acceleration vector for air movement without
/// modifying the input velocity. Perfect for WASM where you want to add the
/// result to your existing velocity vector.
/// 
/// # Arguments
/// * `current_vel` - The current velocity vector (not modified)
/// * `wish_dir` - The desired movement direction (should be normalized)
/// * `wish_speed` - The desired movement speed
/// * `air_accelerate` - Air acceleration value (typically around 10.0)
/// * `max_air_wish_speed` - Maximum air wish speed (typically around 30.0)
/// * `delta_time` - Time since last update in seconds
/// 
/// # Returns
/// The acceleration vector to add to the current velocity
#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = airAccelerate)]
pub fn air_accelerate(
    current_vel: &Vector3,
    wish_dir: &Vector3,
    wish_speed: f32,
    air_accelerate: f32,
    max_air_wish_speed: f32,
    delta_time: f32,
) -> Vector3 {
    air_accelerate_get_acceleration(
        current_vel,
        wish_dir,
        wish_speed,
        air_accelerate,
        max_air_wish_speed,
        delta_time,
    )
}

/// Rust-native version that can either mutate velocity or return acceleration
/// This provides both the old mutating API for server compatibility and
/// the new return-value API for consistency
pub fn air_accelerate_native(
    current_vel: &mut Vector3,
    wish_dir: &Vector3,
    wish_speed: f32,
    air_accelerate: f32,
    max_air_wish_speed: f32,
    delta_time: f32,
) {
    // Calculate the acceleration vector
    let acceleration = air_accelerate_get_acceleration(
        current_vel,
        wish_dir,
        wish_speed,
        air_accelerate,
        max_air_wish_speed,
        delta_time,
    );
    
    // Apply it to the velocity
    current_vel.add(&acceleration);
}

/// Get the air acceleration vector without modifying velocity
/// This implements air movement with proper velocity capping as used in modern movement systems
/// Based on the provided pseudocode with sophisticated velocity projection and capping
pub fn air_accelerate_get_acceleration(
    current_vel: &Vector3,
    wish_dir: &Vector3,
    _wish_speed: f32, // Not used in this implementation, kept for API compatibility
    air_accelerate: f32,
    max_air_wish_speed: f32,
    delta_time: f32,
) -> Vector3 {
    // Project the current velocity onto the movement direction
    let proj_vel = project_vector_onto_vector(current_vel, wish_dir);
    
    // Check if the movement direction is moving towards or away from the projected velocity
    let is_away = wish_dir.dot(&proj_vel) <= 0.0;
    
    // Only apply force if moving away from velocity or velocity is below max air speed
    if proj_vel.magnitude() < max_air_wish_speed || is_away {
        // Calculate the ideal movement force
        let wish_dir_normalized = wish_dir.normalized();
        let mut velocity_change = wish_dir_normalized.multiply_scalar(air_accelerate * delta_time);
        
        // Cap it based on whether we're moving towards or away from current velocity
        if !is_away {
            // Moving towards current velocity - cap to not exceed max air speed
            let max_change = max_air_wish_speed - proj_vel.magnitude();
            velocity_change = clamp_vector_magnitude(velocity_change, max_change);
        } else {
            // Moving away from current velocity - allow more acceleration
            let max_change = max_air_wish_speed + proj_vel.magnitude();
            velocity_change = clamp_vector_magnitude(velocity_change, max_change);
        }
        
        velocity_change
    } else {
        // No acceleration when at max speed and not moving away
        Vector3::new(0.0, 0.0, 0.0)
    }
}

/// Project vector A onto vector B: proj_B(A) = (A · B / |B|²) * B
fn project_vector_onto_vector(a: &Vector3, b: &Vector3) -> Vector3 {
    let b_magnitude_sq = b.length_sq();
    if b_magnitude_sq == 0.0 {
        return Vector3::new(0.0, 0.0, 0.0);
    }
    
    let dot_product = a.dot(b);
    let scalar = dot_product / b_magnitude_sq;
    b.multiply_scalar(scalar)
}

/// Clamp a vector's magnitude to a maximum value
fn clamp_vector_magnitude(vector: Vector3, max_magnitude: f32) -> Vector3 {
    let current_magnitude = vector.magnitude();
    if current_magnitude <= max_magnitude || max_magnitude <= 0.0 {
        vector
    } else {
        vector.multiply_scalar(max_magnitude / current_magnitude)
    }
}

/// Rust-native implementation (not exported to WASM)
impl Vector3 {
    /// Create a new Vector3 (native constructor)
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    /// Direct field access for Rust code
    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn z(&self) -> f32 { self.z }
    
    /// Mutable field access for Rust code
    pub fn set_x_native(&mut self, x: f32) { self.x = x; }
    pub fn set_y_native(&mut self, y: f32) { self.y = y; }
    pub fn set_z_native(&mut self, z: f32) { self.z = z; }
    
    /// Calculate the dot product of this vector with another
    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Add another vector to this vector (mutating)
    pub fn add(&mut self, other: &Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    /// Multiply this vector by a scalar and return a new vector
    pub fn multiply_scalar(&self, scalar: f32) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    /// Get the magnitude (length) of the vector
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Get a normalized version of this vector
    pub fn normalized(&self) -> Vector3 {
        let mag = self.magnitude();
        if mag > 0.0 {
            Vector3 {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        } else {
            Vector3 { x: 0.0, y: 0.0, z: 0.0 }
        }
    }

    /// Get the squared magnitude (length squared) of the vector - faster than magnitude()
    pub fn length_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Set the length of this vector to a specific value (mutating)
    pub fn set_length(&mut self, length: f32) {
        let current_mag = self.magnitude();
        if current_mag > 0.0 {
            let scale = length / current_mag;
            self.x *= scale;
            self.y *= scale;
            self.z *= scale;
        }
    }

    /// Copy values from another vector (mutating)
    pub fn copy(&mut self, other: &Vector3) {
        self.x = other.x;
        self.y = other.y;
        self.z = other.z;
    }

    /// Clone this vector
    pub fn clone(&self) -> Vector3 {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    /// Project this vector onto a plane defined by its normal
    pub fn project_on_plane(&self, plane_normal: &Vector3) -> Vector3 {
        // Formula: v - (v · n) * n
        // where v is the vector, n is the plane normal
        let dot = self.dot(plane_normal);
        let projection = plane_normal.multiply_scalar(dot);
        Vector3 {
            x: self.x - projection.x,
            y: self.y - projection.y,
            z: self.z - projection.z,
        }
    }

    /// Get the length of this vector (alias for magnitude)
    pub fn length(&self) -> f32 {
        self.magnitude()
    }
    
    /// Rust-native plane projection (more efficient)
    pub fn project_on_plane_native(&self, plane_normal: &Vector3) -> Vector3 {
        let dot = self.dot(plane_normal);
        Vector3 {
            x: self.x - plane_normal.x * dot,
            y: self.y - plane_normal.y * dot,
            z: self.z - plane_normal.z * dot,
        }
    }
    
    /// Rust-native length setting
    pub fn set_length_native(&mut self, length: f32) {
        let current_mag = self.magnitude();
        if current_mag > 0.0 {
            let scale = length / current_mag;
            self.x *= scale;
            self.y *= scale;
            self.z *= scale;
        }
    }
}

/// Conversion functions to work with external vector types
impl Vector3 {
    /// Create from individual components - useful for interfacing with other vector types
    pub fn from_components(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }
    
    /// Get components as tuple - useful for interfacing with other vector types
    pub fn to_components(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
}

// Conditional compilation for different target architectures
#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_air_accelerate_basic() {
        let vel = Vector3::new(0.0, 0.0, 0.0);
        let wish_dir = Vector3::new(1.0, 0.0, 0.0);
        
        let acceleration = air_accelerate_get_acceleration(
            &vel,
            &wish_dir,
            100.0,  // wish_speed (not used in new implementation)
            10.0,   // air_accelerate
            30.0,   // max_air_wish_speed
            0.1,    // delta_time
        );
        
        // With new algorithm: starting from zero velocity
        // proj_vel = project(velocity=0, wish_dir) = 0
        // is_away = dot(wish_dir, proj_vel=0) <= 0 = true
        // Since is_away=true, we get: air_accelerate * delta_time = 10.0 * 0.1 = 1.0
        // Clamped by max_air_wish_speed + proj_vel.magnitude() = 30.0 + 0.0 = 30.0
        assert!((acceleration.x - 1.0).abs() < 0.001);
        assert_eq!(acceleration.y, 0.0);
        assert_eq!(acceleration.z, 0.0);
    }

    #[test] 
    fn test_air_accelerate_with_existing_velocity_same_direction() {
        // Test when already moving in the same direction as wish_dir
        let vel = Vector3::new(15.0, 0.0, 0.0); // Moving in same direction
        let wish_dir = Vector3::new(1.0, 0.0, 0.0);
        
        let acceleration = air_accelerate_get_acceleration(
            &vel,
            &wish_dir,
            100.0,  // wish_speed
            10.0,   // air_accelerate  
            30.0,   // max_air_wish_speed
            0.1,    // delta_time
        );
        
        // proj_vel = project(vel=[15,0,0], wish_dir=[1,0,0]) = [15,0,0]
        // is_away = dot([1,0,0], [15,0,0]) <= 0 = 15 <= 0 = false
        // proj_vel.magnitude() = 15.0 < max_air_wish_speed = 30.0, so we accelerate
        // Since !is_away, max_change = 30.0 - 15.0 = 15.0
        // velocity_change = normalized([1,0,0]) * 10.0 * 0.1 = [1.0, 0, 0]
        // Clamped to 15.0, so result = [1.0, 0, 0]
        assert!((acceleration.x - 1.0).abs() < 0.001);
        assert_eq!(acceleration.y, 0.0);
        assert_eq!(acceleration.z, 0.0);
    }

    #[test]
    fn test_air_accelerate_at_max_speed() {
        // Test when at max air speed in the wish direction
        let vel = Vector3::new(30.0, 0.0, 0.0); // At max speed
        let wish_dir = Vector3::new(1.0, 0.0, 0.0);
        
        let acceleration = air_accelerate_get_acceleration(
            &vel,
            &wish_dir,
            100.0,  // wish_speed
            10.0,   // air_accelerate
            30.0,   // max_air_wish_speed
            0.1,    // delta_time
        );
        
        // proj_vel.magnitude() = 30.0 = max_air_wish_speed
        // is_away = false
        // Since proj_vel.magnitude() >= max_air_wish_speed AND !is_away, no acceleration
        assert!((acceleration.x - 0.0).abs() < 0.001);
        assert!((acceleration.y - 0.0).abs() < 0.001);
        assert!((acceleration.z - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_air_accelerate_opposite_direction() {
        // Test when moving opposite to wish direction (should allow more acceleration)
        let vel = Vector3::new(-20.0, 0.0, 0.0); // Moving opposite
        let wish_dir = Vector3::new(1.0, 0.0, 0.0);
        
        let acceleration = air_accelerate_get_acceleration(
            &vel,
            &wish_dir,
            100.0,  // wish_speed
            10.0,   // air_accelerate
            30.0,   // max_air_wish_speed
            0.1,    // delta_time
        );
        
        // proj_vel = project([-20,0,0], [1,0,0]) = [-20,0,0]
        // is_away = dot([1,0,0], [-20,0,0]) <= 0 = -20 <= 0 = true
        // Since is_away=true, max_change = 30.0 + 20.0 = 50.0
        // velocity_change = [1,0,0] * 1.0 = [1.0, 0, 0]
        // Clamped to 50.0, so result = [1.0, 0, 0]
        assert!((acceleration.x - 1.0).abs() < 0.001);
        assert_eq!(acceleration.y, 0.0);
        assert_eq!(acceleration.z, 0.0);
    }

    #[test]
    fn test_air_accelerate_max_speed_clamp() {
        // Test with high air acceleration to see clamping behavior
        let vel = Vector3::new(10.0, 0.0, 0.0);
        let wish_dir = Vector3::new(1.0, 0.0, 0.0);
        
        let acceleration = air_accelerate_get_acceleration(
            &vel,
            &wish_dir,
            100.0,  // wish_speed
            100.0,  // high air_accelerate to test clamping
            30.0,   // max_air_wish_speed
            0.1,    // delta_time
        );
        
        // proj_vel = [10,0,0], magnitude = 10.0 < 30.0, so we accelerate
        // is_away = false (moving same direction)
        // max_change = 30.0 - 10.0 = 20.0
        // velocity_change = [1,0,0] * 100.0 * 0.1 = [10.0, 0, 0]
        // Clamped to 20.0, so result = [10.0, 0, 0] (but clamped magnitude is less)
        // Actually: clamp_vector_magnitude([10,0,0], 20.0) = [10,0,0] since 10 < 20
        assert!((acceleration.x - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_air_accelerate_wasm_api() {
        let vel = Vector3::new(0.0, 0.0, 0.0);
        let wish_dir = Vector3::new(1.0, 0.0, 0.0);
        
        let acceleration = air_accelerate_get_acceleration(&vel, &wish_dir, 100.0, 10.0, 30.0, 0.1);
        
        // Should return the same as the core function
        assert!(acceleration.x > 0.0);
        assert_eq!(acceleration.y, 0.0);
        assert_eq!(acceleration.z, 0.0);
    }

    #[test]
    fn test_air_accelerate_native_api() {
        let mut vel = Vector3::new(0.0, 0.0, 0.0);
        let wish_dir = Vector3::new(1.0, 0.0, 0.0);
        
        air_accelerate_native(
            &mut vel,
            &wish_dir,
            100.0,  // wish_speed
            10.0,   // air_accelerate
            30.0,   // max_air_wish_speed
            0.1,    // delta_time
        );
        
        // Should mutate the velocity directly
        assert!(vel.x > 0.0);
        assert_eq!(vel.y, 0.0);
        assert_eq!(vel.z, 0.0);
    }

    #[test]
    fn test_vector3_operations() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        
        // Test dot product
        let dot = v1.dot(&v2);
        assert_eq!(dot, 32.0); // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
        
        // Test scalar multiplication
        let scaled = v1.multiply_scalar(2.0);
        assert_eq!(scaled.x, 2.0);
        assert_eq!(scaled.y, 4.0);
        assert_eq!(scaled.z, 6.0);
        
        // Test magnitude
        let mag = Vector3::new(3.0, 4.0, 0.0).magnitude();
        assert!((mag - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_plane_projection() {
        // Test projecting a vector onto a horizontal plane (normal pointing up)
        let direction = Vector3::new(1.0, 1.0, 0.0); // Moving forward and up
        let ground_normal = Vector3::new(0.0, 1.0, 0.0); // Y-up
        
        let projected = direction.project_on_plane(&ground_normal);
        
        // Should remove the Y component, keeping only X and Z
        assert!((projected.x - 1.0).abs() < 0.001);
        assert!((projected.y - 0.0).abs() < 0.001);
        assert!((projected.z - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_player_move_basic() {
        let direction = Vector3::new(1.0, 0.0, 0.0);
        let delta_time = 0.1;
        let normal_speed = 1.0;
        
        let result = player_move_core(&direction, delta_time, normal_speed, None);
        
        // Should be direction * delta_time * normal_speed (1.0)
        assert!((result.x - 0.1).abs() < 0.001);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn test_player_move_sprint() {
        let direction = Vector3::new(1.0, 0.0, 0.0);
        let delta_time = 0.1;
        let sprint_speed = 1.5; // 50% faster when sprinting
        
        let result = player_move_core(&direction, delta_time, sprint_speed, None);
        
        // Should be direction * delta_time * 1.5 (sprint multiplier)
        assert!((result.x - 0.15).abs() < 0.001);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn test_player_move_crouch() {
        let direction = Vector3::new(1.0, 0.0, 0.0);
        let delta_time = 0.1;
        let crouch_speed = 0.67; // 33% slower when crouching
        
        let result = player_move_core(&direction, delta_time, crouch_speed, None);
        
        // Should be direction * delta_time * 0.67 (crouch multiplier)
        assert!((result.x - 0.067).abs() < 0.001);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn test_player_move_ground_projection() {
        // Moving diagonally up and forward
        let direction = Vector3::new(1.0, 1.0, 0.0);
        let delta_time = 0.1;
        let normal_speed = 1.0; // Normal speed
        
        // Ground normal pointing straight up
        let ground_hit = RayCollisionHit::new(0.0, 1.0, 0.0, 1.0);
        
        let result = player_move_core(&direction, delta_time, normal_speed, Some(ground_hit));
        
        // Y component should be removed due to ground projection
        // X component should be preserved with original magnitude
        let original_length = direction.length();
        let expected_x = original_length * delta_time * normal_speed; // Since Y is projected out, only X remains
        
        assert!((result.x - expected_x).abs() < 0.001);
        assert!((result.y - 0.0).abs() < 0.001);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn test_vector_length_operations() {
        let mut v = Vector3::new(3.0, 4.0, 0.0);
        assert!((v.length() - 5.0).abs() < 0.001);
        assert!((v.length_sq() - 25.0).abs() < 0.001);
        
        v.set_length(10.0);
        assert!((v.length() - 10.0).abs() < 0.001);
        assert!((v.x - 6.0).abs() < 0.001); // Should be 3 * (10/5) = 6
        assert!((v.y - 8.0).abs() < 0.001); // Should be 4 * (10/5) = 8
    }

    #[test]
    fn test_gravity_influence_basic() {
        let mut gravity_influence_vec = Vector3::new(0.0, 0.0, 0.0);
        let gravity = Vector3::new(0.0, -9.8, 0.0); // Standard gravity
        let delta = 0.1;
        
        let result = super::gravity_influence_core(&mut gravity_influence_vec, &gravity, delta);
        
        // Expected: gravity.y * delta^2 * 0.0875
        // = -9.8 * 0.01 * 0.0875 = -0.008575
        let expected_y = -9.8 * 0.01 * 0.0875;
        assert!((result.y - expected_y).abs() < 0.0001);
        assert_eq!(result.x, 0.0);
        assert_eq!(result.z, 0.0);
        
        // gravity_influence_vec should also be modified
        assert!((gravity_influence_vec.y - expected_y).abs() < 0.0001);
    }

    #[test]
    fn test_gravity_influence_accumulation() {
        let mut gravity_influence_vec = Vector3::new(1.0, 2.0, 3.0); // Starting with some force
        let gravity = Vector3::new(0.0, -9.8, 0.0);
        let delta = 0.1;
        
        let original_x = gravity_influence_vec.x;
        let original_z = gravity_influence_vec.z;
        
        super::gravity_influence_core(&mut gravity_influence_vec, &gravity, delta);
        
        // X and Z should remain unchanged (no gravity in those directions)
        assert_eq!(gravity_influence_vec.x, original_x);
        assert_eq!(gravity_influence_vec.z, original_z);
        
        // Y should have the gravity added
        let expected_y = 2.0 + (-9.8 * 0.01 * 0.0875);
        assert!((gravity_influence_vec.y - expected_y).abs() < 0.0001);
    }

    #[test]
    fn test_gravity_influence_acceleration() {
        let gravity = Vector3::new(0.0, -9.8, 0.0);
        let delta = 0.1;
        
        let acceleration = gravity_influence_acceleration_native(&gravity, delta);
        
        // Should match the calculation from gravity_influence
        let expected_y = -9.8 * 0.01 * 0.0875;
        assert!((acceleration.y - expected_y).abs() < 0.0001);
        assert_eq!(acceleration.x, 0.0);
        assert_eq!(acceleration.z, 0.0);
    }

    #[test]
    fn test_gravity_influence_core() {
        let mut gravity_influence_vec = Vector3::new(0.0, 0.0, 0.0);
        let gravity = Vector3::new(0.0, -9.8, 0.0);
        let delta = 0.1;
        
        gravity_influence_core(&mut gravity_influence_vec, &gravity, delta);
        
        // Should produce the same result as the WASM version
        let expected_y = -9.8 * 0.01 * 0.0875;
        assert!((gravity_influence_vec.y - expected_y).abs() < 0.0001);
        assert_eq!(gravity_influence_vec.x, 0.0);
        assert_eq!(gravity_influence_vec.z, 0.0);
    }

    #[test]
    fn test_gravity_influence_multi_axis() {
        let mut gravity_influence_vec = Vector3::new(0.0, 0.0, 0.0);
        let gravity = Vector3::new(1.0, -9.8, 2.0); // Gravity in all directions
        let delta = 0.05;
        
        super::gravity_influence_core(&mut gravity_influence_vec, &gravity, delta);
        
        let delta_sqrd = delta * delta;
        let scale_factor = delta_sqrd * 0.0875;
        
        assert!((gravity_influence_vec.x - (1.0 * scale_factor)).abs() < 0.0001);
        assert!((gravity_influence_vec.y - (-9.8 * scale_factor)).abs() < 0.0001);
        assert!((gravity_influence_vec.z - (2.0 * scale_factor)).abs() < 0.0001);
    }
}
