use bevy::prelude::*;
use crate::{resources::GameConfig, states::GameState};

// UI marker components
#[derive(Component)]
pub struct DebugUI;

#[derive(Component)]
pub struct FPSText;

// UI setup system
pub fn setup_ui(mut commands: Commands) {
    // Debug UI container
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

// UI update system - placeholder for future UI logic
pub fn update_ui(
    _time: Res<Time>,
    _state: Res<State<GameState>>,
) {
    // YAGNI: Will add menu interactions when needed
}

