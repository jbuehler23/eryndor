use bevy::prelude::*;
use avian3d::{diagnostics::PhysicsDiagnostics, prelude::*};
// Removed Tnua imports - using custom character controller

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
            .add_plugins(PhysicsDiagnosticsPlugin) // Enable physics diagnostics overlay
            .add_plugins(PhysicsDiagnosticsUiPlugin) // UI for toggling physics debug features
            .add_plugins(avian3d::debug_render::PhysicsDebugPlugin::default()) // Enable collision shape visualization
            .insert_resource(Gravity(Vec3::NEG_Y * 9.81)) // Earth-like gravity
            
            // Character controller - Simple MMO-style kinematic controller
            
            // Resources - Global state
            .insert_resource(load_config())
            .init_resource::<InputResource>()
            .init_resource::<CollisionDebugConfig>() // Debug collision interaction
            
            // States - Game flow control
            .init_state::<GameState>()
            
            // Core systems - Order matters for dependencies
            .add_systems(Startup, (
                setup_camera, // Then setup camera to look at player
                setup_ui,
                load_initial_assets,
                setup_animation_assets,
                // setup_character_controller, // Not needed for simple kinematic controller
                // setup_terrain, // TEMPORARILY DISABLED: Complex terrain system with physics mismatch
                setup_simple_terrain, // TESTING: Clean terrain system with perfect physics alignment
                // setup_biomes.after(setup_terrain), // TEMPORARILY DISABLED: Depends on complex terrain system
                load_world_object_assets, // Load forest/nature assets
            ))
            .add_systems(Update, (
                spawn_player_when_assets_loaded.after(load_initial_assets),
                handle_input,
                toggle_collision_debug, // F3 to toggle collision debug
                kinematic_character_controller.after(handle_input), // Simple MMO-style controller
                update_animation_states.after(kinematic_character_controller),
                setup_knight_animations_when_ready, // New system to setup animations when scene loads
                play_animations.after(update_animation_states),
                update_camera.after(kinematic_character_controller),
                update_ui,
                debug_animation_state.after(update_animation_states),
                debug_player_collision.after(kinematic_character_controller), // Debug player-terrain collision
                check_asset_loading,
                // spawn_world_objects.after(load_world_object_assets).after(setup_biomes), // TEMPORARILY DISABLED: Depends on biomes
                update_config_system,
                save_config_on_exit,
                log_performance_metrics,
            ))
            // Debug systems - Only in debug builds
            .add_systems(Update, (
                debug_info,
                collect_performance_metrics,
                // debug_biome_visualization, // TEMPORARILY DISABLED: Depends on biomes  
                debug_terrain_alignment, // Debug the simplified terrain height system
            ).run_if(in_state(GameState::Debug)));
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