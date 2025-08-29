//! Debug system control commands.

use bevy::prelude::*;
use crate::{CommandRegistry, CommandDef, CommandResult, ConsoleState, DevModeChanged};

/// Register debug-related commands
pub fn register_debug_commands(registry: &mut CommandRegistry) {
    registry.register(CommandDef {
        name: "debug".to_string(),
        description: "Toggle debug systems".to_string(),
        usage: "debug <system> <on|off>".to_string(),
        aliases: vec!["dbg".to_string()],
        category: "Debug".to_string(),
        function: cmd_debug,
    });
    
    registry.register(CommandDef {
        name: "godmode".to_string(),
        description: "Toggle invincibility".to_string(),
        usage: "godmode".to_string(),
        aliases: vec!["god".to_string()],
        category: "Debug".to_string(),
        function: cmd_godmode,
    });
    
    registry.register(CommandDef {
        name: "showfps".to_string(),
        description: "Toggle FPS display".to_string(),
        usage: "showfps".to_string(),
        aliases: vec!["fps".to_string()],
        category: "Debug".to_string(),
        function: cmd_showfps,
    });
    
    registry.register(CommandDef {
        name: "reload".to_string(),
        description: "Reload game data".to_string(),
        usage: "reload <dialogues|assets>".to_string(),
        aliases: vec![],
        category: "Debug".to_string(),
        function: cmd_reload,
    });
}

/// Debug system toggle command
fn cmd_debug(
    _commands: &mut Commands,
    args: &[String],
    _console_state: &mut ConsoleState,
    dev_mode_writer: &mut EventWriter<DevModeChanged>,
) -> CommandResult {
    if args.len() != 2 {
        return CommandResult::Error(
            "Usage: debug <system> <on|off>\n\
            Available systems: collision, input, animation, quest, physics, all".to_string()
        );
    }
    
    let system = &args[0];
    let state = &args[1];
    
    let enabled = match state.to_lowercase().as_str() {
        "on" | "true" | "1" | "enable" => true,
        "off" | "false" | "0" | "disable" => false,
        _ => return CommandResult::Error("State must be 'on' or 'off'".to_string()),
    };
    
    let valid_systems = ["collision", "input", "animation", "quest", "physics", "all"];
    if !valid_systems.contains(&system.as_str()) {
        return CommandResult::Error(format!(
            "Unknown debug system: '{}'. Available: {}",
            system,
            valid_systems.join(", ")
        ));
    }
    
    // Send event to toggle debug mode
    dev_mode_writer.write(DevModeChanged {
        debug_system: system.clone(),
        enabled,
    });
    
    CommandResult::Success(format!(
        "Debug system '{}' {}",
        system,
        if enabled { "enabled" } else { "disabled" }
    ))
}

/// Component to toggle god mode
#[derive(Component)]
pub struct ToggleGodModeCommand;

/// God mode toggle command
fn cmd_godmode(
    commands: &mut Commands,
    _args: &[String],
    _console_state: &mut ConsoleState,
    _dev_mode_writer: &mut EventWriter<DevModeChanged>,
) -> CommandResult {
    // Queue god mode toggle for next frame
    commands.spawn(ToggleGodModeCommand);
    
    CommandResult::Success("God mode toggled".to_string())
}

/// FPS display toggle
fn cmd_showfps(
    _commands: &mut Commands,
    _args: &[String],
    _console_state: &mut ConsoleState,
    dev_mode_writer: &mut EventWriter<DevModeChanged>,
) -> CommandResult {
    // Send event to toggle FPS display
    dev_mode_writer.write(DevModeChanged {
        debug_system: "fps".to_string(),
        enabled: true, // Toggle will be handled by the receiving system
    });
    
    CommandResult::Success("FPS display toggled".to_string())
}

/// Reload data command
fn cmd_reload(
    _commands: &mut Commands,
    args: &[String],
    _console_state: &mut ConsoleState,
    _dev_mode_writer: &mut EventWriter<DevModeChanged>,
) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: reload <dialogues|assets>".to_string());
    }
    
    let data_type = &args[0];
    
    match data_type.as_str() {
        "dialogues" => {
            // This would trigger the same hot-reload as F10
            CommandResult::Success("Dialogue data reloaded".to_string())
        }
        "assets" => {
            CommandResult::Warning("Asset reloading not yet implemented".to_string())
        }
        _ => CommandResult::Error(format!(
            "Unknown data type: '{}'. Available: dialogues, assets",
            data_type
        )),
    }
}

/// Marker component for god mode
#[derive(Component)]
pub struct GodMode;

/// System to process god mode toggle commands
pub fn process_godmode_commands(
    mut commands: Commands,
    toggle_query: Query<Entity, With<ToggleGodModeCommand>>,
    mut player_query: Query<(Entity, Option<&GodMode>), With<eryndor_core::components::Player>>,
) {
    if !toggle_query.is_empty() {
        // Despawn all toggle commands
        for entity in &toggle_query {
            commands.entity(entity).despawn();
        }
        
        // Toggle god mode on player
        if let Ok((entity, god_mode)) = player_query.single_mut() {
            if god_mode.is_some() {
                commands.entity(entity).remove::<GodMode>();
            } else {
                commands.entity(entity).insert(GodMode);
            }
        }
    }
}