use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;
use crate::resources::InputResource;
use crate::components::{Player, PlayerMovementConfig, PlayerMovementState};



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
        
        // Fast direction interpolation for responsive MMO turning
        let direction_lerp_speed = 25.0 * time.delta_secs(); // Faster turning for responsive feel
        movement_state.current_direction = movement_state.current_direction
            .lerp(movement_state.target_direction, direction_lerp_speed.clamp(0.0, 1.0));
        
        // Feed movement to Tnua controller with current smoothed speed
        let desired_velocity = movement_state.current_direction * movement_state.current_speed;
        controller.basis(TnuaBuiltinWalk {
            desired_velocity,
            float_height: 0.1,        // Proper float height > center to bottom distance
            acceleration: movement_config.acceleration,
            air_acceleration: movement_config.air_acceleration,
            cling_distance: 1.5,      // Better ground following over uneven terrain
            spring_strength: 400.0,   // Stronger spring for stable character positioning
            spring_dampening: 26.0,   // Balanced dampening for smooth movement
            ..default()
        });
        
        // Rotate player to face movement direction for proper MMO feel
        if !input.mouse_right_held && movement_state.current_direction.length() > 0.1 {
             let target_direction = movement_state.current_direction.normalize();
             let target_rotation = Quat::from_rotation_y(target_direction.x.atan2(target_direction.z));
             
             // Fast rotation for responsive MMO character turning (15 rad/s)
             let t = (15.0 * time.delta_secs()).clamp(0.0, 1.0);
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
                float_height: 0.1,        // Consistent float height
                acceleration: movement_config.acceleration,
                air_acceleration: movement_config.air_acceleration,
                cling_distance: 1.5,      // Better ground following over uneven terrain
                spring_strength: 400.0,   // Stronger spring for stable character positioning
                spring_dampening: 26.0,   // Balanced dampening for smooth movement
                ..default()
            });
        } else {
            // Full stop
            movement_state.current_speed = 0.0;
            movement_state.current_direction = Vec3::ZERO;
            
            // Set zero velocity through Tnua
            controller.basis(TnuaBuiltinWalk {
                desired_velocity: Vec3::ZERO,
                float_height: 0.1,        // Consistent float height
                acceleration: movement_config.acceleration,
                air_acceleration: movement_config.air_acceleration,
                cling_distance: 1.5,      // Better ground following over uneven terrain
                spring_strength: 400.0,   // Stronger spring for stable character positioning
                spring_dampening: 26.0,   // Balanced dampening for smooth movement
                ..default()
            });
        }
    }
    
    // Handle jumping through Tnua
    if input.down { // Space for jumping (input.up is shift for running)
        controller.action(TnuaBuiltinJump {
            height: movement_config.jump_height,
            upslope_extra_gravity: -20.0,   // Extra upward force when jumping uphill
            shorten_extra_gravity: 25.0,    // Faster falling for responsive jump feel
            allow_in_air: false,            // No double jumping for realistic MMO feel
            ..default()
        });
    }
}