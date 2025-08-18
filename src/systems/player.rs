use bevy::prelude::*;
use crate::resources::InputResource;
use crate::components::{Player, PlayerMovement};

/// Player movement system - moves player based on input
/// Following Single Responsibility: only handles player movement
pub fn move_player(
    time: Res<Time>,
    input: Res<InputResource>,
    mut player_query: Query<(&mut Transform, &PlayerMovement), With<Player>>,
    camera_query: Query<&Transform, (With<crate::systems::camera::GameCamera>, Without<Player>)>,
) {
    let Ok((mut player_transform, movement)) = player_query.single_mut() else {
        return;
    };
    
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    // Check for movement input
    let mut movement_dir = Vec3::ZERO;
    
    // Classic WoW behavior: both mouse buttons = move forward
    let both_mouse_forward = input.mouse_left_held && input.mouse_right_held;
    
    if input.forward || both_mouse_forward { movement_dir.z += 1.0; }
    if input.backward { movement_dir.z -= 1.0; }
    if input.left { movement_dir.x -= 1.0; }
    if input.right { movement_dir.x += 1.0; }

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
            movement.run_speed
        } else {
            movement.speed
        };
        
        // Apply movement
        player_transform.translation += world_movement * speed * time.delta_secs();
        
        // Rotate player to face movement direction
        if world_movement.length() > 0.0 {
            let target_rotation = Quat::from_rotation_y(world_movement.z.atan2(world_movement.x) - std::f32::consts::FRAC_PI_2);
            player_transform.rotation = player_transform.rotation.lerp(target_rotation, time.delta_secs() * 10.0);
        }
    }
}