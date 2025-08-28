use bevy::prelude::*;
use crate::components::dialogue::*;
use crate::components::{Player, QuestLog, CharacterSkills};

/// System to handle starting conversations with NPCs
pub fn enhanced_dialogue_interaction_system(
    _commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    dialogue_db: Res<DialogueDatabase>,
    mut active_dialogue: ResMut<ActiveDialogue>,
    mut dialogue_events: EventWriter<DialogueEvent>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut npc_dialogue_query: Query<&mut DialogueState>,
    npc_query: Query<(Entity, &Transform, &DialogueInteractable), Without<Player>>,
) {
    let Ok((player_entity, player_transform)) = player_query.single() else {
        return;
    };
    
    // Handle interaction key (E key)
    if keyboard.just_pressed(KeyCode::KeyE) {
        // If already in dialogue, end it
        if active_dialogue.npc_entity.is_some() {
            if let Some(npc_entity) = active_dialogue.npc_entity {
                dialogue_events.write(DialogueEvent::EndConversation {
                    npc_entity,
                    player_entity,
                });
                end_current_dialogue(&mut active_dialogue);
                info!("💬 Ended dialogue");
            }
            return;
        }
        
        // Find nearest interactable NPC
        let mut nearest_npc: Option<(Entity, f32, &DialogueInteractable)> = None;
        
        for (npc_entity, npc_transform, interactable) in &npc_query {
            let distance = player_transform.translation.distance(npc_transform.translation);
            
            if distance <= interactable.interaction_range {
                match nearest_npc {
                    None => nearest_npc = Some((npc_entity, distance, interactable)),
                    Some((_, nearest_distance, _)) => {
                        if distance < nearest_distance {
                            nearest_npc = Some((npc_entity, distance, interactable));
                        }
                    }
                }
            }
        }
        
        // Start conversation with nearest NPC
        if let Some((npc_entity, _, interactable)) = nearest_npc {
            // Get NPC dialogue from database
            if let Some(npc_dialogue) = dialogue_db.npcs.get(&interactable.npc_id) {
                // Get mutable reference to dialogue state
                if let Ok(mut dialogue_state) = npc_dialogue_query.get_mut(npc_entity) {
                    start_conversation(
                        &mut active_dialogue,
                        &mut dialogue_events,
                        npc_entity,
                        player_entity,
                        npc_dialogue,
                        &mut dialogue_state,
                    );
                }
            } else {
                warn!("❌ No dialogue found for NPC: {}", interactable.npc_id);
            }
        }
    }
    
    // Handle dialogue choice selection (1-4 keys)
    if active_dialogue.npc_entity.is_some() {
        handle_choice_selection(&keyboard, &mut active_dialogue, &mut dialogue_events);
    }
    
    // Update interaction indicators - moved to separate system to avoid query conflicts
}

/// Start a conversation with an NPC
fn start_conversation(
    active_dialogue: &mut ResMut<ActiveDialogue>,
    dialogue_events: &mut EventWriter<DialogueEvent>,
    npc_entity: Entity,
    player_entity: Entity,
    npc_dialogue: &NpcDialogue,
    dialogue_state: &mut DialogueState,
) {
    // Determine which conversation to start
    let conversation_id = dialogue_state.current_conversation
        .as_ref()
        .unwrap_or(&npc_dialogue.default_conversation)
        .clone();
    
    if let Some(conversation) = npc_dialogue.conversations.get(&conversation_id) {
        if let Some(start_node) = conversation.nodes.get(&dialogue_state.current_node) {
            // Set up active dialogue
            active_dialogue.npc_entity = Some(npc_entity);
            active_dialogue.player_entity = Some(player_entity);
            active_dialogue.current_node = Some(start_node.clone());
            active_dialogue.available_choices = start_node.choices.clone();
            active_dialogue.dialogue_history.clear();
            
            // Update NPC dialogue state
            dialogue_state.is_active = true;
            dialogue_state.current_conversation = Some(conversation_id.clone());
            
            // Send event
            dialogue_events.write(DialogueEvent::StartConversation {
                npc_entity,
                player_entity,
                conversation_id: Some(conversation_id),
            });
            
            info!("💬 Started conversation with NPC: {}", npc_dialogue.name);
            display_current_dialogue_node(active_dialogue, npc_dialogue);
        }
    }
}

/// Handle player choice selection during dialogue
fn handle_choice_selection(
    keyboard: &Res<ButtonInput<KeyCode>>,
    active_dialogue: &mut ResMut<ActiveDialogue>,
    dialogue_events: &mut EventWriter<DialogueEvent>,
) {
    let choice_index = if keyboard.just_pressed(KeyCode::Digit1) {
        Some(0)
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        Some(1)
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        Some(2)
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        Some(3)
    } else {
        None
    };
    
    if let Some(index) = choice_index {
        if index < active_dialogue.available_choices.len() {
            let choice = active_dialogue.available_choices[index].clone();
            
            if let Some(npc_entity) = active_dialogue.npc_entity {
                dialogue_events.write(DialogueEvent::ChoiceSelected {
                    npc_entity,
                    choice_id: choice.id.clone(),
                    next_node: choice.next.clone(),
                });
            }
            
            info!("💬 Selected choice: {}", choice.text);
        }
    }
}

