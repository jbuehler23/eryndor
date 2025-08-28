//! Core event definitions for inter-system communication.
//!
//! These events enable decoupled communication between different Eryndor systems
//! while maintaining type safety and clear data flow.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// Re-export core types that events depend on
pub use crate::components::*;

/// Trust level between player and NPCs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustLevel {
    Hostile,
    Suspicious, 
    Neutral,
    Trusting,
    Confidential,
}

impl Default for TrustLevel {
    fn default() -> Self {
        TrustLevel::Neutral
    }
}

/// Evidence strength for quest investigations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceStrength {
    None,
    Weak,
    Moderate,
    Strong,
    Overwhelming,
}

/// Represents a clue discovered during investigation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredClue {
    pub clue_id: String,
    pub quest_id: String,
    pub discovery_time: f64,
    pub discovery_method: String,
    pub description: String,
    pub importance_weight: i32,
    pub related_clues: Vec<String>,
}

/// Dialogue approach styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DialogueApproach {
    #[serde(rename = "casual")]
    Casual,
    #[serde(rename = "observant")]
    Observant,
    #[serde(rename = "inquisitive")]
    Inquisitive,
    #[serde(rename = "diplomatic")]
    Diplomatic,
    #[serde(rename = "direct")]
    Direct,
}

/// Quest actions that can be triggered through dialogue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestAction {
    #[serde(rename = "type")]
    pub action_type: String,
    pub quest_id: Option<String>,
    pub phase: Option<String>,
    pub clues: Option<Vec<String>>,
    pub trust_change: Option<i32>,
}

/// Events for quest system communication
#[derive(Event, Debug, Clone)]
pub enum QuestEvent {
    /// Player discovered a new clue
    ClueDiscovered {
        player_entity: Entity,
        clue: DiscoveredClue,
    },
    /// Quest phase completed
    PhaseCompleted {
        player_entity: Entity,
        quest_id: String,
        phase_id: String,
    },
    /// Quest failed due to player actions
    QuestFailed {
        player_entity: Entity,
        quest_id: String,
        failure_reason: String,
    },
    /// Quest completed with specific resolution
    QuestCompleted {
        player_entity: Entity,
        quest_id: String,
        resolution_path: String,
        consequences: Vec<String>,
    },
    /// NPC relationship changed
    RelationshipChanged {
        player_entity: Entity,
        npc_id: String,
        old_trust: TrustLevel,
        new_trust: TrustLevel,
        reason: String,
    },
    /// Player made investigation notes
    NotesUpdated {
        player_entity: Entity,
        quest_id: String,
        note_content: String,
    },
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

/// Events for character progression
#[derive(Event, Debug, Clone)]
pub enum ProgressionEvent {
    /// Experience gained in a skill
    ExperienceGained {
        player_entity: Entity,
        skill_id: String,
        amount: u64,
        source: String,
    },
    /// Level up occurred
    LevelUp {
        player_entity: Entity,
        skill_id: String,
        old_level: u32,
        new_level: u32,
    },
    /// Item equipped or unequipped
    EquipmentChanged {
        player_entity: Entity,
        slot: String,
        item_id: Option<String>,
    },
}