//! Dialogue system resources and data structures.

use bevy::prelude::*;
use eryndor_core::events::{DialogueApproach, QuestAction};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main dialogue database resource loaded from JSON files
#[derive(Resource, Debug, Clone)]
pub struct DialogueDatabase {
    pub npcs: HashMap<String, NpcDialogue>,
    pub common_phrases: HashMap<String, DialogueNode>,
}

impl Default for DialogueDatabase {
    fn default() -> Self {
        Self {
            npcs: HashMap::new(),
            common_phrases: HashMap::new(),
        }
    }
}

/// Complete dialogue definition for an NPC loaded from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcDialogue {
    pub npc_id: String,
    pub name: String,
    pub description: String,
    pub default_conversation: String,
    pub conversations: HashMap<String, Conversation>,
    pub relationship_effects: RelationshipEffects,
    pub personality_traits: PersonalityTraits,
}

/// A complete conversation tree with multiple nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub title: String,
    pub nodes: HashMap<String, DialogueNode>,
}

/// Individual dialogue node in a conversation tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    pub speaker: DialogueSpeaker,
    pub text: String,
    pub emotion: Option<String>,
    pub choices: Vec<DialogueChoice>,
    #[serde(default)]
    pub clue_flags: Vec<String>,
    #[serde(default)]
    pub quest_progression: Vec<String>,
    pub quest_action: Option<QuestAction>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DialogueSpeaker {
    #[serde(rename = "npc")]
    Npc,
    #[serde(rename = "player")]
    Player,
    #[serde(rename = "narrator")]
    Narrator,
}

/// Player dialogue choice with requirements and consequences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueChoice {
    pub id: String,
    pub text: String,
    pub next: String,
    pub approach: DialogueApproach,
    pub requires: Option<ChoiceRequirements>,
    pub quest_action: Option<QuestAction>,
}

/// Requirements for dialogue choices to be available
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceRequirements {
    pub skills: Option<Vec<SkillRequirement>>,
    pub knowledge: Option<Vec<String>>,
    pub clues: Option<Vec<String>>,
    pub quests: Option<Vec<String>>,
    pub trust_level: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRequirement {
    pub skill: String,
    pub level: u32,
}

/// How different dialogue approaches affect NPC relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipEffects {
    pub trust_building: HashMap<String, i32>,
    pub trust_damaging: HashMap<String, i32>,
}

/// NPC personality traits that affect dialogue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityTraits {
    pub nervous_disposition: Option<f32>,
    pub merchant_instincts: Option<f32>,
    pub guilt_burden: Option<f32>,
    pub desperation_level: Option<f32>,
    pub friendliness: Option<f32>,
    pub suspicion_level: Option<f32>,
    pub helpfulness: Option<f32>,
}

/// Resource to track the current active dialogue
#[derive(Resource, Debug, Clone)]
pub struct ActiveDialogue {
    pub npc_entity: Option<Entity>,
    pub player_entity: Option<Entity>,
    pub current_node: Option<DialogueNode>,
    pub available_choices: Vec<DialogueChoice>,
    pub dialogue_history: Vec<String>,
}

impl Default for ActiveDialogue {
    fn default() -> Self {
        Self {
            npc_entity: None,
            player_entity: None,
            current_node: None,
            available_choices: Vec::new(),
            dialogue_history: Vec::new(),
        }
    }
}