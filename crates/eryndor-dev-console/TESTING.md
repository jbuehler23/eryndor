# Dev Console Testing Guide

Quick testing checklist for the Eryndor Dev Console functionality.

## Prerequisites

1. **Game State**: Make sure game defaults to `InGame` state (not MainMenu)
   - File: `src/states/mod.rs` line 7-8 should have `#[default]` above `InGame`

2. **Build**: Ensure clean build
   ```bash
   cargo build
   ```

## Basic Console Tests

### 1. Console Toggle Test
1. Run: `cargo run --bin eryndor`
2. **Expected**: 3D world should render (terrain, lighting)
3. Press **backtick (`)** key
4. **Expected**: Console UI appears at top half of screen
5. Press **backtick (`)** again
6. **Expected**: Console disappears

### 2. Basic Commands Test
```bash
# Open console and test help
help

# Test clear
clear

# Test basic output
teleport 0 15 0
```

### 3. Character Input Test
1. Open console
2. Type: `hello world 123!@#`
3. **Expected**: All characters appear correctly
4. Use **arrow keys** to move cursor
5. Use **backspace/delete** to edit text
6. **Expected**: Cursor moves and text edits work

### 4. Command History Test
1. Type and execute: `teleport 0 15 0`
2. Type and execute: `help`
3. Type and execute: `clear`
4. Press **↑** arrow key multiple times
5. **Expected**: Previous commands appear in reverse order
6. Press **↓** arrow key
7. **Expected**: Move forward through history

## Movement Commands Test

### 5. Teleport Test
```bash
# Test coordinate teleportation
teleport 0 50 0    # Should go high in air
teleport -70 15 -70  # Back to spawn

# Test named locations
goto origin
goto sky
goto spawn
goto test-zone
```

### 6. Speed Test
```bash
# Test speed changes
speed 2.0    # Double speed
speed 0.5    # Half speed  
speed 1.0    # Normal speed
```

## Entity Commands Test

### 7. NPC Creation Test
```bash
# Test basic NPC spawning
create npc merchant
create npc guard "Captain Smith"
create npc villager "Mary Baker" 10 15 5

# Test all NPC types
create npc merchant Bob 5 15 0
create npc guard Captain -5 15 0
create npc villager Mary 0 15 5
create npc noble Lord -5 15 -5
create npc questgiver Elder 5 15 -5
create npc informant Pete 0 15 -10
```

**Expected Results**:
- Each NPC appears as colored capsule
- Merchant = Gold, Guard = Blue, Villager = Green
- Noble = Purple, Questgiver = Orange, Informant = Gray
- NPCs spawn at specified positions

### 8. NPC Deletion Test
```bash
# Delete by name
delete npc "Captain Smith"
delete npc Bob

# List remaining NPCs (should work but shows placeholder)
list npcs
```

## Zone System Test

### 9. Zone Listing Test
```bash
# List available zones
zone list
```
**Expected**: Shows `test-zone` and `npc-showcase`

### 10. Zone Loading Test
```bash
# Load test zone
zone load test-zone
```
**Expected**: 
- Message: "Queued zone 'test-zone' for loading..."
- NPCs from zone should spawn (merchant, guard, villager)
- Player should see new NPCs in world

```bash
# Load NPC showcase zone
zone load npc-showcase
```
**Expected**:
- Previous NPCs cleared
- All 6 NPC types spawn around origin
- Different colors visible for each type

```bash
# Clear zone
zone clear
```
**Expected**: All NPCs disappear

## Debug Commands Test

### 11. Debug Systems Test
```bash
# Test debug toggles
debug collision on
debug collision off
godmode
showfps
```

## Error Handling Test

### 12. Invalid Commands Test
```bash
# Test error messages
invalidcommand
teleport
teleport abc def
create npc invalidtype
zone load nonexistent
```
**Expected**: Clear error messages with usage hints

## Performance Test

### 13. Console Performance Test
1. Spawn many NPCs:
```bash
create npc merchant Bob1 0 15 0
create npc guard Guard1 1 15 0
create npc villager Mary1 2 15 0
# ... repeat 10-20 times with different positions
```

2. Load zones multiple times:
```bash
zone load test-zone
zone load npc-showcase
zone clear
zone load test-zone
```

**Expected**: Game remains responsive, no lag spikes

## Integration Test

### 14. Complete Workflow Test
```bash
# Complete testing workflow
clear
teleport 0 15 0
debug collision on
create npc merchant "Shop Keep" 5 15 0
zone load npc-showcase
goto sky
speed 2.0
teleport 0 15 0
zone clear
create npc guard Test 0 15 5
delete npc Test
clear
```

**Expected**: All commands execute without errors

## Troubleshooting

### Common Issues:

1. **Console won't open**: Check that backtick (`) key is pressed, not apostrophe (')
2. **Blank screen**: Verify game state defaults to `InGame` in states/mod.rs
3. **NPCs not spawning**: Check console output for error messages
4. **Zone loading fails**: Ensure zone JSON files exist in `crates/eryndor-dev-console/zones/`
5. **Commands not working**: Use `help` to verify command syntax

### Debug Information:

- Console output shows in terminal where you ran `cargo run`
- Error messages appear in both console UI and terminal
- Use `clear` command to clean console output when testing

## Success Criteria

✅ Console toggles with backtick key  
✅ All command categories work (movement, entity, zone, debug)  
✅ Character input and editing functions properly  
✅ Command history navigation works  
✅ NPC spawning with correct colors and positions  
✅ Zone loading spawns correct NPCs  
✅ Error handling provides helpful messages  
✅ Game remains stable during extended console use

## Next Steps

After basic testing passes:
1. Test dialogue system integration with spawned NPCs
2. Test console with different game states
3. Performance testing with large numbers of entities
4. Test console during multiplayer sessions (future)

---

**Note**: This testing assumes the game renders properly (3D world visible). If you see a blank screen, the game may be stuck in MainMenu state - see Prerequisites section.