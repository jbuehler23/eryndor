use bevy::prelude::*;
use avian3d::prelude::*;

pub mod systems;
pub mod components;
pub mod resources;
pub mod states;
pub mod utils;

use systems::*;
use resources::*;
use states::*;
use components::quest::QuestEvent;
use components::dialogue::DialogueEvent;

// Re-export logging setup function for main.rs
pub use systems::logging::setup_logging;

pub struct EryndorPlugin;

impl Plugin for EryndorPlugin {
    fn build(&self, app: &mut App) {
        app
            // Physics - Avian 3D integration
            .add_plugins(PhysicsPlugins::default())
            // .add_plugins(PhysicsDiagnosticsPlugin) // Enable physics diagnostics overlay
            // .add_plugins(PhysicsDiagnosticsUiPlugin) // UI for toggling physics debug features
            // .add_plugins(avian3d::debug_render::PhysicsDebugPlugin::default()) // Enable collision shape visualization
            .insert_resource(Gravity(Vec3::NEG_Y * 9.81)) // Earth-like gravity
            
            // Character controller - Simple MMO-style kinematic controller
            
            // Resources - Global state
            .insert_resource(load_config())
            .init_resource::<InputResource>()
            .init_resource::<CollisionDebugConfig>() // Debug collision interaction
            .init_resource::<GameDebugConfig>() // Configurable debug logging
            .init_resource::<DebugTimer>() // Debug timing control
            // V1 StatsConfig removed with character progression cleanup
            
            // States - Game flow control
            .init_state::<GameState>()
            
            // Core systems - Order matters for dependencies
            .add_systems(Startup, (
                setup_camera,
                setup_ui,
                load_initial_assets,
                load_quest_database,
                load_dialogue_database,
            ))
            
            // Main menu systems - only in MainMenu state
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(Update, handle_main_menu_interactions.run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
            
            // In-game UI systems - only in InGame state
            .add_systems(OnEnter(GameState::InGame), (setup_ingame_ui, setup_combat_system))
            .add_systems(Update, (
                update_skill_overview,
                handle_experience_notifications,
                handle_ingame_escape,
            ).run_if(in_state(GameState::InGame)))
            .add_systems(OnExit(GameState::InGame), cleanup_ingame_ui)
            
            // Quest Journal UI systems - only in InGame state
            .init_resource::<QuestJournalState>()
            .add_systems(Update, (
                toggle_quest_journal,
                manage_quest_journal_state,
                handle_journal_tab_switching,
                handle_quest_selection,
                update_quest_journal_content.after(manage_quest_journal_state),
                cleanup_quest_journal,
            ).run_if(in_state(GameState::InGame)))
            
            // Combat systems - only in InGame state
            .add_systems(Update, (
                spawn_demo_enemies,
                handle_target_selection,
                handle_player_auto_attack,
                cleanup_dead_enemies,
                display_target_health,
            ).run_if(in_state(GameState::InGame)))
            .add_systems(Startup, (
                setup_animation_assets,
                // setup_character_controller, // Not needed for simple kinematic controller
                // setup_terrain, // TEMPORARILY DISABLED: Complex terrain system with physics mismatch
                setup_simple_terrain, // TESTING: Clean terrain system with perfect physics alignment
                // setup_biomes.after(setup_terrain), // TEMPORARILY DISABLED: Depends on complex terrain system
                load_world_object_assets, // Load forest/nature assets
            ).after(load_initial_assets))
            // Core gameplay systems - only in InGame state
            .add_systems(Update, (
                spawn_player_when_assets_loaded.after(load_initial_assets),
                handle_input,
                toggle_collision_debug,
                kinematic_character_controller.after(handle_input),
            ).run_if(in_state(GameState::InGame)))
            // Character progression systems - only in InGame state
            .add_systems(Update, (
                character_level_system,
                skill_usage_system,
                loadout_management_system,
                debug_character_v2_system,
                debug_rested_bonus_system,
                debug_award_character_experience_system,
                debug_quest_rewards_system,
            ).run_if(in_state(GameState::InGame)))
            
            // Quest systems - only in InGame state
            .add_systems(Update, (
                initialize_quest_log,
                quest_start_system,
                investigation_system,
                quest_status_system,
                quest_event_handler,
            ).run_if(in_state(GameState::InGame)))
            .add_event::<QuestEvent>()
            
            // Enhanced Dialogue systems - only in InGame state
            .add_systems(OnEnter(GameState::InGame), (
                spawn_demo_npcs.after(load_dialogue_database),
            ))
            .add_systems(Update, (
                enhanced_dialogue_interaction_system,
                process_dialogue_choice,
                enhanced_dialogue_help_system,
                hot_reload_dialogue_system,
                npc_mouse_hover_system,
                update_npc_indicators.after(npc_mouse_hover_system),
                npc_interaction_prompts,
                debug_npc_info,
            ).run_if(in_state(GameState::InGame)))
            .add_event::<DialogueEvent>()
            // Animation and camera systems - only in InGame state
            .add_systems(Update, (
                update_animation_states.after(kinematic_character_controller),
                setup_knight_animations_when_ready,
                play_animations.after(update_animation_states),
                update_camera.after(kinematic_character_controller),
                debug_animation_state.after(update_animation_states),
                debug_animation_changes.after(update_animation_states),
            ).run_if(in_state(GameState::InGame)))
            // UI and utility systems
            .add_systems(Update, (
                update_ui,
                update_stats_ui,
                debug_player_collision.after(kinematic_character_controller),
                check_asset_loading,
                update_config_system,
                save_config_on_exit,
                log_performance_metrics,
                debug_config_system, // Debug configuration controls
                debug_help_system, // Debug help display
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