use bevy::prelude::*;
use avian3d::prelude::*;
use crate::components::Player;
use crate::utils::{TerrainHeightSampler};
use crate::systems::terrain_simple::sample_terrain_height;
use crate::components::PlayerMovementState;
use crate::resources::{GameDebugConfig, DebugTimer};

/// Debug system for collision and terrain interaction analysis
/// Logs player position, velocity, and terrain height for debugging floating issues

#[derive(Resource)]
pub struct CollisionDebugConfig {
    pub enabled: bool,
    pub log_interval: f32,
    pub last_log_time: f32,
}

impl Default for CollisionDebugConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            log_interval: 2.0, // Log every 2 seconds
            last_log_time: 0.0,
        }
    }
}

/// System to debug player-terrain collision interaction
pub fn debug_player_collision(
    time: Res<Time>,
    debug_config: Res<GameDebugConfig>,
    mut debug_timer: ResMut<DebugTimer>,
    terrain_sampler: Option<Res<TerrainHeightSampler>>,
    gravity: Res<Gravity>,
    player_query: Query<(Entity, &Transform, Option<&LinearVelocity>, Option<&PlayerMovementState>, Option<&RigidBody>, Option<&Collider>), With<Player>>,
    children_query: Query<&Children>,
    collider_query: Query<&Collider>,
) {
    if !debug_config.collision_debug {
        return;
    }
    
    let current_time = time.elapsed_secs_f64();
    if !debug_timer.should_log_collision(current_time, debug_config.debug_update_interval) {
        return;
    }
    
    // Check if we should only log when moving
    if debug_config.debug_only_when_moving {
        let Ok((_, _, linear_velocity, controller_state, _, _)) = player_query.single() else {
            return;
        };
        
        let velocity = if let Some(movement_state) = controller_state {
            Vec3::new(
                movement_state.current_direction.x * movement_state.current_speed,
                movement_state.vertical_velocity,
                movement_state.current_direction.z * movement_state.current_speed
            )
        } else if let Some(linear_vel) = linear_velocity {
            **linear_vel
        } else {
            Vec3::ZERO
        };
        
        // Only log if player is actually moving
        if velocity.length() < 0.1 {
            return;
        }
    }
    
    let Ok((entity, transform, linear_velocity, controller_state, rigidbody, collider)) = player_query.single() else {
        return;
    };
    
    let player_pos = transform.translation;
    
    // Get terrain height at player position using height sampler
    let terrain_height = if let Some(sampler) = terrain_sampler.as_ref() {
        sample_terrain_height(sampler, player_pos.x, player_pos.z)
    } else {
        0.0
    };
    
    // Also get biome info for debugging (using direct sampling for biome type only)
    let (_direct_height, biome) = if let Some(sampler) = terrain_sampler.as_ref() {
        sampler.sample_height_and_biome(player_pos.x, player_pos.z)
    } else {
        (0.0, crate::systems::biomes::BiomeType::Plains)
    };
    
    // Calculate how far above/below terrain the player is
    let height_diff = player_pos.y - terrain_height;
    
    // Use player movement state velocity if available, otherwise fall back to physics velocity
    let velocity = if let Some(movement_state) = controller_state {
        Vec3::new(
            movement_state.current_direction.x * movement_state.current_speed,
            movement_state.vertical_velocity,
            movement_state.current_direction.z * movement_state.current_speed
        )
    } else if let Some(linear_vel) = linear_velocity {
        **linear_vel
    } else {
        Vec3::ZERO
    };
    
    // Check velocity to understand movement state
    let is_moving_horizontally = velocity.x.abs() > 0.1 || velocity.z.abs() > 0.1;
    let is_falling = velocity.y < -0.1;
    let is_rising = velocity.y > 0.1;
    
    info!("COLLISION DEBUG:");
    info!("   Player Entity: {:?}", entity);
    info!("   Player Pos: ({:.2}, {:.2}, {:.2})", player_pos.x, player_pos.y, player_pos.z);
    info!("   Terrain Height (Grid): {:.3}", terrain_height);
    info!("   Biome Type: {:?}", biome);
    info!("   Height Above Ground: {:.3}", height_diff);
    info!("   Velocity: ({:.2}, {:.2}, {:.2})", velocity.x, velocity.y, velocity.z);
    info!("   Velocity Magnitude: {:.4}", velocity.length());
    info!("   Movement State: Horizontal={}, Falling={}, Rising={}", 
          is_moving_horizontally, is_falling, is_rising);
    
    // Player movement state debug info
    if let Some(movement_state) = controller_state {
        info!("   Current Speed: {:.2}", movement_state.current_speed);
        info!("   Target Speed: {:.2}", movement_state.target_speed);
        info!("   Current Direction: ({:.2}, {:.2}, {:.2})",
              movement_state.current_direction.x, movement_state.current_direction.y, movement_state.current_direction.z);
        info!("   Is Jumping: {}", movement_state.is_jumping);
        info!("   Vertical Velocity: {:.2}", movement_state.vertical_velocity);
    }
    
    // Check for child colliders
    let mut has_child_collider = false;
    let mut child_collider_info = None;
    
    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            if let Ok(child_collider) = collider_query.get(child) {
                has_child_collider = true;
                child_collider_info = Some(child_collider);
                break;
            }
        }
    }
    
    // Debug physics components
    info!("   Gravity: {:?}", gravity.0);
    info!("   RigidBody: {:?}", rigidbody);
    info!("   Has Collider: {} (Parent: {}, Child: {})", 
          collider.is_some() || has_child_collider, collider.is_some(), has_child_collider);
    
    if let Some(collider) = collider {
        info!("   Parent Collider: {:?}", collider);
    }
    if let Some(child_collider) = child_collider_info {
        info!("   Child Collider: {:?}", child_collider);
    }
    
    // Flag potential issues
    if height_diff > 1.0 {
        warn!("Player is floating {:.2} units above ground!", height_diff);
    } else if height_diff < -0.5 {
        warn!("Player is {:.2} units below ground surface!", height_diff.abs());
    }
    
    // Check for bouncing behavior
    if velocity.y.abs() > 5.0 {
        warn!("High vertical velocity detected: {:.2} - possible bouncing!", velocity.y);
    }
}

/// Legacy collision debug toggle - now handled by DebugConfig system
/// This function is kept for backwards compatibility but does nothing
pub fn toggle_collision_debug(
    _keys: Res<ButtonInput<KeyCode>>,
    _debug_config: ResMut<CollisionDebugConfig>,
) {
    // This is now handled by the DebugConfig system with F1 key
    // Kept for backwards compatibility to prevent compilation errors
}