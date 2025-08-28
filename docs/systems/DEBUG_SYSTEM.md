# Debug System Documentation

## Overview

The Eryndor Debug System provides comprehensive, configurable debugging capabilities designed for both development productivity and production readiness. Unlike traditional debug systems that are either always-on or always-off, this system offers granular control over debug categories with intelligent spam prevention and performance optimization.

## Core Philosophy

### Developer-Centric Design
- **Granular Control**: Individual debug categories can be toggled independently
- **Performance Conscious**: Minimal impact when debug categories are disabled
- **Spam Prevention**: Intelligent timing controls prevent console flooding
- **Context Aware**: Debug output respects game state (movement-only mode)
- **Production Ready**: Easy to disable all debug output for release builds

### Intelligent Debug Management
- **Category-Based**: Different systems can be debugged independently
- **Timing Control**: Configurable intervals prevent excessive logging
- **Conditional Logging**: Only log when interesting events occur
- **Keyboard Shortcuts**: Quick toggle controls for efficient debugging
- **Help Integration**: Built-in help system for debug controls

## System Architecture

### Core Components

#### GameDebugConfig Resource
```rust
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameDebugConfig {
    // Debug Categories
    pub collision_debug: bool,
    pub input_debug: bool,
    pub animation_debug: bool,
    pub slide_debug: bool,
    pub performance_debug: bool,
    pub quest_debug: bool,
    pub physics_visual_debug: bool,
    
    // Timing Control
    pub debug_update_interval: f32,
    pub debug_only_when_moving: bool,
}
```

#### DebugTimer Resource
```rust
#[derive(Resource, Debug, Default)]
pub struct DebugTimer {
    pub last_collision_log: f64,
    pub last_input_log: f64,
    pub last_slide_log: f64,
}

impl DebugTimer {
    pub fn should_log_collision(&mut self, current_time: f64, interval: f32) -> bool {
        if current_time - self.last_collision_log >= interval as f64 {
            self.last_collision_log = current_time;
            true
        } else {
            false
        }
    }
}
```

### Default Configuration
```rust
impl Default for GameDebugConfig {
    fn default() -> Self {
        Self {
            collision_debug: false,    // Disable by default - very spammy
            input_debug: false,        // Disable by default - very spammy  
            animation_debug: true,     // Keep animation changes - useful info
            slide_debug: false,        // Disable by default - very spammy
            performance_debug: false,  // Disable by default
            quest_debug: true,         // Keep quest info - important for gameplay
            physics_visual_debug: true, // Keep visual debug - useful for development
            
            debug_update_interval: 2.0, // Only log every 2 seconds max
            debug_only_when_moving: true, // Only when something interesting is happening
        }
    }
}
```

## Key Systems

### 1. Debug Category Management

#### Category Toggle Methods
```rust
impl GameDebugConfig {
    pub fn toggle_collision_debug(&mut self) {
        self.collision_debug = !self.collision_debug;
        info!("🔧 Collision debug: {}", if self.collision_debug { "ON" } else { "OFF" });
    }
    
    pub fn toggle_input_debug(&mut self) {
        self.input_debug = !self.input_debug;
        info!("🎮 Input debug: {}", if self.input_debug { "ON" } else { "OFF" });
    }
    
    pub fn toggle_animation_debug(&mut self) {
        self.animation_debug = !self.animation_debug;
        info!("🎭 Animation debug: {}", if self.animation_debug { "ON" } else { "OFF" });
    }
    
    pub fn toggle_slide_debug(&mut self) {
        self.slide_debug = !self.slide_debug;
        info!("🏃 Slide debug: {}", if self.slide_debug { "ON" } else { "OFF" });
    }
}
```

#### Bulk Mode Controls
```rust
impl GameDebugConfig {
    pub fn enable_all(&mut self) {
        self.collision_debug = true;
        self.input_debug = true;
        self.animation_debug = true;
        self.slide_debug = true;
        self.performance_debug = true;
        self.quest_debug = true;
        info!("🚨 All debug modes ENABLED");
    }
    
    pub fn disable_all(&mut self) {
        self.collision_debug = false;
        self.input_debug = false;
        self.animation_debug = false;
        self.slide_debug = false;
        self.performance_debug = false;
        self.quest_debug = false;
        info!("🔇 All debug modes DISABLED");
    }
    
    pub fn production_mode(&mut self) {
        self.collision_debug = false;
        self.input_debug = false;
        self.animation_debug = false;
        self.slide_debug = false;
        self.performance_debug = false;
        self.quest_debug = true; // Keep quest info for gameplay
        self.debug_update_interval = 5.0; // Less frequent updates
        self.debug_only_when_moving = true;
        info!("🎮 Debug set to PRODUCTION mode");
    }
    
    pub fn development_mode(&mut self) {
        self.collision_debug = false; // Still keep spammy ones off
        self.input_debug = false;
        self.animation_debug = true;
        self.slide_debug = false;
        self.performance_debug = true;
        self.quest_debug = true;
        self.debug_update_interval = 1.0;
        self.debug_only_when_moving = false;
        info!("🛠️ Debug set to DEVELOPMENT mode");
    }
}
```

