//! Core traits that define interfaces between Eryndor systems.
//!
//! These traits establish clean boundaries between different crates and systems,
//! enabling loose coupling while maintaining type safety.

use bevy::prelude::*;
use crate::events::*;

/// Trait for systems that can handle quest events
pub trait QuestEventHandler {
    fn handle_quest_event(&mut self, event: &QuestEvent);
}

/// Trait for systems that can handle dialogue events  
pub trait DialogueEventHandler {
    fn handle_dialogue_event(&mut self, event: &DialogueEvent);
}

/// Trait for systems that can handle progression events
pub trait ProgressionEventHandler {
    fn handle_progression_event(&mut self, event: &ProgressionEvent);
}

/// Trait for configuration loading from JSON files
pub trait ConfigLoader<T> {
    type Error;
    
    fn load_from_file(path: &str) -> Result<T, Self::Error>;
    fn load_from_directory(dir: &str) -> Result<Vec<T>, Self::Error>;
    fn validate_config(config: &T) -> Result<(), Self::Error>;
}

/// Trait for systems that provide plugin functionality
pub trait EryndorPlugin {
    fn build(&self, app: &mut App);
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
}