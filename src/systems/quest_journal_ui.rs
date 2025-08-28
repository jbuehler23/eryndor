use bevy::prelude::*;
use crate::components::{Player, QuestLog};
use crate::components::quest::*;

// Quest Journal UI Components
#[derive(Component)]
pub struct QuestJournalUI;

#[derive(Component)]
pub struct QuestJournalWindow;

#[derive(Component)]
pub struct QuestListPanel;

#[derive(Component)]
pub struct QuestDetailsPanel;

#[derive(Component)]
pub struct CluesPanel;

#[derive(Component)]
pub struct NotesPanel;

#[derive(Component)]
pub struct QuestEntry {
    pub quest_id: String,
}

#[derive(Component)]
pub struct ClueEntry {
    pub clue_id: String,
}

#[derive(Component)]
pub struct EvidenceStrengthIndicator;

#[derive(Component)]
pub struct QuestPhaseIndicator;

#[derive(Component)]
pub struct NoteInput;

#[derive(Component)]
pub struct JournalTab {
    pub tab_type: JournalTabType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JournalTabType {
    ActiveQuests,
    CompletedQuests,
    Clues,
    Notes,
}

impl Default for JournalTabType {
    fn default() -> Self {
        JournalTabType::ActiveQuests
    }
}

// Resource to manage journal state
#[derive(Resource, Default)]
pub struct QuestJournalState {
    pub is_visible: bool,
    pub selected_quest: Option<String>,
    pub current_tab: JournalTabType,
    pub scroll_position: f32,
    pub search_filter: String,
}

// System to toggle quest journal visibility
pub fn toggle_quest_journal(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut journal_state: ResMut<QuestJournalState>,
) {
    if keyboard.just_pressed(KeyCode::KeyJ) {
        journal_state.is_visible = !journal_state.is_visible;
        info!("📖 Quest Journal {}", if journal_state.is_visible { "opened" } else { "closed" });
    }
}

// System to setup quest journal UI
pub fn setup_quest_journal(mut commands: Commands, journal_state: Res<QuestJournalState>) {
    // Initialize journal state resource
    if !journal_state.is_visible {
        return;
    }
    
    info!("📚 Setting up Quest Journal UI");

    // Main quest journal window
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(15.0),
            top: Val::Percent(10.0),
            width: Val::Percent(70.0),
            height: Val::Percent(80.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgba(0.05, 0.05, 0.15, 0.95)),
        BorderColor(Color::srgb(0.4, 0.6, 0.8)),
        QuestJournalUI,
        QuestJournalWindow,
    )).with_children(|parent| {
        
        // Journal header with title and close button
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(15.0)),
                border: UiRect::bottom(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.2, 0.9)),
            BorderColor(Color::srgb(0.3, 0.3, 0.4)),
        )).with_children(|parent| {
            
            // Title
            parent.spawn((
                Text::new("Quest Journal"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 1.0)),
            ));

            // Close instruction
            parent.spawn((
                Text::new("Press J to close"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.8)),
            ));
        });

        // Tab navigation bar
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                flex_direction: FlexDirection::Row,
                border: UiRect::bottom(Val::Px(1.0)),
                ..default()
            },
            BorderColor(Color::srgb(0.3, 0.3, 0.4)),
        )).with_children(|parent| {
            
            let tabs = [
                ("Active Quests", JournalTabType::ActiveQuests),
                ("Completed", JournalTabType::CompletedQuests),
                ("Clues", JournalTabType::Clues),
                ("Notes", JournalTabType::Notes),
            ];

            for (tab_name, tab_type) in tabs {
                let is_active = tab_type == journal_state.current_tab;
                
                parent.spawn((
                    Node {
                        width: Val::Percent(25.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::right(Val::Px(1.0)),
                        ..default()
                    },
                    BackgroundColor(if is_active {
                        Color::srgba(0.2, 0.4, 0.6, 0.8)
                    } else {
                        Color::srgba(0.1, 0.1, 0.2, 0.6)
                    }),
                    BorderColor(Color::srgb(0.3, 0.3, 0.4)),
                    JournalTab { tab_type },
                )).with_children(|parent| {
                    parent.spawn((
                        Text::new(tab_name),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(if is_active {
                            Color::srgb(1.0, 1.0, 1.0)
                        } else {
                            Color::srgb(0.8, 0.8, 0.9)
                        }),
                    ));
                });
            }
        });

        // Main content area
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                flex_direction: FlexDirection::Row,
                ..default()
            },
        )).with_children(|parent| {
            
            // Left panel - Quest/Item list
            parent.spawn((
                Node {
                    width: Val::Percent(40.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
                    border: UiRect::right(Val::Px(1.0)),
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.03, 0.03, 0.08, 0.8)),
                BorderColor(Color::srgb(0.3, 0.3, 0.4)),
                QuestListPanel,
            ));

            // Right panel - Details and content
            parent.spawn((
                Node {
                    width: Val::Percent(60.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(15.0)),
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.02, 0.02, 0.06, 0.8)),
                QuestDetailsPanel,
            ));
        });
    });
}

