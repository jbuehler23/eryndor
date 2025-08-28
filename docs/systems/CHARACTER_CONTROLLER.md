# Character Controller Documentation

## Overview

The Eryndor Character Controller system provides precise, responsive character movement that integrates seamlessly with the physics simulation. Unlike simple physics-based movement, this system uses a kinematic approach that offers predictable behavior while maintaining compatibility with the world's physics systems.

## Core Philosophy

### Kinematic Movement Design
- **Predictable Behavior**: Consistent movement response across different conditions
- **Physics Integration**: Seamless interaction with world physics without physics artifacts
- **Network Ready**: Deterministic movement suitable for multiplayer implementation
- **Customizable Feel**: Full control over movement mechanics and responsiveness
- **Performance Optimized**: Efficient collision detection and response

### Player-Centric Experience
- **Responsive Controls**: Immediate response to player input
- **Natural Movement**: Physics-aware movement that feels natural and intuitive
- **Terrain Adaptation**: Automatic handling of slopes, steps, and obstacles
- **Smooth Transitions**: Seamless transitions between movement states
- **Animation Integration**: Movement states drive animation system

## System Architecture

### Core Components

#### PlayerMovementConfig Component
```rust
#[derive(Component, Debug, Clone)]
pub struct PlayerMovementConfig {
    pub walk_speed: f32,
    pub run_speed: f32,
    pub jump_force: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub air_control: f32,
    pub gravity_scale: f32,
    pub ground_friction: f32,
    pub air_friction: f32,
}

impl Default for PlayerMovementConfig {
    fn default() -> Self {
        Self {
            walk_speed: 4.0,
            run_speed: 8.0,
            jump_force: 12.0,
            acceleration: 20.0,
            deceleration: 15.0,
            air_control: 0.3,
            gravity_scale: 2.0,
            ground_friction: 0.9,
            air_friction: 0.1,
        }
    }
}
```

#### PlayerMovementState Component
```rust
#[derive(Component, Debug, Clone)]
pub struct PlayerMovementState {
    pub current_speed: f32,
    pub target_speed: f32,
    pub current_direction: Vec3,
    pub desired_direction: Vec3,
    pub vertical_velocity: f32,
    pub is_grounded: bool,
    pub is_jumping: bool,
    pub can_jump: bool,
    pub coyote_time_remaining: f32,
    pub ground_normal: Vec3,
    pub last_ground_position: Vec3,
}
```

#### Enhanced Character Controller State
```rust
#[derive(Component, Debug, Clone)]
pub struct CharacterControllerState {
    pub velocity: Vec3,
    pub is_grounded: bool,
    pub ground_normal: Vec3,
    pub vertical_velocity: f32,
    pub last_ground_position: Vec3,
    pub movement_state: MovementState,
    pub coyote_time_remaining: f32,
    pub ground_buffer_frames: u32,
    pub last_grounded_frame: u32,
    pub can_jump: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MovementState {
    Idle,
    Walking,
    Running,
    Sliding,
    SteppingUp,
    Falling,
    Landing,
    Jumping,
}
```

## Key Systems

### 1. Kinematic Character Controller

#### Main Controller Loop
```rust
pub fn kinematic_character_controller(
    time: Res<Time>,
    input: Res<InputResource>,
    mut player_query: Query<(Entity, &mut Transform, &PlayerMovementConfig, &mut PlayerMovementState, &Children), With<Player>>,
    camera_query: Query<(&Transform, &GameCamera), (With<GameCamera>, Without<Player>)>,
    spatial_query: SpatialQuery,
) {
    let Ok((player_entity, mut player_transform, movement_config, mut movement_state, children)) = player_query.single_mut() else {
        return;
    };
    
    let Ok((camera_transform, _camera)) = camera_query.single() else {
        return;
    };

    let dt = time.delta_secs();
    
    // Handle mouselook rotation
    handle_mouselook_rotation(&input, &mut player_transform, camera_transform, dt);
    
    // Process movement input
    let movement_input = process_movement_input(&input);
    let desired_movement = apply_camera_relative_movement(movement_input, camera_transform);
    
    // Update movement state
    update_movement_state(&mut movement_state, desired_movement, movement_config, dt);
    
    // Apply movement with collision detection
    apply_kinematic_movement(
        &mut player_transform,
        &mut movement_state,
        &spatial_query,
        player_entity,
        children,
        dt,
    );
}
```

