use bevy::prelude::*;
use crate::components::progression::{SkillType, WeaponType, DamageType, RoleType};
use crate::resources::ProgressionConfig;

/// System to demonstrate JSON configuration integration
pub fn debug_progression_config_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    config: Res<ProgressionConfig>,
    mut debug_visible: Local<bool>,
) {
    // Toggle config debug display with Shift+F2
    if keyboard.pressed(KeyCode::ShiftLeft) && keyboard.just_pressed(KeyCode::F2) {
        *debug_visible = !*debug_visible;
        if *debug_visible {
            info!("Progression config debug ENABLED (Shift+F2 to disable)");
        } else {
            info!("Progression config debug DISABLED");
        }
    }
    
    // Display configuration info if enabled
    if *debug_visible {
        info!("=== PROGRESSION CONFIG DEBUG ===");
        
        // Show some skills
        info!("Available Skills ({}):", config.skills.len());
        for (key, skill_config) in config.skills.iter().take(5) {
            info!("  - {}: {} ({})", key, skill_config.display_name, skill_config.category);
        }
        
        // Show some weapons
        info!("Available Weapons ({}):", config.weapons.len());
        for (key, weapon_config) in config.weapons.iter().take(5) {
            info!("  - {}: {} -> {} ({})", 
                key, weapon_config.display_name, 
                weapon_config.associated_skill, weapon_config.damage_type);
        }
        
        // Show experience calculations
        info!("Experience Requirements:");
        info!("  Character Level 10: {} XP", config.character_experience_for_level(10));
        info!("  Skill Level 10: {} XP", config.skill_experience_for_level("swordsmanship", 10));
        
        // Show role information
        info!("Available Roles ({}):", config.roles.len());
        for (key, role_config) in &config.roles {
            info!("  - {}: {} ({} primary skills)", key, role_config.display_name, role_config.primary_skills.len());
        }
        
        info!("=== END CONFIG DEBUG ===");
    }
}

/// Extension methods for ProgressionConfig to integrate with hardcoded enums
impl ProgressionConfig {
    /// Convert string skill ID to SkillType enum for compatibility
    pub fn skill_id_to_enum(&self, skill_id: &str) -> Option<SkillType> {
        match skill_id {
            "swordsmanship" => Some(SkillType::Swordsmanship),
            "axe_mastery" => Some(SkillType::AxeMastery),
            "mace_skill" => Some(SkillType::MaceSkill),
            "hammer_skill" => Some(SkillType::HammerSkill),
            "spear_mastery" => Some(SkillType::SpearMastery),
            "dagger_mastery" => Some(SkillType::DaggerMastery),
            "shield_defense" => Some(SkillType::ShieldDefense),
            "archery" => Some(SkillType::Archery),
            "crossbow" => Some(SkillType::Crossbow),
            "throwing_weapons" => Some(SkillType::ThrowingWeapons),
            "fire_magic" => Some(SkillType::FireMagic),
            "ice_magic" => Some(SkillType::IceMagic),
            "lightning_magic" => Some(SkillType::LightningMagic),
            "shadow_magic" => Some(SkillType::ShadowMagic),
            "nature_magic" => Some(SkillType::NatureMagic),
            "arcane_magic" => Some(SkillType::ArcanieMagic),
            "restoration" => Some(SkillType::Restoration),
            "divination" => Some(SkillType::Divination),
            "heavy_armor" => Some(SkillType::HeavyArmor),
            "medium_armor" => Some(SkillType::MediumArmor),
            "light_armor" => Some(SkillType::LightArmor),
            "smithing" => Some(SkillType::Smithing),
            "alchemy" => Some(SkillType::Alchemy),
            "enchanting" => Some(SkillType::Enchanting),
            "cooking" => Some(SkillType::Cooking),
            "athletics" => Some(SkillType::Athletics),
            "stealth" => Some(SkillType::Stealth),
            "lockpicking" => Some(SkillType::Lockpicking),
            "pickpocketing" => Some(SkillType::Pickpocketing),
            _ => None,
        }
    }

