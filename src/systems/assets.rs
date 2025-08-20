use bevy::prelude::*;
use bevy::asset::LoadState;
use bevy::gltf::GltfAssetLabel;
use avian3d::prelude::*;
use crate::components::{Player, CharacterModel, CharacterType};

// Asset loading resource to track loaded assets
#[derive(Resource, Default)]
pub struct GameAssets {
    pub models: Vec<Handle<Scene>>,
    pub textures: Vec<Handle<Image>>,
    pub loading_complete: bool,
}

// Character model resource for KayKit adventurer assets
#[derive(Resource)]
pub struct CharacterAssets {
    pub knight: Handle<Scene>,
    pub mage: Handle<Scene>,
    pub rogue: Handle<Scene>,
    pub barbarian: Handle<Scene>,
    pub rogue_hooded: Handle<Scene>,
    // Equipment models for future use
    pub sword_1handed: Handle<Scene>,
    pub shield_round: Handle<Scene>,
}

// Asset loading system - Basic pipeline setup
pub fn load_initial_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Initialize the asset tracking resource
    commands.insert_resource(GameAssets::default());
    
    // Load KayKit character models using proper Bevy 0.16 GLTF syntax
    // Testing different scene indices to find clean character model
    let character_assets = CharacterAssets {
        knight: asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("KayKit_Adventurers_1.0_FREE/Characters/gltf/Knight.glb")
        ),
        mage: Handle::default(), // Temporarily disabled to focus on Knight debugging
        rogue: Handle::default(),
        barbarian: Handle::default(),
        rogue_hooded: Handle::default(),
        
        // Temporarily disabled equipment to focus on character debugging
        sword_1handed: Handle::default(),
        shield_round: Handle::default(),
    };
    
    commands.insert_resource(character_assets);
    
    info!("Knight contains 15 meshes, 76 animations - investigating Scene0 contents");
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

// System to upgrade player from capsule to 3D character model when assets are loaded
pub fn upgrade_player_model(
    mut commands: Commands,
    character_assets: Res<CharacterAssets>,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(Entity, &mut CharacterModel), (With<Player>, With<Mesh3d>)>,
) {
    for (player_entity, mut character_model) in player_query.iter_mut() {
        // Check if the knight model is loaded
        if matches!(asset_server.load_state(&character_assets.knight), LoadState::Loaded) {
            // Remove the capsule mesh components and manual collider
            commands.entity(player_entity)
                .remove::<Mesh3d>()
                .remove::<MeshMaterial3d<StandardMaterial>>()
                .remove::<Collider>(); // Remove manual capsule collider
                
            // Add the 3D character model with manual collider (more reliable for character controller)
            commands.entity(player_entity).insert((
                SceneRoot(character_assets.knight.clone()),
                // Manual character collider - more reliable than GLTF mesh generation for player movement
                Collider::capsule(0.4, 1.8), // Character capsule: radius=0.4, height=1.8
            ));
            
            // Update character model tracking
            character_model.character_type = CharacterType::Knight;
            
            info!("Player upgraded from capsule to Knight 3D model - animations will be setup when scene loads!");
        }
    }
}