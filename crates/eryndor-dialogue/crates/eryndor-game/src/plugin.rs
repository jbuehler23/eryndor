//! Dialogue system plugin for easy integration into Bevy applications.

use bevy::prelude::*;
use eryndor_core::{events::DialogueEvent, traits::EryndorPlugin};

use crate::systems::*;
use crate::resources::*;

/// Plugin that provides dialogue system functionality
pub struct EryndorDialoguePlugin;

impl Plugin for EryndorDialoguePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<DialogueDatabase>()
            .init_resource::<ActiveDialogue>()
            
            // Events
            .add_event::<DialogueEvent>()
            
            // Startup systems
            .add_systems(Startup, load_dialogue_database)
            
            // Update systems
            .add_systems(Update, hot_reload_dialogue_system);
    }
}

impl EryndorPlugin for EryndorDialoguePlugin {
    fn build(&self, app: &mut App) {
        <Self as Plugin>::build(self, app);
    }
    
    fn name(&self) -> &'static str {
        "EryndorDialoguePlugin"
    }
    
    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}