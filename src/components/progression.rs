use bevy::prelude::*;
use std::collections::HashMap;

/// Global character level - represents overall character power and progression
#[derive(Component, Debug, Clone)]
pub struct CharacterLevel {
    /// Current character level (1-50)
    pub level: u32,
    /// Experience points towards next level
    pub experience: u64,
    /// Experience required for next level
    pub experience_to_next_level: u64,
}

impl Default for CharacterLevel {
    fn default() -> Self {
        Self {
            level: 1,
            experience: 0,
            experience_to_next_level: Self::calculate_experience_for_level(2),
        }
    }
}

impl CharacterLevel {
    /// Calculate total experience required to reach a specific level
    pub fn calculate_experience_for_level(level: u32) -> u64 {
        if level <= 1 {
            return 0; // Level 1 requires 0 experience
        }
        // Exponential growth formula: level^2 * 100 + level * 200
        ((level as u64).pow(2) * 100) + ((level as u64) * 200)
    }
    
    /// Add experience and handle level ups
    pub fn gain_experience(&mut self, exp: u64) -> bool {
        self.experience += exp;
        
        if self.experience >= self.experience_to_next_level && self.level < 50 {
            self.level += 1;
            self.experience_to_next_level = Self::calculate_experience_for_level(self.level + 1);
            true // Level up occurred
        } else {
            false // No level up
        }
    }
    
    /// Get progress towards next level (0.0 to 1.0)
    pub fn level_progress(&self) -> f32 {
        if self.level >= 50 {
            return 1.0;
        }
        
        let current_level_exp = Self::calculate_experience_for_level(self.level);
        let next_level_exp = self.experience_to_next_level;
        let progress_in_level = self.experience - current_level_exp;
        let level_exp_range = next_level_exp - current_level_exp;
        
        (progress_in_level as f32) / (level_exp_range as f32)
    }
}

/// Individual skill types - extensible enum for easy addition of new skills
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkillType {
    // Melee Weapon Skills
    Swordsmanship,
    AxeMastery,
    MaceSkill,
    HammerSkill,
    SpearMastery,
    ShieldDefense,
    
    // Ranged Weapon Skills  
    Archery,
    Crossbow,
    DaggerMastery,
    ThrowingWeapons,
    
    // Magic Schools - organized by damage type for easy extension
    FireMagic,
    IceMagic,
    LightningMagic,
    ShadowMagic,
    NatureMagic,
    ArcanieMagic,
    Restoration,
    Divination,
    
    // Armor Mastery
    HeavyArmor,
    MediumArmor,
    LightArmor,
    
    // Crafting Skills - easy to extend with new crafting types
    Smithing,
    Alchemy,
    Enchanting,
    Cooking,
    
    // Utility Skills
    Athletics,
    Stealth,
    Lockpicking,
    Pickpocketing,
}

impl SkillType {
    /// Get all available skill types - automatically includes new skills
    pub fn all_skills() -> Vec<SkillType> {
        vec![
            // Melee Weapons
            SkillType::Swordsmanship,
            SkillType::AxeMastery,
            SkillType::MaceSkill,
            SkillType::HammerSkill,
            SkillType::SpearMastery,
            SkillType::ShieldDefense,
            // Ranged Weapons
            SkillType::Archery,
            SkillType::Crossbow,
            SkillType::DaggerMastery,
            SkillType::ThrowingWeapons,
            // Magic Schools
            SkillType::FireMagic,
            SkillType::IceMagic,
            SkillType::LightningMagic,
            SkillType::ShadowMagic,
            SkillType::NatureMagic,
            SkillType::ArcanieMagic,
            SkillType::Restoration,
            SkillType::Divination,
            // Armor
            SkillType::HeavyArmor,
            SkillType::MediumArmor,
            SkillType::LightArmor,
            // Crafting
            SkillType::Smithing,
            SkillType::Alchemy,
            SkillType::Enchanting,
            SkillType::Cooking,
            // Utility
            SkillType::Athletics,
            SkillType::Stealth,
            SkillType::Lockpicking,
            SkillType::Pickpocketing,
        ]
    }
    
