# Dialogue System Documentation

## Overview

The Eryndor Dialogue System is a sophisticated conversation framework designed to support the game's narrative-first philosophy. Unlike traditional MMORPGs with simple vendor interactions, this system creates dynamic, personality-driven conversations that respond to player actions, relationship history, and world state.

## Core Philosophy

### Personality-Driven Interactions
- **Unique NPC Voices**: Each NPC has distinct speech patterns and personality traits
- **Emotional States**: NPCs have moods that affect their dialogue choices
- **Relationship Building**: Trust levels evolve based on player interactions
- **Contextual Awareness**: NPCs remember previous conversations and player actions

### Dynamic Conversation Flow
- **Branching Narratives**: Multiple dialogue paths based on player choices and world state
- **Consequence Tracking**: Conversations affect future interactions and quest availability
- **Information Gating**: Trust levels and relationships determine information access
- **Time-Sensitive Content**: Some conversations are only available at certain times or conditions

## System Architecture

### Core Components

#### DialogueState Component
```rust
#[derive(Component, Debug, Clone)]
pub struct DialogueState {
    pub current_npc: Option<Entity>,
    pub current_conversation: Option<ConversationTree>,
    pub current_node: String,
    pub conversation_history: Vec<DialogueChoice>,
    pub is_active: bool,
}
```
- Attached to player entities during conversations
- Tracks current conversation state and history
- Manages conversation flow and node transitions

#### ConversationTree Structure
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTree {
    pub npc_id: String,
    pub conversation_id: String,
    pub context: ConversationContext,
    pub nodes: HashMap<String, DialogueNode>,
    pub personality_modifiers: PersonalityModifiers,
}
```
- Represents complete conversation with an NPC
- Contains all dialogue nodes and branching logic
- Includes personality traits that affect conversation flow

#### NPC Personality System
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityModifiers {
    pub verbose_factor: f32,           // How much extra text they add
    pub trust_building_speed: f32,     // How quickly they warm up to player
    pub information_reluctance: f32,   // How hesitant they are to share secrets
    pub speech_patterns: Vec<String>,  // Characteristic phrases and mannerisms
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
```

### Dialogue Node Architecture

#### Individual Dialogue Nodes
```rust
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
```

#### Text Variations by Personality
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextVariation {
    pub condition: PersonalityCondition,
    pub text: String,
    pub personality_modifier: Option<f32>,
}

// Example variations for different NPC moods:
// Calm: "I suppose I could tell you about the merchant..."
// Anxious: "Well, I-I'm not sure I should say, but the merchant seems..."
// Suspicious: "Why are you asking about the merchant? What's it to you?"
```

#### Dialogue Conditions
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueCondition {
    pub condition_type: ConditionType,
    pub required_value: ConditionValue,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    QuestActive(String),
    QuestCompleted(String),
    ClueDiscovered(String),
    TrustLevel(TrustLevel),
    TimeOfDay(u32, u32),         // Hour range
    PlayerChoice(String),         // Previous choice made
    WorldFlag(String),           // Global world state
    RelationshipValue(String, i32), // Relationship threshold
}
```

### Trust and Relationship System

#### Trust Level Progression
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TrustLevel {
    Hostile,        // -100 to -51: NPC actively dislikes player
    Suspicious,     // -50 to -16: NPC is wary and reluctant
    Neutral,        // -15 to 15: Standard interactions
    Friendly,       // 16 to 50: NPC is helpful and open
    Trusted,        // 51 to 85: NPC shares sensitive information
    Confidant,      // 86 to 100: NPC shares secrets and personal matters
}
```

#### NPC Relationship Tracking
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcRelationship {
    pub npc_id: String,
    pub trust_value: i32,
    pub trust_level: TrustLevel,
    pub relationship_history: Vec<InteractionRecord>,
    pub conversation_count: u32,
    pub last_interaction: f64,
    pub special_flags: HashMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionRecord {
    pub timestamp: f64,
    pub conversation_id: String,
    pub choices_made: Vec<String>,
    pub trust_change: i32,
    pub outcomes: Vec<String>,
}
```

## Key Systems

### 1. Conversation Initialization

