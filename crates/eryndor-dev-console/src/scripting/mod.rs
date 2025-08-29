//! Scripting system for command automation.

use bevy::prelude::*;

/// Scripting system resource
#[derive(Resource, Debug, Default)]
pub struct ScriptingSystem {
    pub is_recording: bool,
    pub current_script: Vec<String>,
    pub loaded_scripts: std::collections::HashMap<String, Vec<String>>,
}

impl ScriptingSystem {
    pub fn new() -> Self {
        Self {
            is_recording: false,
            current_script: Vec::new(),
            loaded_scripts: std::collections::HashMap::new(),
        }
    }
    
    pub fn start_recording(&mut self) {
        self.is_recording = true;
        self.current_script.clear();
    }
    
    pub fn stop_recording(&mut self) -> Vec<String> {
        self.is_recording = false;
        std::mem::take(&mut self.current_script)
    }
    
    pub fn record_command(&mut self, command: &str) {
        if self.is_recording {
            self.current_script.push(command.to_string());
        }
    }
    
    pub fn save_script(&mut self, name: String, commands: Vec<String>) {
        self.loaded_scripts.insert(name, commands);
    }
    
    pub fn get_script(&self, name: &str) -> Option<&Vec<String>> {
        self.loaded_scripts.get(name)
    }
}