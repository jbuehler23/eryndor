# Dialogue System Technical Documentation

## Architecture Overview

The Eryndor dialogue system is built using Bevy's ECS (Entity-Component-System) architecture with a file-based approach for dialogue content. The system separates content (JSON files) from logic (Rust systems) to enable non-programmers to create dialogues.

## Core Components

### 1. Dialogue Components (`src/components/dialogue.rs`)

#### `DialogueDatabase` (Resource)
Global storage for all loaded dialogues.
```rust
#[derive(Resource, Debug, Clone)]
pub struct DialogueDatabase {
    pub npcs: HashMap<String, NpcDialogue>,
    pub common_phrases: HashMap<String, DialogueNode>,
}
```

#### `NpcDialogue`
Complete dialogue definition for an NPC loaded from JSON.
```rust
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
```

#### `DialogueState` (Component)
Runtime state tracking for each NPC's dialogue progression.
```rust
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
```

#### `DialogueInteractable` (Component)
Marks NPCs as interactable with dialogue range and priority.
```rust
#[derive(Component, Debug, Clone)]
pub struct DialogueInteractable {
    pub npc_id: String,
    pub interaction_range: f32,
    pub has_new_dialogue: bool,
    pub priority_level: u8,
}
```

#### `ActiveDialogue` (Resource)
Global state for the currently active conversation.
```rust
#[derive(Resource, Debug, Default)]
pub struct ActiveDialogue {
    pub npc_entity: Option<Entity>,
    pub player_entity: Option<Entity>,
    pub current_node: Option<DialogueNode>,
    pub available_choices: Vec<DialogueChoice>,
    pub dialogue_history: Vec<String>,
}
```

### 2. Dialogue Events

Event-driven communication between dialogue systems:
```rust
#[derive(Event, Debug, Clone)]
pub enum DialogueEvent {
    StartConversation {
        npc_entity: Entity,
        player_entity: Entity,
        conversation_id: Option<String>,
    },
    ChoiceSelected {
        npc_entity: Entity,
        choice_id: String,
        next_node: String,
    },
    EndConversation {
        npc_entity: Entity,
        player_entity: Entity,
    },
    NodeChanged {
        npc_entity: Entity,
        previous_node: String,
        new_node: String,
    },
}
```

## Systems Architecture

### 1. Dialogue Loader (`src/systems/dialogue_loader.rs`)

**Startup System**: Loads and validates all dialogue files on game start.

Key Functions:
- `load_dialogue_database()`: Main loading function
- `load_dialogue_files()`: Recursively loads JSON files
- `validate_dialogue_structure()`: Ensures dialogue integrity
- `hot_reload_dialogue_system()`: Runtime reloading (F10 key)

**Validation Features:**
- JSON syntax validation
- Node reference integrity checking
- Circular dependency detection
- Missing conversation/node detection

### 2. Dialogue Interaction (`src/systems/dialogue_interaction.rs`)

**Update System**: Handles player input and dialogue flow.

Key Functions:
- `enhanced_dialogue_interaction_system()`: Main input handler
- `start_conversation()`: Initiates NPC dialogue
- `handle_choice_selection()`: Processes player responses
- `process_dialogue_choice()`: Event handler for choice progression

**Input Mapping:**
- E: Start/End conversation
- 1-4: Select dialogue choices
- F1: Help system
- F11: Debug info

### 3. NPC Spawning (`src/systems/npc_spawning.rs`)

**Startup System**: Creates visual NPCs with dialogue components.

Key Functions:
- `spawn_demo_npcs()`: Creates example NPCs
- `spawn_npc()`: Individual NPC creation
- `update_npc_indicators()`: Visual feedback system
- `npc_interaction_prompts()`: Proximity-based UI

**NPC Visual System:**
- Capsule3D meshes with colored materials
- Static physics colliders (Avian 3D)
- Dynamic material changes for interaction feedback
- Proximity-based glow effects

## Data Flow

### 1. Startup Sequence
```
Game Start → Load Dialogue Files → Validate Structure → 
Create DialogueDatabase → Spawn NPCs → Ready for Interaction
```

### 2. Interaction Flow
```
Player approaches NPC → Proximity detection → Show prompt →
E key pressed → Find nearest NPC → Start conversation →
Display dialogue node → Player selects choice →
Process choice → Move to next node → Repeat or End
```

