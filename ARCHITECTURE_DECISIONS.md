# Eryndor Architecture Decisions Record

## Overview

This document records the key architectural decisions made during Eryndor's development, providing context and rationale for technical choices that shape the game's implementation.

## Decision Records

### ADR-001: ECS Architecture with Bevy Engine
**Date**: Initial Development  
**Status**: ✅ Implemented  

**Context**: Need for a modern, performant game engine with good Rust ecosystem integration.

**Decision**: Adopt Bevy 0.16+ as the primary game engine with strict Entity-Component-System architecture.

**Rationale**:
- **Performance**: ECS provides excellent cache locality and parallelization opportunities
- **Modularity**: Clear separation of concerns between data (Components) and logic (Systems)
- **Rust Integration**: Bevy is built specifically for Rust, leveraging the language's strengths
- **Active Development**: Strong community and regular updates

**Consequences**:
- ✅ Excellent performance and scalability
- ✅ Clear system boundaries and testability
- ✅ Strong type safety and memory management
- ❌ Initial learning curve for ECS patterns
- ❌ Some integration complexity with non-ECS libraries

---

### ADR-002: Avian 3D for Physics Integration
**Date**: Early Development  
**Status**: ✅ Implemented  

**Context**: Need for 3D physics simulation for character movement and world interaction.

**Decision**: Use Avian 3D 0.3 as the physics engine, integrated with Bevy's ECS.

**Rationale**:
- **Future Bevy Integration**: Avian 3D is planned to become Bevy's official physics solution
- **ECS Native**: Designed specifically for ECS architectures
- **Performance**: Better performance characteristics than alternatives
- **Feature Set**: Comprehensive physics simulation suitable for MMORPG needs

**Consequences**:
- ✅ Excellent ECS integration and performance
- ✅ Future-proof choice aligned with Bevy's roadmap
- ✅ Comprehensive feature set for character controllers and world physics
- ❌ Relatively newer library with smaller ecosystem
- ❌ Some advanced features still in development

---

### ADR-003: JSON Configuration System
**Date**: Character Progression Implementation  
**Status**: ✅ Implemented  

**Context**: Need for flexible, designer-friendly configuration of game balance and content.

**Decision**: Implement comprehensive JSON-based configuration system with runtime validation.

**Rationale**:
- **Designer Accessibility**: JSON is human-readable and editable without programming knowledge
- **Version Control**: Text-based format works well with git and diff tools
- **Validation**: Serde provides excellent serialization with validation support
- **Hot Reloading**: JSON files can be reloaded without recompilation
- **External Tools**: Many tools exist for editing and validating JSON

**Consequences**:
- ✅ Easy balance tweaks and content updates
- ✅ Clear separation between code and configuration
- ✅ Designer-friendly workflow
- ✅ Strong validation and error reporting
- ❌ Larger file sizes compared to binary formats
- ❌ Runtime parsing overhead (mitigated by caching)

---

### ADR-004: Narrative-First Game Design
**Date**: Quest System Implementation  
**Status**: ✅ Implemented  

**Context**: Differentiation from traditional MMORPGs and focus on unique player experience.

**Decision**: Prioritize lore-heavy, investigation-based gameplay over traditional action-oriented mechanics.

**Rationale**:
- **Market Differentiation**: Unique positioning in MMORPG space
- **Accessibility**: Text-based gameplay accessible to wider audience
- **Depth**: Emphasizes critical thinking and reading comprehension
- **Community**: Encourages collaborative investigation and discussion
- **Sustainability**: Community-driven content creation reduces development costs

**Consequences**:
- ✅ Unique value proposition and market positioning
- ✅ Strong foundation for educational gaming elements
- ✅ Accessible gameplay for various physical abilities
- ✅ Community engagement through shared investigations
- ❌ Smaller initial target audience than action-focused games
- ❌ Higher content quality requirements for text-based systems

---

### ADR-005: Evidence-Based Quest Progression
**Date**: Quest System Architecture  
**Status**: ✅ Implemented  

**Context**: Need for engaging quest mechanics that support narrative-first design.

**Decision**: Implement evidence-based progression where players must gather and analyze clues to advance.

**Rationale**:
- **Player Agency**: Players actively participate in solving mysteries rather than following markers
- **Replayability**: Multiple solution paths and investigation approaches
- **Immersion**: No quest markers or hand-holding maintains world immersion
- **Skill Development**: Promotes reading comprehension and analytical thinking
- **Community Interaction**: Players can share discoveries and theories

**Consequences**:
- ✅ Highly engaging and immersive quest experience
- ✅ Strong educational value and critical thinking development
- ✅ Natural community building around shared investigations
- ✅ High replayability with multiple solution paths
- ❌ Higher development cost for complex quest content
- ❌ Potential frustration for players expecting traditional quest mechanics

---

### ADR-006: Component-Based Debug System
**Date**: Recent Development  
**Status**: ✅ Implemented  

**Context**: Need for comprehensive debug logging without performance impact in production.

**Decision**: Implement configurable debug system with category-based controls and intelligent timing.

**Rationale**:
- **Performance**: Only active debug categories impact performance
- **Granularity**: Different systems can be debugged independently
- **Developer Experience**: Keyboard shortcuts for quick debug toggling
- **Production Ready**: Easy to disable all debug output for release builds
- **Spam Prevention**: Timing controls prevent console flooding

**Consequences**:
- ✅ Excellent developer productivity and debugging capabilities
- ✅ No performance impact when debug categories are disabled
- ✅ Granular control over debug output
- ✅ Production-ready debug controls
- ❌ Additional complexity in system implementation
- ❌ Requires discipline to use debug categories consistently

---

