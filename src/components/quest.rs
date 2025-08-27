use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core quest component attached to players tracking their quest progress
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct QuestLog {
    /// Currently active quests
    pub active_quests: HashMap<String, QuestProgress>,
    /// Completed quests for reference and continuation checking
    pub completed_quests: HashMap<String, CompletedQuest>,
    /// Player's investigation notes and observations
    pub player_notes: HashMap<String, String>,
    /// Discovered clues across all quests
    pub discovered_clues: HashMap<String, DiscoveredClue>,
    /// NPC relationship tracking for dialogue purposes
    pub npc_relationships: HashMap<String, NpcRelationship>,
}

impl Default for QuestLog {
    fn default() -> Self {
        Self {
            active_quests: HashMap::new(),
            completed_quests: HashMap::new(),
            player_notes: HashMap::new(),
            discovered_clues: HashMap::new(),
            npc_relationships: HashMap::new(),
        }
    }
}

/// Tracks progress through a specific quest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestProgress {
    pub quest_id: String,
    pub current_phase: String,
    pub completed_phases: Vec<String>,
    pub discovered_clues: Vec<String>,
    pub completed_objectives: Vec<String>,
    pub failed_conditions: Vec<String>,
    pub evidence_strength: EvidenceStrength,
    pub investigation_notes: Vec<String>,
    pub start_time: f64, // Game time when quest was started
}

/// Represents the strength of evidence gathered during investigation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceStrength {
    None,
    Weak,
    Moderate,  
    Strong,
    Overwhelming,
}

impl EvidenceStrength {
    /// Calculate evidence strength from number of clues and their quality
    pub fn calculate_from_clues(clues: &[DiscoveredClue]) -> Self {
        let total_weight: i32 = clues.iter().map(|c| c.importance_weight).sum();
        match total_weight {
            0 => EvidenceStrength::None,
            1..=3 => EvidenceStrength::Weak,
            4..=7 => EvidenceStrength::Moderate,
            8..=12 => EvidenceStrength::Strong,
            _ => EvidenceStrength::Overwhelming,
        }
    }
}

/// Information about a completed quest for reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedQuest {
    pub quest_id: String,
    pub completion_time: f64,
    pub resolution_path: String,
    pub final_evidence_strength: EvidenceStrength,
    pub consequences_triggered: Vec<String>,
    pub experience_gained: u64,
    pub items_received: Vec<String>,
}

/// Represents a clue discovered during investigation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredClue {
    pub clue_id: String,
    pub quest_id: String,
    pub discovery_time: f64,
    pub discovery_method: String,
    pub description: String,
    pub importance_weight: i32, // Higher values indicate more crucial clues
    pub related_clues: Vec<String>, // Other clues this connects to
}

/// Tracks relationships with NPCs for dialogue and quest purposes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcRelationship {
    pub npc_id: String,
    pub trust_level: TrustLevel,
    pub information_shared: Vec<String>,
    pub favors_owed: i32,
    pub reputation_modifiers: Vec<String>,
    pub conversation_history: Vec<ConversationRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustLevel {
    Hostile,
    Suspicious,
    Neutral,
    Trusting,
    Confidential,
}

/// Records important conversations for future reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationRecord {
    pub timestamp: f64,
    pub topic: String,
    pub information_revealed: Vec<String>,
    pub player_approach: ConversationApproach,
    pub npc_response: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConversationApproach {
    Direct,
    Subtle,
    Aggressive,
    Friendly,
    Professional,
    Deceptive,
}

