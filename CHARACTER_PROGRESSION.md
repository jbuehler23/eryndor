# Character Progression System

## Overview

Eryndor implements a skill-by-use character progression system where skills advance through practice rather than point allocation. Characters develop naturally through gameplay - using swords improves swordsmanship, casting fire spells advances fire magic, wearing armor builds armor proficiency.

## Core Mechanics

### Character Level
- **Range**: 1-50
- **Experience Sources**: Combat, quest completion, exploration, crafting achievements
- **Benefits**: Equipment access, zone unlocking, trainer availability
- **Formula**: `level^2 * 100 + level * 200` (Level 1 = 0 XP, Level 2 = 500 XP)

### Skill Progression  
- **Range**: 1-50 per skill
- **Advancement**: Usage-based progression with diminishing returns
- **Formula**: `level^1.8 * 50 + level * 25`
- **Rested Bonus**: 1.5x experience when rested at inns/campfires

### Loadout System
- **Components**: Primary weapon, secondary item, armor type, active abilities
- **Player Choice**: Players can identify with any role based on their playstyle
- **Switching**: Only allowed at designated rest points (inns, campfires)
- **Capacity**: Multiple saved loadouts unlock with character progression

## Skills Reference

### Melee Weapon Skills
| Skill | Weapons | Description |
|-------|---------|-------------|
| Swordsmanship | Sword, Two-Handed Sword | Balanced offense/defense combat |
| Axe Mastery | Axe, Two-Handed Axe | High damage, slower attacks |
| Mace Skill | Mace, Two-Handed Mace | Armor penetration focus |
| Hammer Skill | Hammer, Two-Handed Hammer | Crushing damage specialist |
| Spear Mastery | Spear, Pike | Reach advantage combat |
| Shield Defense | Shield | Defensive capabilities |

### Ranged Weapon Skills
| Skill | Weapons | Description |
|-------|---------|-------------|
| Archery | Bow, Longbow | Long-range precision |
| Crossbow | Crossbow, Hand Crossbow | Mechanical ranged weapons |
| Dagger Mastery | Dagger | Fast, precise strikes |
| Throwing Weapons | Throwing Knife, Throwing Axe, Javelin | Thrown projectile combat |

### Magic Schools
| Skill | Weapons | Description |
|-------|---------|-------------|
| Fire Magic | Fire Staff | Elemental fire spells |
| Ice Magic | Ice Staff | Elemental ice spells |
| Lightning Magic | Lightning Staff | Elemental lightning spells |
| Shadow Magic | Shadow Staff | Dark mystical arts |
| Nature Magic | Nature Staff | Natural force manipulation |
| Arcane Magic | Arcane Staff, Magical Orb | Pure magical energy |
| Restoration | Restoration Staff | Healing and support magic |
| Divination | Divination Staff | Foresight and protection |

### Armor Skills
| Skill | Armor Types | Description |
|-------|-------------|-------------|
| Heavy Armor | Plate, Chainmail | Maximum protection |
| Medium Armor | Reinforced Leather | Balanced protection/mobility |
| Light Armor | Cloth, Leather | Mobility and mana efficiency |

### Crafting Skills
| Skill | Focus Area | Description |
|-------|-----------|-------------|
| Smithing | Weapons/Armor | Metalworking and repair |
| Alchemy | Potions/Reagents | Chemical transmutation |
| Enchanting | Magical Enhancement | Item enhancement |
| Cooking | Food/Buffs | Sustenance preparation |

### Utility Skills  
| Skill | Applications | Description |
|-------|-------------|-------------|
| Athletics | Movement/Stamina | Physical conditioning |
| Stealth | Concealment | Avoiding detection |
| Lockpicking | Entry/Containers | Mechanical manipulation |
| Pickpocketing | Theft/Sleight of Hand | Manual dexterity |

## Weapons Reference

### One-Handed Melee
- **Sword**: Balanced weapon, slashing damage
- **Axe**: High damage, slashing damage  
- **Mace**: Armor penetration, bludgeoning damage
- **Hammer**: Crushing attacks, bludgeoning damage
- **Dagger**: Fast attacks, piercing damage

### Two-Handed Melee
- **Two-Handed Sword**: Powerful strikes, slashing damage
- **Two-Handed Axe**: Heavy damage, slashing damage
- **Two-Handed Mace**: Massive impact, bludgeoning damage  
- **Two-Handed Hammer**: Devastating blows, bludgeoning damage
- **Spear/Pike**: Reach advantage, piercing damage

### Ranged Weapons
- **Bow/Longbow**: Traditional archery, piercing damage
- **Crossbow/Hand Crossbow**: Mechanical precision, piercing damage
- **Throwing Weapons**: Knife (piercing), Axe (slashing), Javelin (piercing)