#### NPC Interaction Detection
```rust
pub fn initiate_dialogue(
    npc_entity: Entity,
    player_entity: Entity,
    npc_id: &str,
    dialogue_db: &DialogueDatabase,
    quest_log: &QuestLog,
    relationships: &NpcRelationships,
) -> Result<ConversationTree, DialogueError> {
    // Find available conversations for this NPC
    let available_conversations = dialogue_db
        .find_conversations_for_npc(npc_id)
        .filter(|conv| evaluate_conversation_availability(conv, quest_log, relationships))
        .collect::<Vec<_>>();
    
    // Select best conversation based on context
    let selected_conversation = select_best_conversation(
        &available_conversations,
        quest_log,
        relationships.get(npc_id)
    )?;
    
    Ok(selected_conversation.clone())
}
```

#### Context-Aware Conversation Selection
```rust
pub fn select_best_conversation(
    conversations: &[ConversationTree],
    quest_log: &QuestLog,
    relationship: Option<&NpcRelationship>,
) -> Option<&ConversationTree> {
    let mut scored_conversations = conversations.iter()
        .map(|conv| (conv, calculate_conversation_priority(conv, quest_log, relationship)))
        .collect::<Vec<_>>();
    
    scored_conversations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    scored_conversations.first().map(|(conv, _)| *conv)
}

fn calculate_conversation_priority(
    conversation: &ConversationTree,
    quest_log: &QuestLog,
    relationship: Option<&NpcRelationship>,
) -> f32 {
    let mut priority = 0.0;
    
    // Prioritize quest-related conversations
    match conversation.context.conversation_type {
        ConversationType::QuestInitiation => priority += 100.0,
        ConversationType::QuestInvestigation => priority += 80.0,
        ConversationType::Information => priority += 50.0,
        ConversationType::Casual => priority += 10.0,
        ConversationType::Trading => priority += 30.0,
        ConversationType::Lore => priority += 40.0,
    }
    
    // Adjust for relationship level
    if let Some(rel) = relationship {
        priority += rel.trust_value as f32 * 0.1;
    }
    
    priority
}
```

### 2. Dialogue Node Processing

#### Text Generation with Personality
```rust
pub fn generate_dialogue_text(
    node: &DialogueNode,
    npc_personality: &PersonalityModifiers,
    relationship: Option<&NpcRelationship>,
) -> String {
    // Find appropriate text variation
    let base_text = select_text_variation(node, npc_personality, relationship);
    
    // Apply personality modifications
    let modified_text = apply_personality_modifications(base_text, npc_personality);
    
    // Add speech patterns
    add_speech_patterns(modified_text, &npc_personality.speech_patterns)
}

fn apply_personality_modifications(
    text: String,
    personality: &PersonalityModifiers,
) -> String {
    let mut modified_text = text;
    
    // Apply verbose factor
    if personality.verbose_factor > 1.0 {
        modified_text = add_verbose_elements(modified_text, personality.verbose_factor);
    }
    
    // Apply emotional state modifications
    modified_text = apply_emotional_coloring(modified_text, &personality.emotional_state);
    
    modified_text
}

fn add_verbose_elements(text: String, verbose_factor: f32) -> String {
    if verbose_factor <= 1.0 {
        return text;
    }
    
    // Add filler words, elaborations, and tangents based on verbose factor
    match verbose_factor {
        x if x >= 2.0 => add_extensive_elaboration(text),
        x if x >= 1.5 => add_moderate_elaboration(text),
        _ => add_minor_elaboration(text),
    }
}
```

#### Choice Evaluation and Consequences
```rust
pub fn evaluate_dialogue_choice(
    choice: &DialogueChoice,
    current_state: &DialogueState,
    quest_log: &QuestLog,
    relationship: &mut NpcRelationship,
) -> DialogueResult {
    let mut result = DialogueResult::new();
    
    // Apply immediate consequences
    for consequence in &choice.consequences {
        match consequence {
            DialogueConsequence::TrustChange(amount) => {
                relationship.modify_trust(*amount);
                result.trust_changes.push((*amount, consequence.description.clone()));
            }
            DialogueConsequence::RevealClue(clue_id) => {
                result.revealed_clues.push(clue_id.clone());
            }
            DialogueConsequence::StartQuest(quest_id) => {
                result.quest_triggers.push(quest_id.clone());
            }
            DialogueConsequence::SetWorldFlag(flag_id, value) => {
                result.world_flags.push((flag_id.clone(), value.clone()));
            }
            DialogueConsequence::UnlockFutureDialogue(dialogue_id) => {
                relationship.special_flags.insert(dialogue_id.clone(), true);
            }
        }
    }
    
    // Record interaction in relationship history
    let interaction = InteractionRecord {
        timestamp: current_time(),
        conversation_id: current_state.current_conversation
            .as_ref().unwrap().conversation_id.clone(),
        choices_made: vec![choice.choice_id.clone()],
        trust_change: result.trust_changes.iter().map(|(change, _)| *change).sum(),
        outcomes: result.get_outcome_descriptions(),
    };
    
    relationship.relationship_history.push(interaction);
    relationship.last_interaction = current_time();
    
    result
}
```

