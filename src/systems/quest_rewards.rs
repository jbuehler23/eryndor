use bevy::prelude::*;
use crate::components::{Player, CharacterLevel};

/// Quest reward system with level-scaling experience
/// Implements dynamic experience scaling based on player level
/// rather than static config values
pub struct QuestRewardSystem;

impl QuestRewardSystem {
    /// Calculate base quest experience that scales with player level
    /// Uses the character progression formula to ensure meaningful rewards
    pub fn calculate_quest_experience(quest_type: QuestType, player_level: u32) -> u64 {
        let base_reward = match quest_type {
            QuestType::MainStory => 1000,
            QuestType::SideQuest => 300, 
            QuestType::DailyQuest => 150,
            QuestType::Repeatable => 75,
        };

        // Scale reward based on player level using progression formula
        // Higher level players need proportionally more experience
        let level_scale_factor = if player_level <= 1 {
            1.0
        } else {
            // Use similar scaling to character progression: level^1.5 for smooth curve
            (player_level as f64).powf(1.5) / 10.0
        };

        // Minimum scaling of 1.0, maximum scaling based on level
        let scaled_factor = level_scale_factor.max(1.0);
        
        (base_reward as f64 * scaled_factor) as u64
    }

    /// Award quest experience to player
    pub fn award_quest_experience(
        character_level: &mut CharacterLevel, 
        quest_type: QuestType
    ) -> bool {
        let experience = Self::calculate_quest_experience(quest_type, character_level.level);
        character_level.gain_experience(experience)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestType {
    MainStory,
    SideQuest, 
    DailyQuest,
    Repeatable,
}

/// Debug system to test quest reward scaling
pub fn debug_quest_rewards_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut CharacterLevel, With<Player>>,
) {
    if keyboard.pressed(KeyCode::ControlLeft) && keyboard.just_pressed(KeyCode::F7) {
        if let Ok(mut character_level) = player_query.single_mut() {
            let quest_type = QuestType::SideQuest;
            let experience = QuestRewardSystem::calculate_quest_experience(quest_type, character_level.level);
            let leveled_up = character_level.gain_experience(experience);
            
            info!(
                "DEBUG: Completed {:?} quest! Awarded {} XP (scaled for level {}). Level {} ({:.1}% to next){}",
                quest_type,
                experience,
                character_level.level,
                character_level.level,
                character_level.level_progress() * 100.0,
                if leveled_up { " - level increased" } else { "" }
            );
        }
    }

    // F8 for main story quest
    if keyboard.pressed(KeyCode::ControlLeft) && keyboard.just_pressed(KeyCode::F8) {
        if let Ok(mut character_level) = player_query.single_mut() {
            let quest_type = QuestType::MainStory;
            let experience = QuestRewardSystem::calculate_quest_experience(quest_type, character_level.level);
            let leveled_up = character_level.gain_experience(experience);
            
            info!(
                "DEBUG: Completed {:?} quest! Awarded {} XP (scaled for level {}). Level {} ({:.1}% to next){}",
                quest_type,
                experience,
                character_level.level,
                character_level.level,
                character_level.level_progress() * 100.0,
                if leveled_up { " - level increased" } else { "" }
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quest_experience_scaling() {
        // Level 1 player gets base experience
        assert_eq!(QuestRewardSystem::calculate_quest_experience(QuestType::SideQuest, 1), 300);
        
        // Higher level players get more experience
        let level_5_exp = QuestRewardSystem::calculate_quest_experience(QuestType::SideQuest, 5);
        let level_10_exp = QuestRewardSystem::calculate_quest_experience(QuestType::SideQuest, 10);
        let level_20_exp = QuestRewardSystem::calculate_quest_experience(QuestType::SideQuest, 20);
        
        assert!(level_5_exp > 300);
        assert!(level_10_exp > level_5_exp);
        assert!(level_20_exp > level_10_exp);
        
        info!("Quest experience scaling: L1={}, L5={}, L10={}, L20={}", 
              300, level_5_exp, level_10_exp, level_20_exp);
    }

    #[test]
    fn test_main_story_vs_side_quest_rewards() {
        let player_level = 10;
        let main_story_exp = QuestRewardSystem::calculate_quest_experience(QuestType::MainStory, player_level);
        let side_quest_exp = QuestRewardSystem::calculate_quest_experience(QuestType::SideQuest, player_level);
        
        // Main story should always give more than side quests
        assert!(main_story_exp > side_quest_exp);
        
        // Should maintain the relative ratio (main story is ~3.33x base side quest)
        let ratio = main_story_exp as f64 / side_quest_exp as f64;
        assert!((ratio - 3.33).abs() < 0.1); // Within 10% of expected ratio
    }
}