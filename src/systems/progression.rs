use bevy::prelude::*;
use crate::components::{Player, CharacterLevel, CharacterSkills, CharacterLoadouts, SkillType, WeaponType};
use crate::components::progression::RoleType;

/// System to handle character level progression based on skill advancement
/// Characters gain experience and level up when experience thresholds are met
pub fn character_level_system(
    mut player_query: Query<(&mut CharacterLevel, &CharacterSkills), With<Player>>,
) {
    for (mut character_level, skills) in player_query.iter_mut() {
        // Award character experience based on skill usage
        // This is a simplified system - in full implementation, XP would come from:
        // - Combat kills
        // - Quest completion  
        // - Exploration discoveries
        // - Crafting achievements
        
        let highest_skill = skills.highest_skill_level();
        let expected_character_level = std::cmp::min(50, highest_skill + (skills.average_skill_level() / 5.0) as u32);
        
        // If skills have outpaced character level, award catch-up experience
        if expected_character_level > character_level.level {
            let catch_up_exp = CharacterLevel::calculate_experience_for_level(expected_character_level) 
                - CharacterLevel::calculate_experience_for_level(character_level.level);
            
            if character_level.gain_experience(catch_up_exp / 4) { // Gradual catch-up
                info!("Character level increased to {}", character_level.level);
            }
        }
    }
}

/// System to handle skill-by-use progression
/// Skill-by-use progression system
pub fn skill_usage_system(
    time: Res<Time>,
    mut player_query: Query<&mut CharacterSkills, With<Player>>,
) {
    let dt = time.delta_secs();
    
    for mut skills in player_query.iter_mut() {
        // Update rested bonus timer
        skills.update_rested_bonus(dt);
        
        // Simulate skill usage for testing - in real gameplay, this would be triggered by:
        // - Using abilities (sword skills from sword attacks)
        // - Casting spells (magic skills from spell casting)
        // - Taking damage (armor skills from wearing armor while taking damage)
        // - Successfully completing skill-related actions
        
        // For testing purposes, we'll simulate gradual skill advancement
        // This would be removed in production and replaced with event-driven skill usage
        
        let simulation_rate = 0.1; // Very slow for testing
        if dt > 0.0 && skills.rested_experience_bonus > 1.0 {
            // Simulate skill usage based on viable roles - in real gameplay this would be event-driven
            let viable_roles = skills.get_viable_roles(20); // Roles with at least 20 total skill investment
            let suggested_role = skills.get_suggested_primary_role();
            
            // Practice skills based on current focus
            match suggested_role {
                RoleType::Tank if viable_roles.contains(&RoleType::Tank) => {
                    if skills.use_skill(SkillType::Swordsmanship, 10, simulation_rate) {
                        info!("Swordsmanship skill improved!");
                    }
                    if skills.use_skill(SkillType::ShieldDefense, 10, simulation_rate) {
                        info!("Shield Defense skill improved!");
                    }
                    if skills.use_skill(SkillType::HeavyArmor, 10, simulation_rate) {
                        info!("Heavy Armor skill improved!");
                    }
                },
                RoleType::Healer if viable_roles.contains(&RoleType::Healer) => {
                    if skills.use_skill(SkillType::Restoration, 10, simulation_rate) {
                        info!("Restoration magic skill improved!");
                    }
                    if skills.use_skill(SkillType::LightArmor, 10, simulation_rate) {
                        info!("Light Armor skill improved!");
                    }
                },
                RoleType::Support if viable_roles.contains(&RoleType::Support) => {
                    if skills.use_skill(SkillType::Smithing, 10, simulation_rate) {
                        info!("Smithing skill improved!");
                    }
                    if skills.use_skill(SkillType::Alchemy, 10, simulation_rate) {
                        info!("Alchemy skill improved!");
                    }
                },
                RoleType::Utility if viable_roles.contains(&RoleType::Utility) => {
                    if skills.use_skill(SkillType::Stealth, 10, simulation_rate) {
                        info!("Stealth skill improved!");
                    }
                    if skills.use_skill(SkillType::Athletics, 10, simulation_rate) {
                        info!("Athletics skill improved!");
                    }
                },
                _ => {
                    // Default DPS skills or well-rounded development
                    if skills.use_skill(SkillType::FireMagic, 10, simulation_rate) {
                        info!("Fire Magic skill improved!");
                    }
                    if skills.use_skill(SkillType::Archery, 10, simulation_rate) {
                        info!("Archery skill improved!");
                    }
                },
            }
        }
    }
}

