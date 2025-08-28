# Quest System Documentation

## Overview

The Eryndor Quest System represents a revolutionary approach to MMORPG questing, emphasizing investigation, reading comprehension, and player deduction over traditional "kill X monsters" or "deliver Y items" mechanics. This system is designed to create a contemplative, lore-rich experience that rewards careful reading and analytical thinking.

## Core Philosophy

### Narrative-First Design
- **No Quest Markers**: Players must read descriptions and use deductive reasoning
- **Evidence-Based Progression**: Advancement requires gathering and analyzing clues
- **Multiple Solution Paths**: Different players may solve quests through different approaches
- **Reading Comprehension**: Success depends on careful attention to text and lore
- **World Immersion**: No UI elements break the immersion of being in a living world

### Investigation Mechanics
- **Clue Discovery**: Players find evidence through exploration and dialogue
- **Evidence Strength**: Clues have varying strength values that affect quest progression
- **Hypothesis Validation**: Players must form and test theories about quest solutions
- **Consequence Tracking**: Player choices affect future quest availability and outcomes

## System Architecture

### Core Components

#### QuestDatabase Resource
```rust
#[derive(Resource)]
pub struct QuestDatabase {
    pub quests: HashMap<String, QuestDefinition>,
    pub npcs: HashMap<String, NpcDefinition>,
    pub locations: HashMap<String, LocationDefinition>,
}
```
- Loaded from `config/quests.json`
- Contains all quest definitions and related data
- Validated at startup with error reporting

#### QuestLog Component
```rust
#[derive(Component)]
pub struct QuestLog {
    pub active_quests: HashMap<String, QuestProgress>,
    pub completed_quests: HashMap<String, QuestCompletion>,
    pub failed_quests: HashMap<String, QuestFailure>,
    pub global_flags: HashMap<String, QuestFlag>,
}
```
- Attached to player entities
- Tracks all quest state for the player
- Manages quest history and global world state

#### QuestProgress Component
```rust
#[derive(Component)]
pub struct QuestProgress {
    pub quest_id: String,
    pub current_phase: String,
    pub completed_phases: Vec<String>,
    pub discovered_clues: Vec<DiscoveredClue>,
    pub completed_objectives: Vec<String>,
    pub failed_conditions: Vec<String>,
    pub evidence_strength: EvidenceStrength,
    pub investigation_notes: Vec<InvestigationNote>,
    pub start_time: f64,
    pub last_update: f64,
}
```
- Tracks individual quest progression
- Manages evidence collection and strength calculation
- Supports player note-taking for investigation tracking

### Quest Definition Structure

#### Multi-Phase Architecture
```json
{
  "phases": {
    "initial_observation": {
      "title": "Initial Observations",
      "description": "You notice something suspicious about the merchant's behavior...",
      "objectives": ["observe_merchant_behavior", "note_irregularities"],
      "unlock_conditions": {
        "required_evidence_strength": "None"
      }
    },
    "evidence_gathering": {
      "title": "Gathering Evidence",
      "description": "Your observations suggest deeper investigation is warranted...",
      "objectives": ["find_ledger_discrepancies", "interview_witnesses"],
      "unlock_conditions": {
        "required_evidence_strength": "Weak",
        "completed_objectives": ["observe_merchant_behavior"]
      }
    }
  }
}
```

#### Evidence System
```json
{
  "available_clues": {
    "suspicious_ledger": {
      "id": "suspicious_ledger",
      "name": "Suspicious Ledger Entry",
      "description": "A ledger entry that doesn't match the merchant's story...",
      "strength": 2.0,
      "discovery_method": "Investigation",
      "location": "merchant_shop",
      "requires_careful_reading": true
    }
  }
}
```

#### Evidence Strength Calculation
```rust
pub enum EvidenceStrength {
    None,           // 0.0
    Weak,           // 1.0-2.9
    Moderate,       // 3.0-5.9
    Strong,         // 6.0-8.9
    Overwhelming,   // 9.0+
}

impl EvidenceStrength {
    pub fn calculate_from_clues(clues: &[DiscoveredClue]) -> Self {
        let total_strength: f32 = clues.iter()
            .map(|clue| clue.strength)
            .sum();
        
        match total_strength {
            x if x >= 9.0 => EvidenceStrength::Overwhelming,
            x if x >= 6.0 => EvidenceStrength::Strong,
            x if x >= 3.0 => EvidenceStrength::Moderate,
            x if x >= 1.0 => EvidenceStrength::Weak,
            _ => EvidenceStrength::None,
        }
    }
}
```

