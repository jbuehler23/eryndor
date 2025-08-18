use bevy::prelude::*;

/// Player marker component - identifies the player entity
/// Following Single Responsibility Principle: only marks an entity as the player
#[derive(Component)]
pub struct Player;

/// Player movement component - handles player-specific movement properties
/// Separate from camera for better separation of concerns
#[derive(Component)]
pub struct PlayerMovement {
    pub speed: f32,
    pub run_speed: f32,
    pub is_running: bool,
}

impl Default for PlayerMovement {
    fn default() -> Self {
        Self {
            speed: 5.0,
            run_speed: 10.0,
            is_running: false,
        }
    }
}

/// Player stats component - basic player attributes
/// Kept simple for Phase 1, expandable for later phases
#[derive(Component)]
pub struct PlayerStats {
    pub health: f32,
    pub max_health: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
        }
    }
}