#### Movement Input Processing
```rust
fn process_movement_input(input: &InputResource) -> Vec3 {
    let mut movement_dir = Vec3::ZERO;
    
    // Classic WoW behavior: both mouse buttons = move forward
    let both_mouse_forward = input.mouse_left_held && input.mouse_right_held;
    
    if input.forward || both_mouse_forward { movement_dir.z += 1.0; }
    if input.backward { movement_dir.z -= 1.0; }
    if input.left { movement_dir.x -= 1.0; }
    if input.right { movement_dir.x += 1.0; }
    
    // Normalize to prevent faster diagonal movement
    if movement_dir.length() > 0.0 {
        movement_dir = movement_dir.normalize();
    }
    
    movement_dir
}
```

#### Camera-Relative Movement
```rust
fn apply_camera_relative_movement(movement_input: Vec3, camera_transform: &Transform) -> Vec3 {
    if movement_input.length() == 0.0 {
        return Vec3::ZERO;
    }
    
    // Get camera forward and right vectors (Y component removed for horizontal movement)
    let camera_forward = -camera_transform.local_z();
    let camera_right = camera_transform.local_x();
    
    let horizontal_forward = Vec3::new(camera_forward.x, 0.0, camera_forward.z).normalize();
    let horizontal_right = Vec3::new(camera_right.x, 0.0, camera_right.z).normalize();
    
    // Apply camera-relative movement
    horizontal_forward * movement_input.z + horizontal_right * movement_input.x
}
```

### 2. Advanced Character Controller (Enhanced System)

#### Ground Detection with Coyote Time
```rust
fn update_ground_state_with_coyote_time(
    controller_state: &mut CharacterControllerState,
    is_grounded: bool,
    ground_info: Option<GroundInfo>,
    config: &CharacterControllerConfig,
    dt: f32,
) {
    let was_grounded = controller_state.is_grounded;
    controller_state.is_grounded = is_grounded;
    
    if let Some(ground) = ground_info {
        controller_state.ground_normal = ground.normal;
        controller_state.last_ground_position = ground.point;
    }
    
    // Coyote time implementation
    if was_grounded && !is_grounded {
        // Just left ground - start coyote time
        controller_state.coyote_time_remaining = config.air.coyote_time;
        controller_state.can_jump = true;
    } else if is_grounded {
        // On ground - reset jump state
        controller_state.coyote_time_remaining = config.air.coyote_time;
        controller_state.can_jump = true;
        controller_state.vertical_velocity = 0.0;
    } else {
        // In air - decay coyote time
        controller_state.coyote_time_remaining = 
            (controller_state.coyote_time_remaining - dt).max(0.0);
        
        if controller_state.coyote_time_remaining <= 0.0 {
            controller_state.can_jump = false;
        }
    }
}
```