### 3. Advanced Conversation Features

#### Time-Restricted Conversations
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestrictions {
    pub available_hours: Vec<u32>,     // 0-23
    pub required_location: Option<String>,
    pub seasonal_restrictions: Vec<String>,
}

pub fn check_time_availability(
    restrictions: &TimeRestrictions,
    current_time: &GameTime,
    player_location: &str,
) -> bool {
    // Check hour restrictions
    if !restrictions.available_hours.is_empty() 
        && !restrictions.available_hours.contains(&current_time.hour) {
        return false;
    }
    
    // Check location restrictions
    if let Some(required_loc) = &restrictions.required_location {
        if player_location != required_loc {
            return false;
        }
    }
    
    // Check seasonal restrictions
    if !restrictions.seasonal_restrictions.is_empty() 
        && !restrictions.seasonal_restrictions.contains(&current_time.season) {
        return false;
    }
    
    true
}
```

#### Information Reluctance System
```rust
pub fn check_information_sharing(
    information_sensitivity: f32,
    npc_personality: &PersonalityModifiers,
    relationship: &NpcRelationship,
) -> bool {
    // Base willingness from trust level
    let trust_willingness = match relationship.trust_level {
        TrustLevel::Hostile => 0.0,
        TrustLevel::Suspicious => 0.2,
        TrustLevel::Neutral => 0.5,
        TrustLevel::Friendly => 0.7,
        TrustLevel::Trusted => 0.9,
        TrustLevel::Confidant => 1.0,
    };
    
    // Modify by personality reluctance
    let effective_willingness = trust_willingness * (1.0 - npc_personality.information_reluctance);
    
    // Check if willing to share this level of information
    effective_willingness >= information_sensitivity
}
```

## Integration Points

### 1. Quest System Integration

#### Quest-Driven Conversations
```rust
pub fn find_quest_dialogue_options(
    npc_id: &str,
    active_quests: &HashMap<String, QuestProgress>,
    dialogue_db: &DialogueDatabase,
) -> Vec<DialogueOption> {
    let mut options = Vec::new();
    
    for (quest_id, quest_progress) in active_quests {
        // Find dialogue nodes related to this quest
        let quest_dialogues = dialogue_db.find_quest_dialogues(npc_id, quest_id);
        
        for dialogue in quest_dialogues {
            // Check if evidence requirements are met
            if check_evidence_requirements(&dialogue, quest_progress) {
                options.push(create_quest_dialogue_option(dialogue, quest_progress));
            }
        }
    }
    
    options
}
```

#### Evidence-Based Dialogue Unlocking
```rust
pub fn check_evidence_requirements(
    dialogue_node: &DialogueNode,
    quest_progress: &QuestProgress,
) -> bool {
    for required_clue in &dialogue_node.conditions {
        if let ConditionType::ClueDiscovered(clue_id) = &required_clue.condition_type {
            let has_clue = quest_progress.discovered_clues
                .iter()
                .any(|clue| clue.clue_id == *clue_id);
            
            if !has_clue {
                return false;
            }
        }
    }
    true
}
```

### 2. World State Integration

#### Global Flag Management
```rust
#[derive(Resource)]
pub struct WorldDialogueState {
    pub global_flags: HashMap<String, DialogueFlag>,
    pub npc_relationships: HashMap<String, NpcRelationship>,
    pub conversation_history: Vec<GlobalConversationRecord>,
}

