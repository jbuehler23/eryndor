use bevy::prelude::*;
use avian3d::prelude::*;
use crate::resources::InputResource;
use crate::components::{Player, PlayerMovementConfig};

/// Physics-based player movement system using Avian physics directly
/// Following Single Responsibility: only handles player movement with physics and rotation
pub fn move_player(
    time: Res<Time>,
    input: Res<InputResource>,
    mut player_query: Query<(&mut LinearVelocity, &PlayerMovementConfig, &mut Transform), With<Player>>,
    camera_query: Query<(&Transform, &crate::systems::camera::GameCamera), (With<crate::systems::camera::GameCamera>, Without<Player>)>,
) {
    let Ok((mut velocity, movement_config, mut player_transform)) = player_query.single_mut() else {
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

    // Handle movement
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
        
        // Determine speed (running vs walking)
        let speed = if input.up { // Shift for running
            movement_config.run_speed
        } else {
            movement_config.walk_speed
        };
        
        // Apply velocity directly to physics body
        velocity.x = world_movement.x * speed;
        velocity.z = world_movement.z * speed;
        
        // Rotate player to face movement direction for proper MMO feel
        if !input.mouse_right_held && world_movement.length() > 0.1 {
             let target_direction = world_movement.normalize();
             let target_rotation = Quat::from_rotation_y(target_direction.x.atan2(target_direction.z));
             
             // Smooth rotation interpolation (10 rad/s rotation speed)
             let t = (10.0 * time.delta_secs()).clamp(0.0, 1.0);
             player_transform.rotation = player_transform.rotation.slerp(target_rotation, t);
         }
    } else {
        // Stop horizontal movement when no input
        velocity.x = 0.0;
        velocity.z = 0.0;
    }
    
    // Handle jumping - give upward velocity
    if input.down { // Space for jumping (input.up is shift for running)
        if velocity.y.abs() < 0.1 { // Only jump if not already in air
            velocity.y = (2.0 * 9.81 * movement_config.jump_height).sqrt(); // Physics formula for jump
        }
    }
}