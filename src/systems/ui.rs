use bevy::prelude::*;
use crate::{resources::GameConfig, states::GameState, components::{Player, PlayerStats}};

// UI marker components
#[derive(Component)]
pub struct DebugUI;

#[derive(Component)]
pub struct FPSText;

#[derive(Component)]
pub struct StatsUI;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct ManaBar;

#[derive(Component)]
pub struct StaminaBar;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct ManaText;

#[derive(Component)]
pub struct StaminaText;

// UI setup system
pub fn setup_ui(mut commands: Commands) {
    // Debug UI container (FPS counter)
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        DebugUI,
    )).with_children(|parent| {
        // FPS counter
        parent.spawn((
            Text::new("FPS: --"),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            },
            FPSText,
        ));
    });
    
    // Stats UI container (Health, Mana, Stamina bars)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            bottom: Val::Px(20.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        },
        StatsUI,
    )).with_children(|parent| {
        // Health bar container
        parent.spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Px(24.0),
                border: UiRect::all(Val::Px(2.0)),
                padding: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor(Color::srgb(0.3, 0.3, 0.3)),
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        )).with_children(|parent| {
            // Health bar fill
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.8, 0.2, 0.2)), // Red color
                HealthBar,
            ));
            // Health text overlay
            parent.spawn((
                Text::new("100 / 100"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                HealthText,
            ));
        });
        
        // Mana bar container
        parent.spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Px(20.0),
                border: UiRect::all(Val::Px(2.0)),
                padding: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor(Color::srgb(0.3, 0.3, 0.3)),
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        )).with_children(|parent| {
            // Mana bar fill
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.4, 0.8)), // Blue color
                ManaBar,
            ));
            // Mana text overlay
            parent.spawn((
                Text::new("50 / 50"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ManaText,
            ));
        });
        
        // Stamina bar container
        parent.spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Px(20.0),
                border: UiRect::all(Val::Px(2.0)),
                padding: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor(Color::srgb(0.3, 0.3, 0.3)),
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        )).with_children(|parent| {
            // Stamina bar fill
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.8, 0.2)), // Green color
                StaminaBar,
            ));
            // Stamina text overlay
            parent.spawn((
                Text::new("80 / 80"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                StaminaText,
            ));
        });
    });
}

// Debug info system - YAGNI: Only basic FPS for now
pub fn debug_info(
    time: Res<Time>,
    config: Res<GameConfig>,
    mut fps_query: Query<&mut Text, With<FPSText>>,
) {
    if !config.debug.show_fps {
        return;
    }
    
    let Ok(mut text) = fps_query.single_mut() else {
        return;
    };
    
    let fps = 1.0 / time.delta_secs();
    **text = format!("FPS: {:.1}", fps);
}

// Stats UI update system - updates health, mana, stamina bars and text
pub fn update_stats_ui(
    player_query: Query<&PlayerStats, With<Player>>,
    mut health_bar_query: Query<&mut Node, (With<HealthBar>, Without<ManaBar>, Without<StaminaBar>)>,
    mut mana_bar_query: Query<&mut Node, (With<ManaBar>, Without<HealthBar>, Without<StaminaBar>)>,
    mut stamina_bar_query: Query<&mut Node, (With<StaminaBar>, Without<HealthBar>, Without<ManaBar>)>,
    mut health_text_query: Query<&mut Text, (With<HealthText>, Without<ManaText>, Without<StaminaText>)>,
    mut mana_text_query: Query<&mut Text, (With<ManaText>, Without<HealthText>, Without<StaminaText>)>,
    mut stamina_text_query: Query<&mut Text, (With<StaminaText>, Without<HealthText>, Without<ManaText>)>,
) {
    let Ok(stats) = player_query.single() else {
        return; // No player found
    };
    
    // Update health bar width based on percentage
    if let Ok(mut health_bar) = health_bar_query.single_mut() {
        health_bar.width = Val::Percent(stats.health_percentage() * 100.0);
    }
    
    // Update mana bar width based on percentage
    if let Ok(mut mana_bar) = mana_bar_query.single_mut() {
        mana_bar.width = Val::Percent(stats.mana_percentage() * 100.0);
    }
    
    // Update stamina bar width based on percentage
    if let Ok(mut stamina_bar) = stamina_bar_query.single_mut() {
        stamina_bar.width = Val::Percent(stats.stamina_percentage() * 100.0);
    }
    
    // Update health text
    if let Ok(mut health_text) = health_text_query.single_mut() {
        **health_text = format!("{:.0} / {:.0}", stats.health, stats.max_health);
    }
    
    // Update mana text
    if let Ok(mut mana_text) = mana_text_query.single_mut() {
        **mana_text = format!("{:.0} / {:.0}", stats.mana, stats.max_mana);
    }
    
    // Update stamina text
    if let Ok(mut stamina_text) = stamina_text_query.single_mut() {
        **stamina_text = format!("{:.0} / {:.0}", stats.stamina, stats.max_stamina);
    }
}

// UI update system - placeholder for future UI logic
pub fn update_ui(
    _time: Res<Time>,
    _state: Res<State<GameState>>,
) {
    // YAGNI: Will add menu interactions when needed
}

