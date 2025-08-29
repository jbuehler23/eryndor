//! Entity creation and management commands.

use bevy::prelude::*;
use eryndor_dialogue::components::{NpcInfo, NpcType, DialogueState, DialogueInteractable};
use crate::{CommandRegistry, CommandDef, CommandResult, ConsoleState, DevModeChanged};

/// Component to queue NPC spawning
#[derive(Component)]
pub struct SpawnNpcCommand {
    pub npc_type: NpcType,
    pub name: String,
    pub position: Vec3,
}

/// Component to queue entity deletion
#[derive(Component)]
pub struct DeleteEntityCommand {
    pub entity_type: String,
    pub identifier: String,
}

/// System to process NPC spawn commands
pub fn process_spawn_npc_commands(
    mut commands: Commands,
    spawn_query: Query<(Entity, &SpawnNpcCommand)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, spawn_cmd) in &spawn_query {
        // Generate unique NPC ID
        let npc_id = format!("{:?}_{}", spawn_cmd.npc_type, entity.index());
        
        // Create NPC entity with visual representation
        let _npc_entity = commands.spawn((
            Mesh3d(meshes.add(Capsule3d::new(0.5, 2.0))),
            MeshMaterial3d(materials.add(get_npc_color(spawn_cmd.npc_type))),
            Transform::from_translation(spawn_cmd.position),
            // Dialogue components
            NpcInfo {
                npc_id: npc_id.clone(),
                display_name: spawn_cmd.name.clone(),
                description: format!("A {} created via dev console", format!("{:?}", spawn_cmd.npc_type).to_lowercase()),
                npc_type: spawn_cmd.npc_type,
            },
            DialogueState {
                npc_id: npc_id.clone(),
                ..default()
            },
            DialogueInteractable {
                npc_id,
                ..default()
            },
            Name::new(spawn_cmd.name.clone()),
        )).id();
        
        // Remove the spawn command
        commands.entity(entity).despawn();
    }
}

/// System to process entity deletion commands
pub fn process_delete_entity_commands(
    mut commands: Commands,
    delete_query: Query<(Entity, &DeleteEntityCommand)>,
    npc_query: Query<(Entity, &NpcInfo)>,
) {
    for (command_entity, delete_cmd) in &delete_query {
        match delete_cmd.entity_type.as_str() {
            "npc" => {
                // Find NPC by name or ID
                let mut found = false;
                for (entity, npc_info) in &npc_query {
                    if npc_info.display_name == delete_cmd.identifier || 
                       npc_info.npc_id == delete_cmd.identifier {
                        commands.entity(entity).despawn();
                        found = true;
                        break;
                    }
                }
                if !found {
                    warn!("Could not find NPC with name or ID: {}", delete_cmd.identifier);
                }
            },
            _ => {
                warn!("Unknown entity type for deletion: {}", delete_cmd.entity_type);
            }
        }
        
        // Remove the delete command
        commands.entity(command_entity).despawn();
    }
}

/// Get color for different NPC types
fn get_npc_color(npc_type: NpcType) -> StandardMaterial {
    match npc_type {
        NpcType::Merchant => StandardMaterial::from(Color::srgb(0.8, 0.6, 0.2)), // Gold
        NpcType::Guard => StandardMaterial::from(Color::srgb(0.2, 0.2, 0.8)), // Blue
        NpcType::Villager => StandardMaterial::from(Color::srgb(0.4, 0.8, 0.2)), // Green
        NpcType::Noble => StandardMaterial::from(Color::srgb(0.8, 0.2, 0.8)), // Purple
        NpcType::Questgiver => StandardMaterial::from(Color::srgb(0.8, 0.4, 0.2)), // Orange
        NpcType::Informant => StandardMaterial::from(Color::srgb(0.6, 0.6, 0.6)), // Gray
    }
}

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
    commands: &mut Commands,
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
                return CommandResult::Error("Usage: create npc <type> [name] [x] [y] [z]\nAvailable types: merchant, guard, villager, noble, questgiver, informant".to_string());
            }
            
            let npc_type_str = &args[1].to_lowercase();
            let npc_type = match npc_type_str.as_str() {
                "merchant" => NpcType::Merchant,
                "guard" => NpcType::Guard,
                "villager" => NpcType::Villager,
                "noble" => NpcType::Noble,
                "questgiver" => NpcType::Questgiver,
                "informant" => NpcType::Informant,
                _ => return CommandResult::Error(format!("Invalid NPC type: '{}'. Available: merchant, guard, villager, noble, questgiver, informant", npc_type_str)),
            };
            
            let npc_name = args.get(2).cloned().unwrap_or_else(|| format!("Test {}", npc_type_str));
            
            // Parse position - use name as 3rd arg if it's not a number, otherwise treat args 2-4 as coordinates
            let position = if args.len() >= 5 {
                // Name provided: args[2] = name, args[3-5] = coordinates
                match (args[3].parse::<f32>(), args[4].parse::<f32>(), args.get(5).and_then(|s| s.parse::<f32>().ok()).unwrap_or(16.0)) {
                    (Ok(x), Ok(y), z) => Vec3::new(x, y, z),
                    _ => return CommandResult::Error("Invalid coordinates".to_string()),
                }
            } else if args.len() == 4 {
                // No name, just coordinates: args[2-4] = coordinates
                if args[2].parse::<f32>().is_ok() {
                    match (args[2].parse::<f32>(), args[3].parse::<f32>()) {
                        (Ok(x), Ok(y)) => Vec3::new(x, y, 16.0),
                        _ => return CommandResult::Error("Invalid coordinates".to_string()),
                    }
                } else {
                    // args[2] is name, default position
                    Vec3::new(-65.0, 16.0, -65.0)
                }
            } else {
                // Default position near player spawn
                Vec3::new(-65.0, 16.0, -65.0)
            };
            
            // Queue NPC spawn command
            commands.spawn(SpawnNpcCommand {
                npc_type,
                name: npc_name.clone(),
                position,
            });
            
            CommandResult::Success(format!(
                "Queued {} NPC '{}' for spawn at ({:.1}, {:.1}, {:.1})",
                npc_type_str, npc_name, position.x, position.y, position.z
            ))
        }
        _ => CommandResult::Error(format!("Unknown entity type: '{}'. Available: npc", entity_type)),
    }
}

/// Delete entity command
fn cmd_delete(
    commands: &mut Commands,
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
            // Queue delete command
            commands.spawn(DeleteEntityCommand {
                entity_type: "npc".to_string(),
                identifier: identifier.clone(),
            });
            
            CommandResult::Success(format!("Queued NPC '{}' for deletion", identifier))
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
            // Note: This command would need access to World to query NPCs
            // For now, provide instructions
            CommandResult::Success("NPCs in world:\n[Use this command in-game to see active NPCs]\nTip: NPCs are color-coded by type - Merchants=Gold, Guards=Blue, Villagers=Green".to_string())
        }
        "players" => {
            CommandResult::Success("Players in world:\n[Player listing will be available in multiplayer mode]".to_string())
        }
        _ => CommandResult::Error(format!("Unknown entity type: '{}'. Available: npcs, players", entity_type)),
    }
}