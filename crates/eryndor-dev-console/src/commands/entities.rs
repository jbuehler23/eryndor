//! Entity creation and management commands.

use bevy::prelude::*;
use crate::{CommandRegistry, CommandDef, CommandResult, ConsoleState, DevModeChanged};

/// Register entity-related commands
pub fn register_entity_commands(registry: &mut CommandRegistry) {
    registry.register(CommandDef {
        name: "create".to_string(),
        description: "Create entities in the world".to_string(),
        usage: "create npc <type> [name] [x] [y] [z]".to_string(),
        aliases: vec!["spawn".to_string()],
        category: "Entities".to_string(),
        function: cmd_create,
    });
    
    registry.register(CommandDef {
        name: "delete".to_string(),
        description: "Delete entities from the world".to_string(),
        usage: "delete npc <id|name>".to_string(),
        aliases: vec!["remove".to_string()],
        category: "Entities".to_string(),
        function: cmd_delete,
    });
    
    registry.register(CommandDef {
        name: "list".to_string(),
        description: "List entities in the world".to_string(),
        usage: "list <npcs|players>".to_string(),
        aliases: vec!["ls".to_string()],
        category: "Entities".to_string(),
        function: cmd_list,
    });
}

/// Create entity command
fn cmd_create(
    _commands: &mut Commands,
    args: &[String],
    _console_state: &mut ConsoleState,
    _dev_mode_writer: &mut EventWriter<DevModeChanged>,
) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: create npc <type> [name] [x] [y] [z]".to_string());
    }
    
    let entity_type = &args[0];
    
    match entity_type.as_str() {
        "npc" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: create npc <type> [name] [x] [y] [z]".to_string());
            }
            
            let npc_type = &args[1];
            let npc_name = args.get(2).cloned().unwrap_or_else(|| format!("Test {}", npc_type));
            
            // Default position near player spawn
            let position = if args.len() >= 5 {
                match (args[2].parse::<f32>(), args[3].parse::<f32>(), args[4].parse::<f32>()) {
                    (Ok(x), Ok(y), Ok(z)) => Vec3::new(x, y, z),
                    _ => return CommandResult::Error("Invalid coordinates".to_string()),
                }
            } else {
                Vec3::new(-65.0, 16.0, -65.0) // Near spawn
            };
            
            // TODO: Actually spawn the NPC using the dialogue system's spawn_npc function
            CommandResult::Success(format!(
                "Created {} NPC '{}' at ({:.1}, {:.1}, {:.1})",
                npc_type, npc_name, position.x, position.y, position.z
            ))
        }
        _ => CommandResult::Error(format!("Unknown entity type: '{}'. Available: npc", entity_type)),
    }
}

/// Delete entity command
fn cmd_delete(
    _commands: &mut Commands,
    args: &[String],
    _console_state: &mut ConsoleState,
    _dev_mode_writer: &mut EventWriter<DevModeChanged>,
) -> CommandResult {
    if args.len() < 2 {
        return CommandResult::Error("Usage: delete npc <id|name>".to_string());
    }
    
    let entity_type = &args[0];
    let identifier = &args[1];
    
    match entity_type.as_str() {
        "npc" => {
            // TODO: Find and delete NPC by ID or name
            CommandResult::Success(format!("Deleted NPC '{}'", identifier))
        }
        _ => CommandResult::Error(format!("Unknown entity type: '{}'. Available: npc", entity_type)),
    }
}

/// List entities command
fn cmd_list(
    _commands: &mut Commands,
    args: &[String],
    _console_state: &mut ConsoleState,
    _dev_mode_writer: &mut EventWriter<DevModeChanged>,
) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: list <npcs|players>".to_string());
    }
    
    let entity_type = &args[0];
    
    match entity_type.as_str() {
        "npcs" => {
            // TODO: Query and list all NPCs with their positions and info
            CommandResult::Success("NPCs in world:\n[No NPCs currently implemented in list]".to_string())
        }
        "players" => {
            // TODO: List all players (useful for multiplayer)
            CommandResult::Success("Players in world:\n[Player listing not implemented]".to_string())
        }
        _ => CommandResult::Error(format!("Unknown entity type: '{}'. Available: npcs, players", entity_type)),
    }
}