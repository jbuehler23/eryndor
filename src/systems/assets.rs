use bevy::prelude::*;
use bevy::asset::LoadState;
use bevy::gltf::GltfAssetLabel;
use avian3d::prelude::*;
use crate::components::{Player, PlayerMovementConfig, PlayerMovementState, PlayerStats};

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

// Removed Tnua imports - using simple kinematic approach

// System to spawn the player entity once the assets are loaded
pub fn spawn_player_when_assets_loaded(
    mut commands: Commands,
    character_assets: Res<CharacterAssets>,
    asset_server: Res<AssetServer>,
    mut already_spawned: Local<bool>,
) {
    if *already_spawned {
        return;
    }

    if matches!(asset_server.load_state(&character_assets.knight), LoadState::Loaded) {
        let player_entity = commands.spawn((
            SceneRoot(character_assets.knight.clone()),
            Transform::from_xyz(-70.0, 2.0, -70.0), // Lower spawn position for flat terrain
            RigidBody::Kinematic, // TRUE KINEMATIC - no physics interference
            LockedAxes::new().lock_rotation_x().lock_rotation_z(), // Prevent tipping over
            // Removed LinearVelocity, Friction, Restitution - kinematic bodies don't use them
        )).id();

        // Add collider as child entity positioned at character center
        commands.entity(player_entity).with_children(|children| {
            children.spawn((
                Collider::capsule(0.4, 1.8), // Character body collider
                Transform::from_xyz(0.0, 0.9, 0.0), // Center collider on character body (half height up from feet)
                CollisionMargin(0.01), // Tight collision margin
            ));
        });

        commands.entity(player_entity).insert((
            Player,
            PlayerMovementConfig::default(),
            PlayerMovementState::default(),
            crate::components::PlayerStats::default(),
            crate::components::AnimationController::default(),
            crate::components::CharacterModel::default(),
            crate::components::KnightAnimationSetup::default(),
        ));

        info!("Player spawned with Knight 3D model");
        *already_spawned = true;
    }
}