    /// Convert SkillType enum to string ID for config lookups
    pub fn skill_enum_to_id(&self, skill_type: SkillType) -> &str {
        match skill_type {
            SkillType::Swordsmanship => "swordsmanship",
            SkillType::AxeMastery => "axe_mastery",
            SkillType::MaceSkill => "mace_skill",
            SkillType::HammerSkill => "hammer_skill",
            SkillType::SpearMastery => "spear_mastery",
            SkillType::DaggerMastery => "dagger_mastery",
            SkillType::ShieldDefense => "shield_defense",
            SkillType::Archery => "archery",
            SkillType::Crossbow => "crossbow",
            SkillType::ThrowingWeapons => "throwing_weapons",
            SkillType::FireMagic => "fire_magic",
            SkillType::IceMagic => "ice_magic",
            SkillType::LightningMagic => "lightning_magic",
            SkillType::ShadowMagic => "shadow_magic",
            SkillType::NatureMagic => "nature_magic",
            SkillType::ArcanieMagic => "arcane_magic",
            SkillType::Restoration => "restoration",
            SkillType::Divination => "divination",
            SkillType::HeavyArmor => "heavy_armor",
            SkillType::MediumArmor => "medium_armor",
            SkillType::LightArmor => "light_armor",
            SkillType::Smithing => "smithing",
            SkillType::Alchemy => "alchemy",
            SkillType::Enchanting => "enchanting",
            SkillType::Cooking => "cooking",
            SkillType::Athletics => "athletics",
            SkillType::Stealth => "stealth",
            SkillType::Lockpicking => "lockpicking",
            SkillType::Pickpocketing => "pickpocketing",
        }
    }

    /// Convert string weapon ID to WeaponType enum
    pub fn weapon_id_to_enum(&self, weapon_id: &str) -> Option<WeaponType> {
        match weapon_id {
            "sword" => Some(WeaponType::Sword),
            "two_handed_sword" => Some(WeaponType::TwoHandedSword),
            "axe" => Some(WeaponType::Axe),
            "two_handed_axe" => Some(WeaponType::TwoHandedAxe),
            "mace" => Some(WeaponType::Mace),
            "two_handed_mace" => Some(WeaponType::TwoHandedMace),
            "hammer" => Some(WeaponType::Hammer),
            "two_handed_hammer" => Some(WeaponType::TwoHandedHammer),
            "spear" => Some(WeaponType::Spear),
            "pike" => Some(WeaponType::Pike),
            "dagger" => Some(WeaponType::Dagger),
            "shield" => Some(WeaponType::Shield),
            "bow" => Some(WeaponType::Bow),
            "longbow" => Some(WeaponType::Longbow),
            "crossbow" => Some(WeaponType::Crossbow),
            "hand_crossbow" => Some(WeaponType::HandCrossbow),
            "throwing_knife" => Some(WeaponType::ThrowingKnife),
            "throwing_axe" => Some(WeaponType::ThrowingAxe),
            "javelin" => Some(WeaponType::Javelin),
            "fire_staff" => Some(WeaponType::FireStaff),
            "ice_staff" => Some(WeaponType::IceStaff),
            "lightning_staff" => Some(WeaponType::LightningStaff),
            "shadow_staff" => Some(WeaponType::ShadowStaff),
            "nature_staff" => Some(WeaponType::NatureStaff),
            "arcane_staff" => Some(WeaponType::ArcaneStaff),
            "restoration_staff" => Some(WeaponType::RestorationStaff),
            "divination_staff" => Some(WeaponType::DivinationStaff),
            "magical_orb" => Some(WeaponType::MagicalOrb),
            _ => None,
        }
    }