## Key Systems

### 1. Quest Loading System

#### Startup Process
1. **Configuration Loading**: Read `config/quests.json`
2. **Validation**: Validate quest definitions against schema
3. **Error Handling**: Report parsing errors with detailed information
4. **Database Creation**: Build in-memory quest database
5. **Resource Registration**: Register database as Bevy resource

#### Error Handling
```rust
match serde_json::from_value::<QuestDefinition>(quest_data.clone()) {
    Ok(mut quest_def) => {
        quest_def.id = quest_id.clone();
        quest_database.quests.insert(quest_id.clone(), quest_def);
        info!("✅ Loaded quest: {}", quest_id);
    }
    Err(e) => {
        warn!("❌ Failed to parse quest {}: {}", quest_id, e);
    }
}
```

### 2. Quest Progression System

#### Phase Unlocking Logic
```rust
pub fn check_phase_unlock(
    progress: &QuestProgress,
    quest_def: &QuestDefinition,
    target_phase: &str,
) -> bool {
    let phase = &quest_def.phases[target_phase];
    
    // Check evidence strength requirement
    if progress.evidence_strength < phase.unlock_conditions.required_evidence_strength {
        return false;
    }
    
    // Check completed objectives requirement
    for required_obj in &phase.unlock_conditions.completed_objectives {
        if !progress.completed_objectives.contains(required_obj) {
            return false;
        }
    }
    
    true
}
```

#### Investigation Actions
```rust
pub fn handle_investigation_action(
    action: InvestigationAction,
    quest_progress: &mut QuestProgress,
    quest_db: &QuestDatabase,
) -> InvestigationResult {
    match action {
        InvestigationAction::ExamineClue { clue_id, location } => {
            if let Some(clue_def) = quest_db.find_clue(&clue_id) {
                let discovered_clue = DiscoveredClue {
                    clue_id: clue_id.clone(),
                    name: clue_def.name.clone(),
                    description: clue_def.description.clone(),
                    strength: clue_def.strength,
                    discovery_time: current_time(),
                    discovery_location: location,
                };
                
                quest_progress.discovered_clues.push(discovered_clue);
                quest_progress.evidence_strength = 
                    EvidenceStrength::calculate_from_clues(&quest_progress.discovered_clues);
                
                InvestigationResult::ClueDiscovered
            } else {
                InvestigationResult::NothingFound
            }
        }
        // ... other investigation actions
    }
}
```

### 3. Player Note-Taking System

#### Investigation Notes
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestigationNote {
    pub id: String,
    pub content: String,
    pub timestamp: f64,
    pub related_quest: String,
    pub related_phase: Option<String>,
    pub related_clues: Vec<String>,
    pub player_hypothesis: Option<String>,
}
```

#### Note Management
```rust
pub fn add_investigation_note(
    quest_progress: &mut QuestProgress,
    content: String,
    hypothesis: Option<String>,
) {
    let note = InvestigationNote {
        id: generate_note_id(),
        content,
        timestamp: current_time(),
        related_quest: quest_progress.quest_id.clone(),
        related_phase: Some(quest_progress.current_phase.clone()),
        related_clues: extract_related_clues(&content, &quest_progress.discovered_clues),
        player_hypothesis: hypothesis,
    };
    
    quest_progress.investigation_notes.push(note);
}
```

## Integration Points

### 1. Dialogue System Integration

#### Quest-Related Conversations
- NPCs provide quest information through natural dialogue
- Dialogue options unlock based on discovered evidence
- NPC personalities affect information sharing willingness
- Trust levels determine access to sensitive quest information

#### Implementation Example
```rust
pub fn evaluate_dialogue_availability(
    npc_id: &str,
    dialogue_node: &DialogueNode,
    player_quest_log: &QuestLog,
    npc_relationship: &NpcRelationship,
) -> bool {
    // Check quest requirements
    for required_quest in &dialogue_node.conditions.required_quests {
        if !player_quest_log.active_quests.contains_key(required_quest) {
            return false;
        }
    }
    
    // Check clue requirements
    for required_clue in &dialogue_node.conditions.required_clues {
        let has_clue = player_quest_log.active_quests
            .values()
            .any(|quest| quest.discovered_clues
                .iter()
                .any(|clue| clue.clue_id == *required_clue));
        
        if !has_clue {
            return false;
        }
    }
    
    // Check trust level
    if let Some(required_trust) = dialogue_node.conditions.required_trust_level {
        if npc_relationship.trust_level < required_trust {
            return false;
        }
    }
    
    true
}
```

### 2. World State Integration

#### Global Quest Flags
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestFlag {
    pub flag_id: String,
    pub value: QuestFlagValue,
    pub set_by_quest: String,
    pub set_time: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestFlagValue {
    Boolean(bool),
    Integer(i32),
    String(String),
    Float(f32),
}
```

