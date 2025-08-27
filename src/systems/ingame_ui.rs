use bevy::prelude::*;
use crate::components::{Player, CharacterLevel, CharacterSkills, CharacterLoadouts};
use crate::components::progression::RoleType;
use crate::states::GameState;

// In-game UI Components
#[derive(Component)]
pub struct InGameUI;

#[derive(Component)]
pub struct SkillOverviewPanel;

#[derive(Component)]
pub struct SkillProgressBar {
    pub skill_name: String,
}

#[derive(Component)]
pub struct RoleCapabilityDisplay;

#[derive(Component)]
pub struct ExperienceNotification;

#[derive(Component)]
pub struct QuestObjectiveTracker;

#[derive(Component)]
pub struct LoadoutSwitchUI;

#[derive(Component)]
pub struct HotbarUI;

#[derive(Component)]
pub struct AbilitySlot {
    pub slot_index: usize,
}

// Resource for managing experience notifications
#[derive(Resource, Default)]
pub struct ExperienceNotifications {
    pub pending_notifications: Vec<ExperienceGainEvent>,
}

#[derive(Debug, Clone)]
pub struct ExperienceGainEvent {
    pub skill_name: String,
    pub experience_gained: u64,
    pub new_level: Option<u32>,
    pub timestamp: f32,
}

