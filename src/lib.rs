use bevy::prelude::*;
use avian3d::prelude::*;
// use bevy_tnua::prelude::*;
// use bevy_tnua_avian3d::*;

pub mod systems;
pub mod components;
pub mod resources;
pub mod states;
pub mod utils;

use systems::*;
use resources::*;
use states::*;

// Re-export logging setup function for main.rs
pub use systems::logging::setup_logging;

pub struct EryndorPlugin;

impl Plugin for EryndorPlugin {
    fn build(&self, app: &mut App) {
        app
            // Physics - Avian 3D integration
            .add_plugins(PhysicsPlugins::default())
            .insert_resource(Gravity(Vec3::NEG_Y * 9.81)) // Earth-like gravity
            
            // Character controller - Direct Avian physics for now
            // TODO: Add Tnua integration once API is clarified
            
            // Resources - Global state
            .insert_resource(load_config())
            .init_resource::<InputResource>()
            
            // States - Game flow control
            .init_state::<GameState>()
            
            // Core systems - Order matters for dependencies
            .add_systems(Startup, (
                setup_camera,
                setup_ui,
                load_initial_assets,
                setup_animation_assets,
            ))
            .add_systems(Update, (
                handle_input,
                move_player.after(handle_input),
                update_animation_states.after(move_player),
                play_animations.after(update_animation_states),
                update_camera.after(move_player),
                update_ui,
                debug_animation_state.after(update_animation_states),
                check_asset_loading,
                update_config_system,
                save_config_on_exit,
                log_performance_metrics,
            ))
            // Debug systems - Only in debug builds
            .add_systems(Update, debug_info.run_if(in_state(GameState::Debug)));
    }
}

// System sets removed - using simple .after() ordering for now
// Following YAGNI principle - will add complex scheduling when needed

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;

    #[test]
    fn test_plugin_builds_without_panic() {
        // Test that the plugin can be added to an app without panicking
        let mut app = App::new();
        app.add_plugins(EryndorPlugin);
        // If we reach here, the plugin built successfully
        assert!(true);
    }

    #[test] 
    fn test_game_states_exist() {
        // Verify that all expected game states are defined
        use crate::states::GameState;
        
        let states = vec![
            GameState::Loading,
            GameState::MainMenu,
            GameState::InGame,
            GameState::Paused,
            GameState::Debug,
        ];
        
        assert_eq!(states.len(), 5);
    }
}