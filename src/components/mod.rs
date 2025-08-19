pub mod core;
pub mod player;
pub mod animation;

pub use player::{Player, PlayerMovementConfig, PlayerMovementState, PlayerStats, CharacterType, CharacterModel};
pub use animation::*;