#### Collision Detection and Response
```rust
pub fn collide_and_slide(
    position: Vec3,
    movement_delta: Vec3,
    spatial_query: &SpatialQuery,
    config: &CharacterControllerConfig,
    excluded_entities: &[Entity],
) -> (Vec3, Vec3, CollisionResult) {
    let mut current_position = position;
    let mut remaining_delta = movement_delta;
    let mut final_velocity = movement_delta / dt;
    
    const MAX_ITERATIONS: usize = 4;
    let mut collision_result = CollisionResult::default();
    
    for iteration in 0..MAX_ITERATIONS {
        if remaining_delta.length() < 0.001 {
            break;
        }
        
        // Cast ray to detect collision
        let ray_direction = Dir3::new(remaining_delta.normalize()).unwrap();
        let ray_distance = remaining_delta.length() + config.collision.skin_width;
        
        let filter = SpatialQueryFilter::new()
            .with_excluded_entities(excluded_entities);
        
        if let Some(hit) = spatial_query.cast_ray(
            current_position,
            ray_direction,
            ray_distance,
            true,
            &filter,
        ) {
            let hit_distance = hit.distance - config.collision.skin_width;
            
            if hit_distance > 0.0 {
                // Move to collision point
                let safe_movement = remaining_delta.normalize() * hit_distance;
                current_position += safe_movement;
            }
            
            // Calculate slide direction
            let slide_direction = calculate_slide_direction(hit.normal, config);
            let slide_amount = remaining_delta.dot(slide_direction);
            remaining_delta = slide_direction * slide_amount;
            
            // Update collision result
            collision_result.hit = true;
            collision_result.normal = hit.normal;
            collision_result.is_walkable = is_surface_walkable(&hit.normal, config);
        } else {
            // No collision, move full distance
            current_position += remaining_delta;
            break;
        }
    }
    
    (current_position, final_velocity, collision_result)
}
```

#### Slope Handling
```rust
fn calculate_slope_modifier(
    ground_normal: Vec3,
    movement_direction: Vec3,
    config: &CharacterControllerConfig,
) -> f32 {
    if ground_normal.y <= 0.1 {
        return 0.0; // Wall or ceiling
    }
    
    let slope_angle = ground_normal.dot(Vec3::Y).acos();
    
    if slope_angle <= config.slopes.max_walkable_angle {
        // Walkable slope - apply speed modifier based on steepness
        let slope_factor = 1.0 - (slope_angle / config.slopes.max_walkable_angle) * 0.3;
        
        // Additional modifier based on movement direction relative to slope
        let slope_direction = Vec3::new(ground_normal.x, 0.0, ground_normal.z).normalize();
        let movement_dot = movement_direction.dot(-slope_direction);
        
        if movement_dot > 0.0 {
            // Moving uphill
            slope_factor * (1.0 - movement_dot * config.slopes.uphill_penalty)
        } else {
            // Moving downhill
            slope_factor * (1.0 + movement_dot.abs() * config.slopes.downhill_bonus)
        }
    } else if slope_angle <= config.slopes.slide_threshold_angle {
        // Slideable slope
        config.slopes.slide_speed_modifier
    } else {
        // Too steep - no movement
        0.0
    }
}
```

### 3. Step-Up System

#### Step Detection and Handling
```rust
pub fn attempt_step_up(
    position: Vec3,
    movement_direction: Vec3,
    spatial_query: &SpatialQuery,
    config: &CharacterControllerConfig,
    excluded_entities: &[Entity],
) -> Option<Vec3> {
    let step_height = config.movement.step_height;
    let step_distance = config.movement.step_distance;
    
    // Check if there's a step in front
    let forward_check_start = position + Vec3::Y * step_height;
    let forward_check_end = forward_check_start + movement_direction * step_distance;
    
    let filter = SpatialQueryFilter::new()
        .with_excluded_entities(excluded_entities);
    
    // Cast ray forward at step height
    if let Ok(forward_dir) = Dir3::new(movement_direction.normalize()) {
        if let Some(hit) = spatial_query.cast_ray(
            forward_check_start,
            forward_dir,
            step_distance,
            true,
            &filter,
        ) {
            return None; // Obstacle too high
        }
    }
    
    // Cast ray down to find step surface
    let down_check_start = forward_check_end;
    let down_check_end = down_check_start - Vec3::Y * step_height;
    
    if let Some(hit) = spatial_query.cast_ray(
        down_check_start,
        Dir3::NEG_Y,
        step_height,
        true,
        &filter,
    ) {
        let step_surface_height = hit.point.y;
        let step_up_height = step_surface_height - position.y;
        
        if step_up_height > 0.0 && step_up_height <= step_height {
            // Valid step - return new position
            return Some(Vec3::new(
                hit.point.x,
                step_surface_height + config.collision.skin_width,
                hit.point.z,
            ));
        }
    }
    
    None
}
```

### 4. Integration Systems