### 3. Event Flow
```
DialogueEvent::StartConversation → Update ActiveDialogue →
Display current node → Player input → 
DialogueEvent::ChoiceSelected → Process quest actions →
DialogueEvent::NodeChanged → Continue or EndConversation
```

## File System Integration

### Directory Structure
```
config/dialogues/
├── npcs/           # Individual NPC dialogue files
├── common/         # Shared dialogue components
└── quests/         # Quest-specific dialogues (future)
```

### Hot Reloading
The system monitors file changes and reloads dialogues at runtime:
- File modification detection
- Incremental loading of changed files
- Runtime validation and error reporting
- Live testing during development

### JSON Schema
Dialogues follow a strict JSON schema with validation:
- Required fields enforcement
- Type checking for all properties
- Reference validation between nodes
- Error reporting with file locations

## Integration Points

### 1. Quest System Integration
Dialogue nodes can trigger quest actions:
```json
{
  "quest_action": {
    "type": "start_quest",
    "quest_id": "delivery_mission",
    "phase": "accepted",
    "items": ["package"]
  }
}
```

**Quest Action Types:**
- `start_quest`: Begin new quest
- `quest_assigned`: Mark quest as assigned
- `give_clue`: Provide investigation clues
- `complete_quest`: Mark quest complete

### 2. Character Skill System
Dialogue choices can be locked behind skill requirements:
```json
{
  "skill_requirements": [
    {
      "skill": "intimidation",
      "level": 25
    }
  ]
}
```

### 3. Relationship System
NPCs track relationship changes through dialogue:
```rust
pub struct DialogueState {
    pub trust_level: i32,
    pub relationship_modifiers: HashMap<String, i32>,
}
```

## Performance Considerations

### Memory Management
- Dialogue database loaded once at startup
- Individual NPC states stored as components
- Conversation history pruning for long dialogues
- JSON parsing cached to avoid repeated file reads

### Update Loop Optimization
- Proximity checking using spatial queries
- Input handling only for nearby NPCs
- Event-driven dialogue progression
- Minimal state updates per frame

### File I/O Optimization
- Batch loading of dialogue files
- Incremental hot-reloading
- Async file operations (future enhancement)
- Error caching to avoid repeated validation

## Error Handling

### Compile-Time Safety
- Strong typing for all dialogue components
- Rust's ownership system prevents data races
- Comprehensive error enums for all failure modes

### Runtime Error Recovery
- Graceful fallback for missing dialogue files
- Default dialogue nodes for broken references
- Error logging without game crashes
- Development-mode error display

### Validation Errors
```rust
#[derive(Debug, Clone)]
pub enum DialogueValidationError {
    MissingConversation(String),
    InvalidNodeReference(String, String),
    CircularReference(Vec<String>),
    MalformedJSON(String),
    MissingRequiredField(String, String),
}
```

## Future Enhancements

### Planned Features
1. **Visual Dialogue Editor**: Web-based GUI for dialogue creation
2. **Voice Acting Integration**: Audio clip support for dialogue nodes  
3. **Advanced Branching**: Conditional logic based on game state
4. **Localization Support**: Multi-language dialogue files
5. **Performance Analytics**: Dialogue engagement metrics

### Extensibility Points
- Custom dialogue action types via trait system
- Pluggable validation rules for different content types
- External dialogue source integration (databases, APIs)
- Custom UI rendering for different dialogue presentations

## Testing Strategy

### Unit Tests
- Dialogue loading and parsing
- Node reference validation
- Event processing logic
- Component state management

### Integration Tests
- End-to-end conversation flows
- Quest system integration
- Save/load state persistence
- Hot-reloading functionality

### Manual Testing
- Dialogue flow testing with debug tools
- Player experience testing
- Content validation workflows
- Performance testing with large dialogue sets

## Debug Tools

### Developer Console Commands
- `F10`: Reload all dialogue files
- `F11`: Display NPC debug information
- `F1`: Show player help and controls

### Logging Categories
- `dialogue_loader`: File loading and validation
- `dialogue_interaction`: Player interaction events
- `npc_spawning`: NPC creation and management
- `quest_integration`: Quest action processing

### Debug Overlays (Future)
- Real-time dialogue state visualization
- Conversation flow graphs
- Performance metrics display
- Content validation status

This technical documentation provides the foundation for extending and maintaining the dialogue system as the game grows in complexity.