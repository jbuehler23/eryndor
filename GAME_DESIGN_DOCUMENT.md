# Eryndor Game Design Document

## Game Overview

Eryndor is a fantasy MMORPG that implements classic progression mechanics with modern technical architecture. The game emphasizes skill-by-use character development, player-driven economy, and social interaction systems within a persistent multiplayer world.

### Core Design Principles

- **Skill-Based Progression**: Characters advance through practice rather than point allocation
- **Player Agency**: Flexible role system allows diverse character builds and playstyles  
- **Economic Interdependence**: Player-crafted items and services drive meaningful trade
- **Social Cooperation**: Group content requires coordination and communication
- **Technical Excellence**: Modern architecture supporting scalable multiplayer systems

## Character Progression System

### Skill-by-Use Mechanics
Characters develop 29 individual skills through practical application:
- Combat skills advance through weapon usage and spell casting
- Crafting skills improve through resource processing and item creation
- Utility skills develop through exploration and problem-solving activities

### Character Advancement
- **Character Level**: 1-50 range with experience from combat, quests, and achievements
- **Skill Levels**: 1-50 per skill with usage-based progression and diminishing returns
- **Loadout System**: Equipment combinations suggest archetypal roles without restrictions
- **Rested Experience**: 1.5x bonus when rested at designated locations

### Role Flexibility
The system provides archetypal guidance (Tank, Healer, DPS, Support, Utility) for group formation while allowing complete build freedom:
- Skills suggest multiple potential roles rather than enforcing single categories
- Players choose their group role based on current equipment and preferences
- Hybrid builds are supported and encouraged for versatility

## Combat System Architecture

### Target Selection
- Tab-targeting system for precise enemy selection
- Click-to-target functionality for mouse-based interaction
- Visual indicators for selected targets and combat state
- Target switching with priority logic for nearest/weakest enemies

### Combat Mechanics
- Auto-attack system with weapon-specific timing intervals
- Ability framework supporting hotbar skills with cooldowns and resource costs
- Damage calculation based on weapon stats, character skills, and target defenses
- Status effects system for buffs, debuffs, and damage over time

### Damage Types and Resistances
13 damage classifications create tactical depth:
- **Physical**: Slashing, Piercing, Bludgeoning
- **Elemental**: Fire, Ice, Lightning  
- **Mystical**: Shadow, Nature, Arcane
- **Special**: Healing, Psychic, Holy, Necrotic

## Equipment and Inventory System

### Item Categories
- **Weapons**: 28 types across melee, ranged, and magical classifications
- **Armor**: Heavy, Medium, and Light categories with associated skills
- **Consumables**: Potions, food, and temporary enhancement items
- **Materials**: Resources for crafting and trading systems

### Equipment Mechanics
- Stat modification based on item quality and character skill levels
- Durability system requiring maintenance and repair
- Visual representation of equipped items on character models
- Socket system for item enhancement and customization

### Inventory Management
- Grid-based interface with drag-and-drop functionality
- Weight and space limitations creating resource management decisions
- Equipment slots with stat preview and comparison tooltips
- Storage expansion through containers and housing systems

## World Systems

### Terrain and Environment
- Procedural terrain generation with varied biomes and elevations
- Dynamic weather affecting visibility and gameplay mechanics
- Day/night cycles with gameplay implications for certain activities
- Interactive objects including doors, containers, and resource nodes

### NPC Systems
- Static NPCs for vendor, trainer, and quest giver functionality
- Basic AI behavior trees for movement patterns and interactions
- Dialogue system with branching conversation options
- Reputation system affecting NPC interactions and available services

### Resource Distribution
- Mining nodes for metal and gem extraction
- Herb spawns for alchemical ingredient gathering
- Fishing locations with varied catch types and rarity
- Respawn timers creating competition and economic value

## Economic Systems

### Crafting Mechanics
- Multi-step processes requiring multiple skills and materials
- Quality variations based on character skill levels and tool quality
- Recipe discovery through experimentation and NPC training
- Specialization benefits encouraging focused character development

