use bevy::prelude::*;
use std::fs;
use std::collections::HashMap;
use crate::components::{Player, QuestLog, QuestNpc, InvestigationObject};
use crate::components::quest::*;
use crate::resources::InputResource;

/// System to load quest definitions from JSON configuration
pub fn load_quest_database(mut commands: Commands) {
    info!("📚 Loading quest database from configuration files");
    
    let quest_config_path = "config/quests.json";
    match fs::read_to_string(quest_config_path) {
        Ok(quest_json) => {
            match serde_json::from_str::<serde_json::Value>(&quest_json) {
                Ok(quest_data) => {
                    if let Some(quests_obj) = quest_data.get("quests") {
                        let mut quest_database = QuestDatabase {
                            quests: HashMap::new(),
                            npcs: HashMap::new(),
                            locations: HashMap::new(),
                        };
                        
                        // Parse each quest definition
                        if let Some(quests_map) = quests_obj.as_object() {
                            for (quest_id, quest_data) in quests_map {
                                match serde_json::from_value::<QuestDefinition>(quest_data.clone()) {
                                    Ok(mut quest_def) => {
                                        quest_def.id = quest_id.clone();
                                        quest_database.quests.insert(quest_id.clone(), quest_def);
                                        info!("✅ Loaded quest: {}", quest_id);
                                    }
                                    Err(e) => {
                                        warn!("❌ Failed to parse quest {}: {}", quest_id, e);
                                    }
                                }
                            }
                        }
                        
                        let quest_count = quest_database.quests.len();
                        commands.insert_resource(quest_database);
                        info!("🎯 Quest database loaded with {} quests", quest_count);
                    }
                }
                Err(e) => {
                    error!("❌ Failed to parse quest JSON: {}", e);
                    commands.insert_resource(QuestDatabase {
                        quests: HashMap::new(),
                        npcs: HashMap::new(),
                        locations: HashMap::new(),
                    });
                }
            }
        }
        Err(e) => {
            warn!("⚠️ Could not load quest configuration: {}", e);
            commands.insert_resource(QuestDatabase {
                quests: HashMap::new(),
                npcs: HashMap::new(),
                locations: HashMap::new(),
            });
        }
    }
}

/// System to initialize quest log for new players
pub fn initialize_quest_log(
    mut commands: Commands,
    player_query: Query<Entity, (With<Player>, Without<QuestLog>)>,
) {
    for player_entity in &player_query {
        info!("📖 Initializing quest log for player");
        commands.entity(player_entity).insert(QuestLog::default());
    }
}

/// System to handle starting new quests
pub fn quest_start_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut QuestLog, With<Player>>,
    quest_db: Res<QuestDatabase>,
    time: Res<Time>,
) {
    // Debug key to start the merchant mystery quest
    if keyboard.just_pressed(KeyCode::F9) {
        if let Ok(mut quest_log) = player_query.single_mut() {
            let quest_id = "the_merchants_mystery";
            
            if !quest_log.active_quests.contains_key(quest_id) && !quest_log.completed_quests.contains_key(quest_id) {
                if let Some(quest_def) = quest_db.quests.get(quest_id) {
                    let quest_progress = QuestProgress {
                        quest_id: quest_id.to_string(),
                        current_phase: "initial_observation".to_string(),
                        completed_phases: Vec::new(),
                        discovered_clues: Vec::new(),
                        completed_objectives: Vec::new(),
                        failed_conditions: Vec::new(),
                        evidence_strength: EvidenceStrength::None,
                        investigation_notes: Vec::new(),
                        start_time: time.elapsed_secs_f64(),
                    };
                    
                    quest_log.active_quests.insert(quest_id.to_string(), quest_progress);
                    
                    info!("🎯 Started quest: {}", quest_def.title);
                    info!("📋 {}", quest_def.description);
                    info!("🏛️ Historical Context: {}", quest_def.lore_context.historical_background);
                    
                    // Find and display the first phase
                    if let Some(first_phase) = quest_def.phases.first() {
                        info!("🔍 Current Objective: {}", first_phase.title);
                        info!("📝 {}", first_phase.description);
                        
                        // Show available actions
                        if !first_phase.available_actions.is_empty() {
                            info!("🎬 Available Actions:");
                            for action in &first_phase.available_actions {
                                info!("  • {}", action.replace("_", " "));
                            }
                        }
                    }
                } else {
                    warn!("❌ Quest definition not found: {}", quest_id);
                }
            } else {
                info!("ℹ️ Quest already started or completed");
            }
        }
    }
}