    /// Get the category this skill belongs to - organized for easy extension
    pub fn category(&self) -> SkillCategory {
        match self {
            SkillType::Swordsmanship | SkillType::AxeMastery | SkillType::MaceSkill | 
            SkillType::HammerSkill | SkillType::SpearMastery => SkillCategory::MeleeWeapons,
            
            SkillType::ShieldDefense => SkillCategory::Defense,
            
            SkillType::Archery | SkillType::Crossbow | SkillType::DaggerMastery | 
            SkillType::ThrowingWeapons => SkillCategory::RangedWeapons,
            
            SkillType::FireMagic | SkillType::IceMagic | SkillType::LightningMagic | 
            SkillType::ShadowMagic | SkillType::NatureMagic | SkillType::ArcanieMagic | 
            SkillType::Restoration | SkillType::Divination => SkillCategory::Magic,
            
            SkillType::HeavyArmor | SkillType::MediumArmor | SkillType::LightArmor => SkillCategory::Armor,
            
            SkillType::Smithing | SkillType::Alchemy | SkillType::Enchanting | 
            SkillType::Cooking => SkillCategory::Crafting,
            
            SkillType::Athletics | SkillType::Stealth | SkillType::Lockpicking | 
            SkillType::Pickpocketing => SkillCategory::Utility,
        }
    }
    
    /// Get archetypal role suggestions this skill commonly supports
    /// Note: Skills don't enforce roles - this is for guidance and matchmaking suggestions only
    pub fn suggested_roles(&self) -> Vec<RoleType> {
        match self {
            // Tank-focused skills
            SkillType::ShieldDefense => vec![RoleType::Tank],
            SkillType::HeavyArmor => vec![RoleType::Tank],
            
            // Healer-focused skills  
            SkillType::Restoration => vec![RoleType::Healer],
            SkillType::Divination => vec![RoleType::Healer],
            
            // DPS-focused skills
            SkillType::Swordsmanship | SkillType::AxeMastery | SkillType::MaceSkill |
            SkillType::HammerSkill | SkillType::SpearMastery |
            SkillType::Archery | SkillType::Crossbow | SkillType::DaggerMastery |
            SkillType::ThrowingWeapons |
            SkillType::FireMagic | SkillType::IceMagic | SkillType::LightningMagic |
            SkillType::ShadowMagic | SkillType::NatureMagic | SkillType::ArcanieMagic => vec![RoleType::DPS],
            
            // Multi-role skills - can support different playstyles
            SkillType::LightArmor => vec![RoleType::Healer, RoleType::DPS, RoleType::Utility],
            SkillType::MediumArmor => vec![RoleType::DPS, RoleType::Healer],
            
            // Support skills
            SkillType::Smithing | SkillType::Alchemy | SkillType::Enchanting | 
            SkillType::Cooking => vec![RoleType::Support],
            
            // Utility skills
            SkillType::Athletics | SkillType::Stealth | SkillType::Lockpicking | 
            SkillType::Pickpocketing => vec![RoleType::Utility],
        }
    }
}

/// Skill categories for better organization and easier extension
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillCategory {
    MeleeWeapons,
    RangedWeapons,
    Defense,
    Magic,
    Armor,
    Crafting,
    Utility,
}

/// Role types for character builds - extensible for new playstyles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoleType {
    Tank,
    Healer,
    DPS,
    Support,  // Crafters and support roles
    Utility,  // Stealth and utility focused roles
}

/// Damage types for combat system - easily extensible for new damage schools
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DamageType {
    // Physical damage types
    Slashing,    // Swords, axes
    Piercing,    // Spears, arrows, daggers
    Bludgeoning, // Maces, hammers
    
    // Elemental magic damage
    Fire,
    Ice,
    Lightning,
    
    // Mystical magic damage
    Shadow,
    Nature,
    Arcane,
    
    // Special damage types
    Healing,     // Restoration magic (negative damage)
    Psychic,     // Mental/divination magic
    Holy,        // Divine magic (future expansion)
    Necrotic,    // Death magic (future expansion)
}

/// Individual skill line progression (V2 system)
#[derive(Debug, Clone)]
pub struct SkillLineV2 {
    /// Current skill level (1-50)
    pub level: u32,
    /// Experience points towards next level
    pub experience: u64,
    /// Whether this skill is actively being used (affects XP gain)
    pub active_practice: bool,
    /// Total times this skill has been used (for skill-by-use progression)
    pub usage_count: u64,
}

impl Default for SkillLineV2 {
    fn default() -> Self {
        Self {
            level: 1,
            experience: 0,
            active_practice: false,
            usage_count: 0,
        }
    }
}

impl SkillLineV2 {
    /// Calculate experience required for a specific skill level
    pub fn experience_for_level(level: u32) -> u64 {
        // Skill progression: level^1.8 * 50 + level * 25
        ((level as f64).powf(1.8) * 50.0) as u64 + ((level as u64) * 25)
    }
    
