# Eryndor Dev Console

A comprehensive developer console system for the Eryndor game engine, providing WoW-style Game Master commands for testing, debugging, and content creation.

## Quick Start

### Opening the Console
Press the **backtick key (`)** while in-game to toggle the console.

### Basic Commands
```bash
# Get help
help

# Teleport to coordinates  
teleport 0 15 0

# Spawn an NPC
create npc merchant "Bob the Trader" 10 15 5

# Clear console output
clear
```

## Table of Contents

1. [Installation & Setup](#installation--setup)
2. [Console Interface](#console-interface)
3. [Command Categories](#command-categories)
4. [Complete Command Reference](#complete-command-reference)
5. [Test Zones](#test-zones)
6. [Advanced Usage](#advanced-usage)
7. [Troubleshooting](#troubleshooting)
8. [Development](#development)

## Installation & Setup

The dev console is automatically included when you add the `EryndorDevConsolePlugin` to your game:

```rust
use eryndor_dev_console::prelude::*;

App::new()
    .add_plugins(EryndorDevConsolePlugin)
    .run();
```

### Dependencies
- Bevy 0.16+
- eryndor-core
- eryndor-dialogue
- eryndor-config

## Console Interface

### Visual Layout
```
┌─────────────────────────────────────────────────────┐
│ Console Output Area (scrollable)                    │
│ > teleport 0 15 0                                   │
│ Teleported to (0.0, 15.0, 0.0)                     │
│ > create npc merchant Bob                           │
│ Queued Merchant NPC 'Bob' for spawn                │
│                                                     │
├─────────────────────────────────────────────────────┤
│ > █                                                 │
└─────────────────────────────────────────────────────┘
```

### Controls
- **`** (Backtick): Toggle console visibility
- **Enter**: Execute command
- **↑/↓ Arrow Keys**: Navigate command history
- **←/→ Arrow Keys**: Move cursor in input line
- **Backspace**: Delete character before cursor
- **Delete**: Delete character after cursor
- **Home**: Move cursor to start of line
- **End**: Move cursor to end of line

### Output Colors
- **Blue**: Command input (commands you typed)
- **White**: Normal output
- **Red**: Errors
- **Yellow**: Warnings  
- **Green**: System messages

## Command Categories

### Movement Commands
Fast travel and player positioning.

### Debug Commands  
Toggle debug systems and developer features.

### Entity Commands
Spawn and manage NPCs and other game entities.

### Zone Commands
Load test zones and manage game areas.

### General Commands
Basic console operations and help.

## Complete Command Reference

### General Commands

#### `help` / `?`
Display available commands or get help for specific commands.

```bash
# Show all commands
help

# Get help for specific command (future feature)
help teleport
```

#### `clear` / `cls`
Clear the console output.

```bash
clear
```

### Movement Commands

#### `teleport` / `tp`
Teleport to specific coordinates.

```bash
# Teleport to coordinates
teleport <x> <y> <z>
teleport 0 15 0

# Examples
teleport -70 15 -70    # Spawn area
teleport 0 100 0       # High in the sky
```

#### `goto` / `warp`  
Quick teleport to named locations.

```bash
goto <location>

# Available locations:
goto spawn      # Player spawn area (-70, 15, -70)
goto test-zone  # Test zone (0, 10, 0)
goto origin     # World origin (0, 0, 0)
goto sky        # High altitude (0, 100, 0)
```

#### `speed` / `setspeed`
Modify player movement speed.

```bash
speed <multiplier>

# Examples
speed 1.0    # Normal speed
speed 2.0    # Double speed
speed 0.5    # Half speed
speed 5.0    # Very fast (for testing)
```

#### `fly` *(Placeholder)*
Toggle flight mode (not yet implemented).

#### `noclip` / `ghost` *(Placeholder)*
Toggle collision detection (not yet implemented).

### Debug Commands

#### `debug` / `dbg`
Toggle various debug systems.

```bash
debug <system> <on|off>

# Available systems:
debug collision on     # Show collision boundaries
debug input off        # Hide input debug info
debug animation on     # Show animation states
debug quest on         # Show quest debug info
debug physics on       # Show physics debug info
debug all off          # Disable all debug systems
```

#### `godmode` / `god`
Toggle invincibility mode.

```bash
godmode    # Toggle god mode on/off
```

#### `showfps` / `fps`
Toggle FPS counter display.

```bash
showfps    # Toggle FPS display
```

#### `reload`
Hot-reload game data during development.

```bash
reload <type>

# Available types:
reload dialogues    # Reload dialogue files
reload assets      # Reload game assets (planned)
```

### Entity Commands

#### `create` / `spawn`
Create entities in the game world.

```bash
# Spawn NPC with default name and position
create npc <type>

# Spawn NPC with custom name
create npc <type> <name>

# Spawn NPC with custom name and position  
create npc <type> <name> <x> <y> <z>

# NPC Types:
# - merchant: Gold-colored traders
# - guard: Blue-colored security
# - villager: Green-colored civilians  
# - noble: Purple-colored aristocrats
# - questgiver: Orange-colored quest NPCs
# - informant: Gray-colored info brokers

# Examples:
create npc merchant                           # Default merchant
create npc guard "Captain Smith"              # Named guard
create npc villager "Mary Baker" 10 15 5     # Baker at specific location
```

#### `delete` / `remove`
Remove entities from the world.

```bash
delete <type> <name|id>

# Examples:
delete npc "Captain Smith"    # Delete by name
delete npc Merchant_123       # Delete by ID
```

#### `list` / `ls`
List entities in the world.

```bash
list <type>

# Available types:
list npcs        # List all NPCs
list players     # List all players (multiplayer)
```

### Zone Commands

#### `zone`
Manage test zones and game areas.

```bash
zone <action> [parameters]

# Available actions:
zone list              # List available zones
zone load <name>       # Load a test zone
zone save <name>       # Save current area as zone (planned)
zone create <name>     # Create new zone (planned)

# Examples:
zone list
zone load test-zone
zone load npc-showcase
```

## Test Zones

Pre-configured testing environments for different game systems.

### Available Test Zones

#### `test-zone`
Basic developer testing area with flat terrain.
- **Spawn Point**: (0, 10, 0)
- **Terrain**: Flat
- **NPCs**: Basic merchant, guard, and villager for testing
- **Use Case**: General functionality testing

#### `npc-showcase`
Showcase area demonstrating all NPC types and interactions.
- **Spawn Point**: (0, 15, 0) 
- **Terrain**: Flat
- **NPCs**: All 6 NPC types positioned around spawn
- **Use Case**: Dialogue and NPC system testing

### Creating Custom Zones

Zones are defined in JSON files in the `zones/` directory:

```json
{
  "name": "my-test-zone",
  "description": "Custom test area",
  "spawn_point": [0.0, 15.0, 0.0],
  "terrain_type": "flat",
  "npcs": [
    {
      "npc_type": "merchant", 
      "position": [10.0, 15.0, 0.0],
      "dialogue_id": "merchant_bob",
      "name": "Bob the Trader"
    }
  ],
  "environment": {
    "lighting": "golden_hour",
    "weather": "clear"
  }
}
```

## Advanced Usage

### Command Chaining
Commands are processed individually. For complex operations, use multiple commands:

```bash
clear
teleport 0 15 0
create npc merchant "Shop Keep" 5 15 0
create npc guard "Town Guard" -5 15 0
debug collision on
```

### Position Reference System
Understanding the coordinate system:
- **X-axis**: East (+) / West (-)
- **Y-axis**: Up (+) / Down (-)  
- **Z-axis**: North (+) / South (-)
- **Default spawn**: (-70, 15, -70)
- **World origin**: (0, 0, 0)

### NPC Color Coding
NPCs are visually distinct by type:
- 🟡 **Merchant**: Gold (trade/commerce)
- 🔵 **Guard**: Blue (security/law)
- 🟢 **Villager**: Green (civilian/common)
- 🟣 **Noble**: Purple (aristocracy/politics)  
- 🟠 **Questgiver**: Orange (quests/objectives)
- ⚫ **Informant**: Gray (information/secrets)

### Command History
The console remembers your last 100 commands:
- Use **↑** to recall previous commands
- Use **↓** to move forward in history
- Commands persist until you restart the game

## Troubleshooting

### Console Won't Open
- **Check Key**: Ensure you're pressing backtick (`) not apostrophe (')
- **Console Integration**: Verify `EryndorDevConsolePlugin` is added to your app
- **Build Issues**: Make sure the dev console crate compiled successfully

