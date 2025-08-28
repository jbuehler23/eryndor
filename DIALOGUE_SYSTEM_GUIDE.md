# Eryndor Dialogue System User Guide

## Overview

The Eryndor dialogue system is a powerful, file-based conversation system inspired by YarnSpinner. It allows anyone to create rich, branching dialogues for NPCs without programming knowledge. All dialogues are stored as JSON files that can be easily edited and hot-reloaded during development.

## Quick Start

### 1. Basic Dialogue Structure

Every NPC dialogue is a JSON file in `config/dialogues/npcs/`. Here's the minimal structure:

```json
{
  "npc_id": "my_npc",
  "name": "Friendly Villager",
  "description": "A helpful person in the village",
  "default_conversation": "greeting",
  "conversations": {
    "greeting": {
      "title": "First Meeting",
      "nodes": {
        "start": {
          "speaker": "npc",
          "text": "Hello there, traveler!",
          "choices": [
            {
              "id": "hello",
              "text": "Hello!",
              "next": "friendly_response"
            },
            {
              "id": "bye",
              "text": "I have to go.",
              "next": "end_conversation"
            }
          ]
        },
        "friendly_response": {
          "speaker": "npc", 
          "text": "Nice to meet you! Welcome to our village.",
          "choices": [
            {
              "id": "thanks",
              "text": "Thank you for the welcome.",
              "next": "end_conversation"
            }
          ]
        }
      }
    }
  }
}
```

### 2. Creating Your First NPC

1. Create a new file: `config/dialogues/npcs/your_npc_name.json`
2. Copy the basic structure above
3. Modify the `npc_id`, `name`, and `description`
4. Write your dialogue nodes
5. Save the file - it will auto-reload in the game!

## Core Concepts

### NPCs and Conversations

- **NPC**: A character in the game with multiple conversations
- **Conversation**: A complete dialogue tree (like "greeting", "quest_start", "shop")  
- **Node**: A single piece of dialogue with response choices
- **Choice**: A player response that leads to another node

### Node Structure

Every dialogue node has:
- `speaker`: Who's talking ("npc" or "player")
- `text`: What they say
- `choices`: Array of possible responses (can be empty for end nodes)

### Choice Structure

Every choice has:
- `id`: Unique identifier for this choice
- `text`: What the player says
- `next`: Which node to go to next

## Advanced Features

### 1. Multiple Conversations

NPCs can have multiple conversation trees:

```json
{
  "npc_id": "merchant_bob",
  "name": "Merchant Bob",
  "default_conversation": "greeting",
  "conversations": {
    "greeting": {
      "title": "First Meeting",
      "nodes": { ... }
    },
    "shopping": {
      "title": "Browse Wares", 
      "nodes": { ... }
    },
    "quest_delivery": {
      "title": "Package Delivery",
      "nodes": { ... }
    }
  }
}
```

### 2. Quest Integration

Add quest actions to dialogue nodes:

```json
{
  "speaker": "npc",
  "text": "I have a job for you. Will you help?",
  "quest_action": {
    "type": "start_quest",
    "quest_id": "delivery_mission",
    "phase": "accepted"
  },
  "choices": [...]
}
```

Quest action types:
- `"start_quest"`: Begin a new quest
- `"quest_assigned"`: Mark quest as assigned  
- `"give_clue"`: Provide investigation clues

### 3. Clue System

NPCs can give clues for investigations:

```json
{
  "speaker": "npc",
  "text": "I saw someone suspicious near the old mill...",
  "clue_flags": ["suspicious_person_mill", "mill_location"],
  "choices": [...]
}
```

### 4. Skill Requirements

Lock dialogue choices behind skill levels:

```json
{
  "id": "intimidate",
  "text": "[Intimidation] Tell me what you know!",
  "next": "scared_response",
  "skill_requirements": [
    {
      "skill": "intimidation", 
      "level": 25
    }
  ]
}
```

### 5. Relationship Effects

Dialogue choices can affect NPC relationships:

```json
{
  "id": "generous_offer",
  "text": "Keep the extra coin as thanks.",
  "next": "grateful_response",
  "approach": "generous",
  "relationship_effects": {
    "trust": 5,
    "gratitude": 3
  }
}
```

### 6. Emotional Context

Add emotional context to dialogue:

```json
{
  "speaker": "npc",
  "text": "I... I can't tell you that.",
  "emotion": "nervous",
  "choices": [...]
}
```

## NPC Types and Behaviors

### NPC Types

Set the NPC type to influence default behaviors:

