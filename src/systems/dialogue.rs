use bevy::prelude::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::components::{Player, QuestLog, QuestNpc};
use crate::components::quest::*;

/// Main dialogue system component for managing conversations
#[derive(Component, Debug, Clone)]
pub struct DialogueState {
    pub current_npc: Option<Entity>,
    pub current_conversation: Option<ConversationTree>,
    pub current_node: String,
    pub conversation_history: Vec<DialogueChoice>,
    pub is_active: bool,
}

impl Default for DialogueState {
    fn default() -> Self {
        Self {
            current_npc: None,
            current_conversation: None,
            current_node: "start".to_string(),
            conversation_history: Vec::new(),
            is_active: false,
        }
    }
}

/// Represents a complete conversation tree with an NPC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTree {
    pub npc_id: String,
    pub conversation_id: String,
    pub context: ConversationContext,
    pub nodes: HashMap<String, DialogueNode>,
    pub personality_modifiers: PersonalityModifiers,
}

/// Context for the conversation (quest-related, casual, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    pub conversation_type: ConversationType,
    pub required_quests: Vec<String>,
    pub required_clues: Vec<String>,
    pub time_restrictions: Option<TimeRestrictions>,
    pub relationship_requirements: Option<TrustLevel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConversationType {
    QuestInitiation,
    QuestInvestigation,
    Information,
    Trading,
    Casual,
    Lore,
}

/// Time-based restrictions for conversation availability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestrictions {
    pub available_hours: Vec<u32>, // 0-23
    pub required_location: Option<String>,
    pub seasonal_restrictions: Vec<String>,
}

/// Personality modifiers affecting how NPCs speak and respond
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityModifiers {
    pub verbose_factor: f32,     // How much extra text they add
    pub trust_building_speed: f32, // How quickly they warm up to player
    pub information_reluctance: f32, // How hesitant they are to share secrets
    pub speech_patterns: Vec<String>, // Characteristic phrases and mannerisms
    pub emotional_state: EmotionalState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmotionalState {
    Calm,
    Anxious,
    Excited,
    Suspicious,
    Melancholy,
    Cheerful,
    Angry,
}

/// Individual dialogue node in a conversation tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    pub node_id: String,
    pub speaker: DialogueSpeaker,
    pub text_variations: Vec<TextVariation>,
    pub choices: Vec<DialogueChoice>,
    pub conditions: Vec<DialogueCondition>,
    pub consequences: Vec<DialogueConsequence>,
    pub lore_references: Vec<String>,
    pub clue_revelations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DialogueSpeaker {
    Npc,
    Player,
    Narrator,
}

/// Different versions of text based on context/relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextVariation {
    pub text: String,
    pub conditions: Vec<String>,
    pub personality_weight: f32, // How much this fits the NPC's personality
    pub formality_level: FormalityLevel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormalityLevel {
    Intimate,    // Close friends, family
    Casual,      // Friendly acquaintances
    Neutral,     // Standard interactions
    Formal,      // Professional, respectful
    Hostile,     // Antagonistic, cold
}

/// Player dialogue choices with complex consequences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueChoice {
    pub choice_id: String,
    pub text: String,
    pub approach: ConversationApproach,
    pub next_node: String,
    pub requirements: Vec<ChoiceRequirement>,
    pub skill_checks: Vec<SkillCheck>,
    pub relationship_impact: i32,
    pub information_gained: Vec<String>,
    pub clues_unlocked: Vec<String>,
    pub quest_progression: Vec<String>,
}

/// Requirements for dialogue choices to be available
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceRequirement {
    pub requirement_type: RequirementType,
    pub value: String,
    pub minimum_level: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequirementType {
    HasClue,
    QuestProgress,
    SkillLevel,
    RelationshipLevel,
    ItemInInventory,
    TimeOfDay,
    PreviousChoice,
}

/// Skill checks for advanced dialogue options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillCheck {
    pub skill: String,
    pub difficulty: u32,
    pub success_node: String,
    pub failure_node: String,
    pub partial_success_threshold: Option<u32>,
}

