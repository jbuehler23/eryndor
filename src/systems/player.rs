use bevy::prelude::*;
use avian3d::prelude::*;
use crate::resources::InputResource;
use crate::components::{Player, PlayerMovementConfig, PlayerMovementState};



/// TRUE Kinematic character controller - Godot move_and_slide style
/// Uses manual position updates and spatial queries for collision detection  
pub fn kinematic_character_controller(
    time: Res<Time>,
    input: Res<InputResource>,
    mut player_query: Query<(Entity, &mut Transform, &PlayerMovementConfig, &mut PlayerMovementState, &Children), With<Player>>,
    camera_query: Query<(&Transform, &crate::systems::camera::GameCamera), (With<crate::systems::camera::GameCamera>, Without<Player>)>,
    spatial_query: SpatialQuery,
) {
    let Ok((player_entity, mut player_transform, movement_config, mut movement_state, children)) = player_query.single_mut() else {
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
        
        // Apply movement directly to Transform
        let desired_velocity = movement_state.current_direction * movement_state.current_speed;
        let movement_delta = desired_velocity * time.delta_secs();
        
        // Move character using collide_and_slide
        let new_position = collide_and_slide(
            player_transform.translation, 
            movement_delta,
            &spatial_query,
            player_entity,
            children
        );
        
        // Apply ground snapping for smooth terrain following
        player_transform.translation = apply_ground_snapping(
            new_position,
            &spatial_query,
            player_entity,
            children
        );
        
        // Rotate player to face movement direction
        if !input.mouse_right_held && movement_state.current_direction.length() > 0.1 {
             let target_direction = movement_state.current_direction.normalize();
             let target_rotation = Quat::from_rotation_y(target_direction.x.atan2(target_direction.z));
             
             // Fast rotation for responsive turning
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
            
            let new_position = collide_and_slide(
                player_transform.translation, 
                movement_delta,
                &spatial_query,
                player_entity,
                children
            );
            
            // Apply ground snapping for smooth terrain following
            player_transform.translation = apply_ground_snapping(
                new_position,
                &spatial_query,
                player_entity,
                children
            );
        } else {
            // Full stop
            movement_state.current_speed = 0.0;
            movement_state.current_direction = Vec3::ZERO;
            // No movement - Transform stays where it is
        }
    }
    
    // Handle jumping (Space key)
    if input.down && !movement_state.is_jumping {
        // Check if on ground using spatial query
        if is_grounded(player_transform.translation, &spatial_query, player_entity, children) {
            // Start jump with initial velocity
            let jump_velocity = (2.0 * 9.81 * movement_config.jump_height).sqrt(); // Physics formula: v = sqrt(2gh)
            movement_state.vertical_velocity = jump_velocity;
            movement_state.is_jumping = true;
        }
    }
    
    // Apply manual gravity and vertical movement (since kinematic bodies don't respond to physics gravity)
    apply_gravity_and_vertical_movement(&mut player_transform, &mut movement_state, &spatial_query, time.delta_secs(), player_entity, children);
}

/// Enhanced collide and slide movement with proper surface sliding
/// Uses iterative collision resolution to slide along surfaces instead of stopping
fn collide_and_slide(current_pos: Vec3, movement_delta: Vec3, spatial_query: &SpatialQuery, player_entity: Entity, children: &Children) -> Vec3 {
    const MAX_ITERATIONS: u32 = 4;
    const COLLISION_MARGIN: f32 = 0.02; // 2cm safety margin
    const MAX_WALKABLE_ANGLE: f32 = 45.0_f32.to_radians(); // 45 degrees
    
    let initial_movement_length = movement_delta.length();
    if initial_movement_length < 0.001 {
        return current_pos; // No movement needed
    }

    let mut position = current_pos;
    let mut remaining_movement = movement_delta;
    let capsule_shape = Collider::capsule(0.4, 1.8); // Match character collider size
    
    // Create filter to exclude player entity AND all child colliders
    let mut excluded_entities = vec![player_entity];
    for child in children.iter() {
        excluded_entities.push(child);
    }
    let filter = SpatialQueryFilter::default().with_excluded_entities(excluded_entities);

    for iteration in 0..MAX_ITERATIONS {
        let movement_length = remaining_movement.length();
        if movement_length < 0.001 {
            break; // No significant movement left
        }

        let movement_direction = remaining_movement.normalize();
        let shape_direction = Dir3::new(movement_direction).unwrap_or(Dir3::NEG_Y);
        
        // Shape cast configuration
        let shape_cast_config = ShapeCastConfig {
            max_distance: movement_length + COLLISION_MARGIN,
            ..default()
        };

        // Perform shape cast from capsule center
        if let Some(hit) = spatial_query.cast_shape(
            &capsule_shape,
            position + Vec3::new(0.0, 0.9, 0.0), // Center capsule at character center
            Quat::IDENTITY,
            shape_direction,
            &shape_cast_config,
            &filter
        ) {
            // Collision detected
            let hit_distance = hit.distance;
            let surface_normal = hit.normal1; // Surface normal
            
            // Move to collision point minus safety margin
            let safe_distance = (hit_distance - COLLISION_MARGIN).max(0.0);
            position += movement_direction * safe_distance;
            
            // Calculate remaining movement distance
            let consumed_distance = safe_distance;
            let remaining_distance = movement_length - consumed_distance;
            
            if remaining_distance <= 0.001 {
                break; // No significant movement left
            }

            // Classify surface type based on normal angle with up vector
            let angle_with_up = surface_normal.dot(Vec3::Y).acos();
            
            if angle_with_up <= MAX_WALKABLE_ANGLE {
                // Walkable surface - project movement along the slope
                remaining_movement = project_on_plane(
                    movement_direction * remaining_distance, 
                    surface_normal
                );
            } else {
                // Wall or steep slope - slide along the surface
                remaining_movement = project_on_plane(
                    movement_direction * remaining_distance, 
                    surface_normal
                );
            }
            
            // Reduce remaining movement slightly to prevent infinite bouncing
            remaining_movement *= 0.98;
            
            // Debug output for slope detection
            if iteration == 0 { // Only log first collision to avoid spam
                let angle_degrees = angle_with_up * 180.0 / std::f32::consts::PI;
                if angle_degrees > 5.0 {
                    println!("SLIDE DEBUG: hit surface at {:.1}° angle, sliding with {:.2} remaining movement", 
                             angle_degrees, remaining_movement.length());
                }
            }
        } else {
            // No collision - complete the remaining movement
            position += remaining_movement;
            break;
        }
    }

    position
}