pub fn apply_dialogue_world_effects(
    dialogue_result: &DialogueResult,
    world_state: &mut WorldDialogueState,
    npc_id: &str,
) {
    // Apply world flag changes
    for (flag_id, flag_value) in &dialogue_result.world_flags {
        world_state.global_flags.insert(
            flag_id.clone(),
            DialogueFlag {
                value: flag_value.clone(),
                set_by_npc: npc_id.to_string(),
                set_time: current_time(),
            }
        );
    }
    
    // Record conversation in global history
    let record = GlobalConversationRecord {
        timestamp: current_time(),
        npc_id: npc_id.to_string(),
        conversation_id: dialogue_result.conversation_id.clone(),
        significant_outcomes: dialogue_result.get_significant_outcomes(),
    };
    
    world_state.conversation_history.push(record);
}
```

### 3. Character Development Integration

#### Relationship Building Rewards
```rust
pub fn check_relationship_milestones(
    relationship: &NpcRelationship,
    previous_trust_level: TrustLevel,
) -> Vec<RelationshipReward> {
    let mut rewards = Vec::new();
    
    if relationship.trust_level > previous_trust_level {
        match relationship.trust_level {
            TrustLevel::Friendly => {
                rewards.push(RelationshipReward::NewDialogueOptions);
                rewards.push(RelationshipReward::QuestAccess);
            }
            TrustLevel::Trusted => {
                rewards.push(RelationshipReward::SecretInformation);
                rewards.push(RelationshipReward::SpecialServices);
            }
            TrustLevel::Confidant => {
                rewards.push(RelationshipReward::ExclusiveQuests);
                rewards.push(RelationshipReward::LoreRevelations);
            }
            _ => {}
        }
    }
    
    rewards
}
```

## Example Implementation

### Sample NPC Conversation Setup
```json
{
  "npc_id": "merchant_aldric",
  "conversation_id": "suspicious_behavior_inquiry",
  "context": {
    "conversation_type": "QuestInvestigation",
    "required_quests": ["the_merchants_mystery"],
    "required_clues": [],
    "relationship_requirements": "Neutral"
  },
  "personality_modifiers": {
    "verbose_factor": 1.3,
    "trust_building_speed": 0.8,
    "information_reluctance": 0.6,
    "speech_patterns": ["Well, you see...", "I must say...", "Indeed, quite so..."],
    "emotional_state": "Anxious"
  },
  "nodes": {
    "start": {
      "speaker": "NPC",
      "text_variations": [
        {
          "condition": {"emotional_state": "Anxious"},
          "text": "Oh, h-hello there. I wasn't expecting anyone to... well, what brings you to my humble establishment?"
        },
        {
          "condition": {"emotional_state": "Calm"},
          "text": "Good day to you. How may I assist you with your shopping needs?"
        }
      ],
      "choices": [
        {
          "choice_id": "ask_about_ledger",
          "text": "I noticed some interesting entries in your ledger...",
          "conditions": [{"type": "ClueDiscovered", "value": "suspicious_ledger"}],
          "consequences": [
            {"type": "TrustChange", "value": -5},
            {"type": "UnlockNode", "value": "defensive_response"}
          ]
        }
      ]
    }
  }
}
```

## Development Tools and Debug Features

### Debug Controls
- Dialogue debug logging controlled by GameDebugConfig.quest_debug
- Real-time relationship tracking display
- Conversation tree visualization (planned)
- Trust level monitoring

### Testing Framework
- Unit tests for personality modifier calculations
- Integration tests for conversation flow
- Trust level progression validation
- Dialogue condition evaluation testing

## Future Enhancements

### Planned Features
1. **Voice Integration**: Audio playback with personality-based delivery
2. **Advanced Emotion System**: Dynamic emotional state changes during conversation
3. **Group Conversations**: Multi-NPC dialogue scenarios
4. **Player Reputation**: Global reputation affecting all NPC interactions
5. **Dynamic Personality Generation**: AI-assisted NPC personality creation

### Technical Improvements
1. **Performance Optimization**: Efficient dialogue tree traversal
2. **Memory Management**: Smart loading of conversation content
3. **Localization Support**: Multi-language dialogue system
4. **Editor Tools**: Visual dialogue tree editing interface

## Conclusion

The Eryndor Dialogue System represents a significant advancement in MMORPG NPC interaction design. By incorporating sophisticated personality modeling, relationship tracking, and dynamic conversation flow, it creates engaging, meaningful interactions that support the game's narrative-first philosophy.

The system's deep integration with the quest system and world state management ensures that conversations feel consequential and meaningful, contributing to player immersion and investment in the game world. With its foundation of trust-based information sharing and personality-driven dialogue, it establishes a new standard for NPC interaction in MMORPGs.