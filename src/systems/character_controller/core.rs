use bevy::prelude::*;
use avian3d::prelude::*;
use crate::resources::{InputResource, GameDebugConfig, DebugTimer};
use crate::components::{Player, PlayerMovementConfig, PlayerMovementState};
use super::{config::*, collision::*};

/// Enhanced character controller state for smooth movement
#[derive(Component, Debug, Clone)]
pub struct CharacterControllerState {
    pub velocity: Vec3,
    pub is_grounded: bool,
    pub ground_normal: Vec3,
    pub vertical_velocity: f32,
    pub last_ground_position: Vec3,
    pub movement_state: MovementState,
    // Coyote time and state buffering
    pub coyote_time_remaining: f32,
    pub ground_buffer_frames: u32,
    pub last_grounded_frame: u32,
    pub can_jump: bool,
}

impl Default for CharacterControllerState {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,
            is_grounded: false,
            ground_normal: Vec3::Y,
            vertical_velocity: 0.0,
            last_ground_position: Vec3::ZERO,
            movement_state: MovementState::Idle,
            coyote_time_remaining: 0.0,
            ground_buffer_frames: 0,
            last_grounded_frame: 0,
            can_jump: false,
        }
    }
}

/// Enhanced movement states for better animation integration
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

/// Main character controller system - replaces the old kinematic controller
pub fn enhanced_character_controller(
    time: Res<Time>,
    debug_config: Res<GameDebugConfig>,
    mut debug_timer: ResMut<DebugTimer>,
    input: Res<InputResource>,
    config: Res<CharacterControllerConfig>,
    mut player_query: Query<
        (
            Entity,
            &mut Transform,
            &PlayerMovementConfig,
            &mut PlayerMovementState,
            &mut CharacterControllerState,
            &Children,
        ),
        With<Player>
    >,
    camera_query: Query<
        (&Transform, &crate::systems::camera::GameCamera),
        (With<crate::systems::camera::GameCamera>, Without<Player>)
    >,
    spatial_query: SpatialQuery,
) {
    let Ok((
        player_entity,
        mut player_transform,
        movement_config,
        mut movement_state,
        mut controller_state,
        children,
    )) = player_query.single_mut() else {
        return;
    };

    let Ok((camera_transform, _camera)) = camera_query.single() else {
        return;
    };

    let dt = time.delta_secs();
    
    // Create excluded entities list for collision queries
    let mut excluded_entities = vec![player_entity];
    for child in children.iter() {
        excluded_entities.push(child);
    }

    // Ground detection
    let (is_grounded, ground_info) = CollisionSystem::is_grounded(
        player_transform.translation,
        &spatial_query,
        &config,
        &excluded_entities,
    );
    
    // Debug ground detection on slopes - controlled by DebugConfig
    if debug_config.slide_debug && ground_info.is_some() {
        let ground_result = ground_info.as_ref().unwrap();
        let slope_angle_deg = ground_result.normal.dot(Vec3::Y).acos() * 180.0 / std::f32::consts::PI;
        if slope_angle_deg > 5.0 { // Only log on slopes > 5 degrees
            let current_time = time.elapsed_secs_f64();
            if debug_timer.should_log_slide(current_time, debug_config.debug_update_interval) {
                info!("🏃 SLOPE: pos=({:.1},{:.1},{:.1}) grounded={} slope={:.1}° normal=({:.2},{:.2},{:.2})", 
                      player_transform.translation.x, player_transform.translation.y, player_transform.translation.z,
                      is_grounded, slope_angle_deg, ground_result.normal.x, ground_result.normal.y, ground_result.normal.z);
            }
        }
    }

    // Update ground state with buffering and coyote time
    update_ground_state_with_coyote_time(
        &mut controller_state,
        is_grounded,
        ground_info,
        &config,
        dt,
    );

    // Handle mouselook rotation
    if input.mouse_right_held {
        let camera_forward = -camera_transform.local_z();
        let camera_yaw = camera_forward.x.atan2(camera_forward.z);
        let target_rotation = Quat::from_rotation_y(camera_yaw);
        let t = (config.ground.turn_speed * dt).clamp(0.0, 1.0);
        player_transform.rotation = player_transform.rotation.slerp(target_rotation, t);
    }

    // Calculate desired movement direction
    let mut movement_input = Vec3::ZERO;
    let both_mouse_forward = input.mouse_left_held && input.mouse_right_held;
    
    if input.forward || both_mouse_forward { movement_input.z += 1.0; }
    if input.backward { movement_input.z -= 1.0; }
    if input.left { movement_input.x -= 1.0; }
    if input.right { movement_input.x += 1.0; }
    
    // Debug input detection - controlled by DebugConfig
    if debug_config.input_debug && (input.forward || input.backward || input.left || input.right || both_mouse_forward) {
        let current_time = time.elapsed_secs_f64();
        if debug_timer.should_log_input(current_time, debug_config.debug_update_interval) {
            if !debug_config.debug_only_when_moving || 
               (input.forward || input.backward || input.left || input.right) {
                info!("🎮 CONTROLLER INPUT: forward={} backward={} left={} right={} mouse_forward={} movement_input=({:.2},{:.2},{:.2})", 
                      input.forward, input.backward, input.left, input.right, both_mouse_forward, 
                      movement_input.x, movement_input.y, movement_input.z);
            }
        }
    }

    // Apply camera-relative movement
    let desired_movement = if movement_input.length() > 0.0 {
        movement_input = movement_input.normalize();
        
        let camera_forward = -camera_transform.local_z();
        let camera_right = camera_transform.local_x();
        
        let horizontal_forward = Vec3::new(camera_forward.x, 0.0, camera_forward.z).normalize();
        let horizontal_right = Vec3::new(camera_right.x, 0.0, camera_right.z).normalize();
        
        horizontal_forward * movement_input.z + horizontal_right * movement_input.x
    } else {
        Vec3::ZERO
    };

    // Determine target speed
    let target_speed = if desired_movement.length() > 0.0 {
        if input.up { // Running
            config.ground.run_speed
        } else { // Walking
            config.ground.walk_speed
        }
    } else {
        0.0
    };
    
    // Debug target speed calculation - controlled by DebugConfig
    if debug_config.slide_debug && desired_movement.length() > 0.0 {
        let current_time = time.elapsed_secs_f64();
        if debug_timer.should_log_slide(current_time, debug_config.debug_update_interval) {
            info!("🏃 SPEED: desired_movement_len={:.3}, running={}, target_speed={:.3}", 
                  desired_movement.length(), input.up, target_speed);
        }
    }

    // Handle jumping with coyote time - debug controlled by DebugConfig
    if debug_config.slide_debug && input.down {
        let current_time = time.elapsed_secs_f64();
        if debug_timer.should_log_slide(current_time, debug_config.debug_update_interval) {
            info!("🏃 JUMP: space pressed - is_jumping={}, can_jump={}, grounded={}, coyote_time={:.3}", 
                  movement_state.is_jumping, controller_state.can_jump, controller_state.is_grounded, controller_state.coyote_time_remaining);
        }
    }
    if input.down && !movement_state.is_jumping && controller_state.can_jump {
        let jump_velocity = (2.0 * 9.81 * config.air.jump_height).sqrt();
        controller_state.vertical_velocity = jump_velocity;
        movement_state.is_jumping = true;
        controller_state.movement_state = MovementState::Jumping;
        // Consume coyote time
        controller_state.coyote_time_remaining = 0.0;
        controller_state.can_jump = false;
        if debug_config.slide_debug {
            info!("🏃 JUMP: Jump initiated with velocity={} (coyote time used)", jump_velocity);
        }
    }

    // Apply movement based on grounded state
    if controller_state.is_grounded && controller_state.vertical_velocity <= 0.0 {
        // Ground movement with enhanced collision
        handle_ground_movement(
            &mut player_transform,
            &mut movement_state,
            &mut controller_state,
            desired_movement,
            target_speed,
            &spatial_query,
            &config,
            &excluded_entities,
            dt,
        );
    } else {
        // Air movement with gravity
        handle_air_movement(
            &mut player_transform,
            &mut movement_state,
            &mut controller_state,
            desired_movement,
            target_speed,
            &spatial_query,
            &config,
            &excluded_entities,
            dt,
        );
    }

    // Update movement state for animations
    update_movement_state(&mut controller_state, &movement_state, target_speed, &config);

    // Handle player rotation (when not using mouselook)
    if !input.mouse_right_held && desired_movement.length() > 0.1 {
        let target_direction = desired_movement.normalize();
        let target_rotation = Quat::from_rotation_y(target_direction.x.atan2(target_direction.z));
        let t = (config.ground.turn_speed * dt).clamp(0.0, 1.0);
        player_transform.rotation = player_transform.rotation.slerp(target_rotation, t);
    }
}