### 2. Keyboard Control System

#### Debug Configuration System
```rust
pub fn debug_config_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug_config: ResMut<GameDebugConfig>,
) {
    // Individual category toggles
    if keyboard.just_pressed(KeyCode::F1) {
        debug_config.toggle_collision_debug();
    }
    
    if keyboard.just_pressed(KeyCode::F2) {
        debug_config.toggle_input_debug();
    }
    
    if keyboard.just_pressed(KeyCode::F3) {
        debug_config.toggle_slide_debug();
    }
    
    if keyboard.just_pressed(KeyCode::F4) {
        debug_config.toggle_animation_debug();
    }
    
    // Bulk operations with Shift modifier
    if keyboard.pressed(KeyCode::ShiftLeft) {
        if keyboard.just_pressed(KeyCode::F1) {
            debug_config.enable_all();
        } else if keyboard.just_pressed(KeyCode::F2) {
            debug_config.disable_all();
        } else if keyboard.just_pressed(KeyCode::F3) {
            debug_config.production_mode();
        } else if keyboard.just_pressed(KeyCode::F4) {
            debug_config.development_mode();
        }
    }
}
```

#### Help System
```rust
pub fn debug_help_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    debug_config: Res<GameDebugConfig>,
) {
    if keyboard.just_pressed(KeyCode::F10) {
        info!("🛠️ === DEBUG CONTROLS ===");
        info!("F1 - Toggle Collision Debug ({})", if debug_config.collision_debug { "ON" } else { "OFF" });
        info!("F2 - Toggle Input Debug ({})", if debug_config.input_debug { "ON" } else { "OFF" });
        info!("F3 - Toggle Slide Debug ({})", if debug_config.slide_debug { "ON" } else { "OFF" });
        info!("F4 - Toggle Animation Debug ({})", if debug_config.animation_debug { "ON" } else { "OFF" });
        info!("Shift+F1 - Enable All Debug");
        info!("Shift+F2 - Disable All Debug"); 
        info!("Shift+F3 - Production Mode");
        info!("Shift+F4 - Development Mode");
        info!("F10 - Show this help");
        info!("Update interval: {:.1}s, Only when moving: {}", 
              debug_config.debug_update_interval, 
              debug_config.debug_only_when_moving);
    }
}
```

### 3. System Integration

#### Collision Debug Integration
```rust
pub fn debug_player_collision(
    time: Res<Time>,
    debug_config: Res<GameDebugConfig>,
    mut debug_timer: ResMut<DebugTimer>,
    // ... other parameters
) {
    // Early return if debug disabled
    if !debug_config.collision_debug {
        return;
    }
    
    // Timing control
    let current_time = time.elapsed_secs_f64();
    if !debug_timer.should_log_collision(current_time, debug_config.debug_update_interval) {
        return;
    }
    
    // Movement-only mode check
    if debug_config.debug_only_when_moving {
        // Only log if player is actually moving
        if velocity.length() < 0.1 {
            return;
        }
    }
    
    // Debug logging with emoji categories
    info!("🔧 COLLISION: pos=({:.2},{:.2},{:.2}) grounded={} height_diff={:.3}",
          player_pos.x, player_pos.y, player_pos.z, is_grounded, height_diff);
}
```

