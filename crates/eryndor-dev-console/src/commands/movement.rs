//! Movement and teleportation commands.

use bevy::prelude::*;
use eryndor_core::components::*;
use crate::{CommandRegistry, CommandDef, CommandResult, ConsoleState, DevModeChanged};

/// Component to mark entities that need teleportation
#[derive(Component)]
pub struct TeleportCommand {
    pub position: Vec3,
}

/// Component to mark entities that need speed changes
#[derive(Component)]
pub struct SpeedCommand {
    pub multiplier: f32,
}

/// System to process teleport commands
pub fn process_teleport_commands(
    mut commands: Commands,
    teleport_query: Query<(Entity, &TeleportCommand)>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    for (entity, teleport) in &teleport_query {
        if let Ok(mut transform) = player_query.single_mut() {
            transform.translation = teleport.position;
        }
        commands.entity(entity).despawn();
    }
}

/// System to process speed commands
pub fn process_speed_commands(
    mut commands: Commands,
    speed_query: Query<(Entity, &SpeedCommand)>,
    mut player_query: Query<&mut PlayerMovementConfig, With<Player>>,
) {
    for (entity, speed_cmd) in &speed_query {
        if let Ok(mut config) = player_query.single_mut() {
            let base_speed = 8.0; // Default base speed
            config.base_speed = base_speed * speed_cmd.multiplier;
            config.walk_speed = config.base_speed;
            config.run_speed = config.base_speed * 2.0;
        }
        commands.entity(entity).despawn();
    }
}

/// Register movement-related commands
pub fn register_movement_commands(registry: &mut CommandRegistry) {
    registry.register(CommandDef {
        name: "teleport".to_string(),
        description: "Teleport to coordinates".to_string(),
        usage: "teleport <x> <y> <z>".to_string(),
        aliases: vec!["tp".to_string()],
        category: "Movement".to_string(),
        function: cmd_teleport,
    });
    
    registry.register(CommandDef {
        name: "goto".to_string(),
        description: "Teleport to named location or NPC".to_string(),
        usage: "goto <zone|npc-name>".to_string(),
        aliases: vec!["warp".to_string()],
        category: "Movement".to_string(),
        function: cmd_goto,
    });
    
    registry.register(CommandDef {
        name: "speed".to_string(),
        description: "Set movement speed multiplier".to_string(),
        usage: "speed <multiplier>".to_string(),
        aliases: vec!["setspeed".to_string()],
        category: "Movement".to_string(),
        function: cmd_speed,
    });
    
    registry.register(CommandDef {
        name: "fly".to_string(),
        description: "Toggle flight mode".to_string(),
        usage: "fly".to_string(),
        aliases: vec![],
        category: "Movement".to_string(),
        function: cmd_fly,
    });
    
    registry.register(CommandDef {
        name: "noclip".to_string(),
        description: "Toggle collision detection".to_string(),
        usage: "noclip".to_string(),
        aliases: vec!["ghost".to_string()],
        category: "Movement".to_string(),
        function: cmd_noclip,
    });
}

/// Teleport command implementation
fn cmd_teleport(
    commands: &mut Commands,
    args: &[String],
    _console_state: &mut ConsoleState,
    _dev_mode_writer: &mut EventWriter<DevModeChanged>,
) -> CommandResult {
    if args.len() != 3 {
        return CommandResult::Error("Usage: teleport <x> <y> <z>".to_string());
    }
    
    let x = match args[0].parse::<f32>() {
        Ok(val) => val,
        Err(_) => return CommandResult::Error(format!("Invalid x coordinate: '{}'", args[0])),
    };
    
    let y = match args[1].parse::<f32>() {
        Ok(val) => val,
        Err(_) => return CommandResult::Error(format!("Invalid y coordinate: '{}'", args[1])),
    };
    
    let z = match args[2].parse::<f32>() {
        Ok(val) => val,
        Err(_) => return CommandResult::Error(format!("Invalid z coordinate: '{}'", args[2])),
    };
    
    // Queue teleport command for next frame
    commands.spawn(TeleportCommand {
        position: Vec3::new(x, y, z),
    });
    
    CommandResult::Success(format!("Teleported to ({:.1}, {:.1}, {:.1})", x, y, z))
}

/// Goto command implementation
fn cmd_goto(
    commands: &mut Commands,
    args: &[String],
    _console_state: &mut ConsoleState,
    _dev_mode_writer: &mut EventWriter<DevModeChanged>,
) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: goto <zone|npc-name>".to_string());
    }
    
    let target = &args[0];
    
    // Handle predefined zones
    let position = match target.as_str() {
        "spawn" => Vec3::new(-70.0, 15.0, -70.0),
        "test-zone" => Vec3::new(0.0, 10.0, 0.0),
        "origin" => Vec3::ZERO,
        "sky" => Vec3::new(0.0, 100.0, 0.0),
        _ => {
            // TODO: Find NPC or custom zone by name
            return CommandResult::Error(format!("Unknown location: '{}'. Available: spawn, test-zone, origin, sky", target));
        }
    };
    
    // Queue teleport command for next frame
    commands.spawn(TeleportCommand {
        position,
    });
    
    CommandResult::Success(format!("Teleported to {}", target))
}

/// Speed command implementation
fn cmd_speed(
    commands: &mut Commands,
    args: &[String],
    _console_state: &mut ConsoleState,
    _dev_mode_writer: &mut EventWriter<DevModeChanged>,
) -> CommandResult {
    if args.len() != 1 {
        return CommandResult::Error("Usage: speed <multiplier>".to_string());
    }
    
    let multiplier = match args[0].parse::<f32>() {
        Ok(val) if val > 0.0 => val,
        Ok(_) => return CommandResult::Error("Speed multiplier must be greater than 0".to_string()),
        Err(_) => return CommandResult::Error(format!("Invalid speed multiplier: '{}'", args[0])),
    };
    
    // Queue speed command for next frame
    commands.spawn(SpeedCommand {
        multiplier,
    });
    
    CommandResult::Success(format!("Movement speed set to {:.1}x", multiplier))
}

/// Flight mode toggle (placeholder)
fn cmd_fly(
    _commands: &mut Commands,
    _args: &[String],
    _console_state: &mut ConsoleState,
    _dev_mode_writer: &mut EventWriter<DevModeChanged>,
) -> CommandResult {
    // TODO: Implement flight mode by modifying gravity or physics
    CommandResult::Warning("Flight mode not yet implemented".to_string())
}

/// No-clip toggle (placeholder)
fn cmd_noclip(
    _commands: &mut Commands,
    _args: &[String],
    _console_state: &mut ConsoleState,
    _dev_mode_writer: &mut EventWriter<DevModeChanged>,
) -> CommandResult {
    // TODO: Implement no-clip by disabling player collision
    CommandResult::Warning("No-clip mode not yet implemented".to_string())
}