/// System to handle quest investigation actions
pub fn investigation_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut QuestLog, With<Player>>,
    quest_db: Res<QuestDatabase>,
    time: Res<Time>,
    mut quest_events: EventWriter<QuestEvent>,
) {
    if let Ok(mut quest_log) = player_query.single_mut() {
        // Examine shop carefully (F10)
        if keyboard.just_pressed(KeyCode::F10) {
            handle_shop_examination(&mut quest_log, &quest_db, time.elapsed_secs_f64(), &mut quest_events);
        }
        
        // Question Aldric about travels (F11)
        if keyboard.just_pressed(KeyCode::F11) {
            handle_aldric_questioning(&mut quest_log, &quest_db, time.elapsed_secs_f64(), &mut quest_events);
        }
        
        // Check merchant guild records (F12)
        if keyboard.just_pressed(KeyCode::F12) {
            handle_guild_records_check(&mut quest_log, &quest_db, time.elapsed_secs_f64(), &mut quest_events);
        }
    }
}

fn handle_shop_examination(
    quest_log: &mut QuestLog,
    quest_db: &QuestDatabase,
    current_time: f64,
    quest_events: &mut EventWriter<QuestEvent>,
) {
    let quest_id = "the_merchants_mystery";
    
    if let Some(quest_progress) = quest_log.active_quests.get_mut(quest_id) {
        if quest_progress.current_phase == "initial_observation" {
            info!("🔍 You carefully examine Aldric's shop...");
            info!("The shelves are well-stocked with goods from various regions. Northern furs hang alongside southern silks.");
            info!("Behind the counter, you notice something glinting beneath a stack of ledgers...");
            
            // Discover the mysterious amulet clue
            if !quest_progress.discovered_clues.contains(&"mysterious_amulet".to_string()) {
                let clue = DiscoveredClue {
                    clue_id: "mysterious_amulet".to_string(),
                    quest_id: quest_id.to_string(),
                    discovery_time: current_time,
                    discovery_method: "Thorough examination of shop premises".to_string(),
                    description: "A bone amulet with strange markings hidden beneath the counter, still warm to the touch".to_string(),
                    importance_weight: 3,
                    related_clues: vec!["altered_ledger".to_string()],
                };
                
                quest_progress.discovered_clues.push("mysterious_amulet".to_string());
                quest_log.discovered_clues.insert("mysterious_amulet".to_string(), clue.clone());
                
                info!("🔥 CLUE DISCOVERED: {}", clue.description);
                info!("🤔 The amulet is warm despite the cool morning air. What could be causing this unnatural heat?");
                
                quest_events.write(QuestEvent::ClueDiscovered {
                    player_entity: Entity::PLACEHOLDER, // Would be actual player entity in real system
                    clue,
                });
            } else {
                info!("You've already thoroughly examined the shop and found the mysterious amulet.");
            }
            
            // Update evidence strength
            let clues: Vec<_> = quest_progress.discovered_clues.iter()
                .filter_map(|clue_id| quest_log.discovered_clues.get(clue_id))
                .cloned()
                .collect();
            quest_progress.evidence_strength = EvidenceStrength::calculate_from_clues(&clues);
        } else {
            info!("You've already moved past the initial observation phase of this investigation.");
        }
    } else {
        info!("You need to start the merchant's mystery quest first (F9).");
    }
}