/// System to manage loadout switching at rest points
/// This enforces the rule that loadouts can only be changed at inns, campfires, etc.
pub fn loadout_management_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut CharacterLoadouts, &CharacterSkills), With<Player>>,
) {
    for (mut loadouts, skills) in player_query.iter_mut() {
        // Debug: Toggle ability to switch loadouts with F1
        if keyboard.just_pressed(KeyCode::F1) {
            if loadouts.can_switch_loadouts {
                loadouts.disable_switching();
                info!("🏃 Left rest point - loadout switching disabled");
            } else {
                loadouts.enable_switching();
                info!("🏕️ At rest point - loadout switching enabled");
            }
        }
        
        // Debug: Switch between loadouts with F2/F3 when allowed
        if loadouts.can_switch_loadouts {
            if keyboard.just_pressed(KeyCode::F2) && loadouts.loadouts.len() > 1 {
                let new_index = if loadouts.active_loadout_index == 0 { 
                    loadouts.loadouts.len() - 1 
                } else { 
                    loadouts.active_loadout_index - 1 
                };
                
                if loadouts.switch_loadout(new_index) {
                    if let Some(loadout) = loadouts.active_loadout() {
                        info!("Switched to loadout: {} (Suggested Role: {:?})", loadout.name, loadout.primary_suggested_role());
                    }
                }
            }
            
            if keyboard.just_pressed(KeyCode::F3) {
                let new_index = (loadouts.active_loadout_index + 1) % loadouts.loadouts.len();
                
                if loadouts.switch_loadout(new_index) {
                    if let Some(loadout) = loadouts.active_loadout() {
                        info!("Switched to loadout: {} (Suggested Role: {:?})", loadout.name, loadout.primary_suggested_role());
                    }
                }
            }
            
            // Debug: Create new loadout with F4
            if keyboard.just_pressed(KeyCode::F4) {
                let role_capabilities = [
                    (RoleType::Tank, skills.get_role_capability_score(RoleType::Tank)),
                    (RoleType::Healer, skills.get_role_capability_score(RoleType::Healer)),
                    (RoleType::DPS, skills.get_role_capability_score(RoleType::DPS)),
                    (RoleType::Support, skills.get_role_capability_score(RoleType::Support)),
                    (RoleType::Utility, skills.get_role_capability_score(RoleType::Utility)),
                ];
                
                // Create loadout for the role with highest capability score
                let (primary_role, _) = role_capabilities.iter().max_by_key(|(_, score)| *score).unwrap();
                
                let new_loadout = match primary_role {
                    RoleType::Tank => create_tank_loadout(skills),
                    RoleType::Healer => create_healer_loadout(skills),
                    RoleType::DPS => create_dps_loadout(skills),
                    RoleType::Support => create_support_loadout(skills),
                    RoleType::Utility => create_utility_loadout(skills),
                };
                
                let index = loadouts.add_loadout(new_loadout);
                loadouts.switch_loadout(index);
                
                if let Some(loadout) = loadouts.active_loadout() {
                    info!("Created new {} loadout: {}", 
                          match primary_role {
                              RoleType::Tank => "Tank",
                              RoleType::Healer => "Healer", 
                              RoleType::DPS => "DPS",
                              RoleType::Support => "Support",
                              RoleType::Utility => "Utility",
                          },
                          loadout.name
                    );
                }
            }
        }
    }
}