/// Handle ground-based movement with collision and slope handling
fn handle_ground_movement(
    transform: &mut Transform,
    movement_state: &mut PlayerMovementState,
    controller_state: &mut CharacterControllerState,
    desired_direction: Vec3,
    target_speed: f32,
    spatial_query: &SpatialQuery,
    config: &CharacterControllerConfig,
    excluded_entities: &[Entity],
    dt: f32,
) {
    // Smooth acceleration/deceleration
    let acceleration = if target_speed > movement_state.current_speed {
        config.ground.acceleration
    } else {
        config.ground.deceleration
    };

    let speed_diff = target_speed - movement_state.current_speed;
    let speed_change = speed_diff.signum() * (acceleration * dt).min(speed_diff.abs());
    movement_state.current_speed = (movement_state.current_speed + speed_change).max(0.0);

    // Apply slope speed modifiers
    let slope_modifier = calculate_slope_modifier(controller_state.ground_normal, desired_direction, config);
    let effective_speed = movement_state.current_speed * slope_modifier;
    
    // Debug movement calculations (disabled - too verbose)
    // These detailed movement calculations can be enabled in debug builds if needed

    // Calculate desired velocity
    let desired_velocity = if desired_direction.length() > 0.0 {
        desired_direction.normalize() * effective_speed
    } else {
        Vec3::ZERO
    };

    // Smooth direction changes
    if desired_velocity.length() > 0.0 {
        let direction_lerp_speed = config.ground.turn_speed * dt;
        movement_state.current_direction = movement_state.current_direction
            .lerp(desired_velocity.normalize(), direction_lerp_speed.clamp(0.0, 1.0));
    }

    // Calculate movement delta with slope projection
    let mut current_velocity = movement_state.current_direction * effective_speed;
    
    // Project velocity along slope for smooth terrain following
    if controller_state.is_grounded && config.advanced.enable_ground_snapping {
        current_velocity = project_velocity_on_slope(current_velocity, controller_state.ground_normal);
    }
    
    let movement_delta = current_velocity * dt;

    if movement_delta.length() > 0.001 {
        // Debug move calculations (disabled - too verbose)
        // Apply enhanced collision detection and resolution
        let (new_position, final_velocity, collision_result) = CollisionSystem::collide_and_slide(
            transform.translation,
            movement_delta,
            spatial_query,
            config,
            excluded_entities,
        );

        // Try step-up if we hit a wall-like obstacle
        let final_position = if collision_result.hit && !collision_result.is_walkable {
            if let Some(step_up_position) = CollisionSystem::attempt_step_up(
                transform.translation,
                desired_direction,
                spatial_query,
                config,
                excluded_entities,
            ) {
                controller_state.movement_state = MovementState::SteppingUp;
                step_up_position
            } else {
                new_position
            }
        } else {
            new_position
        };

        // Calculate actual velocity from position change
        let position_delta = final_position - transform.translation;
        controller_state.velocity = position_delta / dt;
        
        transform.translation = final_position;

        // Update movement state based on collision result
        if collision_result.hit {
            match CollisionSystem::classify_surface(&collision_result.normal, config) {
                SurfaceType::Slideable => {
                    controller_state.movement_state = MovementState::Sliding;
                }
                _ => {
                    // Normal movement
                    if effective_speed > 0.1 {
                        controller_state.movement_state = if effective_speed > config.ground.walk_speed * 1.5 {
                            MovementState::Running
                        } else {
                            MovementState::Walking
                        };
                    } else {
                        controller_state.movement_state = MovementState::Idle;
                    }
                }
            }
        }
    } else {
        controller_state.movement_state = MovementState::Idle;
        movement_state.current_direction = Vec3::ZERO;
    }

    // Reset jumping state if grounded
    if controller_state.is_grounded {
        movement_state.is_jumping = false;
        controller_state.vertical_velocity = 0.0;
    }
}

