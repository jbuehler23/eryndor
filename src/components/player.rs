use bevy::prelude::*;

/// Player marker component - identifies the player entity
/// Following Single Responsibility Principle: only marks an entity as the player
#[derive(Component)]
pub struct Player;

/// Player movement configuration - physics-based movement properties
/// Works with Tnua character controller for realistic movement
#[derive(Component)]
pub struct PlayerMovementConfig {
    pub walk_speed: f32,
    pub run_speed: f32,
    pub jump_height: f32,
    pub acceleration: f32,
    pub air_acceleration: f32,
}

impl Default for PlayerMovementConfig {
    fn default() -> Self {
        Self {
            walk_speed: 8.0,
            run_speed: 12.0,
            jump_height: 4.0,
            acceleration: 40.0,
            air_acceleration: 20.0,
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

/// Character type selection for different models
/// Following Open/Closed: Easy to add new character types
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum CharacterType {
    Knight,
    Mage,
    Rogue,
    Barbarian,
    RogueHooded,
}

impl Default for CharacterType {
    fn default() -> Self {
        CharacterType::Knight // Default to knight character
    }
}

/// Component to track which character model is loaded
#[derive(Component)]
pub struct CharacterModel {
    pub character_type: CharacterType,
    pub model_entity: Option<Entity>, // Track the spawned model entity
}

impl Default for CharacterModel {
    fn default() -> Self {
        Self {
            character_type: CharacterType::default(),
            model_entity: None,
        }
    }
}