//! # Eryndor Dev Console
//!
//! Developer console and GM command system for the Eryndor game engine.
//! Provides WoW-style game master commands, test zones, and scripting capabilities.
//!
//! ## Features
//! - Console UI with backtick (`) toggle
//! - GM commands: teleport, create npc, debug controls
//! - Test zone system with JSON definitions
//! - Command scripting and history
//! - Integration with existing Eryndor systems

use bevy::prelude::*;
use eryndor_core::traits::EryndorPlugin;

pub mod console;
pub mod commands;
pub mod zones;
pub mod scripting;

// Re-export commonly used types
pub use commands::*;
pub use zones::*;

/// Plugin that provides dev console functionality
pub struct EryndorDevConsolePlugin;

impl Plugin for EryndorDevConsolePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<ConsoleState>()
            .init_resource::<CommandRegistry>()
            .init_resource::<ConsoleHistory>()
            .init_resource::<ZoneManager>()
            
            // Events
            .add_event::<ConsoleCommand>()
            .add_event::<ZoneTransition>()
            .add_event::<DevModeChanged>()
            
            // Systems
            .add_systems(Update, (
                console_toggle_system,
                console_input_system,
                command_processor_system,
                console_ui_system,
                commands::process_teleport_commands,
                commands::process_speed_commands,
                commands::process_godmode_commands,
                commands::process_spawn_npc_commands,
                commands::process_delete_entity_commands,
            ))
            
            // Command registration
            .add_systems(Startup, register_default_commands);
    }
}

impl EryndorPlugin for EryndorDevConsolePlugin {
    fn build(&self, app: &mut App) {
        <Self as Plugin>::build(self, app);
    }
    
    fn name(&self) -> &'static str {
        "EryndorDevConsolePlugin"
    }
    
    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

/// Main console state resource
#[derive(Resource, Debug)]
pub struct ConsoleState {
    pub is_visible: bool,
    pub current_input: String,
    pub cursor_position: usize,
    pub output_lines: Vec<ConsoleLine>,
}

impl Default for ConsoleState {
    fn default() -> Self {
        Self {
            is_visible: false,
            current_input: String::new(),
            cursor_position: 0,
            output_lines: vec![
                ConsoleLine::system("Eryndor Dev Console initialized. Type 'help' for commands."),
            ],
        }
    }
}

/// Console output line with styling
#[derive(Debug, Clone)]
pub struct ConsoleLine {
    pub text: String,
    pub line_type: ConsoleLineType,
    pub timestamp: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConsoleLineType {
    Command,
    Output,
    Error,
    Warning,
    System,
}

impl ConsoleLine {
    pub fn command(text: impl Into<String>) -> Self {
        Self {
            text: format!("> {}", text.into()),
            line_type: ConsoleLineType::Command,
            timestamp: 0.0, // Will be set by system
        }
    }
    
    pub fn output(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            line_type: ConsoleLineType::Output,
            timestamp: 0.0,
        }
    }
    
    pub fn error(text: impl Into<String>) -> Self {
        Self {
            text: format!("ERROR: {}", text.into()),
            line_type: ConsoleLineType::Error,
            timestamp: 0.0,
        }
    }
    
    pub fn warning(text: impl Into<String>) -> Self {
        Self {
            text: format!("WARNING: {}", text.into()),
            line_type: ConsoleLineType::Warning,
            timestamp: 0.0,
        }
    }
    
    pub fn system(text: impl Into<String>) -> Self {
        Self {
            text: format!("[SYSTEM] {}", text.into()),
            line_type: ConsoleLineType::System,
            timestamp: 0.0,
        }
    }
}

/// Command history resource
#[derive(Resource, Debug, Default)]
pub struct ConsoleHistory {
    pub commands: Vec<String>,
    pub current_index: Option<usize>,
    pub max_history: usize,
}

impl ConsoleHistory {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            current_index: None,
            max_history: 100,
        }
    }
    
    pub fn add_command(&mut self, command: String) {
        if !command.trim().is_empty() {
            self.commands.push(command);
            if self.commands.len() > self.max_history {
                self.commands.remove(0);
            }
            self.current_index = None;
        }
    }
    
    pub fn get_previous(&mut self) -> Option<&String> {
        if self.commands.is_empty() {
            return None;
        }
        
        match self.current_index {
            None => {
                self.current_index = Some(self.commands.len() - 1);
                Some(&self.commands[self.commands.len() - 1])
            }
            Some(index) if index > 0 => {
                self.current_index = Some(index - 1);
                Some(&self.commands[index - 1])
            }
            Some(_) => Some(&self.commands[0]),
        }
    }
    
    pub fn get_next(&mut self) -> Option<&String> {
        match self.current_index {
            Some(index) if index < self.commands.len() - 1 => {
                self.current_index = Some(index + 1);
                Some(&self.commands[index + 1])
            }
            Some(_) => {
                self.current_index = None;
                None
            }
            None => None,
        }
    }
}

/// Console command event
#[derive(Event, Debug, Clone)]
pub struct ConsoleCommand {
    pub command: String,
    pub args: Vec<String>,
}

/// Zone transition event
#[derive(Event, Debug, Clone)]
pub struct ZoneTransition {
    pub from_zone: Option<String>,
    pub to_zone: String,
}

/// Dev mode changed event
#[derive(Event, Debug, Clone)]
pub struct DevModeChanged {
    pub debug_system: String,
    pub enabled: bool,
}

// Re-export system functions
pub use console::{console_toggle_system, console_input_system, console_ui_system, DevConsoleRoot};
pub use commands::{command_processor_system, register_default_commands};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{
        EryndorDevConsolePlugin,
        ConsoleState,
        ConsoleLine,
        ConsoleLineType,
        ConsoleHistory,
        ConsoleCommand,
        ZoneTransition,
        DevModeChanged,
    };
}