#### Animation Integration
```rust
pub fn update_animation_states(
    input: Res<InputResource>,
    mut animation_query: Query<(&mut AnimationController, &PlayerMovementState, &Transform), With<Player>>,
) {
    for (mut anim_controller, movement_state, transform) in animation_query.iter_mut() {
        let is_moving = input.forward || input.backward || input.left || input.right || 
                       (input.mouse_left_held && input.mouse_right_held);
        let is_running = is_moving && input.up;
        let is_jumping = movement_state.is_jumping || movement_state.vertical_velocity > 0.1;
        
        // Create velocity vector for animation system
        let current_velocity = Vec3::new(
            movement_state.current_direction.x * movement_state.current_speed,
            movement_state.vertical_velocity,
            movement_state.current_direction.z * movement_state.current_speed
        );
        
        let state_changed = anim_controller.update_state(
            current_velocity, 
            movement_state.is_grounded,
            is_moving,
            is_running, 
            is_jumping,
            time.delta_secs()
        );
    }
}
```

#### Camera Integration
```rust
fn handle_mouselook_rotation(
    input: &InputResource,
    player_transform: &mut Transform,
    camera_transform: &Transform,
    dt: f32,
) {
    if input.mouse_right_held {
        // Compute camera yaw from its forward vector (ignore pitch)
        let camera_forward = -camera_transform.local_z();
        let camera_yaw = camera_forward.x.atan2(camera_forward.z);
        let target_rotation = Quat::from_rotation_y(camera_yaw);

        // Smooth rotation using slerp
        let t = (15.0 * dt).clamp(0.0, 1.0);
        player_transform.rotation = player_transform.rotation.slerp(target_rotation, t);
    }
}
```

## Configuration System

### Character Controller Configuration
```rust
#[derive(Resource)]
pub struct CharacterControllerConfig {
    pub ground: GroundConfig,
    pub air: AirConfig,
    pub collision: CollisionConfig,
    pub slopes: SlopeConfig,
    pub movement: MovementConfig,
    pub advanced: AdvancedConfig,
}

#[derive(Clone)]
pub struct GroundConfig {
    pub walk_speed: f32,
    pub run_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub turn_speed: f32,
    pub friction: f32,
}

#[derive(Clone)]
pub struct AirConfig {
    pub gravity_scale: f32,
    pub jump_height: f32,
    pub jump_time_to_peak: f32,
    pub jump_time_to_fall: f32,
    pub air_control: f32,
    pub coyote_time: f32,
    pub jump_buffer_time: f32,
    pub terminal_velocity: f32,
}
```

### Performance Configuration
```rust
#[derive(Clone)]
pub struct CollisionConfig {
    pub skin_width: f32,
    pub max_collision_iterations: usize,
    pub step_offset: f32,
    pub ground_check_distance: f32,
    pub ground_check_buffer: f32,
}

#[derive(Clone)]
pub struct SlopeConfig {
    pub max_walkable_angle: f32,
    pub slide_threshold_angle: f32,
    pub uphill_penalty: f32,
    pub downhill_bonus: f32,
    pub slide_speed_modifier: f32,
    pub slide_control_factor: f32,
}
```

## Debug Integration

### Debug Visualization
The character controller integrates with the debug system to provide detailed information about movement state, collision detection, and physics interactions:

```rust
// Collision debug controlled by GameDebugConfig
if debug_config.collision_debug {
    info!("🔧 COLLISION: pos=({:.2},{:.2},{:.2}) grounded={} height_diff={:.3}",
          player_pos.x, player_pos.y, player_pos.z, is_grounded, height_diff);
}

// Movement debug controlled by GameDebugConfig  
if debug_config.slide_debug {
    info!("🏃 SLOPE: pos=({:.1},{:.1},{:.1}) slope={:.1}° normal=({:.2},{:.2},{:.2})", 
          pos.x, pos.y, pos.z, slope_angle, normal.x, normal.y, normal.z);
}
```

### Visual Debug Tools
```rust
pub struct CharacterControllerDebugConfig {
    pub enabled: bool,
    pub show_velocity_vectors: bool,
    pub show_collision_normals: bool,
    pub show_ground_detection: bool,
    pub show_step_up_checks: bool,
    pub show_slope_analysis: bool,
}
```