// System to setup in-game UI
pub fn setup_ingame_ui(mut commands: Commands) {
    info!("🎮 Setting up in-game UI");
    
    // Initialize experience notifications resource
    commands.insert_resource(ExperienceNotifications::default());

    // Main in-game UI container
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        InGameUI,
    )).with_children(|parent| {
        
        // Skill Overview Panel (right side)
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(20.0),
                top: Val::Px(20.0),
                width: Val::Px(320.0),
                height: Val::Px(400.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(15.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor(Color::srgb(0.4, 0.6, 0.8)),
            BackgroundColor(Color::srgba(0.05, 0.05, 0.15, 0.9)),
            SkillOverviewPanel,
        )).with_children(|parent| {
            
            // Panel Title
            parent.spawn((
                Text::new("Character Overview"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(15.0)),
                    align_self: AlignSelf::Center,
                    ..default()
                },
            ));

            // Character Level Display
            parent.spawn((
                Text::new("Level: --"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Suggested Role Display
            parent.spawn((
                Text::new("Suggested Role: --"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                },
            ));

            // Top Skills Display
            parent.spawn((
                Text::new("Top Skills:"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                },
            ));

            // Skill progress bars container
            for i in 0..5 {
                parent.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(25.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    },
                )).with_children(|parent| {
                    // Skill name
                    parent.spawn((
                        Text::new(format!("Skill {}", i + 1)),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        Node {
                            width: Val::Px(100.0),
                            ..default()
                        },
                    ));

                    // Progress bar background
                    parent.spawn((
                        Node {
                            width: Val::Px(120.0),
                            height: Val::Px(16.0),
                            border: UiRect::all(Val::Px(1.0)),
                            padding: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BorderColor(Color::srgb(0.3, 0.3, 0.3)),
                        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                    )).with_children(|parent| {
                        // Progress bar fill
                        parent.spawn((
                            Node {
                                width: Val::Percent(0.0), // Will be updated by system
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.2, 0.6, 0.8)),
                            SkillProgressBar {
                                skill_name: format!("skill_{}", i),
                            },
                        ));
                    });

                    // Level display
                    parent.spawn((
                        Text::new("0"),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        Node {
                            margin: UiRect::left(Val::Px(8.0)),
                            width: Val::Px(30.0),
                            ..default()
                        },
                    ));
                });
            }

            // Role Capabilities Section
            parent.spawn((
                Text::new("Role Capabilities:"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect {
                        top: Val::Px(15.0),
                        bottom: Val::Px(8.0),
                        ..default()
                    },
                    ..default()
                },
            ));

            // Role capability display
            parent.spawn((
                Text::new("Tank: 0 | Healer: 0 | DPS: 0"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                RoleCapabilityDisplay,
            ));
        });

        // Hotbar UI (bottom center)
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                left: Val::Percent(50.0),
                width: Val::Px(320.0),
                height: Val::Px(60.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Px(8.0)),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                // Center the hotbar horizontally (margin offset)
                margin: UiRect::left(Val::Px(-160.0)),
                ..default()
            },
            BorderColor(Color::srgb(0.6, 0.4, 0.2)),
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
            HotbarUI,
        )).with_children(|parent| {
            // Create 4 ability slots
            for i in 0..4 {
                parent.spawn((
                    Node {
                        width: Val::Px(60.0),
                        height: Val::Px(60.0),
                        border: UiRect::all(Val::Px(2.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor(Color::srgb(0.4, 0.4, 0.4)),
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    AbilitySlot { slot_index: i },
                )).with_children(|parent| {
                    // Ability name/icon placeholder
                    parent.spawn((
                        Text::new(format!("{}", i + 1)),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));
                });
            }
        });

        // Quest Objective Tracker (left side)
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                top: Val::Px(20.0),
                width: Val::Px(280.0),
                height: Val::Px(200.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor(Color::srgb(0.8, 0.6, 0.2)),
            BackgroundColor(Color::srgba(0.1, 0.08, 0.05, 0.9)),
            QuestObjectiveTracker,
        )).with_children(|parent| {
            // Quest tracker title
            parent.spawn((
                Text::new("Active Objectives"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.9, 0.7)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    align_self: AlignSelf::Center,
                    ..default()
                },
            ));

            // Sample quest objectives
            parent.spawn((
                Text::new("• Explore the world and test movement"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            ));

            parent.spawn((
                Text::new("• Gain experience in any skill"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            ));

            parent.spawn((
                Text::new("• Switch between different loadouts"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            ));

            parent.spawn((
                Text::new("Press F1-F4 for loadout switching"),
                TextFont {
                    font_size: 10.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::top(Val::Px(15.0)),
                    ..default()
                },
            ));
        });

        // Instructions overlay (bottom left)
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                bottom: Val::Px(100.0),
                width: Val::Px(280.0),
                height: Val::Px(80.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BorderColor(Color::srgb(0.3, 0.3, 0.3)),
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        )).with_children(|parent| {
            parent.spawn((
                Text::new("Controls: WASD = Move | Mouse = Camera | Shift+F1 = Character Info"),
                TextFont {
                    font_size: 10.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));
            
            parent.spawn((
                Text::new("F1-F4 = Loadout Switch | Ctrl+F6 = Add XP | Esc = Main Menu"),
                TextFont {
                    font_size: 10.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                Node {
                    margin: UiRect::top(Val::Px(5.0)),
                    ..default()
                },
            ));
        });
    });
}

// System to update skill overview panel
pub fn update_skill_overview(
    player_query: Query<(&CharacterLevel, &CharacterSkills, &CharacterLoadouts), With<Player>>,
    mut skill_text_query: Query<(&mut Text, &SkillProgressBar)>,
    mut role_capability_query: Query<&mut Text, (With<RoleCapabilityDisplay>, Without<SkillProgressBar>)>,
    mut skill_bar_query: Query<(&mut Node, &SkillProgressBar), Without<Text>>,
) {
    if let Ok((_character_level, skills, _loadouts)) = player_query.single() {
        // Get top 5 skills by level
        let mut skill_levels: Vec<_> = skills.skills.iter()
            .map(|(skill_type, skill_line)| (skill_type, skill_line.level))
            .collect();
        skill_levels.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Update skill progress bars and text
        let mut skill_index = 0;
        for (mut text, _skill_bar) in &mut skill_text_query {
            if skill_index < skill_levels.len() {
                let (skill_type, _level) = skill_levels[skill_index];
                let skill_name = format!("{:?}", skill_type).replace("_", " ");
                **text = skill_name;
                skill_index += 1;
            }
        }
        
        // Update progress bar fills
        skill_index = 0;
        for (mut node, _skill_bar) in &mut skill_bar_query {
            if skill_index < skill_levels.len() {
                let (_, level) = skill_levels[skill_index];
                let progress_percent = (level as f32 / 50.0 * 100.0).min(100.0);
                node.width = Val::Percent(progress_percent);
                skill_index += 1;
            }
        }

        // Update role capabilities
        if let Ok(mut role_text) = role_capability_query.single_mut() {
            let tank_score = skills.get_role_capability_score(RoleType::Tank);
            let healer_score = skills.get_role_capability_score(RoleType::Healer);
            let dps_score = skills.get_role_capability_score(RoleType::DPS);
            let support_score = skills.get_role_capability_score(RoleType::Support);
            let utility_score = skills.get_role_capability_score(RoleType::Utility);
            
            **role_text = format!(
                "Tank: {} | Healer: {} | DPS: {}\nSupport: {} | Utility: {}",
                tank_score, healer_score, dps_score, support_score, utility_score
            );
        }
    }
}

// System to handle experience notifications
pub fn handle_experience_notifications(
    mut notifications: ResMut<ExperienceNotifications>,
    _commands: Commands,
    time: Res<Time>,
) {
    // Remove expired notifications (after 3 seconds)
    notifications.pending_notifications.retain(|notif| {
        time.elapsed_secs() - notif.timestamp < 3.0
    });

    // For now, just log experience gains
    // TODO: Create floating text notifications
    for notification in &notifications.pending_notifications {
        if time.elapsed_secs() - notification.timestamp < 0.1 { // Just created
            if let Some(new_level) = notification.new_level {
                info!("🎉 {} leveled up to {}! Gained {} XP", 
                      notification.skill_name, new_level, notification.experience_gained);
            } else {
                info!("📈 {} gained {} XP", 
                      notification.skill_name, notification.experience_gained);
            }
        }
    }
}

// System to add experience notifications (called by other systems)
pub fn add_experience_notification(
    mut notifications: ResMut<ExperienceNotifications>,
    time: Res<Time>,
    skill_name: String,
    experience_gained: u64,
    new_level: Option<u32>,
) {
    notifications.pending_notifications.push(ExperienceGainEvent {
        skill_name,
        experience_gained,
        new_level,
        timestamp: time.elapsed_secs(),
    });
}

// System to handle escape key for returning to main menu
pub fn handle_ingame_escape(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        info!("Returning to main menu...");
        next_state.set(GameState::MainMenu);
    }
}

// System to cleanup in-game UI
pub fn cleanup_ingame_ui(
    mut commands: Commands,
    ui_query: Query<Entity, With<InGameUI>>,
) {
    for entity in &ui_query {
        commands.entity(entity).despawn();
    }
    info!("🗑️ In-game UI cleaned up");
}