/// Process dialogue choice and move to next node
pub fn process_dialogue_choice(
    mut dialogue_events: EventReader<DialogueEvent>,
    mut quest_events: EventWriter<crate::components::quest::QuestEvent>,
    dialogue_db: Res<DialogueDatabase>,
    mut active_dialogue: ResMut<ActiveDialogue>,
    mut npc_dialogue_states: Query<&mut DialogueState>,
    player_query: Query<Entity, With<Player>>,
) {
    for event in dialogue_events.read() {
        match event {
            DialogueEvent::ChoiceSelected { npc_entity, choice_id: _, next_node } => {
                if let Ok(mut dialogue_state) = npc_dialogue_states.get_mut(*npc_entity) {
                    // Handle special nodes
                    if next_node == "end_conversation" {
                        end_current_dialogue(&mut active_dialogue);
                        dialogue_state.is_active = false;
                        info!("💬 Conversation ended");
                        continue;
                    }
                    
                    // Get NPC dialogue data
                    if let Some(npc_dialogue) = dialogue_db.npcs.get(&dialogue_state.npc_id) {
                        if let Some(conversation) = npc_dialogue.conversations.get(
                            &dialogue_state.current_conversation.clone().unwrap_or_else(|| npc_dialogue.default_conversation.clone())
                        ) {
                            if let Some(next_dialogue_node) = conversation.nodes.get(next_node) {
                                // Update dialogue state
                                dialogue_state.current_node = next_node.clone();
                                dialogue_state.conversation_history.push(next_node.clone());
                                
                                // Update active dialogue
                                active_dialogue.current_node = Some(next_dialogue_node.clone());
                                active_dialogue.available_choices = next_dialogue_node.choices.clone();
                                
                                // Process any quest actions
                                if let Some(quest_action) = &next_dialogue_node.quest_action {
                                    process_quest_action(quest_action, &mut quest_events, *npc_entity);
                                }
                                
                                // Process clue flags
                                for clue_flag in &next_dialogue_node.clue_flags {
                                    dialogue_state.flags_set.push(clue_flag.clone());
                                    info!("🔍 Clue flag set: {}", clue_flag);
                                }
                                
                                display_current_dialogue_node(&active_dialogue, npc_dialogue);
                            }
                        }
                    }
                }
            },
            DialogueEvent::EndConversation { npc_entity, player_entity: _ } => {
                if let Ok(mut dialogue_state) = npc_dialogue_states.get_mut(*npc_entity) {
                    dialogue_state.is_active = false;
                }
                end_current_dialogue(&mut active_dialogue);
            },
            _ => {}
        }
    }
}

/// Process quest actions triggered by dialogue
fn process_quest_action(
    quest_action: &QuestAction,
    quest_events: &mut EventWriter<crate::components::quest::QuestEvent>,
    _npc_entity: Entity,
) {
    match quest_action.action_type.as_str() {
        "start_quest" => {
            if let Some(quest_id) = &quest_action.quest_id {
                // For now, just log the quest start - we'll integrate with the quest system later
                info!("🎯 Quest started via dialogue: {}", quest_id);
                // TODO: Integrate with existing quest start system
            }
        },
        "quest_assigned" => {
            if let (Some(quest_id), Some(phase)) = (&quest_action.quest_id, &quest_action.phase) {
                // Handle quest assignment logic here
                info!("📋 Quest assigned: {} (Phase: {})", quest_id, phase);
            }
        },
        "give_clue" => {
            if let Some(clues) = &quest_action.clues {
                for clue in clues {
                    info!("🔍 Clue provided via dialogue: {}", clue);
                }
            }
        },
        _ => {
            warn!("❓ Unknown quest action type: {}", quest_action.action_type);
        }
    }
}

/// Display the current dialogue node to console (temporary - will be replaced with UI)
fn display_current_dialogue_node(active_dialogue: &ActiveDialogue, npc_dialogue: &NpcDialogue) {
    if let Some(current_node) = &active_dialogue.current_node {
        info!("💬 {}: {}", npc_dialogue.name, current_node.text);
        
        if !active_dialogue.available_choices.is_empty() {
            info!("📝 Dialogue choices:");
            for (i, choice) in active_dialogue.available_choices.iter().enumerate() {
                info!("  {}. {}", i + 1, choice.text);
            }
        }
    }
}

/// End the current dialogue conversation
fn end_current_dialogue(active_dialogue: &mut ResMut<ActiveDialogue>) {
    active_dialogue.npc_entity = None;
    active_dialogue.player_entity = None;
    active_dialogue.current_node = None;
    active_dialogue.available_choices.clear();
    active_dialogue.dialogue_history.clear();
}

// Visual indicators are now handled by the NPC spawning system to avoid query conflicts

/// System to provide help text for dialogue system
pub fn enhanced_dialogue_help_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    active_dialogue: Res<ActiveDialogue>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        info!("💬 DIALOGUE SYSTEM HELP:");
        info!("  E - Interact with nearby NPC / End current dialogue");
        info!("  1-4 - Select dialogue choice (when in conversation)");
        info!("  F10 - Hot reload dialogue files (development)");
        
        if active_dialogue.npc_entity.is_some() {
            info!("  Currently in dialogue - use number keys to select responses");
        } else {
            info!("  Approach an NPC and press E to start conversation");
        }
    }
}