### ADR-007: Personality-Driven NPC Dialogue
**Date**: Dialogue System Implementation  
**Status**: ✅ Implemented  

**Context**: Need for compelling NPC interactions that support narrative focus.

**Decision**: Implement sophisticated NPC personality system with emotional states and relationship tracking.

**Rationale**:
- **Immersion**: NPCs feel like real characters with distinct personalities
- **Replayability**: Different players may have different relationship outcomes
- **Narrative Depth**: Personality affects dialogue availability and content
- **Player Investment**: Relationship building creates emotional engagement
- **Investigation Support**: NPC personalities affect information sharing

**Consequences**:
- ✅ Highly immersive and engaging NPC interactions
- ✅ Strong support for investigation-based gameplay
- ✅ High replayability through relationship variations
- ✅ Foundation for complex social simulation
- ❌ High content creation cost for personality-specific dialogue
- ❌ Complex state management for NPC relationships

---

### ADR-008: Kinematic Character Controller
**Date**: Character Movement Implementation  
**Status**: ✅ Implemented  

**Context**: Need for precise character movement compatible with physics simulation.

**Decision**: Implement custom kinematic character controller with Avian 3D integration.

**Rationale**:
- **Precision**: Direct control over character movement without physics artifacts
- **Compatibility**: Works well with physics simulation for world interaction
- **Predictability**: Consistent movement behavior across different conditions
- **Network Ready**: Deterministic movement suitable for multiplayer
- **Customization**: Full control over movement mechanics and feel

**Consequences**:
- ✅ Precise and predictable character movement
- ✅ Excellent integration with physics world
- ✅ Ready for multiplayer networking
- ✅ Customizable movement feel and mechanics
- ❌ Higher implementation complexity than basic physics-based movement
- ❌ Requires manual handling of physics interactions

---

### ADR-009: Multi-Phase Quest Architecture
**Date**: Quest System Design  
**Status**: ✅ Implemented  

**Context**: Need for complex quest progression supporting investigation gameplay.

**Decision**: Implement multi-phase quest structure with dynamic unlocking based on evidence strength.

**Rationale**:
- **Complexity Support**: Enables sophisticated investigation storylines
- **Player Agency**: Progress unlocked by player deduction rather than simple completion
- **Flexibility**: Different players can approach investigations differently
- **Narrative Control**: Precise control over story pacing and revelation
- **Replayability**: Multiple paths through quest phases

**Consequences**:
- ✅ Sophisticated quest progression supporting complex narratives
- ✅ High player agency and multiple solution approaches
- ✅ Excellent support for investigation-based gameplay
- ✅ Strong foundation for consequence tracking
- ❌ Complex quest content creation and testing
- ❌ Potential for players to get stuck without proper guidance

---

### ADR-010: Skill-by-Use Progression System
**Date**: Character Progression Implementation  
**Status**: ✅ Implemented  

**Context**: Need for character progression that encourages diverse gameplay.

**Decision**: Implement skill-by-use system with 29 skills across 7 categories.

**Rationale**:
- **Natural Progression**: Skills improve through actual use rather than arbitrary point allocation
- **Player Choice**: No character builds are permanently locked out
- **Exploration Encouragement**: Players naturally try different activities
- **Balancing**: Usage-based progression self-balances through natural play patterns
- **Immersion**: Progression feels organic and realistic

**Consequences**:
- ✅ Natural and immersive character progression
- ✅ High player freedom and experimentation
- ✅ Self-balancing through usage patterns
- ✅ Encourages diverse gameplay exploration
- ❌ Complex balancing of skill gain rates
- ❌ Potential for grinding if not carefully designed

## Architectural Principles

### Core Principles

1. **ECS Purity**: Strict adherence to Entity-Component-System patterns
2. **Configuration Over Code**: Game balance and content in JSON files
3. **Narrative First**: All systems designed to support lore-heavy gameplay
4. **Player Agency**: Systems empower player choice and discovery
5. **Performance Consciousness**: Debug and development features with minimal production impact

### Design Patterns

1. **Event-Driven Architecture**: Systems communicate through Bevy's event system
2. **Component Composition**: Complex behaviors built from simple, focused components
3. **Resource Management**: Global state handled through Bevy resources
4. **Validation at Boundaries**: Input validation and error handling at system entry points
5. **Hot-Swappable Configuration**: Runtime configuration loading with fallback handling

### Quality Standards

1. **Testability**: All systems designed for unit and integration testing
2. **Modularity**: Clear boundaries between system responsibilities  
3. **Documentation**: Comprehensive documentation for complex systems
4. **Error Handling**: Graceful degradation and clear error messages
5. **Performance**: Regular profiling and optimization checkpoints

## Future Considerations

### Upcoming Decisions

1. **UI Framework Choice**: Selection of UI system for quest journal and inventory
2. **Audio Integration**: Choice of audio library and integration approach
3. **Networking Architecture**: Client-server implementation strategy
4. **Database Design**: Player progression and world state persistence
5. **Content Pipeline**: Tools and workflow for quest content creation

### Technical Debt

1. **Enhanced Character Controller**: Migration to enhanced controller system
2. **Animation Integration**: Better integration between combat and animation systems
3. **Performance Optimization**: LOD and culling systems for large worlds
4. **Testing Framework**: Comprehensive automated testing setup

## Conclusion

These architectural decisions have shaped Eryndor into a unique narrative-focused MMORPG with strong technical foundations. The emphasis on ECS architecture, configurable systems, and narrative-first design creates a solid platform for the innovative gameplay vision while maintaining performance and maintainability.

The decisions prioritize long-term maintainability, player experience, and technical excellence while supporting the game's core vision of thoughtful, lore-driven gameplay.