### Commands Not Working
- **Syntax**: Check command syntax with `help`
- **Case Sensitivity**: Commands are case-sensitive
- **Arguments**: Ensure you provide required arguments
- **Spaces**: Use spaces between arguments, not commas

### NPCs Not Spawning
- **Position**: Check if NPCs spawn inside terrain or objects
- **Types**: Verify you're using valid NPC types
- **Dependencies**: Ensure dialogue system is active
- **Console Output**: Look for error messages in console

### Performance Issues  
- **Entity Limits**: Avoid spawning too many entities at once
- **Debug Systems**: Turn off unused debug systems
- **Console History**: Clear console periodically with `clear`

### Common Error Messages

| Error | Solution |
|-------|----------|
| `Unknown command: 'xyz'` | Check spelling, use `help` to see available commands |
| `Invalid coordinates` | Use numeric values for x, y, z coordinates |
| `Invalid NPC type: 'xyz'` | Use valid NPC types: merchant, guard, villager, noble, questgiver, informant |
| `Usage: command <args>` | Check command syntax and provide required arguments |

## Development

### Adding New Commands

1. **Create Command Function**:
```rust
fn my_command(
    commands: &mut Commands,
    args: &[String],
    console_state: &mut ConsoleState,
    dev_mode_writer: &mut EventWriter<DevModeChanged>,
) -> CommandResult {
    // Implementation
    CommandResult::Success("Command executed".to_string())
}
```

2. **Register Command**:
```rust
registry.register(CommandDef {
    name: "mycommand".to_string(),
    description: "My custom command".to_string(),
    usage: "mycommand <arg>".to_string(),
    aliases: vec!["mc".to_string()],
    category: "Custom".to_string(),
    function: my_command,
});
```

### Architecture Overview

The dev console uses a component-based architecture:

- **Components**: Store command data for next frame execution
- **Systems**: Process components and execute commands
- **Events**: Communicate between systems
- **Resources**: Store console state and command registry

### Performance Considerations

- Commands are processed over multiple frames to avoid hitches
- Entity operations are queued and batched
- Console output is limited to prevent memory bloat
- Debug systems can be toggled to reduce overhead

### Contributing

To contribute to the dev console:

1. Follow Bevy ECS patterns
2. Use component-based command processing
3. Provide clear error messages
4. Update documentation for new commands
5. Test commands thoroughly

---

**Next Steps**: 
- Zone loading/saving system
- Command scripting and macros  
- Advanced entity management
- Multiplayer admin commands
- Plugin system for custom commands

For more information about Eryndor development, see the main project documentation.