### Trading Infrastructure
- Player-to-player trading with secure exchange windows
- NPC vendors with regional price variations and limited inventories
- Auction house system for server-wide marketplace functionality
- Economic balancing through resource scarcity and demand mechanics

### Currency and Value
- Primary currency earned through quest completion and monster elimination
- Secondary currencies from faction reputation and achievement systems
- Barter economy supporting direct item exchange
- Economic sinks preventing inflation through repair costs and taxes

## Social Systems

### Communication
- Local chat with proximity-based range limitations
- Global channels for server-wide announcements and coordination
- Private messaging system with offline message storage
- Guild channels for organization-specific communication

### Group Formation
- Party system supporting 2-6 players with shared experience and loot
- Role-based matching suggestions without mandatory restrictions
- Leadership mechanics for party coordination and decision-making
- Group objectives requiring coordination and tactical planning

### Guild Organizations
- Member management with customizable rank structures
- Shared resources including banks and housing facilities
- Group objectives and achievements for collective progression
- Inter-guild relationships supporting alliances and conflicts

## Technical Architecture

### Engine and Framework
- **Bevy 0.16**: Entity-Component-System architecture for modular development
- **Avian 3D 0.3**: Physics simulation and collision detection
- **Lightyear 0.18**: Client-server networking with rollback netcode
- **SQLx**: Database abstraction supporting SQLite and PostgreSQL

### Data Management
- JSON configuration files for skills, weapons, and game balance
- Component serialization for save/load functionality
- Database schema supporting player progression and world persistence
- Migration system for content updates and balance changes

### Performance Optimization
- ECS parallelization for system execution efficiency
- Asset streaming and memory management for large world support
- Network optimization through state synchronization and prediction
- Rendering optimizations supporting varied hardware configurations

## Quality Assurance

### Testing Framework
- Unit tests for progression calculations and game mechanics
- Integration tests for system interactions and data consistency
- Performance benchmarks maintaining 60 FPS on target hardware
- Load testing for multiplayer capacity and server stability

### Balance Methodology
- Playtesting sessions with diverse player groups and skill levels
- Statistical analysis of progression rates and economic activity
- Iterative balance adjustments based on player behavior data
- Community feedback integration for long-term game health

## Implementation Roadmap

### Phase 2 Completion (Current Priority)
- Combat system with targeting, auto-attack, and abilities
- Inventory management with equipment and item systems
- NPC interactions including vendors and dialogue
- Audio integration for environmental and combat feedback

### Phase 3 Development (Next Quarter)
- Quest system with objective tracking and reward distribution
- Crafting mechanics with resource gathering and item creation
- Economic foundation including vendors and trading systems
- World interactions supporting containers and environmental objects

### Phase 4 Planning (Multiplayer Foundation)
- Client-server networking implementation with Lightyear
- Player synchronization for movement, combat, and interactions
- Chat system with multiple channel support
- Basic guild functionality with member management

### Long-term Goals (Post-Launch)
- Advanced content including dungeons and raid encounters
- PvP systems with battlegrounds and competitive mechanics
- Housing and customization systems for personal spaces
- Regular content updates with new areas, skills, and equipment

## Success Metrics

### Player Engagement
- Average session duration exceeding 90 minutes
- Character progression milestones reached within expected timeframes
- Social interaction frequency indicating community formation
- Content completion rates across different player demographics

### Technical Performance
- Frame rate stability maintaining 60 FPS minimum
- Network latency under 100ms for responsive gameplay
- Server capacity supporting 100+ concurrent players
- Database performance handling persistent world state

### Economic Health
- Balanced resource distribution preventing extreme scarcity or abundance
- Active trading markets with reasonable price stability
- Crafting system utilization across all available professions
- Currency circulation maintaining economic activity

The game design emphasizes sustainable gameplay loops that encourage both individual progression and social interaction, creating a foundation for long-term player engagement and community development within a persistent multiplayer environment.