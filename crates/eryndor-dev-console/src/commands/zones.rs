//! Zone management commands.

use bevy::prelude::*;
use eryndor_dialogue::components::{NpcInfo, NpcType, DialogueState, DialogueInteractable};
use crate::{CommandRegistry, CommandDef, CommandResult, ConsoleState, DevModeChanged, ZoneTransition};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneDefinition {
    pub name: String,
    pub description: String,
    pub spawn_point: [f32; 3],
    pub terrain_type: String,
    pub npcs: Vec<ZoneNpc>,
    pub environment: ZoneEnvironment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneNpc {
    pub npc_type: String,
    pub position: [f32; 3],
    pub dialogue_id: Option<String>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneEnvironment {
    pub lighting: String,
    pub weather: String,
}

/// Component to queue zone loading
#[derive(Component)]
pub struct LoadZoneCommand {
    pub zone_name: String,
}

/// Component to queue zone clearing
#[derive(Component)]
pub struct ClearZoneCommand;

/// System to process zone loading commands
pub fn process_zone_loading_commands(
    mut commands: Commands,
    load_query: Query<(Entity, &LoadZoneCommand)>,
    clear_query: Query<Entity, With<ClearZoneCommand>>,
    npc_query: Query<Entity, With<NpcInfo>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut zone_transition_writer: EventWriter<ZoneTransition>,
) {
    // Process clear zone commands first
    for entity in &clear_query {
        // Remove all existing NPCs
        for npc_entity in &npc_query {
            commands.entity(npc_entity).despawn();
        }
        commands.entity(entity).despawn();
    }

    // Process zone loading commands
    for (entity, load_cmd) in &load_query {
        if let Ok(zone_def) = load_zone_definition(&load_cmd.zone_name) {
            // Clear existing NPCs first
            commands.spawn(ClearZoneCommand);
            
            // Load NPCs from zone definition
            for zone_npc in &zone_def.npcs {
                let npc_type = match zone_npc.npc_type.to_lowercase().as_str() {
                    "merchant" => NpcType::Merchant,
                    "guard" => NpcType::Guard,
                    "villager" => NpcType::Villager,
                    "noble" => NpcType::Noble,
                    "questgiver" => NpcType::Questgiver,
                    "informant" => NpcType::Informant,
                    _ => {
                        warn!("Unknown NPC type in zone: {}", zone_npc.npc_type);
                        continue;
                    }
                };
                
                let position = Vec3::new(zone_npc.position[0], zone_npc.position[1], zone_npc.position[2]);
                let npc_id = format!("{}_{}", zone_npc.npc_type, zone_npc.name.replace(" ", "_"));
                
                // Spawn NPC
                commands.spawn((
                    Mesh3d(meshes.add(Capsule3d::new(0.5, 2.0))),
                    MeshMaterial3d(materials.add(get_npc_color(npc_type))),
                    Transform::from_translation(position),
                    NpcInfo {
                        npc_id: npc_id.clone(),
                        display_name: zone_npc.name.clone(),
                        description: format!("A {} from the {} zone", zone_npc.npc_type, zone_def.name),
                        npc_type,
                    },
                    DialogueState {
                        npc_id: npc_id.clone(),
                        ..default()
                    },
                    DialogueInteractable {
                        npc_id,
                        ..default()
                    },
                    Name::new(zone_npc.name.clone()),
                ));
            }
            
            // Send zone transition event
            zone_transition_writer.write(ZoneTransition {
                from_zone: None, // We don't track current zone yet
                to_zone: zone_def.name.clone(),
            });
            
            info!("Loaded zone '{}' with {} NPCs", zone_def.name, zone_def.npcs.len());
        } else {
            error!("Failed to load zone: {}", load_cmd.zone_name);
        }
        
        commands.entity(entity).despawn();
    }
}

/// Load zone definition from JSON file
fn load_zone_definition(zone_name: &str) -> Result<ZoneDefinition, Box<dyn std::error::Error>> {
    let zone_path = format!("crates/eryndor-dev-console/zones/{}.json", zone_name);
    let zone_content = fs::read_to_string(&zone_path)
        .map_err(|e| format!("Failed to read zone file {}: {}", zone_path, e))?;
    
    let zone_def: ZoneDefinition = serde_json::from_str(&zone_content)
        .map_err(|e| format!("Failed to parse zone JSON: {}", e))?;
    
    Ok(zone_def)
}

/// Get available zone files
fn list_available_zones() -> Vec<String> {
    let zones_dir = "crates/eryndor-dev-console/zones/";
    
    if let Ok(entries) = fs::read_dir(zones_dir) {
        entries
            .filter_map(|entry| {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("json") {
                        path.file_stem()
                            .and_then(|s| s.to_str())
                            .map(|s| s.to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    } else {
        vec!["test-zone".to_string(), "npc-showcase".to_string()]
    }
}

/// Get NPC color for visual representation
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
    commands: &mut Commands,
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
            let zones = list_available_zones();
            let zone_list = if zones.is_empty() {
                "No zones found".to_string()
            } else {
                format!("Available zones:\n{}", 
                    zones.iter()
                        .map(|zone| format!("- {}", zone))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            };
            CommandResult::Success(zone_list)
        }
        "load" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: zone load <name>".to_string());
            }
            
            let zone_name = &args[1];
            
            // Queue zone loading command
            commands.spawn(LoadZoneCommand {
                zone_name: zone_name.clone(),
            });
            
            CommandResult::Success(format!("Queued zone '{}' for loading...", zone_name))
        }
        "clear" => {
            // Clear current zone
            commands.spawn(ClearZoneCommand);
            CommandResult::Success("Queued zone clearing...".to_string())
        }
        "save" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: zone save <name>".to_string());
            }
            
            let zone_name = &args[1];
            
            // TODO: Save current world state as a zone
            CommandResult::Warning(format!("Zone saving not yet implemented. Would save as '{}'", zone_name))
        }
        "create" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: zone create <name>".to_string());
            }
            
            let zone_name = &args[1];
            
            // TODO: Create new empty zone
            CommandResult::Warning(format!("Zone creation not yet implemented. Would create '{}'", zone_name))
        }
        _ => CommandResult::Error(format!(
            "Unknown zone action: '{}'. Available: list, load, clear, save, create",
            action
        )),
    }
}