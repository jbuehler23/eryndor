use bevy::prelude::*;

/// Player marker component - identifies the player entity
/// Following Single Responsibility Principle: only marks an entity as the player
#[derive(Component)]
pub struct Player;

/// Player movement configuration - physics-based movement properties
/// Works with Tnua character controller for realistic movement
#[derive(Component)]
pub struct PlayerMovementConfig {
    pub walk_speed: f32,
    pub run_speed: f32,
    pub jump_height: f32,
    pub acceleration: f32,
    pub air_acceleration: f32,
    pub deceleration: f32,
}

/// Component to track current movement state for smooth acceleration/deceleration
#[derive(Component)]
pub struct PlayerMovementState {
    pub current_speed: f32,          // Current movement speed
    pub target_speed: f32,           // Target speed to accelerate/decelerate towards
    pub current_direction: Vec3,     // Current movement direction (normalized)
    pub target_direction: Vec3,      // Target movement direction (normalized)
    pub vertical_velocity: f32,      // Current vertical velocity for jumping/falling
    pub is_jumping: bool,            // Whether player is currently jumping
}

impl Default for PlayerMovementConfig {
    fn default() -> Self {
        Self {
            // MMO-optimized speeds for responsive feel
            walk_speed: 7.0,           // Slightly slower for better control precision
            run_speed: 14.0,           // Faster for exciting movement
            jump_height: 3.5,          // Reduced for more realistic jumping
            
            // High responsiveness for competitive MMO feel
            acceleration: 60.0,        // Faster acceleration for snappy movement
            air_acceleration: 25.0,    // Better air control for platforming
            deceleration: 80.0,        // Very fast stopping for precise positioning
        }
    }
}

impl Default for PlayerMovementState {
    fn default() -> Self {
        Self {
            current_speed: 0.0,
            target_speed: 0.0,
            current_direction: Vec3::ZERO,
            target_direction: Vec3::ZERO,
            vertical_velocity: 0.0,
            is_jumping: false,
        }
    }
}

/// Comprehensive player stats component for Phase 2+
/// Includes all vital stats and base attributes needed for MMO gameplay
#[derive(Component, Debug, Clone)]
pub struct PlayerStats {
    // Vital Statistics
    pub health: f32,
    pub max_health: f32,
    pub mana: f32,
    pub max_mana: f32,
    pub stamina: f32,
    pub max_stamina: f32,
    
    // Base Attributes (classic RPG stats)
    pub strength: u32,      // Affects physical damage and carrying capacity
    pub agility: u32,       // Affects attack speed and dodge chance  
    pub intelligence: u32,  // Affects mana pool and spell damage
    pub vitality: u32,      // Affects health pool and health regeneration
    pub wisdom: u32,        // Affects mana regeneration and resistance
    pub luck: u32,          // Affects critical hit chance and rare drops
    
    // Regeneration Rates (points per second)
    pub health_regen: f32,
    pub mana_regen: f32,
    pub stamina_regen: f32,
    
    // Experience and Level (foundation for Phase 3)
    pub experience: u64,
    pub level: u32,
    pub experience_to_next_level: u64,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            // Starting vital stats (level 1 character)
            health: 100.0,
            max_health: 100.0,
            mana: 50.0,
            max_mana: 50.0,
            stamina: 80.0,
            max_stamina: 80.0,
            
            // Starting attributes (balanced starting character)
            strength: 10,
            agility: 10, 
            intelligence: 10,
            vitality: 10,
            wisdom: 10,
            luck: 10,
            
            // Regeneration rates (classic MMO style)
            health_regen: 1.0,  // 1 HP per second
            mana_regen: 2.0,    // 2 MP per second  
            stamina_regen: 5.0, // 5 stamina per second
            
            // Experience system (ready for Phase 3)
            experience: 0,
            level: 1,
            experience_to_next_level: 100,
        }
    }
}