/// Create a tank loadout based on character's skills
fn create_tank_loadout(skills: &CharacterSkills) -> crate::components::Loadout {
    let sword_level = skills.get_skill_level(SkillType::Swordsmanship);
    let axe_level = skills.get_skill_level(SkillType::AxeMastery);
    let shield_level = skills.get_skill_level(SkillType::ShieldDefense);
    
    let primary_weapon = if sword_level >= axe_level {
        WeaponType::Sword
    } else {
        WeaponType::Axe
    };
    
    let secondary_item = if shield_level >= 5 {
        Some(WeaponType::Shield)
    } else {
        None
    };
    
    crate::components::Loadout {
        name: "Tank Build".to_string(),
        primary_weapon,
        secondary_item,
        armor_type: crate::components::ArmorType::Heavy,
        active_abilities: vec![
            "Basic Slash".to_string(),
            if shield_level >= 1 { "Shield Bash".to_string() } else { "Heavy Strike".to_string() },
            if shield_level >= 10 { "Taunt".to_string() } else { "Block".to_string() },
        ],
    }
}

/// Create a healer loadout based on character's skills
fn create_healer_loadout(skills: &CharacterSkills) -> crate::components::Loadout {
    let restoration_level = skills.get_skill_level(SkillType::Restoration);
    
    crate::components::Loadout {
        name: "Healer Build".to_string(),
        primary_weapon: WeaponType::RestorationStaff,
        secondary_item: None,
        armor_type: crate::components::ArmorType::Light,
        active_abilities: vec![
            "Heal".to_string(),
            if restoration_level >= 5 { "Renew".to_string() } else { "Heal".to_string() },
            if restoration_level >= 10 { "Group Heal".to_string() } else { "Renew".to_string() },
        ],
    }
}

/// Create a DPS loadout based on character's skills  
fn create_dps_loadout(skills: &CharacterSkills) -> crate::components::Loadout {
    let fire_level = skills.get_skill_level(SkillType::FireMagic);
    let archery_level = skills.get_skill_level(SkillType::Archery);
    let sword_level = skills.get_skill_level(SkillType::Swordsmanship);
    
    // Choose weapon based on highest DPS skill
    let (primary_weapon, armor_type) = if fire_level >= archery_level && fire_level >= sword_level {
        (WeaponType::FireStaff, crate::components::ArmorType::Light)
    } else if archery_level >= sword_level {
        (WeaponType::Bow, crate::components::ArmorType::Medium)
    } else {
        (WeaponType::TwoHandedSword, crate::components::ArmorType::Medium)
    };
    
    crate::components::Loadout {
        name: "DPS Build".to_string(),
        primary_weapon,
        secondary_item: None,
        armor_type,
        active_abilities: vec![
            match primary_weapon {
                WeaponType::FireStaff => "Firebolt".to_string(),
                WeaponType::Bow => "Quick Shot".to_string(),
                _ => "Basic Slash".to_string(),
            },
        ],
    }
}

/// Create a support loadout based on character's skills
fn create_support_loadout(skills: &CharacterSkills) -> crate::components::Loadout {
    let smithing_level = skills.get_skill_level(SkillType::Smithing);
    let alchemy_level = skills.get_skill_level(SkillType::Alchemy);
    
    // Support builds focus on crafting and utility
    let primary_weapon = if smithing_level >= alchemy_level {
        WeaponType::Hammer // For smithing
    } else {
        WeaponType::Dagger // For gathering reagents
    };
    
    crate::components::Loadout {
        name: "Support Build".to_string(),
        primary_weapon,
        secondary_item: None,
        armor_type: crate::components::ArmorType::Medium, // Balanced protection
        active_abilities: vec![
            "Craft".to_string(),
            "Repair".to_string(),
            "Gather".to_string(),
        ],
    }
}

/// Create a utility loadout based on character's skills
fn create_utility_loadout(skills: &CharacterSkills) -> crate::components::Loadout {
    let stealth_level = skills.get_skill_level(SkillType::Stealth);
    let lockpicking_level = skills.get_skill_level(SkillType::Lockpicking);
    
    // Utility builds focus on stealth and utility skills
    let primary_weapon = WeaponType::Dagger; // Light, quiet weapon
    
    crate::components::Loadout {
        name: "Utility Build".to_string(),
        primary_weapon,
        secondary_item: None,
        armor_type: crate::components::ArmorType::Light, // For stealth
        active_abilities: vec![
            "Stealth".to_string(),
            if lockpicking_level >= 5 { "Advanced Lockpicking".to_string() } else { "Lockpicking".to_string() },
            if stealth_level >= 10 { "Invisibility".to_string() } else { "Hide".to_string() },
        ],
    }
}

