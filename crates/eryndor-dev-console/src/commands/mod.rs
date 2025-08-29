//! Command registry and processing system.

use bevy::prelude::*;
use std::collections::HashMap;
use crate::{ConsoleCommand, ConsoleState, ConsoleLine, DevModeChanged};

pub mod movement;
pub mod entities;
pub mod debug;
pub mod zones;

pub use movement::*;
pub use entities::*;
pub use debug::*;
pub use zones::*;

/// Command function signature
pub type CommandFn = fn(&mut Commands, &[String], &mut ConsoleState, &mut EventWriter<DevModeChanged>) -> CommandResult;

/// Command result
#[derive(Debug)]
pub enum CommandResult {
    Success(String),
    Error(String),
    Warning(String),
}

/// Command definition
#[derive(Clone)]
pub struct CommandDef {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub aliases: Vec<String>,
    pub category: String,
    pub function: CommandFn,
}

/// Command registry resource
#[derive(Resource, Default)]
pub struct CommandRegistry {
    pub commands: HashMap<String, CommandDef>,
    pub categories: Vec<String>,
}

impl CommandRegistry {
    pub fn register(&mut self, command: CommandDef) {
        // Register main command name
        self.commands.insert(command.name.clone(), command.clone());
        
        // Register aliases
        for alias in &command.aliases {
            self.commands.insert(alias.clone(), command.clone());
        }
        
        // Add category if new
        if !self.categories.contains(&command.category) {
            self.categories.push(command.category.clone());
        }
    }
    
    pub fn get(&self, name: &str) -> Option<&CommandDef> {
        self.commands.get(name)
    }
    
    pub fn get_commands_by_category(&self, category: &str) -> Vec<&CommandDef> {
        self.commands
            .values()
            .filter(|cmd| cmd.category == category)
            .collect()
    }
}

/// System to process console commands
pub fn command_processor_system(
    mut commands: Commands,
    mut command_reader: EventReader<ConsoleCommand>,
    mut console_state: ResMut<ConsoleState>,
    command_registry: Res<CommandRegistry>,
    mut dev_mode_writer: EventWriter<DevModeChanged>,
    time: Res<Time>,
) {
    for command_event in command_reader.read() {
        let result = if let Some(command_def) = command_registry.get(&command_event.command) {
            // Execute command
            (command_def.function)(&mut commands, &command_event.args, &mut console_state, &mut dev_mode_writer)
        } else {
            CommandResult::Error(format!("Unknown command: '{}'. Type 'help' for available commands.", command_event.command))
        };
        
        // Add result to console output
        let mut line = match result {
            CommandResult::Success(msg) => ConsoleLine::output(msg),
            CommandResult::Error(msg) => ConsoleLine::error(msg),
            CommandResult::Warning(msg) => ConsoleLine::warning(msg),
        };
        line.timestamp = time.elapsed_secs_f64();
        console_state.output_lines.push(line);
    }
}

/// System to register default commands on startup
pub fn register_default_commands(mut command_registry: ResMut<CommandRegistry>) {
    // Help command
    command_registry.register(CommandDef {
        name: "help".to_string(),
        description: "Show available commands or help for specific command".to_string(),
        usage: "help [command|category]".to_string(),
        aliases: vec!["?".to_string()],
        category: "General".to_string(),
        function: cmd_help,
    });
    
    // Clear command
    command_registry.register(CommandDef {
        name: "clear".to_string(),
        description: "Clear console output".to_string(),
        usage: "clear".to_string(),
        aliases: vec!["cls".to_string()],
        category: "General".to_string(),
        function: cmd_clear,
    });
    
    // Register movement commands
    register_movement_commands(&mut command_registry);
    
    // Register debug commands
    register_debug_commands(&mut command_registry);
    
    // Register entity commands
    register_entity_commands(&mut command_registry);
    
    // Register zone commands
    register_zone_commands(&mut command_registry);
}

/// Help command implementation
fn cmd_help(
    _commands: &mut Commands,
    args: &[String],
    _console_state: &mut ConsoleState,
    _dev_mode_writer: &mut EventWriter<DevModeChanged>,
) -> CommandResult {
    if args.is_empty() {
        CommandResult::Success(
            "Available commands:\n\
            help [command] - Show this help or command details\n\
            clear - Clear console\n\
            teleport <x> <y> <z> - Teleport to coordinates\n\
            tp <zone> - Teleport to named zone\n\
            speed <multiplier> - Set movement speed\n\
            debug <system> <on|off> - Toggle debug systems\n\
            godmode - Toggle invincibility\n\
            create npc <type> [x] [y] [z] - Spawn NPC\n\
            zone list - List available zones\n\
            zone load <name> - Load test zone\n\
            \nType 'help <command>' for detailed usage".to_string()
        )
    } else {
        // TODO: Show detailed help for specific command
        CommandResult::Success(format!("Help for '{}' not yet implemented", args[0]))
    }
}

/// Clear command implementation
fn cmd_clear(
    _commands: &mut Commands,
    _args: &[String],
    console_state: &mut ConsoleState,
    _dev_mode_writer: &mut EventWriter<DevModeChanged>,
) -> CommandResult {
    console_state.output_lines.clear();
    console_state.output_lines.push(ConsoleLine::system("Console cleared"));
    CommandResult::Success("".to_string())
}