    /// Add experience from skill usage
    pub fn gain_experience(&mut self, exp: u64) -> bool {
        self.experience += exp;
        self.usage_count += 1;
        
        let required_exp = Self::experience_for_level(self.level + 1);
        if self.experience >= required_exp && self.level < 50 {
            self.level += 1;
            true // Level up occurred
        } else {
            false // No level up
        }
    }
    
    /// Get progress towards next level (0.0 to 1.0)
    pub fn level_progress(&self) -> f32 {
        if self.level >= 50 {
            return 1.0;
        }
        
        let current_level_exp = Self::experience_for_level(self.level);
        let next_level_exp = Self::experience_for_level(self.level + 1);
        let progress_in_level = self.experience - current_level_exp;
        let level_exp_range = next_level_exp - current_level_exp;
        
        (progress_in_level as f32) / (level_exp_range as f32)
    }
    
    /// Get experience gained from using this skill based on various factors
    pub fn calculate_usage_experience(&self, 
                                     target_level: u32, 
                                     effectiveness_multiplier: f32, 
                                     rested_bonus: f32) -> u64 {
        // Base XP depends on skill level vs target level
        let level_difference = if target_level > self.level {
            (target_level - self.level) as f32 * 0.2
        } else {
            1.0 - ((self.level - target_level) as f32 * 0.1).min(0.8)
        };
        
        let base_xp = 10.0 + (target_level as f32 * 2.0);
        let modified_xp = base_xp * level_difference * effectiveness_multiplier * rested_bonus;
        
        modified_xp.max(1.0) as u64
    }
}

/// Component containing all skill lines for a character
#[derive(Component, Debug, Clone)]
pub struct CharacterSkills {
    pub skills: HashMap<SkillType, SkillLineV2>,
    /// Bonus experience multiplier from resting at inns (1.0 = no bonus, 1.5 = +50%)
    pub rested_experience_bonus: f32,
    /// Time remaining for rested bonus (in seconds)
    pub rested_time_remaining: f32,
}

impl Default for CharacterSkills {
    fn default() -> Self {
        let mut skills = HashMap::new();
        
        // Initialize all skills at level 1
        for skill_type in SkillType::all_skills() {
            skills.insert(skill_type, SkillLineV2::default());
        }
        
        Self {
            skills,
            rested_experience_bonus: 1.0,
            rested_time_remaining: 0.0,
        }
    }
}

impl CharacterSkills {
    /// Get skill level for a specific skill type
    pub fn get_skill_level(&self, skill_type: SkillType) -> u32 {
        self.skills.get(&skill_type).map(|s| s.level).unwrap_or(1)
    }
    
    /// Get skill line for a specific skill type
    pub fn get_skill_line(&self, skill_type: SkillType) -> Option<&SkillLineV2> {
        self.skills.get(&skill_type)
    }
    
    /// Get mutable skill line for a specific skill type
    pub fn get_skill_line_mut(&mut self, skill_type: SkillType) -> Option<&mut SkillLineV2> {
        self.skills.get_mut(&skill_type)
    }
    
    /// Use a skill and gain experience
    pub fn use_skill(&mut self, 
                     skill_type: SkillType, 
                     target_level: u32, 
                     effectiveness: f32) -> bool {
        if let Some(skill_line) = self.skills.get_mut(&skill_type) {
            let exp_gained = skill_line.calculate_usage_experience(
                target_level, 
                effectiveness, 
                self.rested_experience_bonus
            );
            
            skill_line.gain_experience(exp_gained)
        } else {
            false
        }
    }
    
    /// Apply rested bonus from staying at inns/campfires
    pub fn apply_rested_bonus(&mut self, bonus_multiplier: f32, duration_seconds: f32) {
        self.rested_experience_bonus = bonus_multiplier;
        self.rested_time_remaining = duration_seconds;
    }
    
    /// Update rested bonus over time
    pub fn update_rested_bonus(&mut self, delta_seconds: f32) {
        if self.rested_time_remaining > 0.0 {
            self.rested_time_remaining -= delta_seconds;
            if self.rested_time_remaining <= 0.0 {
                self.rested_experience_bonus = 1.0;
                self.rested_time_remaining = 0.0;
            }
        }
    }
    
