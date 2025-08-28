//! Dialogue system components for attaching to entities.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Component attached to NPCs to track dialogue state
#[derive(Component, Debug, Clone)]
pub struct DialogueState {
    pub npc_id: String,
    pub current_conversation: Option<String>,
    pub current_node: String,
    pub conversation_history: Vec<String>,
    pub flags_set: Vec<String>,
    pub is_active: bool,
    pub trust_level: i32,
    pub relationship_modifiers: HashMap<String, i32>,
}

impl Default for DialogueState {
    fn default() -> Self {
        Self {
            npc_id: String::new(),
            current_conversation: None,
            current_node: "start".to_string(),
            conversation_history: Vec::new(),
            flags_set: Vec::new(),
            is_active: false,
            trust_level: 0,
            relationship_modifiers: HashMap::new(),
        }
    }
}

/// Component to mark entities that can be interacted with for dialogue
#[derive(Component, Debug, Clone)]
pub struct DialogueInteractable {
    pub npc_id: String,
    pub interaction_range: f32,
    pub has_new_dialogue: bool,
    pub priority_level: u32,
}

impl Default for DialogueInteractable {
    fn default() -> Self {
        Self {
            npc_id: String::new(),
            interaction_range: 3.0,
            has_new_dialogue: false,
            priority_level: 0,
        }
    }
}

/// Component to mark NPCs with their basic information
#[derive(Component, Debug, Clone)]
pub struct NpcInfo {
    pub npc_id: String,
    pub display_name: String,
    pub description: String,
    pub npc_type: NpcType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpcType {
    Merchant,
    Guard,
    Villager,
    Noble,
    Questgiver,
    Informant,
}

/// Component for NPCs that can give quests
#[derive(Component, Debug, Clone)]
pub struct QuestGiver {
    pub available_quests: Vec<String>,
    pub completed_quests: Vec<String>,
    pub quest_requirements: HashMap<String, QuestRequirements>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestRequirements {
    pub level_requirement: Option<u32>,
    pub skill_requirements: Vec<crate::resources::SkillRequirement>,
    pub prerequisite_quests: Vec<String>,
}