use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Central debug configuration for controlling various debug outputs
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameDebugConfig {
    /// Enable collision debug logging
    pub collision_debug: bool,
    /// Enable input debug logging  
    pub input_debug: bool,
    /// Enable animation debug logging
    pub animation_debug: bool,
    /// Enable slide/movement debug logging
    pub slide_debug: bool,
    /// Enable performance debug logging
    pub performance_debug: bool,
    /// Enable quest debug logging
    pub quest_debug: bool,
    /// Enable physics debug visual rendering
    pub physics_visual_debug: bool,
    
    /// Debug update frequency (in seconds) - how often to log debug info
    pub debug_update_interval: f32,
    /// Only log debug info when player is moving
    pub debug_only_when_moving: bool,
}

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

impl GameDebugConfig {
    /// Toggle collision debug on/off
    pub fn toggle_collision_debug(&mut self) {
        self.collision_debug = !self.collision_debug;
        info!("🔧 Collision debug: {}", if self.collision_debug { "ON" } else { "OFF" });
    }
    
    /// Toggle input debug on/off
    pub fn toggle_input_debug(&mut self) {
        self.input_debug = !self.input_debug;
        info!("🎮 Input debug: {}", if self.input_debug { "ON" } else { "OFF" });
    }
    
    /// Toggle slide/movement debug on/off
    pub fn toggle_slide_debug(&mut self) {
        self.slide_debug = !self.slide_debug;
        info!("🏃 Slide debug: {}", if self.slide_debug { "ON" } else { "OFF" });
    }
    
    /// Toggle animation debug on/off
    pub fn toggle_animation_debug(&mut self) {
        self.animation_debug = !self.animation_debug;
        info!("🎭 Animation debug: {}", if self.animation_debug { "ON" } else { "OFF" });
    }
    
    /// Enable all debug modes (for troubleshooting)
    pub fn enable_all(&mut self) {
        self.collision_debug = true;
        self.input_debug = true;
        self.animation_debug = true;
        self.slide_debug = true;
        self.performance_debug = true;
        self.quest_debug = true;
        info!("🚨 All debug modes ENABLED");
    }
    
    /// Disable all debug modes (for clean gameplay)
    pub fn disable_all(&mut self) {
        self.collision_debug = false;
        self.input_debug = false;
        self.animation_debug = false;
        self.slide_debug = false;
        self.performance_debug = false;
        self.quest_debug = false;
        info!("🔇 All debug modes DISABLED");
    }
    
    /// Set to production mode (minimal logging)
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
    
    /// Set to development mode (more logging)
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

/// System to handle debug configuration key bindings
pub fn debug_config_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug_config: ResMut<GameDebugConfig>,
) {
    // F1 - Toggle collision debug
    if keyboard.just_pressed(KeyCode::F1) {
        debug_config.toggle_collision_debug();
    }
    
    // F2 - Toggle input debug
    if keyboard.just_pressed(KeyCode::F2) {
        debug_config.toggle_input_debug();
    }
    
    // F3 - Toggle slide debug
    if keyboard.just_pressed(KeyCode::F3) {
        debug_config.toggle_slide_debug();
    }
    
    // F4 - Toggle animation debug
    if keyboard.just_pressed(KeyCode::F4) {
        debug_config.toggle_animation_debug();
    }
    
    // Shift + F1 - Enable all debug
    if keyboard.pressed(KeyCode::ShiftLeft) && keyboard.just_pressed(KeyCode::F1) {
        debug_config.enable_all();
    }
    
    // Shift + F2 - Disable all debug
    if keyboard.pressed(KeyCode::ShiftLeft) && keyboard.just_pressed(KeyCode::F2) {
        debug_config.disable_all();
    }
    
    // Shift + F3 - Production mode
    if keyboard.pressed(KeyCode::ShiftLeft) && keyboard.just_pressed(KeyCode::F3) {
        debug_config.production_mode();
    }
    
    // Shift + F4 - Development mode
    if keyboard.pressed(KeyCode::ShiftLeft) && keyboard.just_pressed(KeyCode::F4) {
        debug_config.development_mode();
    }
}

/// System to show debug help
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

/// Resource to track debug timing
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
    
    pub fn should_log_input(&mut self, current_time: f64, interval: f32) -> bool {
        if current_time - self.last_input_log >= interval as f64 {
            self.last_input_log = current_time;
            true
        } else {
            false
        }
    }
    
    pub fn should_log_slide(&mut self, current_time: f64, interval: f32) -> bool {
        if current_time - self.last_slide_log >= interval as f64 {
            self.last_slide_log = current_time;
            true
        } else {
            false
        }
    }
}