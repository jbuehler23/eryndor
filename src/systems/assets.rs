use bevy::prelude::*;
use bevy::asset::LoadState;

// Asset loading resource to track loaded assets
#[derive(Resource, Default)]
pub struct GameAssets {
    pub models: Vec<Handle<Scene>>,
    pub textures: Vec<Handle<Image>>,
    pub loading_complete: bool,
}

// Asset loading system - Basic pipeline setup
pub fn load_initial_assets(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
) {
    // Initialize the asset tracking resource
    commands.insert_resource(GameAssets::default());
    
    // Load basic assets for the game
    // Following YAGNI - only loading what we need for Phase 1
    
    // Example: Load a test model (commented out since we don't have the file yet)
    // let model_handle: Handle<Scene> = asset_server.load("models/test_model.glb");
    
    info!("Asset loading pipeline initialized");
}

// Asset loading progress system
pub fn check_asset_loading(
    mut game_assets: ResMut<GameAssets>,
    asset_server: Res<AssetServer>,
) {
    if game_assets.loading_complete {
        return;
    }

    // Check if all tracked assets are loaded
    let mut all_loaded = true;
    
    for handle in &game_assets.models {
        if !matches!(asset_server.load_state(handle.id()), LoadState::Loaded) {
            all_loaded = false;
            break;
        }
    }
    
    if all_loaded {
        game_assets.loading_complete = true;
        info!("All assets loaded successfully");
    }
}