// System to update quest journal content
pub fn update_quest_journal_content(
    mut commands: Commands,
    journal_state: Res<QuestJournalState>,
    player_query: Query<&QuestLog, With<Player>>,
    quest_db: Res<QuestDatabase>,
    mut quest_list_query: Query<Entity, With<QuestListPanel>>,
    mut quest_details_query: Query<Entity, With<QuestDetailsPanel>>,
) {
    if !journal_state.is_visible {
        return;
    }

    let Ok(quest_log) = player_query.single() else {
        return;
    };

    // Note: For simplicity, we'll rebuild content without clearing 
    // This means content will accumulate. TODO: Implement proper child clearing

    match journal_state.current_tab {
        JournalTabType::ActiveQuests => {
            update_active_quests_tab(&mut commands, &quest_log, &quest_db, &journal_state, &quest_list_query, &quest_details_query);
        }
        JournalTabType::CompletedQuests => {
            update_completed_quests_tab(&mut commands, &quest_log, &quest_list_query, &quest_details_query);
        }
        JournalTabType::Clues => {
            update_clues_tab(&mut commands, &quest_log, &quest_list_query, &quest_details_query);
        }
        JournalTabType::Notes => {
            update_notes_tab(&mut commands, &quest_log, &quest_list_query, &quest_details_query);
        }
    }
}