/// Component for NPCs that can be investigated or questioned
#[derive(Component, Debug, Clone)]
pub struct QuestNpc {
    pub npc_id: String,
    pub personality_type: NpcPersonality,
    pub knowledge_base: Vec<String>, // Topics this NPC knows about
    pub secrets: Vec<NpcSecret>,     // Information requiring trust/skill to obtain
    pub conversation_state: ConversationState,
    pub daily_schedule: Option<NpcSchedule>, // When and where to find them
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NpcPersonality {
    Honest,      // Direct answers, trustworthy information
    Evasive,     // Avoids direct questions, requires patience
    Gossipy,     // Shares information freely but reliability varies
    Paranoid,    // Suspicious, needs trust building
    Scholarly,   // Verbose, buries key info in lengthy explanations
    Secretive,   // Has valuable info but requires specific approaches
}

#[derive(Debug, Clone)]
pub struct NpcSecret {
    pub topic: String,
    pub required_trust: TrustLevel,
    pub revelation_conditions: Vec<String>,
    pub information_value: i32,
}

#[derive(Debug, Clone, Default)]
pub struct ConversationState {
    pub topics_discussed: Vec<String>,
    pub current_trust: TrustLevel,
    pub information_revealed: Vec<String>,
    pub last_conversation_time: f64,
    pub conversation_count: u32,
}

impl Default for TrustLevel {
    fn default() -> Self {
        TrustLevel::Neutral
    }
}

/// NPC daily schedule for realistic world interactions
#[derive(Debug, Clone)]
pub struct NpcSchedule {
    pub morning_location: String,   // 6-12
    pub afternoon_location: String, // 12-18
    pub evening_location: String,   // 18-24
    pub night_location: String,     // 0-6
    pub special_conditions: Vec<ScheduleCondition>,
}

#[derive(Debug, Clone)]
pub struct ScheduleCondition {
    pub condition: String,
    pub affected_times: Vec<String>,
    pub alternative_location: String,
}

/// Component for interactive objects that may contain clues or lore
#[derive(Component, Debug, Clone)]
pub struct InvestigationObject {
    pub object_id: String,
    pub examination_text: String,
    pub requires_skill: Option<String>, // Some objects need specific skills to understand
    pub hidden_information: Vec<HiddenInformation>,
    pub clues_provided: Vec<String>,
    pub examination_count: u32, // Track how many times player has examined this
}

#[derive(Debug, Clone)]
pub struct HiddenInformation {
    pub information: String,
    pub reveal_condition: RevealCondition,
    pub skill_requirement: Option<(String, u32)>, // (skill_name, minimum_level)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RevealCondition {
    FirstExamination,
    RepeatedExamination(u32), // Reveals after N examinations
    HasClue(String),          // Reveals only if player has specific clue
    TimeOfDay(u32, u32),      // Reveals only during certain hours
    HasEvidence(EvidenceStrength), // Reveals based on overall evidence strength
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

/// Resource containing all quest definitions loaded from JSON
#[derive(Resource, Debug, Clone)]
pub struct QuestDatabase {
    pub quests: HashMap<String, QuestDefinition>,
    pub npcs: HashMap<String, NpcDefinition>,
    pub locations: HashMap<String, LocationDefinition>,
}

/// Quest definition structure matching JSON schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestDefinition {
    pub id: String,
    pub title: String,
    pub category: String,
    pub difficulty: String,
    pub estimated_duration: String,
    pub description: String,
    pub lore_context: LoreContext,
    pub phases: Vec<QuestPhase>,
    pub rewards: QuestRewards,
    pub narrative_themes: NarrativeThemes,
    pub replayability: ReplayabilityInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoreContext {
    pub historical_background: String,
    pub world_impact: String,
    pub character_significance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestPhase {
    pub phase_id: String,
    pub title: String,
    pub description: String,
    pub prerequisites: Option<PhasePrerequisites>,
    pub objectives: Vec<QuestObjective>,
    pub available_actions: Vec<String>,
    pub clues_to_discover: Vec<ClueDefinition>,
    pub failure_conditions: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhasePrerequisites {
    pub completed_phases: Vec<String>,
    pub required_clues: Vec<String>,
    pub minimum_evidence: Option<u32>,
    pub minimum_testimonies: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestObjective {
    #[serde(rename = "type")]
    pub objective_type: String,
    pub description: String,
    pub completion_criteria: serde_json::Value, // Flexible structure for different criteria types
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClueDefinition {
    pub clue_id: String,
    pub description: String,
    pub discovery_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestRewards {
    pub base_experience: u64,
    pub skill_bonuses: Vec<SkillBonus>,
    pub unique_items: Vec<UniqueItemReward>,
    pub unlocked_content: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillBonus {
    pub skill: String,
    pub experience: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueItemReward {
    pub item_id: String,
    pub name: String,
    pub description: String,
    pub mechanical_benefit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeThemes {
    pub primary: String,
    pub secondary: String,
    pub moral_complexity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayabilityInfo {
    pub multiple_solution_paths: bool,
    pub hidden_details: String,
    pub player_choice_impact: String,
}

/// NPC definition for quest system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcDefinition {
    pub id: String,
    pub name: String,
    pub profession: String,
    pub personality: String,
    pub knowledge_topics: Vec<String>,
    pub conversation_hints: Vec<String>,
    pub location_schedule: HashMap<String, String>, // time_period -> location
}

/// Location definition for quest context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub accessible_times: Vec<String>,
    pub investigation_objects: Vec<String>,
    pub atmospheric_details: Vec<String>,
}