```json
{
  "npc_id": "town_guard",
  "name": "Guard Captain",
  "npc_type": "Guard",
  ...
}
```

Available types:
- `Merchant`: Trade-focused, business personality
- `Guard`: Authoritative, law enforcement
- `Villager`: Friendly, general information
- `Questgiver`: Quest-focused interactions
- `Noble`: Formal, high-status interactions
- `Informant`: Secretive, information broker

### Personality Traits

Define NPC personality for dynamic dialogue:

```json
{
  "personality_traits": {
    "nervous_disposition": 0.8,
    "merchant_instincts": 0.6,
    "guilt_burden": 0.3,
    "desperation_level": 0.4
  }
}
```

Traits range from 0.0 to 1.0 and affect dialogue presentation.

## File Organization

### Directory Structure

```
config/dialogues/
├── npcs/                    # Individual NPC dialogues
│   ├── merchant_aldric.json
│   ├── town_guard.json
│   └── village_elder.json
├── common/                  # Reusable dialogue components  
│   ├── greetings.json
│   └── farewells.json
└── quests/                  # Quest-specific dialogues
    ├── tutorial_quest.json
    └── main_storyline.json
```

### Naming Conventions

- **Files**: `snake_case.json` (e.g., `merchant_aldric.json`)
- **NPC IDs**: `snake_case` (e.g., `"merchant_aldric"`)
- **Conversation IDs**: `snake_case` (e.g., `"greeting"`, `"quest_start"`)
- **Node IDs**: `snake_case` (e.g., `"start"`, `"nervous_response"`)
- **Choice IDs**: `snake_case` (e.g., `"agree_to_help"`, `"ask_for_more_gold"`)

## Writing Guidelines

### 1. Dialogue Writing Tips

**Keep It Natural**
```json
// Good - sounds like how people talk
"text": "Look, I don't know much, but I saw something weird last night."

// Avoid - too formal/stiff
"text": "I must inform you that I witnessed an unusual occurrence yesterday evening."
```

**Show Personality**
```json
// Nervous merchant
"text": "Oh! Um, welcome to my shop. Don't mind the mess..."

// Confident guard  
"text": "State your business, citizen. Make it quick."
```

### 2. Branching Best Practices

**Meaningful Choices**
```json
"choices": [
  {
    "id": "help_freely",
    "text": "I'll help you for free.",
    "next": "grateful_response"
  },
  {
    "id": "demand_payment", 
    "text": "What's in it for me?",
    "next": "business_negotiation"
  },
  {
    "id": "decline_help",
    "text": "Sorry, I can't help right now.",
    "next": "disappointed_response"
  }
]
```

**Avoid Dead Ends**
```json
// Good - gives player options
{
  "speaker": "npc",
  "text": "I see. Well, if you change your mind, come back.",
  "choices": [
    {
      "id": "reconsider",
      "text": "Actually, maybe I can help.",
      "next": "start"
    },
    {
      "id": "leave",
      "text": "I'll think about it.",
      "next": "end_conversation"
    }
  ]
}

// Avoid - player stuck with no options
{
  "speaker": "npc",
  "text": "Fine, I don't need your help anyway.",
  "choices": []
}
```

### 3. Quest Integration

**Clear Quest Offers**
```json
{
  "speaker": "npc",
  "text": "My delivery cart broke down outside town. Could you bring this package to the blacksmith?",
  "quest_action": {
    "type": "start_quest",
    "quest_id": "delivery_quest",
    "items": ["mysterious_package"]
  },
  "choices": [
    {
      "id": "accept_delivery",
      "text": "I'll deliver it for you.",
      "next": "quest_accepted"
    },
    {
      "id": "ask_payment",
      "text": "What's the reward?",
      "next": "payment_discussion"
    }
  ]
}
```

## Game Controls

### Player Controls
- **E**: Start conversation with nearby NPC / End current conversation
- **1-4**: Select dialogue choice during conversation
- **F1**: Show help and controls
- **F11**: Debug NPC information (development)

### Developer Features
- **F10**: Hot-reload dialogue files (automatically detects changes)
- Real-time dialogue validation and error reporting
- Debug logging for dialogue flow and state changes

## Common Patterns

### 1. Shop Interaction
```json
{
  "speaker": "npc",
  "text": "Welcome to my shop! What can I get for you?",
  "choices": [
    {
      "id": "browse_wares",
      "text": "Show me what you have.",
      "next": "shop_menu"
    },
    {
      "id": "ask_special_items",
      "text": "Do you have anything special?",
      "next": "special_inventory"
    },
    {
      "id": "just_looking",
      "text": "Just browsing, thanks.",
      "next": "end_conversation"
    }
  ]
}
```