/// Conditions that must be met for a dialogue node to appear
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueCondition {
    pub condition_type: ConditionType,
    pub parameters: HashMap<String, String>,
    pub invert: bool, // If true, condition must NOT be met
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionType {
    QuestActive,
    QuestCompleted,
    HasClue,
    RelationshipLevel,
    TimeOfDay,
    FirstMeeting,
    InventoryContains,
    EvidenceStrength,
}

/// Consequences of dialogue choices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueConsequence {
    pub consequence_type: ConsequenceType,
    pub target: String,
    pub value: i32,
    pub description: String,
    pub permanent: bool, // If true, cannot be undone
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsequenceType {
    RelationshipChange,
    QuestProgression,
    ClueRevelation,
    InformationGained,
    ReputationChange,
    ItemGained,
    ItemLost,
    LocationRevealed,
}

/// Resource containing all dialogue definitions
#[derive(Resource, Debug, Clone, Default)]
pub struct DialogueDatabase {
    pub conversations: HashMap<String, ConversationTree>,
    pub npc_default_conversations: HashMap<String, Vec<String>>,
    pub lore_database: HashMap<String, LoreEntry>,
}

/// Lore entries that can be referenced in dialogue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoreEntry {
    pub id: String,
    pub title: String,
    pub content: String,
    pub related_topics: Vec<String>,
    pub accessibility_level: LoreAccessibility,
    pub historical_period: Option<String>,
    pub cultural_significance: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoreAccessibility {
    CommonKnowledge,  // Everyone knows this
    Regional,         // Known in specific areas
    Professional,     // Known by specific professions
    Secret,           // Hidden knowledge
    Forbidden,        // Dangerous or taboo knowledge
}

/// Events for dialogue system
#[derive(Event, Debug, Clone)]
pub enum DialogueEvent {
    ConversationStarted {
        player_entity: Entity,
        npc_entity: Entity,
        conversation_id: String,
    },
    ConversationEnded {
        player_entity: Entity,
        npc_entity: Entity,
        reason: ConversationEndReason,
    },
    ChoiceMade {
        player_entity: Entity,
        npc_entity: Entity,
        choice: DialogueChoice,
    },
    InformationRevealed {
        player_entity: Entity,
        npc_entity: Entity,
        information: String,
        importance: InformationImportance,
    },
    RelationshipChanged {
        player_entity: Entity,
        npc_entity: Entity,
        old_level: TrustLevel,
        new_level: TrustLevel,
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConversationEndReason {
    PlayerChoice,
    NpcRefusal,
    QuestProgression,
    TimeExpired,
    PlayerLeft,
    HostileAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InformationImportance {
    Trivial,
    Interesting,
    Important,
    Crucial,
    WorldChanging,
}

/// Legacy Note: This file contains obsolete hard-coded dialogue functions.
/// All dialogue content is now dynamically loaded from JSON files in config/dialogues/
/// 
/// TODO: This entire system should be cleaned up since we use:
/// - dialogue_loader.rs for JSON loading
/// - dialogue_interaction.rs for player interaction  
/// - components/dialogue.rs for data structures
///
/// This file is kept temporarily for any remaining component definitions
/// but the conversation creation functions are deprecated.

// === DEPRECATED FUNCTIONS BELOW - REMOVE IN CLEANUP ===
    let mut nodes = HashMap::new();
    
    // Start node - initial greeting
    nodes.insert("start".to_string(), DialogueNode {
        node_id: "start".to_string(),
        speaker: DialogueSpeaker::Npc,
        text_variations: vec![
            TextVariation {
                text: "Ah, welcome to my humble establishment! I am Aldric Goldweaver, purveyor of fine goods from across the realm. How may I serve you today?".to_string(),
                conditions: vec!["first_meeting".to_string()],
                personality_weight: 1.0,
                formality_level: FormalityLevel::Formal,
            },
            TextVariation {
                text: "Good to see you again, friend! Business has been... interesting lately. What brings you by?".to_string(),
                conditions: vec!["return_customer".to_string()],
                personality_weight: 1.0,
                formality_level: FormalityLevel::Casual,
            }
        ],
        choices: vec![
            DialogueChoice {
                choice_id: "ask_about_goods".to_string(),
                text: "I notice you have some interesting northern goods. Tell me about your recent travels.".to_string(),
                approach: ConversationApproach::Professional,
                next_node: "northern_travel_response".to_string(),
                requirements: vec![],
                skill_checks: vec![],
                relationship_impact: 0,
                information_gained: vec!["northern_trade_interest".to_string()],
                clues_unlocked: vec![],
                quest_progression: vec![],
            },
            DialogueChoice {
                choice_id: "casual_greeting".to_string(),
                text: "Just browsing, thanks. How has business been?".to_string(),
                approach: ConversationApproach::Friendly,
                next_node: "business_talk".to_string(),
                requirements: vec![],
                skill_checks: vec![],
                relationship_impact: 1,
                information_gained: vec![],
                clues_unlocked: vec![],
                quest_progression: vec![],
            },
            DialogueChoice {
                choice_id: "direct_investigation".to_string(),
                text: "People in town say you've been acting strangely since your return. Is everything alright?".to_string(),
                approach: ConversationApproach::Direct,
                next_node: "defensive_response".to_string(),
                requirements: vec![
                    ChoiceRequirement {
                        requirement_type: RequirementType::QuestProgress,
                        value: "the_merchants_mystery".to_string(),
                        minimum_level: Some(1),
                    }
                ],
                skill_checks: vec![],
                relationship_impact: -1,
                information_gained: vec!["aldric_nervousness".to_string()],
                clues_unlocked: vec![],
                quest_progression: vec![],
            }
        ],
        conditions: vec![],
        consequences: vec![],
        lore_references: vec!["merchant_guild_history".to_string()],
        clue_revelations: vec![],
    });
    
    // Northern travel response
    nodes.insert("northern_travel_response".to_string(), DialogueNode {
        node_id: "northern_travel_response".to_string(),
        speaker: DialogueSpeaker::Npc,
        text_variations: vec![
            TextVariation {
                text: "Ah yes, the northern routes! Quite profitable this time of year, though... *glances around nervously* ...the journey was shorter than usual. Only took me 8 days there and back, can you believe it? Found some excellent trading opportunities along the way.".to_string(),
                conditions: vec![],
                personality_weight: 1.0,
                formality_level: FormalityLevel::Casual,
            }
        ],
        choices: vec![
            DialogueChoice {
                choice_id: "question_timing".to_string(),
                text: "Eight days? That seems remarkably fast for a northern trading expedition. Most merchants take weeks.".to_string(),
                approach: ConversationApproach::Subtle,
                next_node: "timing_explanation".to_string(),
                requirements: vec![],
                skill_checks: vec![
                    SkillCheck {
                        skill: "investigation".to_string(),
                        difficulty: 15,
                        success_node: "timing_contradiction_caught".to_string(),
                        failure_node: "timing_explanation".to_string(),
                        partial_success_threshold: Some(10),
                    }
                ],
                relationship_impact: 0,
                information_gained: vec!["travel_time_focus".to_string()],
                clues_unlocked: vec!["travel_time_inconsistency".to_string()],
                quest_progression: vec!["investigation_deepens".to_string()],
            },
            DialogueChoice {
                choice_id: "compliment_efficiency".to_string(),
                text: "That's impressive efficiency! You must have your routes well planned.".to_string(),
                approach: ConversationApproach::Friendly,
                next_node: "pleased_response".to_string(),
                requirements: vec![],
                skill_checks: vec![],
                relationship_impact: 2,
                information_gained: vec!["aldric_pride_in_efficiency".to_string()],
                clues_unlocked: vec![],
                quest_progression: vec![],
            }
        ],
        conditions: vec![],
        consequences: vec![],
        lore_references: vec!["northern_trade_routes".to_string()],
        clue_revelations: vec!["travel_time_inconsistency".to_string()],
    });
    
    ConversationTree {
        npc_id: "aldric_goldweaver".to_string(),
        conversation_id: "aldric_mystery_talk".to_string(),
        context: ConversationContext {
            conversation_type: ConversationType::QuestInvestigation,
            required_quests: vec!["the_merchants_mystery".to_string()],
            required_clues: vec![],
            time_restrictions: None,
            relationship_requirements: None,
        },
        nodes,
        personality_modifiers: PersonalityModifiers {
            verbose_factor: 1.2,
            trust_building_speed: 0.8,
            information_reluctance: 0.6,
            speech_patterns: vec![
                "can you believe it?".to_string(),
                "*glances around nervously*".to_string(),
                "quite profitable".to_string(),
            ],
            emotional_state: EmotionalState::Anxious,
        },
    }
}

fn create_sample_lore_database() -> HashMap<String, LoreEntry> {
    let mut lore = HashMap::new();
    
    lore.insert("northern_trade_routes".to_string(), LoreEntry {
        id: "northern_trade_routes".to_string(),
        title: "The Northern Trade Routes".to_string(),
        content: "The northern territories have been connected to our region through treacherous mountain passes for centuries. Traditional trading expeditions require careful planning, as the journey typically takes 14-21 days depending on weather conditions and the specific settlements visited. The routes pass through three major waypoints: Ironhold Crossing, the Whispering Pines, and the Northern Gate settlement.".to_string(),
        related_topics: vec!["merchant_guild_history".to_string(), "mountain_passes".to_string()],
        accessibility_level: LoreAccessibility::CommonKnowledge,
        historical_period: Some("Current Era".to_string()),
        cultural_significance: Some("Economic lifeline for many communities".to_string()),
    });
    
    lore.insert("merchant_guild_history".to_string(), LoreEntry {
        id: "merchant_guild_history".to_string(),
        title: "The Merchant's Guild Legacy".to_string(),
        content: "Founded three generations ago by the renowned trader Matthias Goldweaver, the local merchant's guild has maintained strict standards for ethical trading practices. The guild's charter explicitly prohibits the transport of cursed items, stolen goods, or materials obtained through coercion. Recent years have seen increased pressure on merchants to maintain competitive pricing while upholding these traditional values.".to_string(),
        related_topics: vec!["goldweaver_family".to_string(), "cursed_bargains".to_string()],
        accessibility_level: LoreAccessibility::Regional,
        historical_period: Some("Past 75 years".to_string()),
        cultural_significance: Some("Foundation of regional economic stability".to_string()),
    });
    
    lore.insert("cursed_bargains".to_string(), LoreEntry {
        id: "cursed_bargains".to_string(),
        title: "Tales of Cursed Bargains".to_string(),
        content: "Elder stories speak of mysterious figures who appear to merchants in dire straits, offering 'beneficial exchanges' that seem too good to be true. These beings, described as wearing deep hoods that obscure their features, provide valuable goods in exchange for the merchant's agreement to transport certain items - items that grow warm to the touch and seem to whisper in the darkness. Those who accept such bargains often find temporary prosperity followed by inexplicable misfortune.".to_string(),
        related_topics: vec!["hooded_figures".to_string(), "warm_objects".to_string()],
        accessibility_level: LoreAccessibility::Secret,
        historical_period: Some("Ancient warnings, recent occurrences".to_string()),
        cultural_significance: Some("Cautionary tales about unethical shortcuts".to_string()),
    });
    
    lore
}

/// System to handle dialogue interactions
pub fn dialogue_interaction_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut DialogueState, With<Player>>,
    dialogue_db: Res<DialogueDatabase>,
    mut dialogue_events: EventWriter<DialogueEvent>,
) {
    if let Ok(mut dialogue_state) = player_query.single_mut() {
        // Start conversation with Aldric (F5 key for demo)
        if keyboard.just_pressed(KeyCode::F5) && !dialogue_state.is_active {
            start_conversation(&mut dialogue_state, "aldric_mystery_talk", &dialogue_db, &mut dialogue_events);
        }
        
        // Handle conversation choices (1-3 keys)
        if dialogue_state.is_active {
            if keyboard.just_pressed(KeyCode::Digit1) {
                handle_dialogue_choice(&mut dialogue_state, 0, &dialogue_db, &mut dialogue_events);
            } else if keyboard.just_pressed(KeyCode::Digit2) {
                handle_dialogue_choice(&mut dialogue_state, 1, &dialogue_db, &mut dialogue_events);
            } else if keyboard.just_pressed(KeyCode::Digit3) {
                handle_dialogue_choice(&mut dialogue_state, 2, &dialogue_db, &mut dialogue_events);
            }
            
            // Exit conversation (ESC)
            if keyboard.just_pressed(KeyCode::Escape) {
                end_conversation(&mut dialogue_state, ConversationEndReason::PlayerChoice, &mut dialogue_events);
            }
        }
    }
}

fn start_conversation(
    dialogue_state: &mut DialogueState,
    conversation_id: &str,
    dialogue_db: &DialogueDatabase,
    dialogue_events: &mut EventWriter<DialogueEvent>,
) {
    if let Some(conversation) = dialogue_db.conversations.get(conversation_id) {
        info!("💬 Starting conversation with {}", conversation.npc_id);
        
        dialogue_state.current_conversation = Some(conversation.clone());
        dialogue_state.current_node = "start".to_string();
        dialogue_state.is_active = true;
        
        // Display the starting dialogue
        display_current_dialogue(dialogue_state, dialogue_db);
        
        dialogue_events.write(DialogueEvent::ConversationStarted {
            player_entity: Entity::PLACEHOLDER,
            npc_entity: Entity::PLACEHOLDER,
            conversation_id: conversation_id.to_string(),
        });
    } else {
        warn!("❌ Conversation not found: {}", conversation_id);
    }
}

fn handle_dialogue_choice(
    dialogue_state: &mut DialogueState,
    choice_index: usize,
    dialogue_db: &DialogueDatabase,
    dialogue_events: &mut EventWriter<DialogueEvent>,
) {
    if let Some(ref conversation) = dialogue_state.current_conversation {
        if let Some(current_node) = conversation.nodes.get(&dialogue_state.current_node) {
            if choice_index < current_node.choices.len() {
                let choice = &current_node.choices[choice_index];
                
                info!("You chose: {}", choice.text);
                
                // Process choice consequences
                if choice.relationship_impact != 0 {
                    info!("🤝 Relationship impact: {:+}", choice.relationship_impact);
                }
                
                for info_gained in &choice.information_gained {
                    info!("📚 Information gained: {}", info_gained);
                }
                
                for clue in &choice.clues_unlocked {
                    info!("🔍 Clue unlocked: {}", clue);
                }
                
                // Move to next node
                dialogue_state.current_node = choice.next_node.clone();
                dialogue_state.conversation_history.push(choice.clone());
                
                // Display new dialogue
                display_current_dialogue(dialogue_state, dialogue_db);
                
                dialogue_events.write(DialogueEvent::ChoiceMade {
                    player_entity: Entity::PLACEHOLDER,
                    npc_entity: Entity::PLACEHOLDER,
                    choice: choice.clone(),
                });
            }
        }
    }
}

fn display_current_dialogue(
    dialogue_state: &DialogueState,
    dialogue_db: &DialogueDatabase,
) {
    if let Some(ref conversation) = dialogue_state.current_conversation {
        if let Some(current_node) = conversation.nodes.get(&dialogue_state.current_node) {
            // Display NPC text
            if !current_node.text_variations.is_empty() {
                let text = &current_node.text_variations[0].text; // Use first variation for now
                info!("💬 {}: {}", conversation.npc_id, text);
            }
            
            // Display available choices
            if !current_node.choices.is_empty() {
                info!("Choose your response:");
                for (i, choice) in current_node.choices.iter().enumerate() {
                    info!("  {}: {}", i + 1, choice.text);
                }
                info!("Press 1-{} to choose, ESC to exit", current_node.choices.len());
            } else {
                // No choices available - end conversation
                info!("💬 Conversation ended.");
            }
        }
    }
}

fn end_conversation(
    dialogue_state: &mut DialogueState,
    reason: ConversationEndReason,
    dialogue_events: &mut EventWriter<DialogueEvent>,
) {
    info!("💬 Conversation ended ({:?})", reason);
    
    dialogue_events.write(DialogueEvent::ConversationEnded {
        player_entity: Entity::PLACEHOLDER,
        npc_entity: Entity::PLACEHOLDER,
        reason,
    });
    
    dialogue_state.is_active = false;
    dialogue_state.current_conversation = None;
    dialogue_state.current_node = "start".to_string();
}

/// System to display dialogue help and status
pub fn dialogue_help_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    dialogue_state_query: Query<&DialogueState, With<Player>>,
) {
    if keyboard.just_pressed(KeyCode::F6) {
        if let Ok(dialogue_state) = dialogue_state_query.single() {
            info!("💬 === DIALOGUE SYSTEM HELP ===");
            info!("F5 - Start conversation with Aldric (demo)");
            info!("1-3 - Choose dialogue options during conversation");
            info!("ESC - Exit current conversation");
            info!("F6 - Show this help");
            
            if dialogue_state.is_active {
                info!("🗣️ Currently in conversation");
                if let Some(ref conversation) = dialogue_state.current_conversation {
                    info!("📍 Speaking with: {}", conversation.npc_id);
                    info!("🎭 Current node: {}", dialogue_state.current_node);
                }
            } else {
                info!("💭 No active conversation");
            }
        }
    }
}