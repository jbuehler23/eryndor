use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;
use crate::resources::InputResource;
use crate::components::{Player, PlayerMovementConfig, PlayerMovementState};

/// System to spawn the player with Tnua character controller
/// Following Single Responsibility: only handles player entity creation
pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn the player entity with physics - initially with fallback capsule
    let player_entity = commands.spawn((
        // Visual representation - Start with fallback, will be replaced when assets load
        Mesh3d(meshes.add(Capsule3d::new(0.5, 1.8))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.4, 0.8), // Blue player fallback
            ..default()
        })),
        
        Transform::from_xyz(-70.0, 15.0, -70.0), // Safe spawn location away from world objects
        
        // Physics components - Avian 3D with Tnua character controller
        RigidBody::Dynamic, // Dynamic for gravity and realistic physics
        
        // Manual character capsule collider - reasonable size for human character  
        Collider::capsule(0.4, 1.8), // Character capsule: radius=0.4, height=1.8
        
        // Tnua character controller components
        TnuaController::default(),
        // Sensor shape for ground detection (slightly smaller than main collider)
        TnuaAvian3dSensorShape(Collider::cylinder(0.35, 0.1)), // radius=0.35, height=0.1
        
        // Lock rotation on X and Z axes, allow Y-axis rotation for turning
        LockedAxes::new().lock_rotation_x().lock_rotation_z(),
        
        // Physics properties for character movement
        LinearVelocity::default(),
        Friction::new(0.7), // Ground friction for stopping
        Restitution::new(0.0), // No bounce when hitting things
    )).id();
    
    // Add game components separately to avoid bundle size limits
    commands.entity(player_entity).insert((
        Player,
        PlayerMovementConfig::default(),
        PlayerMovementState::default(), // Smooth movement state tracking
        crate::components::PlayerStats::default(),
        crate::components::AnimationController::default(),
        crate::components::CharacterModel::default(), // Track character model type
        crate::components::KnightAnimationSetup::default(), // Track animation setup
    ));
    
    info!("Player spawned with fallback capsule - will upgrade to 3D model when assets load");
}