    /// Get total skill investment that suggests a particular role
    /// Note: This is for guidance only, not restrictive role assignment
    pub fn get_role_capability_score(&self, role: RoleType) -> u32 {
        SkillType::all_skills()
            .iter()
            .filter_map(|skill| {
                let skill_level = self.get_skill_level(*skill);
                if skill.suggested_roles().contains(&role) {
                    Some(skill_level)
                } else {
                    None
                }
            })
            .sum()
    }
    
    /// Get all roles this character could reasonably fulfill based on skill investments
    /// Returns roles with a minimum capability threshold
    pub fn get_viable_roles(&self, min_capability: u32) -> Vec<RoleType> {
        let all_roles = vec![
            RoleType::Tank, 
            RoleType::Healer, 
            RoleType::DPS, 
            RoleType::Support, 
            RoleType::Utility
        ];
        
        all_roles.into_iter()
            .filter(|&role| self.get_role_capability_score(role) >= min_capability)
            .collect()
    }
    
    /// Get suggested primary role based on highest capability score (for UI suggestions only)
    /// Note: This doesn't restrict the player - just provides a suggestion
    pub fn get_suggested_primary_role(&self) -> RoleType {
        let all_roles = vec![
            RoleType::Tank, 
            RoleType::Healer, 
            RoleType::DPS, 
            RoleType::Support, 
            RoleType::Utility
        ];
        
        all_roles.into_iter()
            .max_by_key(|&role| self.get_role_capability_score(role))
            .unwrap_or(RoleType::DPS) // Default to DPS if no clear preference
    }
    
    /// Check if character has a versatile build (can fulfill multiple roles)
    pub fn is_versatile_build(&self) -> bool {
        self.get_viable_roles(30).len() >= 2
    }
    
    /// Get highest skill level (used for some calculations)
    pub fn highest_skill_level(&self) -> u32 {
        self.skills.values().map(|s| s.level).max().unwrap_or(1)
    }
    
    /// Get average skill level across all skills
    pub fn average_skill_level(&self) -> f32 {
        let total_levels: u32 = self.skills.values().map(|s| s.level).sum();
        total_levels as f32 / self.skills.len() as f32
    }
}

/// Equipment types for loadout system - extensible for new weapon types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WeaponType {
    // One-handed Melee Weapons
    Sword,
    Axe,
    Mace,
    Hammer,
    Dagger,
    
    // Two-handed Melee Weapons
    TwoHandedSword,
    TwoHandedAxe,
    TwoHandedMace,
    TwoHandedHammer,
    Spear,
    Pike,
    
    // Shields and Off-hand Items
    Shield,
    MagicalOrb,
    
    // Ranged Weapons
    Bow,
    Longbow,
    Crossbow,
    HandCrossbow,
    ThrowingKnife,
    ThrowingAxe,
    Javelin,
    
    // Magic Staves - organized by magic school
    FireStaff,
    IceStaff,
    LightningStaff,
    ShadowStaff,
    NatureStaff,
    ArcaneStaff,
    RestorationStaff,
    DivinationStaff,
}

impl WeaponType {
    /// Get the skill type associated with this weapon - extensible mapping
    pub fn associated_skill(&self) -> SkillType {
        match self {
            // Sword weapons
            WeaponType::Sword | WeaponType::TwoHandedSword => SkillType::Swordsmanship,
            
            // Axe weapons
            WeaponType::Axe | WeaponType::TwoHandedAxe => SkillType::AxeMastery,
            
            // Mace/Hammer weapons
            WeaponType::Mace | WeaponType::TwoHandedMace => SkillType::MaceSkill,
            WeaponType::Hammer | WeaponType::TwoHandedHammer => SkillType::HammerSkill,
            
            // Spear weapons
            WeaponType::Spear | WeaponType::Pike => SkillType::SpearMastery,
            
            // Shield
            WeaponType::Shield => SkillType::ShieldDefense,
            
            // Bow weapons
            WeaponType::Bow | WeaponType::Longbow => SkillType::Archery,
            WeaponType::Crossbow | WeaponType::HandCrossbow => SkillType::Crossbow,
            
            // Dagger and throwing weapons
            WeaponType::Dagger => SkillType::DaggerMastery,
            WeaponType::ThrowingKnife | WeaponType::ThrowingAxe | 
            WeaponType::Javelin => SkillType::ThrowingWeapons,
            
            // Magic staves by school
            WeaponType::FireStaff => SkillType::FireMagic,
            WeaponType::IceStaff => SkillType::IceMagic,
            WeaponType::LightningStaff => SkillType::LightningMagic,
            WeaponType::ShadowStaff => SkillType::ShadowMagic,
            WeaponType::NatureStaff => SkillType::NatureMagic,
            WeaponType::ArcaneStaff => SkillType::ArcanieMagic,
            WeaponType::RestorationStaff => SkillType::Restoration,
            WeaponType::DivinationStaff => SkillType::Divination,
            
            // Orbs follow the wielder's primary magic skill (default to arcane)
            WeaponType::MagicalOrb => SkillType::ArcanieMagic,
        }
    }
    
