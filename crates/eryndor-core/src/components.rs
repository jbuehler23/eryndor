//! Core component definitions that are shared across all Eryndor systems.
//! 
//! These components represent the fundamental building blocks of game entities
//! and are used by multiple systems throughout the game engine.

use bevy::prelude::*;

/// Core player identification marker component  
#[derive(Component, Debug)]
pub struct Player;

/// Player movement configuration component
#[derive(Component, Debug)]
pub struct PlayerMovementConfig {
    pub base_speed: f32,
    pub run_speed: f32,
    pub walk_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub friction: f32,
    pub jump_height: f32,
}

impl Default for PlayerMovementConfig {
    fn default() -> Self {
        Self {
            base_speed: 8.0,
            run_speed: 16.0,
            walk_speed: 8.0,
            acceleration: 50.0,
            deceleration: 40.0,
            friction: 20.0,
            jump_height: 1.5,
        }
    }
}

/// Player movement state tracking component
#[derive(Component, Debug)]
pub struct PlayerMovementState {
    pub is_moving: bool,
    pub is_running: bool,
    pub velocity: Vec3,
    pub last_movement_input: Vec3,
    pub target_speed: f32,
    pub current_speed: f32,
    pub target_direction: Vec3,
    pub current_direction: Vec3,
    pub is_jumping: bool,
    pub vertical_velocity: f32,
}

impl Default for PlayerMovementState {
    fn default() -> Self {
        Self {
            is_moving: false,
            is_running: false,
            velocity: Vec3::ZERO,
            last_movement_input: Vec3::ZERO,
            target_speed: 0.0,
            current_speed: 0.0,
            target_direction: Vec3::ZERO,
            current_direction: Vec3::ZERO,
            is_jumping: false,
            vertical_velocity: 0.0,
        }
    }
}

/// Character type selection for different models
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
        CharacterType::Knight
    }
}

/// Component to track which character model is loaded
#[derive(Component)]
pub struct CharacterModel {
    pub character_type: CharacterType,
    pub model_entity: Option<Entity>,
}

impl Default for CharacterModel {
    fn default() -> Self {
        Self {
            character_type: CharacterType::default(),
            model_entity: None,
        }
    }
}