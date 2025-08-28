use bevy::prelude::*;
use std::{fs, path::Path};
use crate::components::dialogue::*;

/// System to load dialogue database from JSON files
pub fn load_dialogue_database(mut commands: Commands) {
    info!("📚 Loading dialogue database from configuration files");
    
    let dialogue_base_path = "config/dialogues";
    let mut dialogue_database = DialogueDatabase::default();
    
    // Load NPC dialogue files
    let npc_path = Path::new(dialogue_base_path).join("npcs");
    if let Ok(entries) = fs::read_dir(&npc_path) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".json") {
                    let file_path = entry.path();
                    match load_npc_dialogue_file(&file_path) {
                        Ok(npc_dialogue) => {
                            let npc_id = npc_dialogue.npc_id.clone();
                            dialogue_database.npcs.insert(npc_id.clone(), npc_dialogue);
                            info!("✅ Loaded NPC dialogue: {}", npc_id);
                        }
                        Err(e) => {
                            error!("❌ Failed to load dialogue file {:?}: {}", file_path, e);
                        }
                    }
                }
            }
        }
    } else {
        warn!("⚠️ NPC dialogue directory not found: {:?}", npc_path);
    }
    
    // Load common dialogue files
    let common_path = Path::new(dialogue_base_path).join("common");
    if let Ok(entries) = fs::read_dir(&common_path) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".json") {
                    let file_path = entry.path();
                    match load_common_dialogue_file(&file_path) {
                        Ok(common_nodes) => {
                            dialogue_database.common_phrases.extend(common_nodes);
                            info!("✅ Loaded common dialogue: {}", file_name);
                        }
                        Err(e) => {
                            error!("❌ Failed to load common dialogue file {:?}: {}", file_path, e);
                        }
                    }
                }
            }
        }
    }
    
    let npc_count = dialogue_database.npcs.len();
    let common_count = dialogue_database.common_phrases.len();
    
    commands.insert_resource(dialogue_database);
    commands.insert_resource(ActiveDialogue::default());
    
    info!("🎭 Dialogue database loaded with {} NPCs and {} common phrases", npc_count, common_count);
}

/// Load an individual NPC dialogue file
fn load_npc_dialogue_file(file_path: &Path) -> Result<NpcDialogue, Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string(file_path)?;
    let npc_dialogue: NpcDialogue = serde_json::from_str(&file_content)?;
    
    // Validate the dialogue structure
    validate_npc_dialogue(&npc_dialogue)?;
    
    Ok(npc_dialogue)
}

/// Load common dialogue phrases
fn load_common_dialogue_file(file_path: &Path) -> Result<std::collections::HashMap<String, DialogueNode>, Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string(file_path)?;
    let common_data: serde_json::Value = serde_json::from_str(&file_content)?;
    
    let mut common_phrases = std::collections::HashMap::new();
    
    if let Some(phrases_obj) = common_data.get("phrases").and_then(|p| p.as_object()) {
        for (phrase_id, phrase_data) in phrases_obj {
            match serde_json::from_value::<DialogueNode>(phrase_data.clone()) {
                Ok(dialogue_node) => {
                    common_phrases.insert(phrase_id.clone(), dialogue_node);
                }
                Err(e) => {
                    warn!("❌ Failed to parse common phrase {}: {}", phrase_id, e);
                }
            }
        }
    }
    
    Ok(common_phrases)
}

/// Validate NPC dialogue structure for common errors
fn validate_npc_dialogue(npc_dialogue: &NpcDialogue) -> Result<(), Box<dyn std::error::Error>> {
    // Check that default conversation exists
    if !npc_dialogue.conversations.contains_key(&npc_dialogue.default_conversation) {
        return Err(format!(
            "Default conversation '{}' not found for NPC '{}'", 
            npc_dialogue.default_conversation, 
            npc_dialogue.npc_id
        ).into());
    }
    
    // Validate conversation structure
    for (conversation_id, conversation) in &npc_dialogue.conversations {
        // Check that all conversations have a "start" node
        if !conversation.nodes.contains_key("start") {
            return Err(format!(
                "Conversation '{}' for NPC '{}' missing required 'start' node",
                conversation_id,
                npc_dialogue.npc_id
            ).into());
        }
        
        // Validate that all choice "next" references point to existing nodes
        for (node_id, node) in &conversation.nodes {
            for choice in &node.choices {
                if !choice.next.is_empty() && !conversation.nodes.contains_key(&choice.next) && choice.next != "end_conversation" {
                    warn!(
                        "⚠️ Choice '{}' in node '{}' of conversation '{}' references non-existent node '{}'",
                        choice.id, node_id, conversation_id, choice.next
                    );
                }
            }
        }
    }
    
    Ok(())
}