/// Project a vector onto a plane defined by its normal
/// This is the core of the sliding algorithm - removes the component perpendicular to the surface
fn project_on_plane(vector: Vec3, plane_normal: Vec3) -> Vec3 {
    vector - plane_normal * vector.dot(plane_normal)
}

/// Check if character is on ground using downward raycast
fn is_grounded(pos: Vec3, spatial_query: &SpatialQuery, player_entity: Entity, children: &Children) -> bool {
    let ray_origin = pos;
    let ray_direction = Dir3::NEG_Y;
    let max_distance = 3.0; // Increased to ensure we can detect ground from spawn height
    
    // Create filter to exclude player entity AND all child colliders  
    let mut excluded_entities = vec![player_entity];
    for child in children.iter() {
        excluded_entities.push(child);
    }
    
    let filter = SpatialQueryFilter::default().with_excluded_entities(excluded_entities);
    
    if let Some(hit) = spatial_query.cast_ray(
        ray_origin, 
        ray_direction, 
        max_distance, 
        true, 
        &filter
    ) {
        // Consider grounded only if we hit something at a reasonable distance
        // Distance should be > 0.1 (to avoid self-collision) and < 1.2 (reasonable ground distance)
        hit.distance > 0.1 && hit.distance <= 1.2
    } else {
        false
    }
}

/// Apply ground snapping to keep character smoothly following terrain contours
/// This prevents floating gaps when moving over uneven terrain and slopes
fn apply_ground_snapping(position: Vec3, spatial_query: &SpatialQuery, player_entity: Entity, children: &Children) -> Vec3 {
    const SNAP_DISTANCE: f32 = 0.3; // Maximum distance to snap to ground
    const MIN_SNAP_DISTANCE: f32 = 0.05; // Minimum gap before snapping kicks in
    const MAX_WALKABLE_ANGLE: f32 = 45.0_f32.to_radians(); // 45 degrees
    
    // Create filter to exclude player entity AND all child colliders
    let mut excluded_entities = vec![player_entity];
    for child in children.iter() {
        excluded_entities.push(child);
    }
    let filter = SpatialQueryFilter::default().with_excluded_entities(excluded_entities);
    
    // Cast downward from player position
    if let Some(hit) = spatial_query.cast_ray(
        position,
        Dir3::NEG_Y,
        SNAP_DISTANCE,
        true,
        &filter
    ) {
        // Check if the surface is walkable
        let angle_with_up = hit.normal.dot(Vec3::Y).acos();
        
        if angle_with_up <= MAX_WALKABLE_ANGLE {
            // Surface is walkable, check if we need to snap
            if hit.distance > MIN_SNAP_DISTANCE && hit.distance <= SNAP_DISTANCE {
                // Snap to ground
                let snap_adjustment = hit.distance - MIN_SNAP_DISTANCE;
                return position - Vec3::Y * snap_adjustment;
            }
        }
    }
    
    // No snapping needed or possible
    position
}

/// Apply gravity and handle vertical movement for kinematic character (jumping/falling)
fn apply_gravity_and_vertical_movement(
    transform: &mut Transform, 
    movement_state: &mut PlayerMovementState, 
    spatial_query: &SpatialQuery, 
    delta_time: f32, 
    player_entity: Entity, 
    children: &Children
) {
    // Apply gravity to vertical velocity
    movement_state.vertical_velocity -= 9.81 * delta_time;
    
    // Calculate vertical movement delta
    let vertical_delta = Vec3::Y * movement_state.vertical_velocity * delta_time;
    
    // Apply vertical movement using collide_and_slide
    let new_position = collide_and_slide(transform.translation, vertical_delta, spatial_query, player_entity, children);
    
    // Check if we hit something (landed or hit ceiling)
    let actual_movement = new_position - transform.translation;
    let intended_movement = vertical_delta;
    
    // If we didn't move as much as intended in Y direction, we hit something
    if (actual_movement.y - intended_movement.y).abs() > 0.001 {
        // Check if we're moving downward (landing) or upward (hitting ceiling)
        if movement_state.vertical_velocity < 0.0 {
            // Landing - stop vertical movement and mark as grounded
            movement_state.vertical_velocity = 0.0;
            movement_state.is_jumping = false;
        } else {
            // Hit ceiling - stop upward movement but continue falling
            movement_state.vertical_velocity = 0.0;
        }
    }
    
    transform.translation = new_position;
    
    // Extra check: if we're on ground and not intentionally jumping, stop vertical movement
    if !movement_state.is_jumping && is_grounded(transform.translation, spatial_query, player_entity, children) {
        movement_state.vertical_velocity = 0.0;
    }
}