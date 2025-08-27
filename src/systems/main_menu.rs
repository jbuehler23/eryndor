use bevy::prelude::*;
use crate::states::GameState;

// Main Menu UI Components
#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct RoleSelectionButton {
    pub role_type: RoleType,
}

#[derive(Component)]
pub struct CharacterTypeButton {
    pub character_type: CharacterType,
}

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct RolePreviewPanel;

#[derive(Component)]
pub struct RoleDescriptionText;

#[derive(Component)]
pub struct LoadoutPreviewText;

// Role types for character selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoleType {
    Tank,
    Healer,
    DPS,
    Support,
    Utility,
}

impl RoleType {
    pub fn display_name(self) -> &'static str {
        match self {
            RoleType::Tank => "Tank",
            RoleType::Healer => "Healer",
            RoleType::DPS => "DPS",
            RoleType::Support => "Support",
            RoleType::Utility => "Utility",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            RoleType::Tank => "Defensive specialist focused on protecting allies and controlling enemies. High health, strong armor, threat generation abilities.",
            RoleType::Healer => "Support specialist focused on healing and protecting allies. Restoration magic, mana efficiency, group support abilities.",
            RoleType::DPS => "Damage specialist focused on eliminating enemies efficiently. High damage output, critical strikes, diverse combat abilities.",
            RoleType::Support => "Utility specialist focused on crafting and providing useful services. Resource gathering, item creation, economic gameplay.",
            RoleType::Utility => "Versatile specialist focused on exploration and problem-solving. Stealth, lockpicking, mobility, and discovery abilities.",
        }
    }

    pub fn primary_skills(self) -> &'static [&'static str] {
        match self {
            RoleType::Tank => &["Swordsmanship", "Shield Defense", "Heavy Armor"],
            RoleType::Healer => &["Restoration", "Light Armor", "Divination"],
            RoleType::DPS => &["Fire Magic", "Archery", "Swordsmanship"],
            RoleType::Support => &["Smithing", "Alchemy", "Enchanting"],
            RoleType::Utility => &["Stealth", "Lockpicking", "Athletics"],
        }
    }

    pub fn recommended_weapons(self) -> &'static [&'static str] {
        match self {
            RoleType::Tank => &["Sword", "Shield", "Axe"],
            RoleType::Healer => &["Restoration Staff"],
            RoleType::DPS => &["Fire Staff", "Bow", "Two-handed Sword"],
            RoleType::Support => &["Hammer", "Dagger"],
            RoleType::Utility => &["Dagger", "Light weapons"],
        }
    }

    pub fn color(self) -> Color {
        match self {
            RoleType::Tank => Color::srgb(0.8, 0.4, 0.2), // Orange-brown
            RoleType::Healer => Color::srgb(0.2, 0.8, 0.4), // Green
            RoleType::DPS => Color::srgb(0.8, 0.2, 0.2), // Red
            RoleType::Support => Color::srgb(0.4, 0.4, 0.8), // Blue
            RoleType::Utility => Color::srgb(0.6, 0.2, 0.8), // Purple
        }
    }
}

// Character model types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterType {
    Knight,
    Mage,
    Rogue,
    Barbarian,
    RogueHooded,
}

impl CharacterType {
    pub fn display_name(self) -> &'static str {
        match self {
            CharacterType::Knight => "Knight",
            CharacterType::Mage => "Mage",
            CharacterType::Rogue => "Rogue",
            CharacterType::Barbarian => "Barbarian",
            CharacterType::RogueHooded => "Hooded Rogue",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            CharacterType::Knight => "Noble warrior with balanced combat abilities and strong defense",
            CharacterType::Mage => "Scholarly spellcaster with powerful magic and mystical knowledge",
            CharacterType::Rogue => "Agile fighter specializing in stealth and precise strikes",
            CharacterType::Barbarian => "Fierce warrior with raw strength and primal combat techniques",
            CharacterType::RogueHooded => "Mysterious assassin with advanced stealth and utility skills",
        }
    }
}

// Resource to track character selection state
#[derive(Resource, Default)]
pub struct CharacterSelection {
    pub selected_role: Option<RoleType>,
    pub selected_character_type: Option<CharacterType>,
    pub ready_to_start: bool,
}

