use bevy::prelude::*;
use crate::resources::{GameConfig, save_config};

// Save config on app exit
pub fn save_config_on_exit(
    config: Res<GameConfig>,
    mut exit_events: EventReader<AppExit>,
) {
    for _event in exit_events.read() {
        save_config(&config);
        info!("Configuration saved");
    }
}

// System to update config resource when it changes
pub fn update_config_system(
    config: Res<GameConfig>,
) {
    if config.is_changed() && !config.is_added() {
        save_config(&config);
        info!("Configuration updated and saved");
    }
}