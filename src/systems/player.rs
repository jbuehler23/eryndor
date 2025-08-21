use bevy::prelude::*;
use avian3d::prelude::*;
use crate::resources::InputResource;
use crate::components::{Player, PlayerMovementConfig, PlayerMovementState};



/// TRUE Kinematic character controller - Godot move_and_slide style
/// Uses manual position updates and spatial queries for collision detection  
pub fn kinematic_character_controller(
    time: Res<Time>,
    input: Res<InputResource>,
    mut player_query: Query<(&mut Transform, &PlayerMovementConfig, &mut PlayerMovementState), With<Player>>,
    camera_query: Query<(&Transform, &crate::systems::camera::GameCamera), (With<crate::systems::camera::GameCamera>, Without<Player>)>,
    spatial_query: SpatialQuery,
) {
    let Ok((mut player_transform, movement_config, mut movement_state)) = player_query.single_mut() else {
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

    // Handle movement with direct velocity control (no physics conflicts)
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
        let direction_lerp_speed = 25.0 * time.delta_secs();
        movement_state.current_direction = movement_state.current_direction
            .lerp(movement_state.target_direction, direction_lerp_speed.clamp(0.0, 1.0));
        
        // Apply movement directly to Transform (TRUE kinematic approach)
        let desired_velocity = movement_state.current_direction * movement_state.current_speed;
        let movement_delta = desired_velocity * time.delta_secs();
        
        // Move character using collide_and_slide
        player_transform.translation = collide_and_slide(
            player_transform.translation, 
            movement_delta,
            &spatial_query
        );
        
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
            
            // Apply decelerated movement using collide_and_slide
            let desired_velocity = movement_state.current_direction * movement_state.current_speed;
            let movement_delta = desired_velocity * time.delta_secs();
            
            player_transform.translation = collide_and_slide(
                player_transform.translation, 
                movement_delta,
                &spatial_query
            );
        } else {
            // Full stop
            movement_state.current_speed = 0.0;
            movement_state.current_direction = Vec3::ZERO;
            // No movement - Transform stays where it is
        }
    }
    
    // Handle manual gravity and jumping (kinematic bodies don't have automatic gravity)
    if input.down { // Space for jumping
        // Check if on ground using spatial query
        if is_grounded(player_transform.translation, &spatial_query) {
            // Add vertical velocity component for jumping
            // This will be handled by manual gravity application
        }
    }
    
    // Apply manual gravity (since kinematic bodies don't respond to physics gravity)
    apply_manual_gravity(&mut player_transform, &spatial_query, time.delta_secs());
}

/// Collide and slide movement - the core of smooth character movement
/// This prevents getting stuck on terrain edges and provides smooth sliding
fn collide_and_slide(current_pos: Vec3, movement_delta: Vec3, _spatial_query: &SpatialQuery) -> Vec3 {
    // For now, simple implementation - apply movement directly
    // TODO: Add actual collision detection and sliding response
    current_pos + movement_delta
}

/// Check if character is on ground using downward raycast
fn is_grounded(pos: Vec3, spatial_query: &SpatialQuery) -> bool {
    let ray_origin = pos;
    let ray_direction = Dir3::NEG_Y;
    let max_distance = 1.1;
    
    if let Some(_hit) = spatial_query.cast_ray(
        ray_origin, 
        ray_direction, 
        max_distance, 
        true, 
        &SpatialQueryFilter::default()
    ) {
        true
    } else {
        false
    }
}

/// Apply manual gravity for kinematic character
fn apply_manual_gravity(transform: &mut Transform, spatial_query: &SpatialQuery, delta_time: f32) {
    // Simple ground check - if not grounded, apply gravity
    if !is_grounded(transform.translation, spatial_query) {
        let gravity_delta = Vec3::NEG_Y * 9.81 * delta_time;
        transform.translation = collide_and_slide(transform.translation, gravity_delta, spatial_query);
    }
}