// System to setup main menu UI
pub fn setup_main_menu(mut commands: Commands) {
    info!("🎮 Setting up main menu UI");

    // Initialize character selection resource
    commands.insert_resource(CharacterSelection::default());

    // Main menu container
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgb(0.05, 0.05, 0.1)), // Dark blue background
        MainMenuUI,
    )).with_children(|parent| {
        // Game title
        parent.spawn((
            Text::new("ERYNDOR"),
            TextFont {
                font_size: 72.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            Node {
                margin: UiRect::bottom(Val::Px(40.0)),
                ..default()
            },
        ));

        // Subtitle
        parent.spawn((
            Text::new("Fantasy MMORPG - Stakeholder Demo"),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.8)),
            Node {
                margin: UiRect::bottom(Val::Px(60.0)),
                ..default()
            },
        ));

        // Character selection section
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                margin: UiRect::bottom(Val::Px(40.0)),
                ..default()
            },
        )).with_children(|parent| {
            // Role selection title
            parent.spawn((
                Text::new("Choose Your Role"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Role buttons container
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(15.0),
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            )).with_children(|parent| {
                for role in [RoleType::Tank, RoleType::Healer, RoleType::DPS, RoleType::Support, RoleType::Utility] {
                    parent.spawn((
                        Button,
                        Node {
                            width: Val::Px(140.0),
                            height: Val::Px(60.0),
                            border: UiRect::all(Val::Px(2.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BorderColor(role.color()),
                        BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
                        RoleSelectionButton { role_type: role },
                    )).with_children(|parent| {
                        parent.spawn((
                            Text::new(role.display_name()),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
                }
            });

            // Character type selection title
            parent.spawn((
                Text::new("Choose Your Character"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                },
            ));

            // Character type buttons container
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(12.0),
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            )).with_children(|parent| {
                for character_type in [CharacterType::Knight, CharacterType::Mage, CharacterType::Rogue, CharacterType::Barbarian] {
                    parent.spawn((
                        Button,
                        Node {
                            width: Val::Px(120.0),
                            height: Val::Px(50.0),
                            border: UiRect::all(Val::Px(2.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BorderColor(Color::srgb(0.4, 0.4, 0.4)),
                        BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
                        CharacterTypeButton { character_type },
                    )).with_children(|parent| {
                        parent.spawn((
                            Text::new(character_type.display_name()),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
                }
            });
        });

        // Role preview panel (initially hidden)
        parent.spawn((
            Node {
                width: Val::Px(600.0),
                height: Val::Px(200.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                margin: UiRect::bottom(Val::Px(30.0)),
                display: Display::None, // Initially hidden
                ..default()
            },
            BorderColor(Color::srgb(0.3, 0.3, 0.3)),
            BackgroundColor(Color::srgb(0.08, 0.08, 0.12)),
            RolePreviewPanel,
        )).with_children(|parent| {
            // Role description
            parent.spawn((
                Text::new("Select a role to see details"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                RoleDescriptionText,
            ));

            // Loadout preview
            parent.spawn((
                Text::new(""),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::top(Val::Px(15.0)),
                    ..default()
                },
                LoadoutPreviewText,
            ));
        });

        // Start game button
        parent.spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(60.0),
                border: UiRect::all(Val::Px(3.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderColor(Color::srgb(0.2, 0.8, 0.2)),
            BackgroundColor(Color::srgb(0.1, 0.6, 0.1)),
            StartGameButton,
        )).with_children(|parent| {
            parent.spawn((
                Text::new("Start Demo"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });

        // Instructions
        parent.spawn((
            Text::new("Select a role and character type, then click Start Demo"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(0.6, 0.6, 0.6)),
            Node {
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            },
        ));
    });
}

// System to handle main menu interactions
pub fn handle_main_menu_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor, Option<&RoleSelectionButton>, Option<&CharacterTypeButton>, Option<&StartGameButton>),
        (Changed<Interaction>, With<Button>),
    >,
    mut character_selection: ResMut<CharacterSelection>,
    mut role_description_query: Query<&mut Text, (With<RoleDescriptionText>, Without<LoadoutPreviewText>)>,
    mut loadout_preview_query: Query<&mut Text, (With<LoadoutPreviewText>, Without<RoleDescriptionText>)>,
    mut role_preview_panel: Query<&mut Node, With<RolePreviewPanel>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background_color, _border_color, role_button, character_button, start_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(role_button) = role_button {
                    // Role selection
                    character_selection.selected_role = Some(role_button.role_type);
                    info!("Selected role: {:?}", role_button.role_type);
                    
                    // Update role preview panel
                    if let Ok(mut panel_node) = role_preview_panel.single_mut() {
                        panel_node.display = Display::Flex;
                    }
                    
                    if let Ok(mut description_text) = role_description_query.single_mut() {
                        **description_text = role_button.role_type.description().to_string();
                    }
                    
                    if let Ok(mut loadout_text) = loadout_preview_query.single_mut() {
                        **loadout_text = format!(
                            "Primary Skills: {}\nRecommended Weapons: {}",
                            role_button.role_type.primary_skills().join(", "),
                            role_button.role_type.recommended_weapons().join(", ")
                        );
                    }
                    
                    *background_color = Color::srgb(0.2, 0.2, 0.3).into();
                } else if let Some(character_button) = character_button {
                    // Character type selection
                    character_selection.selected_character_type = Some(character_button.character_type);
                    info!("Selected character type: {:?}", character_button.character_type);
                    
                    *background_color = Color::srgb(0.2, 0.2, 0.3).into();
                } else if start_button.is_some() {
                    // Start game
                    if character_selection.selected_role.is_some() && character_selection.selected_character_type.is_some() {
                        info!("Starting demo with role {:?} and character {:?}", 
                              character_selection.selected_role, 
                              character_selection.selected_character_type);
                        next_state.set(GameState::InGame);
                    } else {
                        warn!("Cannot start demo - missing role or character selection");
                    }
                }
            }
            Interaction::Hovered => {
                if role_button.is_some() {
                    *background_color = Color::srgb(0.15, 0.15, 0.2).into();
                } else {
                    *background_color = Color::srgb(0.15, 0.15, 0.2).into();
                }
            }
            Interaction::None => {
                if role_button.is_some() {
                    *background_color = Color::srgb(0.1, 0.1, 0.15).into();
                } else {
                    *background_color = Color::srgb(0.1, 0.1, 0.15).into();
                }
            }
        }
    }
}

// System to cleanup main menu UI when leaving the main menu state
pub fn cleanup_main_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in &menu_query {
        commands.entity(entity).despawn();
    }
    info!("🗑️ Main menu UI cleaned up");
}