use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use ron::ser::to_string_pretty;

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct GameConfig {
    pub graphics: GraphicsConfig,
    pub input: InputConfig,
    pub debug: DebugConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GraphicsConfig {
    pub vsync: bool,
    pub msaa_samples: u32,
    pub render_scale: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InputConfig {
    pub mouse_sensitivity: f32,
    pub invert_y: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DebugConfig {
    pub show_fps: bool,
    pub show_entity_count: bool,
    pub wireframe_mode: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            graphics: GraphicsConfig {
                vsync: true,
                msaa_samples: 4,
                render_scale: 1.0,
            },
            input: InputConfig {
                mouse_sensitivity: 0.002,
                invert_y: false,
            },
            debug: DebugConfig {
                show_fps: true,
                show_entity_count: false,
                wireframe_mode: false,
            },
        }
    }
}

// Configuration loading system
pub fn load_config() -> GameConfig {
    match std::fs::read_to_string("config.ron") {
        Ok(contents) => {
            ron::from_str(&contents).unwrap_or_else(|_| {
                warn!("Failed to parse config file, using defaults");
                GameConfig::default()
            })
        }
        Err(_) => {
            info!("No config file found, creating default");
            let config = GameConfig::default();
            save_config(&config);
            config
        }
    }
}

// Save configuration
pub fn save_config(config: &GameConfig) {
    if let Ok(serialized) = to_string_pretty(config, ron::ser::PrettyConfig::default()) {
        if let Err(e) = std::fs::write("config.ron", serialized) {
            error!("Failed to save config: {}", e);
        }
    }
}