    /// Convert string damage type ID to DamageType enum
    pub fn damage_type_id_to_enum(&self, damage_type_id: &str) -> Option<DamageType> {
        match damage_type_id {
            "slashing" => Some(DamageType::Slashing),
            "piercing" => Some(DamageType::Piercing),
            "bludgeoning" => Some(DamageType::Bludgeoning),
            "fire" => Some(DamageType::Fire),
            "ice" => Some(DamageType::Ice),
            "lightning" => Some(DamageType::Lightning),
            "shadow" => Some(DamageType::Shadow),
            "nature" => Some(DamageType::Nature),
            "arcane" => Some(DamageType::Arcane),
            "healing" => Some(DamageType::Healing),
            "psychic" => Some(DamageType::Psychic),
            "holy" => Some(DamageType::Holy),
            "necrotic" => Some(DamageType::Necrotic),
            _ => None,
        }
    }

    /// Convert string role ID to RoleType enum
    pub fn role_id_to_enum(&self, role_id: &str) -> Option<RoleType> {
        match role_id {
            "tank" => Some(RoleType::Tank),
            "healer" => Some(RoleType::Healer),
            "dps" => Some(RoleType::DPS),
            "support" => Some(RoleType::Support),
            "utility" => Some(RoleType::Utility),
            _ => None,
        }
    }

    /// Get weapon config from WeaponType enum
    pub fn get_weapon_config_by_enum(&self, weapon_type: WeaponType) -> Option<&crate::resources::progression_config::WeaponConfig> {
        let weapon_id = match weapon_type {
            WeaponType::Sword => "sword",
            WeaponType::TwoHandedSword => "two_handed_sword",
            WeaponType::Axe => "axe",
            WeaponType::TwoHandedAxe => "two_handed_axe",
            WeaponType::Mace => "mace",
            WeaponType::TwoHandedMace => "two_handed_mace",
            WeaponType::Hammer => "hammer",
            WeaponType::TwoHandedHammer => "two_handed_hammer",
            WeaponType::Spear => "spear",
            WeaponType::Pike => "pike",
            WeaponType::Dagger => "dagger",
            WeaponType::Shield => "shield",
            WeaponType::Bow => "bow",
            WeaponType::Longbow => "longbow",
            WeaponType::Crossbow => "crossbow",
            WeaponType::HandCrossbow => "hand_crossbow",
            WeaponType::ThrowingKnife => "throwing_knife",
            WeaponType::ThrowingAxe => "throwing_axe",
            WeaponType::Javelin => "javelin",
            WeaponType::FireStaff => "fire_staff",
            WeaponType::IceStaff => "ice_staff",
            WeaponType::LightningStaff => "lightning_staff",
            WeaponType::ShadowStaff => "shadow_staff",
            WeaponType::NatureStaff => "nature_staff",
            WeaponType::ArcaneStaff => "arcane_staff",
            WeaponType::RestorationStaff => "restoration_staff",
            WeaponType::DivinationStaff => "divination_staff",
            WeaponType::MagicalOrb => "magical_orb",
        };
        self.get_weapon(weapon_id)
    }

    /// Get skill config from SkillType enum  
    pub fn get_skill_config_by_enum(&self, skill_type: SkillType) -> Option<&crate::resources::progression_config::SkillConfig> {
        let skill_id = self.skill_enum_to_id(skill_type);
        self.get_skill(skill_id)
    }

    /// Get damage type config from DamageType enum
    pub fn get_damage_type_config_by_enum(&self, damage_type: DamageType) -> Option<&crate::resources::progression_config::DamageTypeConfig> {
        let damage_type_id = match damage_type {
            DamageType::Slashing => "slashing",
            DamageType::Piercing => "piercing", 
            DamageType::Bludgeoning => "bludgeoning",
            DamageType::Fire => "fire",
            DamageType::Ice => "ice",
            DamageType::Lightning => "lightning",
            DamageType::Shadow => "shadow",
            DamageType::Nature => "nature",
            DamageType::Arcane => "arcane",
            DamageType::Healing => "healing",
            DamageType::Psychic => "psychic",
            DamageType::Holy => "holy",
            DamageType::Necrotic => "necrotic",
        };
        self.get_damage_type(damage_type_id)
    }