/// Tnua-based player movement system with professional character controller
/// Following Single Responsibility: handles player movement input and feeds it to Tnua controller
pub fn tnua_player_controls(
    time: Res<Time>,
    input: Res<InputResource>,
    mut player_query: Query<(&mut TnuaController, &PlayerMovementConfig, &mut PlayerMovementState, &mut Transform), With<Player>>,
    camera_query: Query<(&Transform, &crate::systems::camera::GameCamera), (With<crate::systems::camera::GameCamera>, Without<Player>)>,
) {
    let Ok((mut controller, movement_config, mut movement_state, mut player_transform)) = player_query.single_mut() else {
        return;
    };
    
    let Ok((camera_transform, _camera)) = camera_query.single() else {
        return;
    };

    // If right mouse button held -> mouselook: rotate player to match camera yaw
    if input.mouse_right_held {
        // compute camera yaw from its forward vector (ignore pitch)
        let camera_forward = -camera_transform.local_z();
        let camera_yaw = camera_forward.x.atan2(camera_forward.z);
        let target_rotation = Quat::from_rotation_y(camera_yaw);

        // clamp slerp factor to [0,1]
        let t = (15.0 * time.delta_secs()).clamp(0.0, 1.0);
        player_transform.rotation = player_transform.rotation.slerp(target_rotation, t);
    }
    
    // Check for movement input
    let mut movement_dir = Vec3::ZERO;
    
    // Classic WoW behavior: both mouse buttons = move forward
    let both_mouse_forward = input.mouse_left_held && input.mouse_right_held;
    
    if input.forward || both_mouse_forward { movement_dir.z += 1.0; }
    if input.backward { movement_dir.z -= 1.0; }
    if input.left { movement_dir.x -= 1.0; }
    if input.right { movement_dir.x += 1.0; }

    // Handle movement with Tnua's smooth acceleration/deceleration
    if movement_dir.length() > 0.0 {
        movement_dir = movement_dir.normalize();
        
        // Calculate movement relative to camera direction (but ignore camera pitch)
        let camera_forward = -camera_transform.local_z();
        let camera_right = camera_transform.local_x();
        
        // Project camera directions onto horizontal plane
        let horizontal_forward = Vec3::new(camera_forward.x, 0.0, camera_forward.z).normalize();
        let horizontal_right = Vec3::new(camera_right.x, 0.0, camera_right.z).normalize();
        
        // Calculate world movement direction
        let world_movement = horizontal_forward * movement_dir.z + horizontal_right * movement_dir.x;
        
        // Determine target speed (running vs walking)
        let target_speed = if input.up { // Shift for running
            movement_config.run_speed
        } else {
            movement_config.walk_speed
        };
        
        // Update movement state for animation system
        movement_state.target_speed = target_speed;
        movement_state.target_direction = world_movement;
        
        // Smooth acceleration towards target speed
        let acceleration_rate = movement_config.acceleration * time.delta_secs();
        let speed_diff = movement_state.target_speed - movement_state.current_speed;
        
        if speed_diff.abs() > 0.1 {
            let speed_change = speed_diff.signum() * acceleration_rate.min(speed_diff.abs());
            movement_state.current_speed += speed_change;
        } else {
            movement_state.current_speed = movement_state.target_speed;
        }
        
        // Smooth direction interpolation for turning
        let direction_lerp_speed = 15.0 * time.delta_secs();
        movement_state.current_direction = movement_state.current_direction
            .lerp(movement_state.target_direction, direction_lerp_speed.clamp(0.0, 1.0));
        
        // Feed movement to Tnua controller with current smoothed speed
        let desired_velocity = movement_state.current_direction * movement_state.current_speed;
        controller.basis(TnuaBuiltinWalk {
            desired_velocity,
            float_height: 0.0,        // No float - direct ground contact
            acceleration: movement_config.acceleration,
            air_acceleration: movement_config.air_acceleration,
            ..default()
        });
        
        // Rotate player to face movement direction for proper MMO feel
        if !input.mouse_right_held && movement_state.current_direction.length() > 0.1 {
             let target_direction = movement_state.current_direction.normalize();
             let target_rotation = Quat::from_rotation_y(target_direction.x.atan2(target_direction.z));
             
             // Smooth rotation interpolation (10 rad/s rotation speed)
             let t = (10.0 * time.delta_secs()).clamp(0.0, 1.0);
             player_transform.rotation = player_transform.rotation.slerp(target_rotation, t);
         }
    } else {
        // Smooth deceleration when no input
        movement_state.target_speed = 0.0;
        movement_state.target_direction = Vec3::ZERO;
        
        let deceleration_rate = movement_config.deceleration * time.delta_secs();
        
        if movement_state.current_speed > 0.1 {
            movement_state.current_speed = (movement_state.current_speed - deceleration_rate).max(0.0);
            
            // Apply decelerated movement through Tnua
            let desired_velocity = movement_state.current_direction * movement_state.current_speed;
            controller.basis(TnuaBuiltinWalk {
                desired_velocity,
                float_height: 0.0,
                acceleration: movement_config.acceleration,
                air_acceleration: movement_config.air_acceleration,
                ..default()
            });
        } else {
            // Full stop
            movement_state.current_speed = 0.0;
            movement_state.current_direction = Vec3::ZERO;
            
            // Set zero velocity through Tnua
            controller.basis(TnuaBuiltinWalk {
                desired_velocity: Vec3::ZERO,
                float_height: 0.0,
                acceleration: movement_config.acceleration,
                air_acceleration: movement_config.air_acceleration,
                ..default()
            });
        }
    }
    
    // Handle jumping through Tnua
    if input.down { // Space for jumping (input.up is shift for running)
        controller.action(TnuaBuiltinJump {
            height: movement_config.jump_height,
            ..default()
        });
    }
}