fn update_active_quests_tab(
    commands: &mut Commands,
    quest_log: &QuestLog,
    quest_db: &QuestDatabase,
    journal_state: &QuestJournalState,
    quest_list_query: &Query<Entity, With<QuestListPanel>>,
    quest_details_query: &Query<Entity, With<QuestDetailsPanel>>,
) {
    // Update quest list panel
    if let Ok(list_entity) = quest_list_query.single() {
        commands.entity(list_entity).with_children(|parent| {
            
            // Quest list header
            parent.spawn((
                Text::new("Active Quests"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // List active quests
            if quest_log.active_quests.is_empty() {
                parent.spawn((
                    Text::new("No active quests"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.6, 0.6, 0.7)),
                ));
            } else {
                for (quest_id, quest_progress) in &quest_log.active_quests {
                    let is_selected = journal_state.selected_quest.as_ref() == Some(quest_id);
                    
                    parent.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            min_height: Val::Px(60.0),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(8.0)),
                            margin: UiRect::bottom(Val::Px(5.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BackgroundColor(if is_selected {
                            Color::srgba(0.2, 0.3, 0.5, 0.8)
                        } else {
                            Color::srgba(0.1, 0.1, 0.2, 0.6)
                        }),
                        BorderColor(if is_selected {
                            Color::srgb(0.4, 0.5, 0.8)
                        } else {
                            Color::srgb(0.3, 0.3, 0.4)
                        }),
                        QuestEntry { quest_id: quest_id.clone() },
                    )).with_children(|parent| {
                        
                        // Quest title
                        if let Some(quest_def) = quest_db.quests.get(quest_id) {
                            parent.spawn((
                                Text::new(&quest_def.title),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 1.0)),
                            ));
                        } else {
                            parent.spawn((
                                Text::new(quest_id),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.8, 0.8, 0.9)),
                            ));
                        }

                        // Current phase
                        parent.spawn((
                            Text::new(format!("Phase: {}", quest_progress.current_phase.replace("_", " "))),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.7, 0.8)),
                            Node {
                                margin: UiRect::top(Val::Px(3.0)),
                                ..default()
                            },
                        ));

                        // Evidence strength indicator
                        let evidence_color = match quest_progress.evidence_strength {
                            EvidenceStrength::None => Color::srgb(0.5, 0.5, 0.5),
                            EvidenceStrength::Weak => Color::srgb(0.8, 0.4, 0.4),
                            EvidenceStrength::Moderate => Color::srgb(0.8, 0.6, 0.4),
                            EvidenceStrength::Strong => Color::srgb(0.4, 0.8, 0.4),
                            EvidenceStrength::Overwhelming => Color::srgb(0.4, 0.8, 0.8),
                        };
                        
                        parent.spawn((
                            Text::new(format!("Evidence: {:?}", quest_progress.evidence_strength)),
                            TextFont {
                                font_size: 11.0,
                                ..default()
                            },
                            TextColor(evidence_color),
                            Node {
                                margin: UiRect::top(Val::Px(2.0)),
                                ..default()
                            },
                        ));
                    });
                }
            }
        });
    }

    // Update quest details panel
    if let Ok(details_entity) = quest_details_query.single() {
        commands.entity(details_entity).with_children(|parent| {
            
            if let Some(selected_quest_id) = &journal_state.selected_quest {
                if let Some(quest_progress) = quest_log.active_quests.get(selected_quest_id) {
                    if let Some(quest_def) = quest_db.quests.get(selected_quest_id) {
                        
                        // Quest title and description
                        parent.spawn((
                            Text::new(&quest_def.title),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 1.0)),
                            Node {
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                        ));

                        parent.spawn((
                            Text::new(&quest_def.description),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.8, 0.8, 0.9)),
                            Node {
                                margin: UiRect::bottom(Val::Px(15.0)),
                                ..default()
                            },
                        ));

                        // Current phase details
                        if let Some(current_phase) = quest_def.phases.iter()
                            .find(|p| p.phase_id == quest_progress.current_phase) {
                            parent.spawn((
                                Text::new("Current Objectives:"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.8, 0.6)),
                                Node {
                                    margin: UiRect {
                                        top: Val::Px(10.0),
                                        bottom: Val::Px(8.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));

                            parent.spawn((
                                Text::new(&current_phase.description),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.8, 0.8, 0.9)),
                                Node {
                                    margin: UiRect::bottom(Val::Px(10.0)),
                                    ..default()
                                },
                            ));

                            // Objectives list
                            for objective in &current_phase.objectives {
                                let is_completed = quest_progress.completed_objectives.contains(&objective.description);
                                parent.spawn((
                                    Text::new(format!("{} {}", 
                                        if is_completed { "✓" } else { "•" },
                                        objective.description.replace("_", " ")
                                    )),
                                    TextFont {
                                        font_size: 13.0,
                                        ..default()
                                    },
                                    TextColor(if is_completed {
                                        Color::srgb(0.6, 0.8, 0.6)
                                    } else {
                                        Color::srgb(0.8, 0.8, 0.8)
                                    }),
                                    Node {
                                        margin: UiRect::bottom(Val::Px(3.0)),
                                        ..default()
                                    },
                                ));
                            }
                        }

                        // Discovered clues section
                        if !quest_progress.discovered_clues.is_empty() {
                            parent.spawn((
                                Text::new("Discovered Clues:"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.8, 0.9, 0.6)),
                                Node {
                                    margin: UiRect {
                                        top: Val::Px(15.0),
                                        bottom: Val::Px(8.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));

                            for clue_id in &quest_progress.discovered_clues {
                                if let Some(clue) = quest_log.discovered_clues.get(clue_id) {
                                    parent.spawn((
                                        Node {
                                            width: Val::Percent(100.0),
                                            flex_direction: FlexDirection::Column,
                                            padding: UiRect::all(Val::Px(8.0)),
                                            margin: UiRect::bottom(Val::Px(5.0)),
                                            border: UiRect::all(Val::Px(1.0)),
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgba(0.1, 0.2, 0.1, 0.6)),
                                        BorderColor(Color::srgb(0.3, 0.5, 0.3)),
                                    )).with_children(|parent| {
                                        
                                        parent.spawn((
                                            Text::new(&clue.description),
                                            TextFont {
                                                font_size: 12.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                        ));

                                        parent.spawn((
                                            Text::new(format!("Importance: {} | Found: {}",
                                                clue.importance_weight,
                                                clue.discovery_method
                                            )),
                                            TextFont {
                                                font_size: 10.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                            Node {
                                                margin: UiRect::top(Val::Px(3.0)),
                                                ..default()
                                            },
                                        ));
                                    });
                                }
                            }
                        }

                        // Investigation notes
                        if !quest_progress.investigation_notes.is_empty() {
                            parent.spawn((
                                Text::new("Investigation Notes:"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.7, 0.6)),
                                Node {
                                    margin: UiRect {
                                        top: Val::Px(15.0),
                                        bottom: Val::Px(8.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));

                            for note in &quest_progress.investigation_notes {
                                parent.spawn((
                                    Text::new(format!("• {}", note)),
                                    TextFont {
                                        font_size: 12.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                    Node {
                                        margin: UiRect::bottom(Val::Px(3.0)),
                                        ..default()
                                    },
                                ));
                            }
                        }
                    }
                }
            } else {
                // No quest selected
                parent.spawn((
                    Text::new("Select a quest to view details"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.6, 0.6, 0.7)),
                    Node {
                        justify_self: JustifySelf::Center,
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                ));
            }
        });
    }
}

fn update_completed_quests_tab(
    commands: &mut Commands,
    quest_log: &QuestLog,
    quest_list_query: &Query<Entity, With<QuestListPanel>>,
    quest_details_query: &Query<Entity, With<QuestDetailsPanel>>,
) {
    if let Ok(list_entity) = quest_list_query.single() {
        commands.entity(list_entity).with_children(|parent| {
            parent.spawn((
                Text::new("Completed Quests"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            if quest_log.completed_quests.is_empty() {
                parent.spawn((
                    Text::new("No completed quests yet"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.6, 0.6, 0.7)),
                ));
            } else {
                for (quest_id, completed_quest) in &quest_log.completed_quests {
                    parent.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            min_height: Val::Px(50.0),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(8.0)),
                            margin: UiRect::bottom(Val::Px(5.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.1, 0.2, 0.1, 0.6)),
                        BorderColor(Color::srgb(0.3, 0.5, 0.3)),
                    )).with_children(|parent| {
                        parent.spawn((
                            Text::new(quest_id),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.8, 0.9, 0.8)),
                        ));

                        parent.spawn((
                            Text::new(format!("Resolution: {}", completed_quest.resolution_path)),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.8, 0.7)),
                            Node {
                                margin: UiRect::top(Val::Px(3.0)),
                                ..default()
                            },
                        ));
                    });
                }
            }
        });
    }
}

