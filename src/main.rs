use bevy::prelude::*;
use eryndor::{EryndorPlugin, setup_logging};

fn main() {
    // Initialize logging first
    setup_logging();
    
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Eryndor".into(),
                resolution: (1024.0, 768.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EryndorPlugin)
        .run();
}