### Magic Staves (All Two-Handed)
- **Elemental**: Fire, Ice, Lightning - respective elemental damage
- **Mystical**: Shadow, Nature, Arcane - mystical damage types
- **Support**: Restoration (healing), Divination (psychic)

### Secondary Items
- **Shield**: Defensive bonuses, bludgeoning damage when used offensively
- **Magical Orb**: Arcane damage, spell enhancement

## Damage Types

### Physical Damage
- **Slashing**: Swords, axes - effective against light armor
- **Piercing**: Spears, arrows, daggers - bypasses armor gaps
- **Bludgeoning**: Maces, hammers - effective against heavy armor

### Elemental Damage  
- **Fire**: Heat and combustion effects
- **Ice**: Freezing and slowing effects
- **Lightning**: Electrical and stunning effects

### Mystical Damage
- **Shadow**: Dark energy, fear effects
- **Nature**: Natural forces, poison/growth
- **Arcane**: Pure magical energy
- **Healing**: Restoration energy (negative damage)
- **Psychic**: Mental effects, confusion

### Future Expansion
- **Holy**: Divine magic (planned)
- **Necrotic**: Death magic (planned)

## Role System

Roles serve as archetypal guides and matchmaking categories for group content. Players are free to build their characters however they choose - roles simply help identify what you can contribute to a group.

### Tank
- **Group Function**: Damage mitigation and enemy control for group protection
- **Suggested Skills**: Shield Defense, Heavy Armor, plus any melee weapons
- **Build Examples**: Shield + Sword + Heavy Armor, or Two-handed + Heavy Armor
- **Matchmaking**: Essential for most group content requiring damage mitigation

### Healer  
- **Group Function**: Party support, health maintenance, and damage prevention
- **Suggested Skills**: Restoration, Divination, Light/Medium Armor
- **Build Examples**: Restoration Staff + Light Armor, or Hybrid Magic + Medium Armor
- **Matchmaking**: Essential for most group content requiring health support

### DPS (Damage Per Second)
- **Group Function**: Maximum damage output to eliminate enemies efficiently
- **Suggested Skills**: Any weapon skills, offensive magic, Medium Armor
- **Build Examples**: Any damage-focused weapon and armor combination
- **Matchmaking**: Multiple DPS typically needed for group content

### Support
- **Group Function**: Equipment crafting, consumable creation, utility services
- **Suggested Skills**: Smithing, Alchemy, Enchanting, Cooking
- **Build Examples**: Focused crafting specialization or multi-craft generalist
- **Matchmaking**: Optional but valuable for sustained group activities

### Utility
- **Group Function**: Exploration, locks, traps, scouting, alternative solutions
- **Suggested Skills**: Stealth, Athletics, Lockpicking, Pickpocketing
- **Build Examples**: Scout/thief builds focused on mobility and problem-solving
- **Matchmaking**: Situational - valuable for exploration and puzzle content

**Note**: These are suggestions, not restrictions. Players can queue for multiple roles if their build supports it, or create entirely unique hybrid builds.

## Experience Progression

### Character Level Experience Requirements
```
Level 1: 0 XP (starting level)
Level 2: 500 XP
Level 3: 1,500 XP  
Level 4: 2,800 XP
Level 5: 4,500 XP
...
Level 50: 270,500 XP (maximum)
```

### Skill Level Experience Requirements
```
Level 1: 0 XP (starting level)
Level 2: 75 XP
Level 3: 207 XP
Level 4: 375 XP
Level 5: 575 XP
...
Level 50: ~31,000 XP per skill
```

## Configuration System

The progression system uses JSON-based configuration files for complete extensibility:

- **Skills**: `config/skills.json` - Categories, roles, experience formulas
- **Weapons**: `config/weapons.json` - Associated skills, damage types, properties  
- **Damage Types**: `config/damage_types.json` - Resistances, effects, interactions
- **Roles**: `config/roles.json` - Skill requirements, equipment restrictions
- **Progression**: `config/progression.json` - Experience curves, unlock requirements

This data-driven approach enables rapid balancing, modding support, and feature expansion without code modifications.

## Debug Controls

**Note**: These are development/testing controls and will be removed in production.

- **Shift+F1**: Toggle character progression debug display
- **F1**: Toggle loadout switching availability (simulates rest points)
- **F2/F3**: Switch between loadouts (when at rest points)
- **F4**: Create new loadout based on highest role investment
- **Ctrl+F1**: Apply 5-minute rested bonus
- **Ctrl+Shift+F6**: Award 500 character experience for testing

---

*This document reflects the current implementation and will be updated as the system evolves.*