/// Handle air-based movement with gravity
fn handle_air_movement(
    transform: &mut Transform,
    movement_state: &mut PlayerMovementState,
    controller_state: &mut CharacterControllerState,
    desired_direction: Vec3,
    target_speed: f32,
    spatial_query: &SpatialQuery,
    config: &CharacterControllerConfig,
    excluded_entities: &[Entity],
    dt: f32,
) {
    // Apply gravity
    let gravity_acceleration = -9.81 * config.air.gravity_scale;
    controller_state.vertical_velocity += gravity_acceleration * dt;
    
    // Limit fall speed
    controller_state.vertical_velocity = controller_state.vertical_velocity.max(-config.air.fall_speed_limit);

    // Air control for horizontal movement
    let air_movement = if desired_direction.length() > 0.0 {
        let air_speed = target_speed * config.air.air_control;
        desired_direction.normalize() * air_speed * dt
    } else {
        Vec3::ZERO
    };

    // Vertical movement
    let vertical_movement = Vec3::Y * controller_state.vertical_velocity * dt;
    
    // Combined movement
    let total_movement = air_movement + vertical_movement;

    if total_movement.length() > 0.001 {
        let (new_position, final_velocity, collision_result) = CollisionSystem::collide_and_slide(
            transform.translation,
            total_movement,
            spatial_query,
            config,
            excluded_entities,
        );

        transform.translation = new_position;

        // Handle ground collision during fall
        if collision_result.hit && collision_result.normal.y > 0.7 {
            controller_state.is_grounded = true;
            controller_state.vertical_velocity = 0.0;
            movement_state.is_jumping = false;
            controller_state.movement_state = MovementState::Landing;
        }
    }

    // Set air movement state
    if controller_state.vertical_velocity > 0.1 {
        controller_state.movement_state = MovementState::Jumping;
    } else {
        controller_state.movement_state = MovementState::Falling;
    }
}