## Performance Considerations

### Optimization Strategies

#### Efficient Ground Detection
```rust
fn is_grounded_optimized(
    position: Vec3,
    spatial_query: &SpatialQuery,
    config: &CharacterControllerConfig,
    excluded_entities: &[Entity],
) -> (bool, Option<GroundInfo>) {
    let ground_check_distance = config.collision.ground_check_distance;
    
    // Use shape cast instead of ray cast for more reliable detection
    let filter = SpatialQueryFilter::new()
        .with_excluded_entities(excluded_entities);
    
    if let Some(hit) = spatial_query.cast_shape(
        &Collider::capsule(config.collision.capsule_radius, config.collision.capsule_height),
        position,
        Quat::IDENTITY,
        Dir3::NEG_Y,
        ground_check_distance,
        true,
        &filter,
    ) {
        let ground_info = GroundInfo {
            point: hit.point,
            normal: hit.normal,
            distance: hit.distance,
        };
        
        (true, Some(ground_info))
    } else {
        (false, None)
    }
}
```

#### Collision Caching
- Spatial queries are expensive - cache results when possible
- Use shape casting instead of multiple ray casts
- Implement early-out conditions for common cases
- Use appropriate collision filters to reduce query complexity

### Memory Management
- Minimize allocations in movement update loops
- Reuse vectors and data structures where possible
- Use efficient data structures for collision queries
- Cache frequently accessed configuration values

## Integration Points

### Physics System Integration
- Kinematic bodies for predictable movement
- Physics queries for collision detection
- Proper handling of physics materials and friction
- Integration with rigid body physics for world objects

### Animation System Integration
- Movement state drives animation state machine
- Velocity information for animation blending
- Ground state affects animation transitions
- Jump and landing state synchronization

### Camera System Integration  
- Camera-relative movement calculation
- Mouselook rotation synchronization
- Smooth camera following during movement
- Collision-aware camera positioning

### Audio System Integration (Planned)
- Footstep audio based on movement state
- Surface material detection for audio variation
- Movement speed affects audio timing
- Environmental audio occlusion during movement

## Future Enhancements

### Planned Features
1. **Network Prediction**: Client-side prediction with server reconciliation
2. **Advanced Movement**: Wall-running, climbing, swimming mechanics
3. **Dynamic Terrain**: Runtime terrain modification support
4. **Physics Materials**: Surface-specific movement properties
5. **Movement Abilities**: Dash, teleport, and special movement skills

### Technical Improvements
1. **Performance Optimization**: SIMD operations for collision detection
2. **Determinism**: Fully deterministic movement for networked play
3. **Accessibility**: Movement assistance options for different player needs
4. **Customization**: Runtime movement parameter adjustment
5. **Analytics**: Movement pattern tracking and analysis

## Best Practices

### For Developers
1. **Predictable Movement**: Ensure consistent behavior across different conditions
2. **Performance Awareness**: Profile collision detection and optimize hot paths
3. **Debug Integration**: Use debug systems to understand movement behavior
4. **Configuration Driven**: Make movement feel adjustable through configuration
5. **Network Consideration**: Design with multiplayer synchronization in mind

### For Configuration
1. **Balanced Parameters**: Test movement feel across different player preferences
2. **Accessibility**: Provide options for different physical abilities
3. **Performance Scaling**: Consider different hardware capabilities
4. **Cultural Preferences**: Account for different movement expectations
5. **Gameplay Integration**: Ensure movement supports intended gameplay mechanics

## Conclusion

The Eryndor Character Controller represents a sophisticated approach to character movement that balances precision, performance, and player experience. By using kinematic movement with physics integration, it provides predictable and responsive character control while maintaining compatibility with the world's physics simulation.

The system's modular design allows for extensive customization and future enhancement while providing solid foundations for both single-player and multiplayer gameplay. With its comprehensive debug integration and performance optimization, it serves as a robust foundation for the game's movement mechanics and player experience.