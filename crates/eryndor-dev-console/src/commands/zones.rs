//! Zone management commands.

use bevy::prelude::*;
use crate::{CommandRegistry, CommandDef, CommandResult, ConsoleState, DevModeChanged};

/// Register zone-related commands
pub fn register_zone_commands(registry: &mut CommandRegistry) {
    registry.register(CommandDef {
        name: "zone".to_string(),
        description: "Manage test zones".to_string(),
        usage: "zone <list|load|save|create> [name]".to_string(),
        aliases: vec![],
        category: "Zones".to_string(),
        function: cmd_zone,
    });
}

/// Zone management command
fn cmd_zone(
    _commands: &mut Commands,
    args: &[String],
    _console_state: &mut ConsoleState,
    _dev_mode_writer: &mut EventWriter<DevModeChanged>,
) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: zone <list|load|save|create> [name]".to_string());
    }
    
    let action = &args[0];
    
    match action.as_str() {
        "list" => {
            CommandResult::Success(
                "Available zones:\n\
                - test-zone: Basic testing area\n\
                - npc-showcase: NPC interaction testing\n\
                - dialogue-test: Dialogue system testing".to_string()
            )
        }
        "load" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: zone load <name>".to_string());
            }
            
            let zone_name = &args[1];
            
            // TODO: Actually load the zone and trigger zone transition
            CommandResult::Success(format!("Loading zone '{}'...", zone_name))
        }
        "save" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: zone save <name>".to_string());
            }
            
            let zone_name = &args[1];
            
            // TODO: Save current world state as a zone
            CommandResult::Success(format!("Saved current state as zone '{}'", zone_name))
        }
        "create" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: zone create <name>".to_string());
            }
            
            let zone_name = &args[1];
            
            // TODO: Create new empty zone
            CommandResult::Success(format!("Created new zone '{}'", zone_name))
        }
        _ => CommandResult::Error(format!(
            "Unknown zone action: '{}'. Available: list, load, save, create",
            action
        )),
    }
}