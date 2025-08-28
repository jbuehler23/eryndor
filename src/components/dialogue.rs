use bevy::prelude::*;
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DialogueApproach {
    #[serde(rename = "casual")]
    Casual,
    #[serde(rename = "observant")]
    Observant,
    #[serde(rename = "inquisitive")]
    Inquisitive,
    #[serde(rename = "investigative")]
    Investigative,
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "supportive")]
    Supportive,
    #[serde(rename = "helpful")]
    Helpful,
    #[serde(rename = "defensive")]
    Defensive,
    #[serde(rename = "analytical")]
    Analytical,
    #[serde(rename = "curious")]
    Curious,
    #[serde(rename = "patient")]
    Patient,
    #[serde(rename = "assertive")]
    Assertive,
    #[serde(rename = "diplomatic")]
    Diplomatic,
    #[serde(rename = "heroic")]
    Heroic,
    #[serde(rename = "cautious")]
    Cautious,
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

/// Quest-related actions that can be triggered by dialogue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestAction {
    #[serde(rename = "type")]
    pub action_type: String,
    pub quest_id: Option<String>,
    pub phase: Option<String>,
    pub clues: Option<Vec<String>>,
    pub items: Option<Vec<String>>,
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
    pub priority_level: u32, // For quest-important NPCs
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

/// Events for dialogue system communication
#[derive(Event, Debug, Clone)]
pub enum DialogueEvent {
    StartConversation {
        npc_entity: Entity,
        player_entity: Entity,
        conversation_id: Option<String>,
    },
    EndConversation {
        npc_entity: Entity,
        player_entity: Entity,
    },
    ChoiceSelected {
        npc_entity: Entity,
        choice_id: String,
        next_node: String,
    },
    QuestActionTriggered {
        action: QuestAction,
        npc_entity: Entity,
    },
    RelationshipChanged {
        npc_entity: Entity,
        approach: DialogueApproach,
        trust_delta: i32,
    },
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
    pub skill_requirements: Vec<SkillRequirement>,
    pub prerequisite_quests: Vec<String>,
}