fn update_clues_tab(
    commands: &mut Commands,
    quest_log: &QuestLog,
    quest_list_query: &Query<Entity, With<QuestListPanel>>,
    quest_details_query: &Query<Entity, With<QuestDetailsPanel>>,
) {
    if let Ok(list_entity) = quest_list_query.single() {
        commands.entity(list_entity).with_children(|parent| {
            parent.spawn((
                Text::new("All Discovered Clues"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            if quest_log.discovered_clues.is_empty() {
                parent.spawn((
                    Text::new("No clues discovered yet"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.6, 0.6, 0.7)),
                ));
            } else {
                for (clue_id, clue) in &quest_log.discovered_clues {
                    parent.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(8.0)),
                            margin: UiRect::bottom(Val::Px(5.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.2, 0.1, 0.2, 0.6)),
                        BorderColor(Color::srgb(0.5, 0.3, 0.5)),
                        ClueEntry { clue_id: clue_id.clone() },
                    )).with_children(|parent| {
                        parent.spawn((
                            Text::new(&clue.description),
                            TextFont {
                                font_size: 13.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        ));

                        parent.spawn((
                            Text::new(format!("Quest: {} | Importance: {} | Method: {}",
                                clue.quest_id,
                                clue.importance_weight,
                                clue.discovery_method
                            )),
                            TextFont {
                                font_size: 10.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.7, 0.8)),
                            Node {
                                margin: UiRect::top(Val::Px(3.0)),
                                ..default()
                            },
                        ));
                    });
                }
            }
        });
    }
}

