//! # Eryndor Dialogue System
//!
//! A comprehensive dialogue system for the Eryndor game engine that supports:
//! - JSON-driven conversation trees with branching dialogue
//! - Relationship tracking and trust levels
//! - Quest integration and progression
//! - Skill-based dialogue requirements
//! - Hot-reloading of dialogue files
//!
//! ## Features
//! - YarnSpinner-inspired dialogue format
//! - Event-driven architecture for loose coupling
//! - Extensible personality and relationship systems
//! - Rich dialogue choice consequences

use bevy::prelude::*;
use eryndor_core::events::DialogueEvent;

pub mod components;
pub mod resources;
pub mod systems;
pub mod plugin;

// Re-export commonly used types
pub use components::*;
pub use resources::*;
pub use plugin::EryndorDialoguePlugin;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{
        components::*,
        resources::*,
        systems::*,
        plugin::EryndorDialoguePlugin,
    };
    pub use eryndor_core::events::DialogueEvent;
}