use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Main resource containing all progression configuration data
#[derive(Resource, Debug, Clone)]
pub struct ProgressionConfig {
    pub skills: HashMap<String, SkillConfig>,
    pub weapons: HashMap<String, WeaponConfig>,
    pub damage_types: HashMap<String, DamageTypeConfig>,
    pub roles: HashMap<String, RoleConfig>,
    pub character_progression: CharacterProgressionConfig,
    pub skill_progression: SkillProgressionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillConfig {
    pub display_name: String,
    pub description: String,
    pub category: String,
    pub associated_weapons: Vec<String>,
    pub role_categories: Vec<String>,
    pub max_level: u32,
    pub abilities: HashMap<u32, Vec<AbilityUnlock>>, // Level -> Abilities
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityUnlock {
    pub name: String,
    pub ability_type: String, // "Active", "Passive", "Quest", "Trainer"
    pub description: String,
    pub requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponConfig {
    pub display_name: String,
    pub description: String,
    pub associated_skill: String,
    pub damage_type: String,
    pub weapon_class: String, // "OneHanded", "TwoHanded", "Ranged", "Magic"
    pub base_damage: f32,
    pub attack_speed: f32,
    pub range: f32,
    pub critical_chance: f32,
    pub durability: u32,
    pub level_requirement: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageTypeConfig {
    pub display_name: String,
    pub description: String,
    pub damage_category: String, // "Physical", "Elemental", "Mystical", "Special"
    pub color_hex: String, // For UI display
    pub resistances: HashMap<String, f32>, // Armor type -> resistance value
    pub special_effects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleConfig {
    pub display_name: String,
    pub description: String,
    pub primary_skills: Vec<String>,
    pub secondary_skills: Vec<String>,
    pub recommended_weapons: Vec<String>,
    pub recommended_armor: String,
    pub role_bonuses: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterProgressionConfig {
    pub max_level: u32,
    pub base_experience: f32,
    pub level_multiplier: f32,
    pub level_exponent: f32,
    pub experience_sources: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillProgressionConfig {
    pub max_level: u32,
    pub base_experience: f32,
    pub level_multiplier: f32,
    pub level_exponent: f32,
    pub rested_bonus_multiplier: f32,
    pub rested_bonus_duration: f32,
}

impl ProgressionConfig {
    /// Load configuration from JSON files in the specified directory
    pub fn load_from_directory<P: AsRef<Path>>(config_dir: P) -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = config_dir.as_ref();
        
        // Load skills configuration
        let skills_path = config_path.join("skills.json");
        let skills_content = fs::read_to_string(&skills_path)
            .map_err(|e| format!("Failed to read skills.json: {}", e))?;
        let skills: HashMap<String, SkillConfig> = serde_json::from_str(&skills_content)
            .map_err(|e| format!("Failed to parse skills.json: {}", e))?;

        // Load weapons configuration
        let weapons_path = config_path.join("weapons.json");
        let weapons_content = fs::read_to_string(&weapons_path)
            .map_err(|e| format!("Failed to read weapons.json: {}", e))?;
        let weapons: HashMap<String, WeaponConfig> = serde_json::from_str(&weapons_content)
            .map_err(|e| format!("Failed to parse weapons.json: {}", e))?;

        // Load damage types configuration
        let damage_types_path = config_path.join("damage_types.json");
        let damage_types_content = fs::read_to_string(&damage_types_path)
            .map_err(|e| format!("Failed to read damage_types.json: {}", e))?;
        let damage_types: HashMap<String, DamageTypeConfig> = serde_json::from_str(&damage_types_content)
            .map_err(|e| format!("Failed to parse damage_types.json: {}", e))?;

        // Load roles configuration
        let roles_path = config_path.join("roles.json");
        let roles_content = fs::read_to_string(&roles_path)
            .map_err(|e| format!("Failed to read roles.json: {}", e))?;
        let roles: HashMap<String, RoleConfig> = serde_json::from_str(&roles_content)
            .map_err(|e| format!("Failed to parse roles.json: {}", e))?;

        // Load progression configuration
        let progression_path = config_path.join("progression.json");
        let progression_content = fs::read_to_string(&progression_path)
            .map_err(|e| format!("Failed to read progression.json: {}", e))?;
        
        #[derive(Deserialize)]
        struct ProgressionFile {
            character_progression: CharacterProgressionConfig,
            skill_progression: SkillProgressionConfig,
        }
        
        let progression_file: ProgressionFile = serde_json::from_str(&progression_content)
            .map_err(|e| format!("Failed to parse progression.json: {}", e))?;

        Ok(ProgressionConfig {
            skills,
            weapons,
            damage_types,
            roles,
            character_progression: progression_file.character_progression,
            skill_progression: progression_file.skill_progression,
        })
    }

    /// Calculate experience required for a character level
    pub fn character_experience_for_level(&self, level: u32) -> u64 {
        if level <= 1 {
            return 0;
        }
        
        let config = &self.character_progression;
        let level_f = level as f32;
        
        (level_f.powf(config.level_exponent) * config.level_multiplier + 
         level_f * config.base_experience) as u64
    }

    /// Calculate experience required for a skill level
    pub fn skill_experience_for_level(&self, _skill_id: &str, level: u32) -> u64 {
        if level <= 1 {
            return 0;
        }
        
        let config = &self.skill_progression;
        let level_f = level as f32;
        
        (level_f.powf(config.level_exponent) * config.level_multiplier + 
         level_f * config.base_experience) as u64
    }

    /// Get skill configuration by ID
    pub fn get_skill(&self, skill_id: &str) -> Option<&SkillConfig> {
        self.skills.get(skill_id)
    }

    /// Get weapon configuration by ID
    pub fn get_weapon(&self, weapon_id: &str) -> Option<&WeaponConfig> {
        self.weapons.get(weapon_id)
    }

    /// Get damage type configuration by ID
    pub fn get_damage_type(&self, damage_type_id: &str) -> Option<&DamageTypeConfig> {
        self.damage_types.get(damage_type_id)
    }

    /// Get role configuration by ID
    pub fn get_role(&self, role_id: &str) -> Option<&RoleConfig> {
        self.roles.get(role_id)
    }

    /// Get abilities available at a specific skill level
    pub fn get_abilities_for_skill_level(&self, skill_id: &str, level: u32) -> Vec<&AbilityUnlock> {
        if let Some(skill) = self.get_skill(skill_id) {
            let mut abilities = Vec::new();
            for unlock_level in 1..=level {
                if let Some(level_abilities) = skill.abilities.get(&unlock_level) {
                    abilities.extend(level_abilities);
                }
            }
            abilities
        } else {
            Vec::new()
        }
    }

    /// Get all weapons associated with a skill
    pub fn get_weapons_for_skill(&self, skill_id: &str) -> Vec<&WeaponConfig> {
        self.weapons
            .values()
            .filter(|weapon| weapon.associated_skill == skill_id)
            .collect()
    }

    /// Get all skills for a role category
    pub fn get_skills_for_role(&self, role_id: &str) -> Vec<&SkillConfig> {
        if let Some(role) = self.get_role(role_id) {
            let mut skills = Vec::new();
            
            // Add primary skills
            for skill_id in &role.primary_skills {
                if let Some(skill) = self.get_skill(skill_id) {
                    skills.push(skill);
                }
            }
            
            // Add secondary skills
            for skill_id in &role.secondary_skills {
                if let Some(skill) = self.get_skill(skill_id) {
                    skills.push(skill);
                }
            }
            
            skills
        } else {
            Vec::new()
        }
    }
}

/// Load progression configuration from the config directory
pub fn load_progression_config() -> ProgressionConfig {
    let config_dir = "config";
    
    match ProgressionConfig::load_from_directory(config_dir) {
        Ok(config) => {
            info!("Successfully loaded progression configuration from {}", config_dir);
            info!("Loaded {} skills, {} weapons, {} damage types, {} roles", 
                  config.skills.len(), config.weapons.len(), 
                  config.damage_types.len(), config.roles.len());
            config
        },
        Err(e) => {
            warn!("Failed to load progression configuration: {}", e);
            warn!("Using fallback hardcoded configuration");
            
            // Create minimal fallback configuration
            let mut skills = HashMap::new();
            skills.insert("swordsmanship".to_string(), SkillConfig {
                display_name: "Swordsmanship".to_string(),
                description: "Master of blade combat".to_string(),
                category: "Melee".to_string(),
                associated_weapons: vec!["sword".to_string(), "two_handed_sword".to_string()],
                role_categories: vec!["tank".to_string(), "dps".to_string()],
                max_level: 50,
                abilities: HashMap::new(),
            });

            let mut weapons = HashMap::new();
            weapons.insert("sword".to_string(), WeaponConfig {
                display_name: "Sword".to_string(),
                description: "A balanced one-handed blade".to_string(),
                associated_skill: "swordsmanship".to_string(),
                damage_type: "slashing".to_string(),
                weapon_class: "OneHanded".to_string(),
                base_damage: 10.0,
                attack_speed: 1.8,
                range: 1.5,
                critical_chance: 0.05,
                durability: 100,
                level_requirement: 1,
            });

            let mut damage_types = HashMap::new();
            damage_types.insert("slashing".to_string(), DamageTypeConfig {
                display_name: "Slashing".to_string(),
                description: "Sharp blade damage".to_string(),
                damage_category: "Physical".to_string(),
                color_hex: "#FF4444".to_string(),
                resistances: HashMap::new(),
                special_effects: Vec::new(),
            });

            let mut roles = HashMap::new();
            roles.insert("tank".to_string(), RoleConfig {
                display_name: "Tank".to_string(),
                description: "Defensive specialist".to_string(),
                primary_skills: vec!["swordsmanship".to_string(), "shield_defense".to_string()],
                secondary_skills: vec!["heavy_armor".to_string()],
                recommended_weapons: vec!["sword".to_string(), "shield".to_string()],
                recommended_armor: "heavy".to_string(),
                role_bonuses: HashMap::new(),
            });

            ProgressionConfig {
                skills,
                weapons,
                damage_types,
                roles,
                character_progression: CharacterProgressionConfig {
                    max_level: 50,
                    base_experience: 200.0,
                    level_multiplier: 100.0,
                    level_exponent: 2.0,
                    experience_sources: HashMap::new(),
                },
                skill_progression: SkillProgressionConfig {
                    max_level: 50,
                    base_experience: 25.0,
                    level_multiplier: 50.0,
                    level_exponent: 1.8,
                    rested_bonus_multiplier: 1.5,
                    rested_bonus_duration: 300.0,
                },
            }
        }
    }
}