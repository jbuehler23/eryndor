//! # Eryndor Core
//! 
//! Core traits, events, and shared components for the Eryndor game engine.
//! This crate provides the foundational types and interfaces that other Eryndor
//! crates depend on, ensuring clean separation between systems while enabling
//! communication through well-defined interfaces.

pub mod components;
pub mod events;
pub mod traits;

// Re-export commonly used types
pub use components::*;
pub use events::*;
pub use traits::*;

/// Version information for the core library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");