//! Test zone system for developer testing.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Zone manager resource
#[derive(Resource, Debug, Default)]
pub struct ZoneManager {
    pub current_zone: Option<String>,
    pub available_zones: Vec<String>,
}

/// Zone definition structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneDefinition {
    pub name: String,
    pub description: String,
    pub spawn_point: Vec3,
    pub terrain_type: String,
    pub npcs: Vec<ZoneNpc>,
    pub environment: ZoneEnvironment,
}

/// NPC definition within a zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneNpc {
    pub npc_type: String,
    pub position: Vec3,
    pub dialogue_id: Option<String>,
    pub name: Option<String>,
}

/// Environment settings for a zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneEnvironment {
    pub lighting: String,
    pub weather: String,
}

impl ZoneManager {
    pub fn new() -> Self {
        Self {
            current_zone: None,
            available_zones: vec![
                "test-zone".to_string(),
                "npc-showcase".to_string(),
                "dialogue-test".to_string(),
            ],
        }
    }
    
    pub fn load_zone(&mut self, zone_name: &str) -> Result<ZoneDefinition, String> {
        // TODO: Load zone from JSON file
        match zone_name {
            "test-zone" => Ok(ZoneDefinition {
                name: "test-zone".to_string(),
                description: "Basic developer testing area".to_string(),
                spawn_point: Vec3::new(0.0, 10.0, 0.0),
                terrain_type: "flat".to_string(),
                npcs: vec![
                    ZoneNpc {
                        npc_type: "merchant".to_string(),
                        position: Vec3::new(10.0, 10.0, 10.0),
                        dialogue_id: Some("merchant_aldric".to_string()),
                        name: Some("Test Merchant".to_string()),
                    },
                ],
                environment: ZoneEnvironment {
                    lighting: "noon".to_string(),
                    weather: "clear".to_string(),
                },
            }),
            _ => Err(format!("Unknown zone: {}", zone_name)),
        }
    }
}