#### World State Consequences
- Quest completion affects global world state
- NPC behavior changes based on quest outcomes
- Future quest availability depends on previous choices
- Environmental changes reflect player actions

### 3. Character Progression Integration

#### Experience Rewards
```rust
pub fn award_quest_experience(
    quest_completion: &QuestCompletion,
    quest_def: &QuestDefinition,
    character: &mut CharacterLevel,
) {
    let base_experience = quest_def.rewards.experience;
    
    // Bonus for thorough investigation
    let thoroughness_bonus = calculate_thoroughness_bonus(&quest_completion);
    
    // Bonus for creative solutions
    let creativity_bonus = calculate_creativity_bonus(&quest_completion);
    
    let total_experience = base_experience + thoroughness_bonus + creativity_bonus;
    character.add_experience(total_experience);
    
    info!("📈 Gained {} experience from quest completion!", total_experience);
}
```

## Development Tools and Debug Features

### Debug Controls
- **F9**: Start demo quest "The Merchant's Mystery"
- Debug logging controlled by GameDebugConfig.quest_debug
- Quest state inspection through debug UI
- Evidence strength visualization

### Testing and Validation
- JSON schema validation for quest definitions
- Unit tests for evidence strength calculations
- Integration tests for quest phase progression
- Performance monitoring for quest database queries

## Example Quest: "The Merchant's Mystery"

### Overview
A multi-phase investigation quest demonstrating the system's capabilities:

1. **Initial Observation**: Player notices suspicious merchant behavior
2. **Evidence Gathering**: Collect clues through investigation and dialogue
3. **Pattern Recognition**: Analyze evidence to form theories
4. **Confrontation**: Use evidence to confront the merchant
5. **Resolution**: Multiple possible outcomes based on evidence quality and approach

### Key Features Demonstrated
- Evidence-based progression (no quest markers)
- Multiple investigation approaches
- NPC dialogue integration with trust requirements
- Consequence tracking for future interactions
- Player note-taking for hypothesis development

## Future Enhancements

### Planned Features
1. **Quest Journal UI**: Rich text display with note-taking interface
2. **Hypothesis Validation**: System to test and validate player theories
3. **Community Investigation**: Shared discoveries between players
4. **Dynamic Quest Generation**: AI-assisted quest creation tools
5. **Advanced Consequence Tracking**: Long-term world state changes

### Technical Improvements
1. **Performance Optimization**: Efficient quest database queries
2. **Memory Management**: Smart loading/unloading of quest content
3. **Save System Integration**: Persistent quest state across sessions
4. **Network Readiness**: Multiplayer-compatible quest synchronization

## Conclusion

The Eryndor Quest System represents a fundamental shift from traditional MMORPG quest mechanics toward a more thoughtful, investigation-based approach. By emphasizing reading comprehension, deductive reasoning, and player agency, this system creates a unique gameplay experience that rewards careful attention and critical thinking.

The architecture provides a solid foundation for complex narrative content while maintaining the flexibility needed for diverse quest types and player approaches. With its evidence-based progression system and deep integration with dialogue and world state systems, it establishes Eryndor as a truly unique entry in the MMORPG genre.