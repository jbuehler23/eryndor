# System Architecture Diagrams

This document provides visual representations of Eryndor's system architecture, showing how different components interact and data flows through the system.

## System Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           ERYNDOR GAME ARCHITECTURE                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐        │
│  │   INPUT LAYER   │    │   GAME LOGIC    │    │  RENDER LAYER   │        │
│  │                 │    │                 │    │                 │        │
│  │ • Keyboard      │    │ • ECS Systems   │    │ • 3D Rendering  │        │
│  │ • Mouse         │────►│ • Components    │────►│ • UI Rendering │        │
│  │ • Controller    │    │ • Resources     │    │ • Audio Output  │        │
│  └─────────────────┘    └─────────────────┘    └─────────────────┘        │
│                                   │                                         │
│                          ┌─────────────────┐                              │
│                          │ CONFIGURATION   │                              │
│                          │                 │                              │
│                          │ • JSON Files    │                              │
│                          │ • Validation    │                              │
│                          │ • Hot Reload    │                              │
│                          └─────────────────┘                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Core ECS Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      BEVY ECS ARCHITECTURE OVERVIEW                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ENTITIES                    COMPONENTS                    SYSTEMS          │
│  ┌─────────┐                ┌─────────────┐              ┌─────────────┐    │
│  │ Player  │───────────────►│ Position    │              │ Movement    │    │
│  │         │                │ Health      │              │ Combat      │    │
│  │         │                │ Inventory   │              │ Dialogue    │    │
│  │         │                │ QuestLog    │              │ Quest       │    │
│  └─────────┘                │ Stats       │              │ Physics     │    │
│                              └─────────────┘              │ Rendering   │    │
│  ┌─────────┐                                              │ Animation   │    │
│  │ NPC     │───────────────►┌─────────────┐              │ AI          │    │
│  │         │                │ Position    │              │ Terrain     │    │
│  │         │                │ Personality │              └─────────────┘    │
│  │         │                │ Dialogue    │                      │         │
│  │         │                │ AI State    │              ┌─────────────┐    │
│  └─────────┘                └─────────────┘              │ RESOURCES   │    │
│                                                          │             │    │
│  ┌─────────┐                ┌─────────────┐              │ • GameState │    │
│  │ Terrain │───────────────►│ Position    │              │ • Time      │    │
│  │         │                │ Mesh        │              │ • Config    │    │
│  │         │                │ Collider    │              │ • QuestDB   │    │
│  │         │                │ Material    │              │ • DialogueDB│    │
│  └─────────┘                └─────────────┘              └─────────────┘    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## System Interaction Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        SYSTEM INTERACTION DIAGRAM                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│    INPUT           GAMEPLAY          NARRATIVE         WORLD               │
│  ┌─────────┐      ┌─────────┐      ┌─────────┐      ┌─────────┐           │
│  │ Player  │ ──── │ Combat  │ ──── │ Quest   │ ──── │ Physics │           │
│  │ Input   │      │ System  │      │ System  │      │ System  │           │
│  └─────────┘      └─────────┘      └─────────┘      └─────────┘           │
│       │                │                │                │                 │
│       │                │                │                │                 │
│       ▼                ▼                ▼                ▼                 │
│  ┌─────────┐      ┌─────────┐      ┌─────────┐      ┌─────────┐           │
│  │ Movement│      │ Target  │      │Dialogue │      │ Terrain │           │
│  │ System  │      │Selection│      │ System  │      │ System  │           │
│  └─────────┘      └─────────┘      └─────────┘      └─────────┘           │
│       │                │                │                │                 │
│       │                │                │                │                 │
│       ▼                ▼                ▼                ▼                 │
│  ┌─────────┐      ┌─────────┐      ┌─────────┐      ┌─────────┐           │
│  │Character│      │ Stats   │      │   NPC   │      │Collision│           │
│  │Controller│     │ System  │      │   AI    │      │Detection│           │
│  └─────────┘      └─────────┘      └─────────┘      └─────────┘           │
│       │                │                │                │                 │
│       └────────────────┴────────────────┴────────────────┘                 │
│                                   │                                        │
│                                   ▼                                        │
│                           ┌─────────────┐                                  │
│                           │   EVENTS    │                                  │
│                           │             │                                  │
│                           │ • Combat    │                                  │
│                           │ • Quest     │                                  │
│                           │ • Dialogue  │                                  │
│                           │ • Movement  │                                  │
│                           └─────────────┘                                  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Quest System Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         QUEST SYSTEM ARCHITECTURE                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  QUEST DEFINITION (JSON)              RUNTIME SYSTEMS                      │
│  ┌─────────────────────┐              ┌─────────────────────┐              │
│  │ • Quest Phases      │──────────────►│ Quest Progression  │              │
│  │ • Available Clues   │              │ System              │              │
│  │ • NPC References    │              └─────────────────────┘              │
│  │ • Unlock Conditions │                        │                          │
│  │ • Evidence Rules    │                        ▼                          │
│  └─────────────────────┘              ┌─────────────────────┐              │
│             │                          │ Evidence Tracking  │              │
│             │                          │ System              │              │
│             ▼                          └─────────────────────┘              │
│  ┌─────────────────────┐                        │                          │
│  │ Quest Database      │                        ▼                          │
│  │ (Runtime Resource)  │              ┌─────────────────────┐              │
│  └─────────────────────┘              │ Player Quest Log    │              │
│             │                          │ (Component)         │              │
│             │                          │                     │              │
│             └──────────────────────────►│ • Active Quests    │              │
│                                        │ • Completed Quests  │              │
│                                        │ • Discovered Clues  │              │
│                                        │ • Evidence Strength │              │
│                                        │ • Investigation     │              │
│                                        │   Notes             │              │
│                                        └─────────────────────┘              │
│                                                  │                          │
│                                                  ▼                          │
│                                        ┌─────────────────────┐              │
│                                        │ Quest Events        │              │
│                                        │                     │              │
│                                        │ • QuestStarted      │              │
│                                        │ • ClueDiscovered    │              │
│                                        │ • PhaseUnlocked     │              │
│                                        │ • QuestCompleted    │              │
│                                        └─────────────────────┘              │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Dialogue System Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                       DIALOGUE SYSTEM ARCHITECTURE                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  NPC PERSONALITY                       DIALOGUE TREE                       │
│  ┌─────────────────────┐              ┌─────────────────────┐              │
│  │ • Base Personality  │              │ • Root Node         │              │
│  │ • Emotional State   │              │ • Branch Conditions │              │
│  │ • Trust Level       │              │ • Response Options  │              │
│  │ • Relationship      │              │ • Personality       │              │
│  │   History           │              │   Modifiers         │              │
│  └─────────────────────┘              └─────────────────────┘              │
│             │                                    │                          │
│             ▼                                    ▼                          │
│  ┌─────────────────────┐              ┌─────────────────────┐              │
│  │ Personality         │              │ Dialogue Evaluation │              │
│  │ Modifiers           │──────────────►│ System              │              │
│  │                     │              │                     │              │
│  │ • Confidence: -0.2  │              │ • Trust Check       │              │
│  │ • Openness: +0.5    │              │ • Quest Check       │              │
│  │ • Cooperation: +0.1 │              │ • Evidence Check    │              │
│  └─────────────────────┘              │ • Personality Apply │              │
│                                        └─────────────────────┘              │
│                                                  │                          │
│                                                  ▼                          │
│  ┌─────────────────────┐              ┌─────────────────────┐              │
│  │ Quest Integration   │              │ Player Response     │              │
│  │                     │◄─────────────│ Selection           │              │
│  │ • Required Clues    │              │                     │              │
│  │ • Evidence Strength │              │ • Available Options │              │
│  │ • Quest Progression │              │ • Dialogue UI       │              │
│  └─────────────────────┘              │ • Trust Impact      │              │
│                                        └─────────────────────┘              │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Character Controller Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    CHARACTER CONTROLLER ARCHITECTURE                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  INPUT PROCESSING                      MOVEMENT LOGIC                      │
│  ┌─────────────────────┐              ┌─────────────────────┐              │
│  │ • Keyboard Input    │              │ Kinematic Controller│              │
│  │ • Mouse Input       │──────────────►│                     │              │
│  │ • Controller Input  │              │ • Velocity Calc     │              │
│  │ • Input Mapping     │              │ • Ground Detection  │              │
│  └─────────────────────┘              │ • Jump Logic        │              │
│             │                          │ • Coyote Time      │              │
│             ▼                          │ • Step Up System   │              │
│  ┌─────────────────────┐              └─────────────────────┘              │
│  │ Movement State      │                        │                          │
│  │                     │                        ▼                          │
│  │ • Walking           │              ┌─────────────────────┐              │
│  │ • Running           │              │ Physics Integration │              │
│  │ • Jumping           │              │                     │              │
│  │ • Falling           │              │ • Collision         │              │
│  │ • On Ground         │              │ • Slope Handling    │              │
│  └─────────────────────┘              │ • Surface Detection │              │
│                                        └─────────────────────┘              │
│                                                  │                          │
│                                                  ▼                          │
│                                        ┌─────────────────────┐              │
│                                        │ Transform Update    │              │
│                                        │                     │              │
│                                        │ • Position          │              │
│                                        │ • Rotation          │              │
│                                        │ • Camera Following  │              │
│                                        │ • Animation Sync    │              │
│                                        └─────────────────────┘              │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Data Flow Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              DATA FLOW DIAGRAM                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  CONFIGURATION FILES                    RUNTIME DATA                       │
│  ┌─────────────────────┐              ┌─────────────────────┐              │
│  │ config/quests.json  │──────────────►│ QuestDatabase       │              │
│  │ config/dialogue.json│──────────────►│ DialogueDatabase    │              │
│  │ config/skills.json  │──────────────►│ SkillDefinitions    │              │
│  │ config/debug.json   │──────────────►│ GameDebugConfig     │              │
│  └─────────────────────┘              └─────────────────────┘              │
│             │                                    │                          │
│             ▼                                    ▼                          │
│  ┌─────────────────────┐              ┌─────────────────────┐              │
│  │ Validation Layer    │              │ Component Data      │              │
│  │                     │              │                     │              │
│  │ • Schema Validation │              │ • Player Stats      │              │
│  │ • Error Reporting   │              │ • Quest Progress    │              │
│  │ • Hot Reload        │              │ • NPC States        │              │
│  └─────────────────────┘              │ • World State       │              │
│                                        └─────────────────────┘              │
│                                                  │                          │
│                                                  ▼                          │
│                                        ┌─────────────────────┐              │
│                                        │ System Processing   │              │
│                                        │                     │              │
│                                        │ • Game Logic        │              │
│                                        │ • State Updates     │              │
│                                        │ • Event Generation  │              │
│                                        └─────────────────────┘              │
│                                                  │                          │
│                                                  ▼                          │
│                                        ┌─────────────────────┐              │
│                                        │ Render Pipeline     │              │
│                                        │                     │              │
│                                        │ • 3D Rendering      │              │
│                                        │ • UI Rendering      │              │
│                                        │ • Debug Overlay     │              │
│                                        └─────────────────────┘              │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## System Dependencies

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                            SYSTEM DEPENDENCIES                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  FOUNDATIONAL LAYER                                                         │
│  ┌───────────────────────────────────────────────────────────────────────┐ │
│  │ Bevy Engine • Avian Physics • Serde Serialization                    │ │
│  └───────────────────────────────────────────────────────────────────────┘ │
│                                      │                                      │
│  CORE SYSTEMS LAYER                  ▼                                      │
│  ┌───────────────────────────────────────────────────────────────────────┐ │
│  │ Debug System • Asset Loading • Configuration Management              │ │
│  └───────────────────────────────────────────────────────────────────────┘ │
│                                      │                                      │
│  GAMEPLAY SYSTEMS LAYER              ▼                                      │
│  ┌───────────────────────────────────────────────────────────────────────┐ │
│  │ Character Controller • Physics • Terrain • Animation                 │ │
│  └───────────────────────────────────────────────────────────────────────┘ │
│                                      │                                      │
│  NARRATIVE SYSTEMS LAYER             ▼                                      │
│  ┌───────────────────────────────────────────────────────────────────────┐ │
│  │ Quest System • Dialogue System • NPC AI • Progression                │ │
│  └───────────────────────────────────────────────────────────────────────┘ │
│                                      │                                      │
│  INTERACTION LAYER                   ▼                                      │
│  ┌───────────────────────────────────────────────────────────────────────┐ │
│  │ Combat System • Target Selection • Player Input • Camera             │ │
│  └───────────────────────────────────────────────────────────────────────┘ │
│                                      │                                      │
│  PRESENTATION LAYER                  ▼                                      │
│  ┌───────────────────────────────────────────────────────────────────────┐ │
│  │ UI Systems • Rendering Pipeline • Audio • Debug Overlay              │ │
│  └───────────────────────────────────────────────────────────────────────┘ │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Event System Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          EVENT SYSTEM ARCHITECTURE                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  EVENT PRODUCERS                       EVENT BUS                           │
│  ┌─────────────────────┐              ┌─────────────────────┐              │
│  │ • Player Input      │              │ Bevy Event System   │              │
│  │ • Combat System     │──────────────►│                     │              │
│  │ • Quest System      │              │ • Event Queue       │              │
│  │ • Dialogue System   │              │ • Event Routing     │              │
│  │ • Physics System    │              │ • Event Filtering   │              │
│  └─────────────────────┘              └─────────────────────┘              │
│                                                  │                          │
│                                                  ▼                          │
│  EVENT TYPES                           EVENT CONSUMERS                      │
│  ┌─────────────────────┐              ┌─────────────────────┐              │
│  │ • CombatEvent       │              │ • UI Systems        │              │
│  │ • QuestEvent        │              │ • Audio Systems     │              │
│  │ • DialogueEvent     │              │ • Animation Systems │              │
│  │ • MovementEvent     │◄─────────────│ • Debug Systems     │              │
│  │ • InteractionEvent  │              │ • State Managers    │              │
│  │ • DebugEvent        │              │ • Progression       │              │
│  └─────────────────────┘              └─────────────────────┘              │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Memory and Performance Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    MEMORY AND PERFORMANCE ARCHITECTURE                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  COMPONENT STORAGE                     SYSTEM SCHEDULING                    │
│  ┌─────────────────────┐              ┌─────────────────────┐              │
│  │ ECS Archetype       │              │ Parallel Systems    │              │
│  │ Storage             │              │                     │              │
│  │                     │              │ • Movement          │              │
│  │ • Cache Locality    │              │ • Physics           │              │
│  │ • Memory Pools      │              │ • Animation         │              │
│  │ • Component Arrays  │              │ • Rendering         │              │
│  └─────────────────────┘              └─────────────────────┘              │
│             │                                    │                          │
│             ▼                                    ▼                          │
│  ┌─────────────────────┐              ┌─────────────────────┐              │
│  │ Query Optimization  │              │ Resource Management │              │
│  │                     │              │                     │              │
│  │ • Entity Filtering  │              │ • Asset Loading     │              │
│  │ • Component Access  │              │ • Memory Recycling  │              │
│  │ • Change Detection  │              │ • Garbage Collection│              │
│  └─────────────────────┘              └─────────────────────┘              │
│                                                  │                          │
│                                                  ▼                          │
│                                        ┌─────────────────────┐              │
│                                        │ Debug Performance   │              │
│                                        │ Monitoring          │              │
│                                        │                     │              │
│                                        │ • Frame Timing      │              │
│                                        │ • Memory Usage      │              │
│                                        │ • System Profiling  │              │
│                                        └─────────────────────┘              │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Configuration System Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      CONFIGURATION SYSTEM ARCHITECTURE                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  JSON SOURCE FILES                     LOADING PIPELINE                    │
│  ┌─────────────────────┐              ┌─────────────────────┐              │
│  │ config/             │              │ Configuration       │              │
│  │ ├── quests.json     │──────────────►│ Loader              │              │
│  │ ├── dialogue.json   │              │                     │              │
│  │ ├── skills.json     │              │ • File Reading      │              │
│  │ ├── debug.json      │              │ • JSON Parsing      │              │
│  │ └── assets.json     │              │ • Validation        │              │
│  └─────────────────────┘              │ • Error Handling    │              │
│                                        └─────────────────────┘              │
│                                                  │                          │
│                                                  ▼                          │
│  VALIDATION LAYER                      RUNTIME RESOURCES                    │
│  ┌─────────────────────┐              ┌─────────────────────┐              │
│  │ Schema Validation   │              │ • QuestDatabase     │              │
│  │                     │              │ • DialogueDatabase  │              │
│  │ • Type Checking     │◄─────────────│ • SkillDefinitions  │              │
│  │ • Required Fields   │              │ • GameDebugConfig   │              │
│  │ • Cross References  │              │ • AssetConfig       │              │
│  │ • Business Rules    │              └─────────────────────┘              │
│  └─────────────────────┘                        │                          │
│             │                                    ▼                          │
│             ▼                          ┌─────────────────────┐              │
│  ┌─────────────────────┐              │ Hot Reload System   │              │
│  │ Error Reporting     │              │                     │              │
│  │                     │              │ • File Watching     │              │
│  │ • Parse Errors      │              │ • Change Detection  │              │
│  │ • Validation Fails  │              │ • Runtime Update    │              │
│  │ • Missing Files     │              │ • Fallback Handling │              │
│  │ • Clear Messages    │              └─────────────────────┘              │
│  └─────────────────────┘                                                   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Summary

These diagrams illustrate the sophisticated architecture of Eryndor's game systems:

1. **ECS Foundation**: Pure Entity-Component-System architecture with clear separation of data and logic
2. **Event-Driven Communication**: Systems communicate through Bevy's event system for loose coupling
3. **Configuration-Driven Design**: JSON files control game behavior with comprehensive validation
4. **Layered Architecture**: Clear dependency hierarchy from foundational to presentation layers
5. **Performance-Conscious Design**: ECS archetype storage and parallel system execution
6. **Sophisticated Narrative Systems**: Evidence-based quests and personality-driven dialogue
7. **Modular Debug System**: Category-based debugging with performance controls

The architecture supports the game's narrative-first philosophy while maintaining excellent performance and maintainability through strict adherence to SOLID principles and ECS patterns.