/// Calculate speed modifier based on slope angle and movement direction
fn calculate_slope_modifier(ground_normal: Vec3, movement_direction: Vec3, config: &CharacterControllerConfig) -> f32 {
    if movement_direction.length() < 0.1 {
        return 1.0;
    }

    // Calculate if we're going uphill or downhill
    let slope_direction = Vec3::new(-ground_normal.x, 0.0, -ground_normal.z).normalize();
    let movement_dot = movement_direction.dot(slope_direction);

    if movement_dot > 0.1 {
        // Going downhill
        config.slopes.downhill_speed_multiplier
    } else if movement_dot < -0.1 {
        // Going uphill
        config.slopes.uphill_speed_multiplier
    } else {
        // Perpendicular to slope
        1.0
    }
}

/// Project velocity onto slope surface for smooth terrain following
fn project_velocity_on_slope(velocity: Vec3, ground_normal: Vec3) -> Vec3 {
    // Only project horizontal movement, preserve intended direction
    let horizontal_velocity = Vec3::new(velocity.x, 0.0, velocity.z);
    
    if horizontal_velocity.length() < 0.001 {
        return velocity; // No horizontal movement to project
    }
    
    // Project horizontal velocity onto the slope plane
    let projected_horizontal = horizontal_velocity - ground_normal * horizontal_velocity.dot(ground_normal);
    
    // Preserve vertical component (jumping, falling)
    Vec3::new(projected_horizontal.x, velocity.y, projected_horizontal.z)
}

/// Update ground state with coyote time and buffering
fn update_ground_state_with_coyote_time(
    controller_state: &mut CharacterControllerState,
    is_grounded: bool,
    ground_info: Option<CollisionResult>,
    config: &CharacterControllerConfig,
    dt: f32,
) {
    // Update ground normal if we have ground info
    if let Some(ground_result) = ground_info {
        controller_state.ground_normal = ground_result.normal;
    }

    let was_grounded = controller_state.is_grounded;
    controller_state.is_grounded = is_grounded;

    // Handle coyote time
    if config.advanced.enable_coyote_time {
        if is_grounded {
            // Reset coyote time when grounded
            controller_state.coyote_time_remaining = config.advanced.coyote_time_duration;
            controller_state.can_jump = true;
            controller_state.last_ground_position = controller_state.last_ground_position; // Could update this
        } else {
            // Reduce coyote time when airborne
            controller_state.coyote_time_remaining = (controller_state.coyote_time_remaining - dt).max(0.0);
            controller_state.can_jump = controller_state.coyote_time_remaining > 0.0;
        }
    } else {
        // Standard jumping without coyote time
        controller_state.can_jump = is_grounded;
    }

    // Ground state buffering could be added here for further smoothing
    if config.advanced.ground_state_buffer_frames > 0 {
        if is_grounded {
            controller_state.ground_buffer_frames = config.advanced.ground_state_buffer_frames;
        } else if controller_state.ground_buffer_frames > 0 {
            controller_state.ground_buffer_frames -= 1;
            // Keep grounded state for buffer frames
            controller_state.is_grounded = controller_state.ground_buffer_frames > 0;
        }
    }
}

/// Update movement state for animation system integration
fn update_movement_state(
    controller_state: &mut CharacterControllerState,
    movement_state: &PlayerMovementState,
    target_speed: f32,
    config: &CharacterControllerConfig,
) {
    // Only update if not in special states (jumping, landing, etc.)
    if matches!(
        controller_state.movement_state,
        MovementState::Idle | MovementState::Walking | MovementState::Running
    ) {
        controller_state.movement_state = if target_speed > 0.1 {
            if target_speed > config.ground.walk_speed * 1.5 {
                MovementState::Running
            } else {
                MovementState::Walking
            }
        } else {
            MovementState::Idle
        };
    }
}

/// Initialize character controller components
pub fn setup_character_controller(
    mut commands: Commands,
    mut player_query: Query<Entity, (With<Player>, Without<CharacterControllerState>)>,
) {
    for player_entity in player_query.iter_mut() {
        commands.entity(player_entity).insert(CharacterControllerState::default());
    }
}

/// Character controller plugin for easy integration
pub struct CharacterControllerPlugin {
    pub config: CharacterControllerConfig,
}

impl Default for CharacterControllerPlugin {
    fn default() -> Self {
        Self {
            config: CharacterControllerConfig::mmo_optimized(),
        }
    }
}

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(self.config.clone())
            .add_systems(Startup, setup_character_controller)
            .add_systems(Update, enhanced_character_controller);
    }
}