use bevy::prelude::*;
use crate::components::{Player, PlayerStats};

/// Stats regeneration system - handles natural recovery of health, mana, and stamina
/// Runs every frame and applies regeneration based on delta time
pub fn stats_regeneration_system(
    time: Res<Time>,
    mut player_query: Query<&mut PlayerStats, With<Player>>,
) {
    let dt = time.delta_secs();
    
    for mut stats in player_query.iter_mut() {
        // Store regen values to avoid borrowing issues
        let health_regen = stats.health_regen;
        let mana_regen = stats.mana_regen;
        let stamina_regen = stats.stamina_regen;
        
        // Health regeneration (only if not at full health)
        if stats.health < stats.max_health {
            stats.heal(health_regen * dt);
        }
        
        // Mana regeneration (only if not at full mana)
        if stats.mana < stats.max_mana {
            stats.restore_mana(mana_regen * dt);
        }
        
        // Stamina regeneration (only if not at full stamina)
        if stats.stamina < stats.max_stamina {
            stats.restore_stamina(stamina_regen * dt);
        }
    }
}

/// System to recalculate player stats when attributes change
/// Should be called whenever base attributes are modified
pub fn recalculate_player_stats_system(
    mut player_query: Query<&mut PlayerStats, (With<Player>, Changed<PlayerStats>)>,
) {
    for mut stats in player_query.iter_mut() {
        // Only recalculate if attributes might have changed
        // This prevents infinite loops from the recalculate function modifying the component
        let current_max_health = stats.calculate_max_health();
        let current_max_mana = stats.calculate_max_mana();
        let current_max_stamina = stats.calculate_max_stamina();
        
        // Only recalculate if values are different (indicating attribute change)
        if stats.max_health != current_max_health 
        || stats.max_mana != current_max_mana 
        || stats.max_stamina != current_max_stamina {
            stats.recalculate_max_values();
        }
    }
}

/// Debug system to display player stats (F5 to toggle)
pub fn debug_player_stats_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug_visible: Local<bool>,
    player_query: Query<&PlayerStats, With<Player>>,
) {
    // Toggle debug display with F5
    if keyboard.just_pressed(KeyCode::F5) {
        *debug_visible = !*debug_visible;
        if *debug_visible {
            info!("Player stats debug ENABLED (F5 to disable)");
        } else {
            info!("Player stats debug DISABLED");
        }
    }
    
    // Display stats if enabled
    if *debug_visible {
        if let Ok(stats) = player_query.single() {
            info!(
                "STATS: HP {:.0}/{:.0} ({:.1}%) | MP {:.0}/{:.0} ({:.1}%) | ST {:.0}/{:.0} ({:.1}%) | LVL {}",
                stats.health, stats.max_health, stats.health_percentage() * 100.0,
                stats.mana, stats.max_mana, stats.mana_percentage() * 100.0,
                stats.stamina, stats.max_stamina, stats.stamina_percentage() * 100.0,
                stats.level
            );
            info!(
                "ATTRS: STR {} | AGI {} | INT {} | VIT {} | WIS {} | LCK {} | XP {}/{}",
                stats.strength, stats.agility, stats.intelligence,
                stats.vitality, stats.wisdom, stats.luck,
                stats.experience, stats.experience_to_next_level
            );
        }
    }
}

/// System to handle temporary stat modifications (buffs/debuffs foundation for Phase 3)
/// This is a placeholder system that will be expanded in Phase 3
pub fn stats_modification_system(
    // Add buff/debuff components here in Phase 3
    mut _player_query: Query<&mut PlayerStats, With<Player>>,
) {
    // TODO Phase 3: Implement buff/debuff system
    // - Temporary attribute bonuses/penalties
    // - Status effects (poison, blessing, etc.)  
    // - Equipment stat bonuses
    // - Spell effects on stats
}

/// Resource to track global stat configuration
#[derive(Resource)]
pub struct StatsConfig {
    /// Global multiplier for all regeneration rates
    pub regen_multiplier: f32,
    /// Whether regeneration is enabled
    pub regen_enabled: bool,
    /// Debug mode for stats system
    pub debug_mode: bool,
}

impl Default for StatsConfig {
    fn default() -> Self {
        Self {
            regen_multiplier: 1.0,
            regen_enabled: true,
            debug_mode: false,
        }
    }
}