fn handle_aldric_questioning(
    quest_log: &mut QuestLog,
    quest_db: &QuestDatabase,
    current_time: f64,
    quest_events: &mut EventWriter<QuestEvent>,
) {
    let quest_id = "the_merchants_mystery";
    
    if let Some(quest_progress) = quest_log.active_quests.get_mut(quest_id) {
        if quest_progress.current_phase == "initial_observation" {
            info!("💬 You approach Aldric with carefully chosen questions about his recent travels...");
            info!("'Ah, the northern routes!' he says, his voice brightening artificially. 'Made excellent time, I did!'");
            info!("'Only took me 8 days there and back, though I managed to acquire quite a collection.'");
            info!("🤨 You notice he's displaying goods that would typically take weeks to properly trade for...");
            
            // Discover the travel time inconsistency clue
            if !quest_progress.discovered_clues.contains(&"travel_time_inconsistency".to_string()) {
                let clue = DiscoveredClue {
                    clue_id: "travel_time_inconsistency".to_string(),
                    quest_id: quest_id.to_string(),
                    discovery_time: current_time,
                    discovery_method: "Careful questioning about travel logistics".to_string(),
                    description: "Aldric claims to have made the northern journey in 8 days, but the goods he carries suggest a 3-week expedition".to_string(),
                    importance_weight: 4,
                    related_clues: vec!["altered_ledger".to_string()],
                };
                
                quest_progress.discovered_clues.push("travel_time_inconsistency".to_string());
                quest_log.discovered_clues.insert("travel_time_inconsistency".to_string(), clue.clone());
                
                info!("⚡ CLUE DISCOVERED: {}", clue.description);
                info!("🎭 Something doesn't add up. Either Aldric has found a way to bend time, or he's not telling the whole truth...");
                
                quest_events.write(QuestEvent::ClueDiscovered {
                    player_entity: Entity::PLACEHOLDER,
                    clue,
                });
            } else {
                info!("You've already questioned Aldric about his travel times and noticed the inconsistency.");
            }
            
            // Update evidence strength
            let clues: Vec<_> = quest_progress.discovered_clues.iter()
                .filter_map(|clue_id| quest_log.discovered_clues.get(clue_id))
                .cloned()
                .collect();
            quest_progress.evidence_strength = EvidenceStrength::calculate_from_clues(&clues);
        } else {
            info!("You've already questioned Aldric extensively about his travels.");
        }
    } else {
        info!("You need to start the merchant's mystery quest first (F9).");
    }
}

fn handle_guild_records_check(
    quest_log: &mut QuestLog,
    quest_db: &QuestDatabase,
    current_time: f64,
    quest_events: &mut EventWriter<QuestEvent>,
) {
    let quest_id = "the_merchants_mystery";
    
    if let Some(quest_progress) = quest_log.active_quests.get_mut(quest_id) {
        if quest_progress.current_phase == "initial_observation" {
            info!("📚 You discretely examine the merchant guild's public records...");
            info!("Standard trading logs show normal travel times to northern settlements: 14-21 days typical.");
            info!("Cross-referencing with Aldric's recent entries...");
            info!("🔍 His ledger shows signs of alterations - dates scratched out and rewritten in different ink.");
            
            // Discover the altered ledger clue
            if !quest_progress.discovered_clues.contains(&"altered_ledger".to_string()) {
                let clue = DiscoveredClue {
                    clue_id: "altered_ledger".to_string(),
                    quest_id: quest_id.to_string(),
                    discovery_time: current_time,
                    discovery_method: "Academic analysis of documentation".to_string(),
                    description: "Recent entries in the merchant's ledger show signs of erasure and rewriting".to_string(),
                    importance_weight: 2,
                    related_clues: vec!["travel_time_inconsistency".to_string(), "mysterious_amulet".to_string()],
                };
                
                quest_progress.discovered_clues.push("altered_ledger".to_string());
                quest_log.discovered_clues.insert("altered_ledger".to_string(), clue.clone());
                
                info!("📝 CLUE DISCOVERED: {}", clue.description);
                info!("🕵️ Someone has been tampering with the official records. The question is: Aldric or someone else?");
                
                quest_events.write(QuestEvent::ClueDiscovered {
                    player_entity: Entity::PLACEHOLDER,
                    clue,
                });
            } else {
                info!("You've already examined the guild records and noticed the alterations.");
            }
            
            // Update evidence strength and check for phase progression
            let clues: Vec<_> = quest_progress.discovered_clues.iter()
                .filter_map(|clue_id| quest_log.discovered_clues.get(clue_id))
                .cloned()
                .collect();
            quest_progress.evidence_strength = EvidenceStrength::calculate_from_clues(&clues);
            
            // Check if player has enough evidence to progress to next phase
            if quest_progress.evidence_strength != EvidenceStrength::None 
                && quest_progress.discovered_clues.len() >= 2 {
                info!("🎯 PHASE COMPLETED: Initial Observation");
                info!("📈 Evidence Strength: {:?}", quest_progress.evidence_strength);
                info!("🚪 New Phase Available: Gathering Testimonies");
                info!("💡 TIP: You now have enough evidence to question other merchants and townspeople about Aldric's behavior.");
                info!("🗣️ Try talking to Marta the Blacksmith, Elder Thorne, or Benny the stable hand for more information.");
                
                quest_progress.completed_phases.push(quest_progress.current_phase.clone());
                quest_progress.current_phase = "witness_interviews".to_string();
                
                quest_events.write(QuestEvent::PhaseCompleted {
                    player_entity: Entity::PLACEHOLDER,
                    quest_id: quest_id.to_string(),
                    phase_id: "initial_observation".to_string(),
                });
            }
        } else {
            info!("You've already examined the guild records thoroughly.");
        }
    } else {
        info!("You need to start the merchant's mystery quest first (F9).");
    }
}