/// Debug system to display character progression info
pub fn debug_character_v2_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug_visible: Local<bool>,
    player_query: Query<(&CharacterLevel, &CharacterSkills, &CharacterLoadouts), With<Player>>,
) {
    // Toggle debug display with Shift+F1
    if keyboard.pressed(KeyCode::ShiftLeft) && keyboard.just_pressed(KeyCode::F1) {
        *debug_visible = !*debug_visible;
        if *debug_visible {
            info!("Character progression debug enabled (Shift+F1 to disable)");
        } else {
            info!("Character progression debug disabled");
        }
    }
    
    // Display character info if enabled
    if *debug_visible {
        if let Ok((character_level, skills, loadouts)) = player_query.single() {
            info!(
                "CHARACTER: Level {} ({:.1}% to next) | Suggested Role: {:?} | Versatile: {} | Viable Roles: {:?}",
                character_level.level,
                character_level.level_progress() * 100.0,
                skills.get_suggested_primary_role(),
                skills.is_versatile_build(),
                skills.get_viable_roles(20)
            );
            
            info!(
                "SKILLS: Sword {} | Fire {} | Restoration {} | Shield {} | Heavy Armor {}",
                skills.get_skill_level(SkillType::Swordsmanship),
                skills.get_skill_level(SkillType::FireMagic),
                skills.get_skill_level(SkillType::Restoration),
                skills.get_skill_level(SkillType::ShieldDefense),
                skills.get_skill_level(SkillType::HeavyArmor)
            );
            
            if let Some(active_loadout) = loadouts.active_loadout() {
                info!(
                    "LOADOUT: '{}' | Weapon: {:?} | Secondary: {:?} | Armor: {:?} | Role: {:?}",
                    active_loadout.name,
                    active_loadout.primary_weapon,
                    active_loadout.secondary_item,
                    active_loadout.armor_type,
                    active_loadout.primary_suggested_role()
                );
            }
            
            info!(
                "REST STATUS: Can switch loadouts: {} | Rested bonus: {:.1}x ({}s remaining)",
                loadouts.can_switch_loadouts,
                skills.rested_experience_bonus,
                skills.rested_time_remaining
            );
            
            info!(
                "ROLE INVESTMENTS: Tank {} pts | Healer {} pts | DPS {} pts",
                skills.get_role_capability_score(RoleType::Tank),
                skills.get_role_capability_score(RoleType::Healer),
                skills.get_role_capability_score(RoleType::DPS)
            );
        }
    }
}

/// System to simulate rested bonus application (for testing)
pub fn debug_rested_bonus_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut CharacterSkills, With<Player>>,
) {
    // Ctrl+F1 to apply rested bonus
    if keyboard.pressed(KeyCode::ControlLeft) && keyboard.just_pressed(KeyCode::F1) {
        for mut skills in player_query.iter_mut() {
            skills.apply_rested_bonus(1.5, 300.0); // 50% bonus for 5 minutes
            info!("Applied rested bonus: +50% experience for 5 minutes");
        }
    }
}

/// Award experience for testing (Ctrl+Shift+F6)
pub fn debug_award_character_experience_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut CharacterLevel, With<Player>>,
) {
    if keyboard.pressed(KeyCode::ControlLeft) && keyboard.pressed(KeyCode::ShiftLeft) && keyboard.just_pressed(KeyCode::F6) {
        if let Ok(mut character_level) = player_query.single_mut() {
            let exp_award = 500;
            let leveled_up = character_level.gain_experience(exp_award);
            
            info!(
                "DEBUG: Awarded {} character XP! Level {} ({:.1}% to next){}",
                exp_award,
                character_level.level,
                character_level.level_progress() * 100.0,
                if leveled_up { " - level increased" } else { "" }
            );
        }
    }
}