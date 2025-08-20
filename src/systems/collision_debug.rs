use bevy::prelude::*;
use avian3d::prelude::*;
use crate::components::Player;
use crate::systems::terrain::{TerrainHeightSampler, sample_terrain_height};

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
    mut debug_config: ResMut<CollisionDebugConfig>,
    terrain_sampler: Option<Res<TerrainHeightSampler>>,
    gravity: Res<Gravity>,
    player_query: Query<(Entity, &Transform, &LinearVelocity, Option<&RigidBody>, Option<&Collider>), With<Player>>,
) {
    if !debug_config.enabled {
        return;
    }
    
    let current_time = time.elapsed_secs();
    if current_time - debug_config.last_log_time < debug_config.log_interval {
        return;
    }
    
    debug_config.last_log_time = current_time;
    
    let Ok((entity, transform, velocity, rigidbody, collider)) = player_query.single() else {
        return;
    };
    
    let player_pos = transform.translation;
    
    // Get terrain height at player position
    let terrain_height = if let Some(sampler) = terrain_sampler.as_ref() {
        sample_terrain_height(sampler, player_pos.x, player_pos.z)
    } else {
        0.0
    };
    
    // Calculate how far above/below terrain the player is
    let height_diff = player_pos.y - terrain_height;
    
    // Check velocity to understand movement state
    let is_moving_horizontally = velocity.x.abs() > 0.1 || velocity.z.abs() > 0.1;
    let is_falling = velocity.y < -0.1;
    let is_rising = velocity.y > 0.1;
    
    info!("COLLISION DEBUG:");
    info!("   Player Entity: {:?}", entity);
    info!("   Player Pos: ({:.2}, {:.2}, {:.2})", player_pos.x, player_pos.y, player_pos.z);
    info!("   Terrain Height: {:.2}", terrain_height);
    info!("   Height Above Ground: {:.2}", height_diff);
    info!("   Velocity: ({:.2}, {:.2}, {:.2})", velocity.x, velocity.y, velocity.z);
    info!("   Velocity Magnitude: {:.4}", velocity.length());
    info!("   Movement State: Horizontal={}, Falling={}, Rising={}", 
          is_moving_horizontally, is_falling, is_rising);
    
    // Debug physics components
    info!("   Gravity: {:?}", gravity.0);
    info!("   RigidBody: {:?}", rigidbody);
    info!("   Has Collider: {}", collider.is_some());
    if let Some(collider) = collider {
        info!("   Collider Type: {:?}", collider);
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

/// System to toggle collision debug rendering (when F3 is pressed)
pub fn toggle_collision_debug(
    keys: Res<ButtonInput<KeyCode>>,
    mut debug_config: ResMut<CollisionDebugConfig>,
) {
    if keys.just_pressed(KeyCode::F3) {
        debug_config.enabled = !debug_config.enabled;
        info!("Collision debug {}", if debug_config.enabled { "ENABLED" } else { "DISABLED" });
    }
}