/// System to display current quest status and available actions
pub fn quest_status_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<&QuestLog, With<Player>>,
    quest_db: Res<QuestDatabase>,
) {
    // Show quest journal (J key)
    if keyboard.just_pressed(KeyCode::KeyJ) {
        if let Ok(quest_log) = player_query.single() {
            info!("📖 === QUEST JOURNAL ===");
            
            if quest_log.active_quests.is_empty() {
                info!("📭 No active quests. Press F9 to start 'The Merchant's Mystery'");
            } else {
                for (quest_id, progress) in &quest_log.active_quests {
                    if let Some(quest_def) = quest_db.quests.get(quest_id) {
                        info!("🎯 Quest: {} ({})", quest_def.title, quest_def.difficulty);
                        info!("📍 Current Phase: {}", progress.current_phase);
                        info!("🔍 Evidence Strength: {:?}", progress.evidence_strength);
                        info!("🧩 Clues Found: {}/{}", progress.discovered_clues.len(), "Unknown");
                        
                        // Show current phase description
                        if let Some(current_phase) = quest_def.phases.iter()
                            .find(|phase| phase.phase_id == progress.current_phase) {
                            info!("📝 Current Objective: {}", current_phase.description);
                            
                            if !current_phase.available_actions.is_empty() {
                                info!("🎬 Available Actions:");
                                for action in &current_phase.available_actions {
                                    match action.as_str() {
                                        "examine_shop_carefully" => info!("  F10 - Examine shop carefully"),
                                        "question_aldric_about_travels" => info!("  F11 - Question Aldric about travels"),
                                        "check_merchant_guild_records" => info!("  F12 - Check merchant guild records"),
                                        _ => info!("  • {}", action.replace("_", " ")),
                                    }
                                }
                            }
                        }
                        
                        // Show discovered clues
                        if !progress.discovered_clues.is_empty() {
                            info!("🔍 Discovered Clues:");
                            for clue_id in &progress.discovered_clues {
                                if let Some(clue) = quest_log.discovered_clues.get(clue_id) {
                                    info!("  • {}", clue.description);
                                }
                            }
                        }
                        
                        info!(""); // Empty line for readability
                    }
                }
            }
            
            info!("🎮 Controls: J - Quest Journal, F9 - Start Quest, F10-F12 - Investigation Actions");
        }
    }
}

/// System to handle quest events and provide feedback
pub fn quest_event_handler(
    mut quest_events: EventReader<QuestEvent>,
) {
    for event in quest_events.read() {
        match event {
            QuestEvent::ClueDiscovered { clue, .. } => {
                info!("🎉 New clue discovered in {}: {}", clue.quest_id, clue.clue_id);
            }
            QuestEvent::PhaseCompleted { quest_id, phase_id, .. } => {
                info!("✅ Quest phase completed: {} - {}", quest_id, phase_id);
            }
            QuestEvent::QuestCompleted { quest_id, resolution_path, .. } => {
                info!("🏆 Quest completed: {} via {}", quest_id, resolution_path);
            }
            QuestEvent::QuestFailed { quest_id, failure_reason, .. } => {
                warn!("❌ Quest failed: {} - {}", quest_id, failure_reason);
            }
            _ => {}
        }
    }
}