### 2. Information Gathering
```json
{
  "speaker": "npc",  
  "text": "You're asking about strange happenings? Well...",
  "choices": [
    {
      "id": "press_for_details",
      "text": "Please, any information would help.",
      "next": "reluctant_information"
    },
    {
      "id": "offer_payment",
      "text": "I can pay for good information.",
      "next": "paid_information"
    },
    {
      "id": "intimidate",
      "text": "[Intimidation] Tell me what you know.",
      "next": "scared_revelation",
      "skill_requirements": [{"skill": "intimidation", "level": 20}]
    }
  ]
}
```

### 3. Quest Follow-up
```json
{
  "speaker": "npc",
  "text": "Did you deliver that package to the blacksmith?",
  "choices": [
    {
      "id": "quest_complete",
      "text": "Yes, it's delivered safely.",
      "next": "quest_reward",
      "quest_action": {
        "type": "complete_quest",
        "quest_id": "delivery_quest"
      }
    },
    {
      "id": "still_working",
      "text": "I'm still working on it.",
      "next": "patience_response"
    }
  ]
}
```

## Troubleshooting

### Common Issues

**Dialogue Not Loading**
- Check JSON syntax with a validator
- Ensure file is in correct directory (`config/dialogues/npcs/`)
- Verify `npc_id` matches filename (without .json)

**NPC Not Responding**
- Make sure NPC is spawned in game
- Check that `npc_id` in dialogue file matches spawned NPC
- Verify `default_conversation` exists

**Choices Not Appearing**
- Ensure `choices` array has valid entries
- Check that `next` node IDs exist in the conversation
- Verify choice text is not empty

**Quest Actions Not Working**
- Confirm quest system is initialized
- Check quest IDs match those in quest database
- Verify quest action syntax

### Validation Errors

The system provides helpful error messages:
- Missing required fields
- Invalid node references  
- Circular conversation loops
- Malformed JSON syntax

## Examples

### Complete Merchant Example
```json
{
  "npc_id": "weapon_smith",
  "name": "Gareth the Smith",
  "description": "A burly blacksmith with calloused hands",
  "default_conversation": "greeting",
  "conversations": {
    "greeting": {
      "title": "Meeting the Smith",
      "nodes": {
        "start": {
          "speaker": "npc",
          "text": "Welcome to my forge! Best weapons in the kingdom.",
          "emotion": "proud",
          "choices": [
            {
              "id": "browse_weapons",
              "text": "I'd like to see your weapons.",
              "next": "show_weapons"
            },
            {
              "id": "custom_order",
              "text": "Can you make something custom?",
              "next": "custom_work"
            },
            {
              "id": "just_looking",
              "text": "Just looking around.",
              "next": "browsing_response"
            }
          ]
        },
        "show_weapons": {
          "speaker": "npc",
          "text": "Here's what I have in stock. Each piece is crafted with care.",
          "choices": [
            {
              "id": "buy_sword",
              "text": "I'll take this sword.",
              "next": "sword_purchase"
            },
            {
              "id": "too_expensive",
              "text": "These prices are too high.",
              "next": "price_negotiation"
            },
            {
              "id": "maybe_later",
              "text": "Let me think about it.",
              "next": "end_conversation"
            }
          ]
        },
        "custom_work": {
          "speaker": "npc",
          "text": "Aye, I can craft something special. What did you have in mind?",
          "quest_action": {
            "type": "start_quest",
            "quest_id": "custom_weapon_order"
          },
          "choices": [
            {
              "id": "magic_sword",
              "text": "A sword that can channel magic.",
              "next": "complex_order",
              "skill_requirements": [
                {"skill": "enchanting", "level": 15}
              ]
            },
            {
              "id": "simple_request",
              "text": "Just a well-balanced blade.",
              "next": "simple_order"
            }
          ]
        }
      }
    }
  },
  "relationship_effects": {
    "trust_building": {
      "respectful": 2,
      "appreciative": 3
    },
    "trust_damaging": {
      "rude": -2,
      "dismissive": -1
    }
  },
  "personality_traits": {
    "craftsman_pride": 0.9,
    "business_sense": 0.6,
    "straightforward": 0.8
  }
}
```

This guide should help anyone create rich, engaging dialogues for the Eryndor game. The system is designed to be accessible to writers while providing the depth needed for complex narrative experiences.