impl PlayerStats {
    /// Calculate max health based on vitality attribute
    /// Formula: base 80 + (vitality * 20) 
    pub fn calculate_max_health(&self) -> f32 {
        80.0 + (self.vitality as f32 * 20.0)
    }
    
    /// Calculate max mana based on intelligence attribute
    /// Formula: base 30 + (intelligence * 15)
    pub fn calculate_max_mana(&self) -> f32 {
        30.0 + (self.intelligence as f32 * 15.0)
    }
    
    /// Calculate max stamina based on agility and vitality
    /// Formula: base 60 + (agility * 8) + (vitality * 12)
    pub fn calculate_max_stamina(&self) -> f32 {
        60.0 + (self.agility as f32 * 8.0) + (self.vitality as f32 * 12.0)
    }
    
    /// Update max values based on current attributes
    pub fn recalculate_max_values(&mut self) {
        let old_health_ratio = self.health / self.max_health;
        let old_mana_ratio = self.mana / self.max_mana;
        let old_stamina_ratio = self.stamina / self.max_stamina;
        
        self.max_health = self.calculate_max_health();
        self.max_mana = self.calculate_max_mana();
        self.max_stamina = self.calculate_max_stamina();
        
        // Maintain current percentage of vital stats
        self.health = (self.max_health * old_health_ratio).min(self.max_health);
        self.mana = (self.max_mana * old_mana_ratio).min(self.max_mana);
        self.stamina = (self.max_stamina * old_stamina_ratio).min(self.max_stamina);
    }
    
    /// Take damage, returns true if character died
    pub fn take_damage(&mut self, damage: f32) -> bool {
        self.health = (self.health - damage).max(0.0);
        self.health <= 0.0
    }
    
    /// Heal for specified amount, won't exceed max health
    pub fn heal(&mut self, amount: f32) {
        self.health = (self.health + amount).min(self.max_health);
    }
    
    /// Consume mana, returns true if successful
    pub fn consume_mana(&mut self, amount: f32) -> bool {
        if self.mana >= amount {
            self.mana -= amount;
            true
        } else {
            false
        }
    }
    
    /// Consume stamina, returns true if successful  
    pub fn consume_stamina(&mut self, amount: f32) -> bool {
        if self.stamina >= amount {
            self.stamina -= amount;
            true
        } else {
            false
        }
    }
    
    /// Restore mana, won't exceed max mana
    pub fn restore_mana(&mut self, amount: f32) {
        self.mana = (self.mana + amount).min(self.max_mana);
    }
    
    /// Restore stamina, won't exceed max stamina
    pub fn restore_stamina(&mut self, amount: f32) {
        self.stamina = (self.stamina + amount).min(self.max_stamina);
    }
    
    /// Get health percentage (0.0 to 1.0)
    pub fn health_percentage(&self) -> f32 {
        if self.max_health > 0.0 {
            self.health / self.max_health
        } else {
            0.0
        }
    }
    
    /// Get mana percentage (0.0 to 1.0)  
    pub fn mana_percentage(&self) -> f32 {
        if self.max_mana > 0.0 {
            self.mana / self.max_mana
        } else {
            0.0
        }
    }
    
    /// Get stamina percentage (0.0 to 1.0)
    pub fn stamina_percentage(&self) -> f32 {
        if self.max_stamina > 0.0 {
            self.stamina / self.max_stamina
        } else {
            0.0
        }
    }
}

/// Character type selection for different models
/// Following Open/Closed: Easy to add new character types
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum CharacterType {
    Knight,
    Mage,
    Rogue,
    Barbarian,
    RogueHooded,
}

impl Default for CharacterType {
    fn default() -> Self {
        CharacterType::Knight // Default to knight character
    }
}

/// Component to track which character model is loaded
#[derive(Component)]
pub struct CharacterModel {
    pub character_type: CharacterType,
    pub model_entity: Option<Entity>, // Track the spawned model entity
}

impl Default for CharacterModel {
    fn default() -> Self {
        Self {
            character_type: CharacterType::default(),
            model_entity: None,
        }
    }
}