#### Input Debug Integration
```rust
pub fn handle_input(
    time: Res<Time>,
    debug_config: Res<GameDebugConfig>,
    mut debug_timer: ResMut<DebugTimer>,
    // ... other parameters
) {
    // Input processing logic...
    
    // Configurable debug logging
    if debug_config.input_debug && 
       (input_resource.forward || input_resource.backward || input_resource.left || input_resource.right) {
        let current_time = time.elapsed_secs_f64();
        if debug_timer.should_log_input(current_time, debug_config.debug_update_interval) {
            // Only log if movement is happening or we're not limiting to movement
            if !debug_config.debug_only_when_moving || 
               (input_resource.forward || input_resource.backward || input_resource.left || input_resource.right) {
                info!("🎮 INPUT: W={} S={} A={} D={} Space={} Shift={}", 
                      input_resource.forward, input_resource.backward, input_resource.left, 
                      input_resource.right, input_resource.down, input_resource.up);
            }
        }
    }
}
```

#### Animation Debug Integration
```rust
pub fn debug_animation_state(
    debug_config: Res<GameDebugConfig>,
    animation_query: Query<&AnimationController, With<Player>>,
    mut debug_text_query: Query<&mut Text, With<FPSText>>,
) {
    if !debug_config.animation_debug {
        return;
    }
    
    // Display animation state in UI debug text
    if let Ok(anim_controller) = animation_query.single() {
        if let Ok(mut text) = debug_text_query.single_mut() {
            let current_text = &text.0;
            let fps_part = current_text.split('\n').next().unwrap_or(current_text);
            
            text.0 = format!(
                "{}\nAnim: {:?}", 
                fps_part,
                anim_controller.current_state
            );
        }
    }
}

pub fn debug_animation_changes(
    debug_config: Res<GameDebugConfig>,
    animation_query: Query<&AnimationController, (With<Player>, Changed<AnimationController>)>,
) {
    if !debug_config.animation_debug {
        return;
    }
    
    for anim_controller in animation_query.iter() {
        info!(
            "🎭 Animation state changed: {:?} -> {:?}", 
            anim_controller.previous_state, 
            anim_controller.current_state
        );
    }
}
```

## Debug Categories

### 1. Collision Debug (F1)
**Purpose**: Track character-terrain collision and physics interactions
**When to Use**: Investigating floating issues, character controller problems
**Output Examples**:
```
🔧 COLLISION: pos=(10.0,5.2,15.0) grounded=true height_diff=0.150
   Velocity: (2.1, 0.0, 1.5) Magnitude: 2.6
   Movement State: Horizontal=true, Falling=false, Rising=false
```

### 2. Input Debug (F2)
**Purpose**: Monitor keyboard and mouse input detection
**When to Use**: Debugging input responsiveness or control issues
**Output Examples**:
```
🎮 INPUT: W=true S=false A=false D=true Space=false Shift=true
🎮 CONTROLLER INPUT: forward=true backward=false left=false right=true mouse_forward=false
```

### 3. Slide Debug (F3)
**Purpose**: Track movement mechanics, slopes, and jumping
**When to Use**: Investigating movement feel, terrain interaction problems
**Output Examples**:
```
🏃 SLOPE: pos=(5.1,8.2,12.0) grounded=true slope=12.3° normal=(0.02,0.98,0.21)
🏃 SPEED: desired_movement_len=1.000, running=true, target_speed=8.500
🏃 JUMP: space pressed - is_jumping=false, can_jump=true, grounded=true, coyote_time=0.000
```

### 4. Animation Debug (F4)
**Purpose**: Monitor animation state transitions and triggers
**When to Use**: Debugging animation issues or state machine problems
**Output Examples**:
```
🎭 Animation state changed: Idle -> Walk
🎭 Animation state changed: Walk -> Run
```

### 5. Performance Debug
**Purpose**: Monitor frame rates, system performance, and resource usage
**When to Use**: Identifying performance bottlenecks or optimization opportunities

### 6. Quest Debug
**Purpose**: Track quest progression, evidence gathering, and dialogue events
**When to Use**: Debugging quest system or investigating narrative flow issues
**Default**: Enabled (important for gameplay feedback)

### 7. Physics Visual Debug
**Purpose**: Visual collision shape rendering and physics debug visualization
**When to Use**: Understanding collision boundaries and physics interactions
**Default**: Enabled (useful for development)

## Performance Considerations

### Zero-Cost Abstraction
```rust
// When debug_config.collision_debug is false, this entire function returns immediately
pub fn debug_player_collision(
    debug_config: Res<GameDebugConfig>,
    // ... other parameters
) {
    if !debug_config.collision_debug {
        return; // No performance cost beyond this boolean check
    }
    
    // Expensive debug calculations only happen when needed
    let complex_debug_info = calculate_complex_debug_metrics();
    info!("Debug info: {:?}", complex_debug_info);
}
```