/// Hot reload system for dialogue files during development
pub fn hot_reload_dialogue_system(
    mut dialogue_db: ResMut<DialogueDatabase>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // F10 key for hot reloading dialogues
    if keyboard.just_pressed(KeyCode::F10) {
        info!("🔄 Hot reloading dialogue database...");
        
        // Clear existing dialogue
        dialogue_db.npcs.clear();
        dialogue_db.common_phrases.clear();
        
        // Reload dialogue files (simplified reload - in production this would be more sophisticated)
        let dialogue_base_path = "config/dialogues";
        let npc_path = Path::new(dialogue_base_path).join("npcs");
        
        if let Ok(entries) = fs::read_dir(&npc_path) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".json") {
                        let file_path = entry.path();
                        match load_npc_dialogue_file(&file_path) {
                            Ok(npc_dialogue) => {
                                let npc_id = npc_dialogue.npc_id.clone();
                                dialogue_db.npcs.insert(npc_id.clone(), npc_dialogue);
                                info!("✅ Reloaded NPC dialogue: {}", npc_id);
                            }
                            Err(e) => {
                                error!("❌ Failed to reload dialogue file {:?}: {}", file_path, e);
                            }
                        }
                    }
                }
            }
        }
        
        info!("✅ Dialogue database hot reload completed!");
    }
}

/// System to validate dialogue requirements against player state
pub fn validate_dialogue_choice_requirements(
    choice_requirements: &ChoiceRequirements,
    _player_skills: &crate::components::CharacterSkills, // TODO: Implement skill checking
    _player_quest_log: &crate::components::QuestLog, // TODO: Implement quest checking
) -> bool {
    // For now, return true - we'll implement proper validation later
    // This is a placeholder for the full requirement checking system
    
    // TODO: Check skill requirements
    if let Some(_skills) = &choice_requirements.skills {
        // Validate player has required skill levels
    }
    
    // TODO: Check knowledge requirements
    if let Some(_knowledge) = &choice_requirements.knowledge {
        // Check if player has discovered required knowledge
    }
    
    // TODO: Check clue requirements
    if let Some(_clues) = &choice_requirements.clues {
        // Check if player has discovered required clues
    }
    
    // TODO: Check quest requirements  
    if let Some(_quests) = &choice_requirements.quests {
        // Check if player has completed/started required quests
    }
    
    // TODO: Check trust level requirements
    if let Some(_trust_level) = choice_requirements.trust_level {
        // Check if NPC trust level meets requirement
    }
    
    true // For now, allow all choices
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dialogue_validation() {
        // Test dialogue validation logic
        let test_dialogue = NpcDialogue {
            npc_id: "test_npc".to_string(),
            name: "Test NPC".to_string(),
            description: "A test NPC".to_string(),
            default_conversation: "greeting".to_string(),
            conversations: std::collections::HashMap::new(),
            relationship_effects: RelationshipEffects {
                trust_building: std::collections::HashMap::new(),
                trust_damaging: std::collections::HashMap::new(),
            },
            personality_traits: PersonalityTraits {
                nervous_disposition: None,
                merchant_instincts: None,
                guilt_burden: None,
                desperation_level: None,
                friendliness: None,
                suspicion_level: None,
                helpfulness: None,
            },
        };
        
        // Should fail validation because default conversation doesn't exist
        assert!(validate_npc_dialogue(&test_dialogue).is_err());
    }
}