fn update_notes_tab(
    commands: &mut Commands,
    quest_log: &QuestLog,
    quest_list_query: &Query<Entity, With<QuestListPanel>>,
    quest_details_query: &Query<Entity, With<QuestDetailsPanel>>,
) {
    if let Ok(list_entity) = quest_list_query.single() {
        commands.entity(list_entity).with_children(|parent| {
            parent.spawn((
                Text::new("Player Notes"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            if quest_log.player_notes.is_empty() {
                parent.spawn((
                    Text::new("No notes written yet"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.6, 0.6, 0.7)),
                ));
            } else {
                for (note_id, note_content) in &quest_log.player_notes {
                    parent.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(8.0)),
                            margin: UiRect::bottom(Val::Px(5.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.2, 0.2, 0.1, 0.6)),
                        BorderColor(Color::srgb(0.5, 0.5, 0.3)),
                    )).with_children(|parent| {
                        parent.spawn((
                            Text::new(note_content),
                            TextFont {
                                font_size: 13.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.8)),
                        ));
                    });
                }
            }
        });
    }
}

// System to handle tab switching
pub fn handle_journal_tab_switching(
    mut journal_state: ResMut<QuestJournalState>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !journal_state.is_visible {
        return;
    }

    if keyboard.just_pressed(KeyCode::Digit1) {
        journal_state.current_tab = JournalTabType::ActiveQuests;
        journal_state.selected_quest = None;
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        journal_state.current_tab = JournalTabType::CompletedQuests;
        journal_state.selected_quest = None;
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        journal_state.current_tab = JournalTabType::Clues;
        journal_state.selected_quest = None;
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        journal_state.current_tab = JournalTabType::Notes;
        journal_state.selected_quest = None;
    }
}

// System to handle quest selection
pub fn handle_quest_selection(
    mut journal_state: ResMut<QuestJournalState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<&QuestLog, With<Player>>,
) {
    if !journal_state.is_visible || journal_state.current_tab != JournalTabType::ActiveQuests {
        return;
    }

    let Ok(quest_log) = player_query.single() else {
        return;
    };

    // Simple quest selection with Up/Down arrow keys
    if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::ArrowDown) {
        let quest_ids: Vec<String> = quest_log.active_quests.keys().cloned().collect();
        if !quest_ids.is_empty() {
            let current_index = journal_state.selected_quest
                .as_ref()
                .and_then(|id| quest_ids.iter().position(|x| x == id))
                .unwrap_or(0);
                
            let new_index = if keyboard.just_pressed(KeyCode::ArrowUp) {
                if current_index > 0 { current_index - 1 } else { quest_ids.len() - 1 }
            } else {
                (current_index + 1) % quest_ids.len()
            };
            
            journal_state.selected_quest = Some(quest_ids[new_index].clone());
        }
    }
}

// System to cleanup quest journal UI
pub fn cleanup_quest_journal(
    mut commands: Commands,
    journal_query: Query<Entity, With<QuestJournalUI>>,
    journal_state: Res<QuestJournalState>,
) {
    if journal_state.is_visible {
        return;
    }

    for entity in &journal_query {
        commands.entity(entity).despawn();
    }
}

// System to ensure proper journal state management
pub fn manage_quest_journal_state(
    mut commands: Commands,
    journal_state: Res<QuestJournalState>,
    journal_query: Query<Entity, With<QuestJournalUI>>,
) {
    let has_ui = !journal_query.is_empty();
    
    if journal_state.is_visible && !has_ui {
        // Should show UI but doesn't exist - create it
        setup_quest_journal(commands, journal_state);
    } else if !journal_state.is_visible && has_ui {
        // Should hide UI but exists - remove it
        for entity in &journal_query {
            commands.entity(entity).despawn();
        }
    }
}