use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    InGame,
    Paused,
    Debug,
}

// State transition events
#[derive(Event)]
pub struct StateTransitionEvent {
    pub from: GameState,
    pub to: GameState,
}