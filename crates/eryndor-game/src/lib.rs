//! # Eryndor Game
//!
//! Main game application that orchestrates all Eryndor systems.

use bevy::prelude::*;
use avian3d::prelude::*;
use eryndor_dialogue::prelude::*;
use eryndor_dev_console::prelude::*;

pub struct EryndorPlugin;

impl Plugin for EryndorPlugin {
    fn build(&self, app: &mut App) {
        app
            // Physics - Avian 3D integration
            .add_plugins(PhysicsPlugins::default())
            .insert_resource(Gravity(Vec3::NEG_Y * 9.81))
            
            // Add Eryndor crate plugins
            .add_plugins(EryndorDialoguePlugin)
            .add_plugins(EryndorDevConsolePlugin);
    }
}

// Re-export logging setup function for main.rs
pub fn setup_logging() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,wgpu_core=warn,wgpu_hal=warn", env!("CARGO_PKG_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}