    /// Calculate experience for skill using config values
    pub fn calculate_skill_experience(&self, skill_type: SkillType, level: u32) -> u64 {
        let skill_id = self.skill_enum_to_id(skill_type);
        self.skill_experience_for_level(skill_id, level)
    }

    /// Get abilities unlocked for a skill at a given level
    pub fn get_skill_abilities(&self, skill_type: SkillType, level: u32) -> Vec<String> {
        let skill_id = self.skill_enum_to_id(skill_type);
        self.get_abilities_for_skill_level(skill_id, level)
            .into_iter()
            .map(|ability| ability.name.clone())
            .collect()
    }
}

/// System to validate progression configuration at startup
pub fn validate_progression_config_system(config: Res<ProgressionConfig>) {
    info!("Validating progression configuration...");
    
    let mut warnings = Vec::new();
    let mut errors = Vec::new();
    
    // Validate skills
    for (skill_id, skill_config) in &config.skills {
        if skill_config.display_name.is_empty() {
            warnings.push(format!("Skill '{}' has empty display name", skill_id));
        }
        
        if skill_config.max_level == 0 || skill_config.max_level > 100 {
            warnings.push(format!("Skill '{}' has invalid max level: {}", skill_id, skill_config.max_level));
        }

        // Validate weapon associations
        for weapon_id in &skill_config.associated_weapons {
            if !config.weapons.contains_key(weapon_id) {
                errors.push(format!("Skill '{}' references unknown weapon '{}'", skill_id, weapon_id));
            }
        }
    }
    
    // Validate weapons
    for (weapon_id, weapon_config) in &config.weapons {
        if weapon_config.display_name.is_empty() {
            warnings.push(format!("Weapon '{}' has empty display name", weapon_id));
        }
        
        if !config.skills.contains_key(&weapon_config.associated_skill) {
            errors.push(format!("Weapon '{}' references unknown skill '{}'", weapon_id, weapon_config.associated_skill));
        }
        
        if !config.damage_types.contains_key(&weapon_config.damage_type) {
            errors.push(format!("Weapon '{}' references unknown damage type '{}'", weapon_id, weapon_config.damage_type));
        }
        
        if weapon_config.base_damage <= 0.0 {
            warnings.push(format!("Weapon '{}' has invalid base damage: {}", weapon_id, weapon_config.base_damage));
        }
        
        if weapon_config.attack_speed <= 0.0 {
            warnings.push(format!("Weapon '{}' has invalid attack speed: {}", weapon_id, weapon_config.attack_speed));
        }
    }
    
    // Validate roles
    for (role_id, role_config) in &config.roles {
        if role_config.display_name.is_empty() {
            warnings.push(format!("Role '{}' has empty display name", role_id));
        }
        
        // Validate skill references
        for skill_id in &role_config.primary_skills {
            if !config.skills.contains_key(skill_id) {
                errors.push(format!("Role '{}' references unknown primary skill '{}'", role_id, skill_id));
            }
        }
        
        for skill_id in &role_config.secondary_skills {
            if !config.skills.contains_key(skill_id) {
                errors.push(format!("Role '{}' references unknown secondary skill '{}'", role_id, skill_id));
            }
        }
        
        // Validate weapon references
        for weapon_id in &role_config.recommended_weapons {
            if !config.weapons.contains_key(weapon_id) {
                errors.push(format!("Role '{}' references unknown recommended weapon '{}'", role_id, weapon_id));
            }
        }
    }
    
    // Report validation results
    if !warnings.is_empty() {
        warn!("Configuration validation warnings:");
        for warning in warnings {
            warn!("  - {}", warning);
        }
    }
    
    if !errors.is_empty() {
        error!("Configuration validation errors:");
        for error in errors {
            error!("  - {}", error);
        }
        panic!("Configuration validation failed! Please fix the errors above.");
    } else {
        info!("Progression configuration validation passed successfully!");
    }
}