    /// Check if this weapon requires two hands - comprehensive coverage
    pub fn is_two_handed(&self) -> bool {
        matches!(self, 
            // Two-handed melee weapons
            WeaponType::TwoHandedSword | WeaponType::TwoHandedAxe |
            WeaponType::TwoHandedMace | WeaponType::TwoHandedHammer |
            WeaponType::Spear | WeaponType::Pike |
            
            // Ranged weapons (most are two-handed)
            WeaponType::Bow | WeaponType::Longbow | WeaponType::Crossbow |
            
            // All magic staves are two-handed
            WeaponType::FireStaff | WeaponType::IceStaff | WeaponType::LightningStaff |
            WeaponType::ShadowStaff | WeaponType::NatureStaff | WeaponType::ArcaneStaff |
            WeaponType::RestorationStaff | WeaponType::DivinationStaff
        )
    }
    
    /// Get weapon damage type for future damage system integration
    pub fn damage_type(&self) -> DamageType {
        match self {
            // Physical melee damage
            WeaponType::Sword | WeaponType::TwoHandedSword => DamageType::Slashing,
            WeaponType::Axe | WeaponType::TwoHandedAxe => DamageType::Slashing,
            WeaponType::Mace | WeaponType::TwoHandedMace => DamageType::Bludgeoning,
            WeaponType::Hammer | WeaponType::TwoHandedHammer => DamageType::Bludgeoning,
            WeaponType::Spear | WeaponType::Pike => DamageType::Piercing,
            WeaponType::Dagger => DamageType::Piercing,
            
            // Ranged physical damage
            WeaponType::Bow | WeaponType::Longbow | WeaponType::Crossbow | 
            WeaponType::HandCrossbow => DamageType::Piercing,
            WeaponType::ThrowingKnife | WeaponType::Javelin => DamageType::Piercing,
            WeaponType::ThrowingAxe => DamageType::Slashing,
            
            // Magic damage by school
            WeaponType::FireStaff => DamageType::Fire,
            WeaponType::IceStaff => DamageType::Ice,
            WeaponType::LightningStaff => DamageType::Lightning,
            WeaponType::ShadowStaff => DamageType::Shadow,
            WeaponType::NatureStaff => DamageType::Nature,
            WeaponType::ArcaneStaff => DamageType::Arcane,
            WeaponType::RestorationStaff => DamageType::Healing,
            WeaponType::DivinationStaff => DamageType::Psychic,
            
            // Special items
            WeaponType::Shield => DamageType::Bludgeoning,
            WeaponType::MagicalOrb => DamageType::Arcane,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArmorType {
    Heavy,   // Plate, chainmail - reduces damage, supports tanking
    Light,   // Cloth, leather - increases mana, supports spellcasting
    Medium,  // Balanced - compromise between protection and mobility
}

impl ArmorType {
    /// Get the skill type associated with this armor
    pub fn associated_skill(&self) -> Option<SkillType> {
        match self {
            ArmorType::Heavy => Some(SkillType::HeavyArmor),
            ArmorType::Light => Some(SkillType::LightArmor),
            ArmorType::Medium => None, // Medium armor doesn't have specific skill requirements
        }
    }
}

/// Individual loadout configuration for role switching
#[derive(Debug, Clone)]
pub struct Loadout {
    pub name: String,
    pub primary_weapon: WeaponType,
    pub secondary_item: Option<WeaponType>, // Shield, off-hand weapon, or orb
    pub armor_type: ArmorType,
    pub active_abilities: Vec<String>, // Ability names/IDs for hotbar
}

impl Loadout {
    /// Get suggested roles this loadout could fulfill
    /// Note: This provides suggestions, not restrictions - players choose their actual role
    pub fn suggested_roles(&self) -> Vec<RoleType> {
        let mut roles = Vec::new();
        
        // Tank suggestions - defensive combinations
        if matches!(self.secondary_item, Some(WeaponType::Shield)) || 
           matches!(self.armor_type, ArmorType::Heavy) {
            roles.push(RoleType::Tank);
        }
        
        // Healer suggestions - restoration focused
        if matches!(self.primary_weapon, WeaponType::RestorationStaff | WeaponType::DivinationStaff) {
            roles.push(RoleType::Healer);
        }
        
        // DPS suggestions - most weapon combinations can do damage
        if !matches!(self.primary_weapon, WeaponType::Shield) {
            roles.push(RoleType::DPS);
        }
        
        // Support suggestions - equipment-agnostic role
        roles.push(RoleType::Support);
        
        // Utility suggestions - light armor builds
        if matches!(self.armor_type, ArmorType::Light) || 
           matches!(self.primary_weapon, WeaponType::Dagger) {
            roles.push(RoleType::Utility);
        }
        
        // Always return at least DPS as an option
        if roles.is_empty() {
            roles.push(RoleType::DPS);
        }
        
        roles
    }
    
    /// Get the most likely role suggestion for this loadout (for UI convenience)
    pub fn primary_suggested_role(&self) -> RoleType {
        let suggested = self.suggested_roles();
        
        // Prioritize specialized roles over generic ones
        if suggested.contains(&RoleType::Tank) && matches!(self.secondary_item, Some(WeaponType::Shield)) {
            RoleType::Tank
        } else if suggested.contains(&RoleType::Healer) {
            RoleType::Healer
        } else {
            suggested.first().copied().unwrap_or(RoleType::DPS)
        }
    }
    
    /// Check if this loadout is valid given character's skill levels
    pub fn is_valid_for_skills(&self, skills: &CharacterSkills) -> bool {
        // Check primary weapon skill requirement
        let primary_skill_level = skills.get_skill_level(self.primary_weapon.associated_skill());
        if primary_skill_level < 5 { // Minimum skill level to use advanced loadouts
            return false;
        }
        
        // Check secondary item skill requirement
        if let Some(secondary) = &self.secondary_item {
            let secondary_skill_level = skills.get_skill_level(secondary.associated_skill());
            if secondary_skill_level < 5 {
                return false;
            }
        }
        
        // Check armor skill requirement
        if let Some(armor_skill) = self.armor_type.associated_skill() {
            let armor_skill_level = skills.get_skill_level(armor_skill);
            if armor_skill_level < 10 {
                return false;
            }
        }
        
        true
    }
}

/// Component for managing character loadouts
#[derive(Component, Debug, Clone)]
pub struct CharacterLoadouts {
    pub loadouts: Vec<Loadout>,
    pub active_loadout_index: usize,
    pub can_switch_loadouts: bool, // True when at rest points
}

impl Default for CharacterLoadouts {
    fn default() -> Self {
        // Start with a basic DPS loadout
        let default_loadout = Loadout {
            name: "Basic Warrior".to_string(),
            primary_weapon: WeaponType::Sword,
            secondary_item: None,
            armor_type: ArmorType::Medium,
            active_abilities: vec![
                "Basic Slash".to_string(),
            ],
        };
        
        Self {
            loadouts: vec![default_loadout],
            active_loadout_index: 0,
            can_switch_loadouts: false,
        }
    }
}

impl CharacterLoadouts {
    /// Get the currently active loadout
    pub fn active_loadout(&self) -> Option<&Loadout> {
        self.loadouts.get(self.active_loadout_index)
    }
    
    /// Switch to a different loadout (only when at rest points)
    pub fn switch_loadout(&mut self, index: usize) -> bool {
        if self.can_switch_loadouts && index < self.loadouts.len() {
            self.active_loadout_index = index;
            true
        } else {
            false
        }
    }
    
    /// Add a new loadout
    pub fn add_loadout(&mut self, loadout: Loadout) -> usize {
        self.loadouts.push(loadout);
        self.loadouts.len() - 1
    }
    
    /// Remove a loadout (can't remove if it's the only one or currently active)
    pub fn remove_loadout(&mut self, index: usize) -> bool {
        if self.loadouts.len() > 1 && index != self.active_loadout_index && index < self.loadouts.len() {
            self.loadouts.remove(index);
            
            // Adjust active index if necessary
            if self.active_loadout_index > index {
                self.active_loadout_index -= 1;
            }
            true
        } else {
            false
        }
    }
    
    /// Enable loadout switching (when at rest points)
    pub fn enable_switching(&mut self) {
        self.can_switch_loadouts = true;
    }
    
    /// Disable loadout switching (when leaving rest points)
    pub fn disable_switching(&mut self) {
        self.can_switch_loadouts = false;
    }
}