### Timing Control Benefits
- **Prevents Console Flooding**: Debug output respects timing intervals
- **Reduces Performance Impact**: Less frequent logging means lower overhead
- **Maintains Readability**: Controlled output frequency keeps logs readable
- **Configurable Intervals**: Different update frequencies for different needs

### Memory Efficiency
- **Minimal State**: DebugTimer only stores simple timestamps
- **No String Allocation**: Debug output uses efficient formatting
- **Conditional Compilation**: Debug code can be stripped in release builds

## Integration with Bevy Systems

### System Registration
```rust
// In lib.rs
.add_systems(Update, (
    // ... other systems
    debug_config_system, // Debug configuration controls
    debug_help_system,   // Debug help display
))
```

### Resource Initialization
```rust
// Initialize debug resources
.init_resource::<GameDebugConfig>() // Configurable debug logging
.init_resource::<DebugTimer>()      // Debug timing control
```

### Cross-System Integration
- **Consistent Interface**: All systems use the same debug config pattern
- **Centralized Control**: Single resource controls all debug categories
- **Event-Driven**: Debug state changes can trigger events if needed
- **Persistent Settings**: Debug configuration can be saved/loaded

## Development Workflow

### Typical Debug Session
1. **F10**: Show current debug status and available controls
2. **F1-F4**: Enable specific debug categories as needed
3. **Investigate**: Use targeted debug output to understand issues
4. **Shift+F2**: Disable all debug when done
5. **Shift+F3**: Set production mode for testing

### Common Debug Scenarios

#### Character Movement Issues
```
1. Enable collision debug (F1) and slide debug (F3)
2. Move character around problematic area
3. Observe collision detection and movement calculations
4. Adjust character controller parameters based on output
```

#### Animation Problems
```
1. Enable animation debug (F4)
2. Perform actions that should trigger animations
3. Observe state transitions and timing
4. Verify animation controller logic
```

#### Input Responsiveness
```
1. Enable input debug (F2)
2. Test various key combinations
3. Verify input detection and timing
4. Check for input conflicts or delays
```

## Future Enhancements

### Planned Features
1. **Visual Debug Overlays**: 3D debug visualization for collision shapes and physics
2. **Performance Profiling**: Detailed timing analysis for system performance
3. **Debug Recording**: Save debug sessions for later analysis
4. **Remote Debug**: Network-based debug control for multiplayer testing
5. **Debug Scripts**: Automated debug scenario execution

### Advanced Debug Tools
1. **Memory Profiler Integration**: Track memory usage and allocation patterns
2. **Network Debug**: Packet inspection and network performance monitoring
3. **AI Debug**: Behavior tree visualization and decision tracking
4. **Asset Debug**: Asset loading and caching performance analysis

### Configuration Enhancements
1. **Per-System Intervals**: Different timing intervals for each debug category
2. **Conditional Debug**: Debug rules based on game state or player actions
3. **Debug Profiles**: Saved debug configuration profiles for different scenarios
4. **External Configuration**: JSON-based debug configuration files

## Best Practices

### For Developers
1. **Use Appropriate Categories**: Choose the right debug category for your output
2. **Respect Timing**: Use DebugTimer to prevent spam
3. **Meaningful Messages**: Include context and useful information in debug output
4. **Performance Conscious**: Always check debug flags before expensive operations
5. **Consistent Formatting**: Use emoji prefixes and clear formatting

### For System Integration
1. **Early Returns**: Check debug flags first to avoid unnecessary work
2. **Conditional Complexity**: Only perform complex debug calculations when needed
3. **Resource Sharing**: Use shared DebugTimer for timing control
4. **Error Handling**: Debug systems should never crash the main game

### For Production
1. **Clean Builds**: Ensure debug output can be completely disabled
2. **Performance Testing**: Verify minimal impact when debug is disabled
3. **Configuration Management**: Provide easy way to set production debug levels
4. **Documentation**: Maintain clear documentation of debug controls

## Conclusion

The Eryndor Debug System represents a sophisticated approach to game development debugging that balances developer productivity with production performance. By providing granular control, intelligent spam prevention, and seamless integration across all game systems, it enables efficient debugging without compromising game performance.

The system's design philosophy of "debug when needed, silent when not" ensures that developers can access detailed debugging information when required while maintaining clean, performant builds for production deployment. With its comprehensive keyboard controls and help system, it provides an accessible yet powerful debugging environment that enhances the development workflow without getting in the way of normal gameplay.