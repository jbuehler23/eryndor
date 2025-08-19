use bevy::prelude::*;
use bevy::gltf::GltfAssetLabel;
use avian3d::prelude::*;
use rand::prelude::*;
use crate::systems::terrain::{TerrainHeightSampler, sample_terrain_height_with_variation};
use crate::systems::biomes::{BiomeSystem, get_biome_density_multipliers};

/// World object system for spawning and managing environmental objects
/// Following Single Responsibility: handles forest/nature asset placement and management

/// Component to mark world objects (trees, rocks, bushes)
#[derive(Component)]
pub struct WorldObject;

/// Different types of world objects
#[derive(Component, Clone, Debug)]
pub enum WorldObjectType {
    Tree(TreeType),
    Rock(RockType),
    Bush(BushType),
    Grass(GrassType),
}

/// Tree variations from KayKit pack
#[derive(Clone, Debug)]
pub enum TreeType {
    Tree1A, Tree1B, Tree1C,
    Tree2A, Tree2B, Tree2C, Tree2D, Tree2E,
    Tree3A, Tree3B, Tree3C,
    Tree4A, Tree4B, Tree4C,
    TreeBare1A, TreeBare1B, TreeBare1C,
    TreeBare2A, TreeBare2B, TreeBare2C,
}

/// Rock variations from KayKit pack
#[derive(Clone, Debug)]
pub enum RockType {
    Rock1A, Rock1B, Rock1C, Rock1D, Rock1E, Rock1F, Rock1G, Rock1H,
    Rock1I, Rock1J, Rock1K, Rock1L, Rock1M, Rock1N, Rock1O, Rock1P, Rock1Q,
    Rock2A, Rock2B, Rock2C, Rock2D, Rock2E, Rock2F, Rock2G, Rock2H,
    Rock3A, Rock3B, Rock3C, Rock3D, Rock3E, Rock3F, Rock3G, Rock3H,
    Rock3I, Rock3J, Rock3K, Rock3L, Rock3M, Rock3N, Rock3O, Rock3P, Rock3Q, Rock3R,
}

/// Bush variations from KayKit pack
#[derive(Clone, Debug)]
pub enum BushType {
    Bush1A, Bush1B, Bush1C, Bush1D, Bush1E, Bush1F, Bush1G,
    Bush2A, Bush2B, Bush2C, Bush2D, Bush2E, Bush2F,
    Bush3A, Bush3B, Bush3C,
    Bush4A, Bush4B, Bush4C, Bush4D, Bush4E, Bush4F,
}

/// Grass variations from KayKit pack
#[derive(Clone, Debug)]
pub enum GrassType {
    Grass1A, Grass1B, Grass1C, Grass1D,
    Grass2A, Grass2B, Grass2C, Grass2D,
}

/// World object spawning configuration
#[derive(Resource)]
pub struct WorldObjectConfig {
    pub tree_density: f32,     // Trees per 100 square units
    pub rock_density: f32,     // Rocks per 100 square units  
    pub bush_density: f32,     // Bushes per 100 square units
    pub grass_density: f32,    // Grass clumps per 100 square units
    pub spawn_radius: f32,     // Distance from center to spawn objects
    pub min_spacing: f32,      // Minimum spacing between objects
}

impl Default for WorldObjectConfig {
    fn default() -> Self {
        Self {
            tree_density: 2.0,    // 2 trees per 100 units = sparse forest
            rock_density: 1.5,    // 1.5 rocks per 100 units
            bush_density: 3.0,    // 3 bushes per 100 units
            grass_density: 5.0,   // 5 grass clumps per 100 units
            spawn_radius: 100.0,  // 100 unit radius from center
            min_spacing: 3.0,     // 3 units minimum spacing
        }
    }
}

/// Resource to hold world object asset handles
#[derive(Resource, Default)]
pub struct WorldObjectAssets {
    pub trees: Vec<Handle<Scene>>,
    pub rocks: Vec<Handle<Scene>>,
    pub bushes: Vec<Handle<Scene>>,
    pub grass: Vec<Handle<Scene>>,
}

impl TreeType {
    fn get_path(&self) -> &'static str {
        match self {
            TreeType::Tree1A => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Tree_1_A_Color1.gltf",
            TreeType::Tree1B => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Tree_1_B_Color1.gltf",
            TreeType::Tree1C => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Tree_1_C_Color1.gltf",
            TreeType::Tree2A => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Tree_2_A_Color1.gltf",
            TreeType::Tree2B => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Tree_2_B_Color1.gltf",
            TreeType::Tree2C => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Tree_2_C_Color1.gltf",
            TreeType::Tree2D => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Tree_2_D_Color1.gltf",
            TreeType::Tree2E => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Tree_2_E_Color1.gltf",
            TreeType::Tree3A => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Tree_3_A_Color1.gltf",
            TreeType::Tree3B => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Tree_3_B_Color1.gltf",
            TreeType::Tree3C => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Tree_3_C_Color1.gltf",
            TreeType::Tree4A => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Tree_4_A_Color1.gltf",
            TreeType::Tree4B => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Tree_4_B_Color1.gltf",
            TreeType::Tree4C => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Tree_4_C_Color1.gltf",
            TreeType::TreeBare1A => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Tree_Bare_1_A_Color1.gltf",
            TreeType::TreeBare1B => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Tree_Bare_1_B_Color1.gltf",
            TreeType::TreeBare1C => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Tree_Bare_1_C_Color1.gltf",
            TreeType::TreeBare2A => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Tree_Bare_2_A_Color1.gltf",
            TreeType::TreeBare2B => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Tree_Bare_2_B_Color1.gltf",
            TreeType::TreeBare2C => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Tree_Bare_2_C_Color1.gltf",
        }
    }
    
    fn get_all_types() -> Vec<TreeType> {
        vec![
            TreeType::Tree1A, TreeType::Tree1B, TreeType::Tree1C,
            TreeType::Tree2A, TreeType::Tree2B, TreeType::Tree2C, TreeType::Tree2D, TreeType::Tree2E,
            TreeType::Tree3A, TreeType::Tree3B, TreeType::Tree3C,
            TreeType::Tree4A, TreeType::Tree4B, TreeType::Tree4C,
            TreeType::TreeBare1A, TreeType::TreeBare1B, TreeType::TreeBare1C,
            TreeType::TreeBare2A, TreeType::TreeBare2B, TreeType::TreeBare2C,
        ]
    }
}

impl RockType {
    fn get_path(&self) -> &'static str {
        match self {
            RockType::Rock1A => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Rock_1_A_Color1.gltf",
            RockType::Rock1B => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Rock_1_B_Color1.gltf",
            RockType::Rock1C => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Rock_1_C_Color1.gltf",
            RockType::Rock1D => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Rock_1_D_Color1.gltf",
            RockType::Rock1E => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Rock_1_E_Color1.gltf",
            // Add more rock types as needed - there are many variations
            _ => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Rock_1_A_Color1.gltf", // Fallback
        }
    }
    
    fn get_common_types() -> Vec<RockType> {
        vec![
            RockType::Rock1A, RockType::Rock1B, RockType::Rock1C, RockType::Rock1D, RockType::Rock1E,
            RockType::Rock2A, RockType::Rock2B, RockType::Rock2C,
            RockType::Rock3A, RockType::Rock3B, RockType::Rock3C,
        ]
    }
}

impl BushType {
    fn get_path(&self) -> &'static str {
        match self {
            BushType::Bush1A => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Bush_1_A_Color1.gltf",
            BushType::Bush1B => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Bush_1_B_Color1.gltf",
            BushType::Bush1C => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Bush_1_C_Color1.gltf",
            BushType::Bush2A => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Bush_2_A_Color1.gltf",
            BushType::Bush2B => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Bush_2_B_Color1.gltf",
            BushType::Bush3A => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Bush_3_A_Color1.gltf",
            _ => "KayKit_Forest_Nature_Pack_1.0_FREE/Assets/gltf/Bush_1_A_Color1.gltf", // Fallback
        }
    }
    
    fn get_common_types() -> Vec<BushType> {
        vec![
            BushType::Bush1A, BushType::Bush1B, BushType::Bush1C,
            BushType::Bush2A, BushType::Bush2B,
            BushType::Bush3A,
        ]
    }
}

/// System to load world object assets
pub fn load_world_object_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut world_assets = WorldObjectAssets::default();
    
    info!("🌲 Loading KayKit Forest Nature Pack assets...");
    
    // Load tree assets using correct GLTF syntax
    for tree_type in TreeType::get_all_types() {
        let handle = asset_server.load(
            GltfAssetLabel::Scene(0).from_asset(tree_type.get_path())
        );
        world_assets.trees.push(handle);
    }
    
    // Load rock assets using correct GLTF syntax
    for rock_type in RockType::get_common_types() {
        let handle = asset_server.load(
            GltfAssetLabel::Scene(0).from_asset(rock_type.get_path())
        );
        world_assets.rocks.push(handle);
    }
    
    // Load bush assets using correct GLTF syntax
    for bush_type in BushType::get_common_types() {
        let handle = asset_server.load(
            GltfAssetLabel::Scene(0).from_asset(bush_type.get_path())
        );
        world_assets.bushes.push(handle);
    }
    
    commands.insert_resource(world_assets);
    commands.insert_resource(WorldObjectConfig::default());
    commands.insert_resource(WorldObjectsSpawned::default());
    
    info!("🌲 World object assets loading started - {} trees, {} rocks, {} bushes", 
          TreeType::get_all_types().len(), 
          RockType::get_common_types().len(), 
          BushType::get_common_types().len());
}

/// Resource to track if world objects have been spawned
#[derive(Resource, Default)]
pub struct WorldObjectsSpawned(pub bool);

/// System to spawn world objects around the terrain with biome influence
pub fn spawn_world_objects(
    mut commands: Commands,
    world_assets: Res<WorldObjectAssets>,
    config: Res<WorldObjectConfig>,
    asset_server: Res<AssetServer>,
    mut spawned: ResMut<WorldObjectsSpawned>,
    terrain_sampler: Option<Res<TerrainHeightSampler>>,
    biome_system: Option<Res<BiomeSystem>>,
) {
    // Only spawn once
    if spawned.0 {
        return;
    }
    
    // Only spawn once when assets are loaded
    if world_assets.trees.is_empty() {
        return;
    }
    
    // Wait for terrain sampler to be available
    let Some(terrain_sampler) = terrain_sampler else {
        return; // Wait for terrain to be generated first
    };
    
    // Wait for biome system to be available
    let Some(biome_system) = biome_system else {
        return; // Wait for biomes to be initialized first
    };
    
    // Check if at least some assets are loaded
    let mut loaded_trees = 0;
    for handle in &world_assets.trees {
        if matches!(asset_server.load_state(handle.id()), bevy::asset::LoadState::Loaded) {
            loaded_trees += 1;
        }
    }
    
    if loaded_trees < 3 {
        return; // Wait for multiple assets to load
    }
    
    info!("🌲 World objects spawn ready - {}/{} trees loaded, beginning spawn...", 
          loaded_trees, world_assets.trees.len());
    
    let mut rng = thread_rng();
    let mut spawned_positions = Vec::new();
    
    // Calculate number of objects to spawn based on area and density
    let area = std::f32::consts::PI * config.spawn_radius * config.spawn_radius;
    let tree_count = ((area / 100.0) * config.tree_density) as u32;
    let rock_count = ((area / 100.0) * config.rock_density) as u32;
    let bush_count = ((area / 100.0) * config.bush_density) as u32;
    
    // Spawn objects with biome-influenced density
    let mut actual_trees = 0;
    let mut actual_rocks = 0;
    let mut actual_bushes = 0;
    
    // Spawn trees with biome density influence
    for _ in 0..tree_count {
        if let Some(position) = find_valid_spawn_position_with_biome(
            &mut rng, &spawned_positions, &config, &terrain_sampler, &biome_system, "tree"
        ) {
            spawn_tree(&mut commands, &world_assets, &mut rng, position);
            spawned_positions.push(position);
            actual_trees += 1;
        }
    }
    
    // Spawn rocks with biome density influence
    for _ in 0..rock_count {
        if let Some(position) = find_valid_spawn_position_with_biome(
            &mut rng, &spawned_positions, &config, &terrain_sampler, &biome_system, "rock"
        ) {
            spawn_rock(&mut commands, &world_assets, &mut rng, position);
            spawned_positions.push(position);
            actual_rocks += 1;
        }
    }
    
    // Spawn bushes with biome density influence
    for _ in 0..bush_count {
        if let Some(position) = find_valid_spawn_position_with_biome(
            &mut rng, &spawned_positions, &config, &terrain_sampler, &biome_system, "bush"
        ) {
            spawn_bush(&mut commands, &world_assets, &mut rng, position);
            spawned_positions.push(position);
            actual_bushes += 1;
        }
    }
    
    info!("🌲 World objects spawned successfully: {} trees, {} rocks, {} bushes in {}m radius", 
          actual_trees, actual_rocks, actual_bushes, config.spawn_radius);
    info!("🌍 Biome system applied - object distribution varies by forest/plains/rocky zones");
    
    // Mark as spawned so we don't spawn again
    spawned.0 = true;
}

/// Find a valid position to spawn an object, ensuring proper spacing
fn find_valid_spawn_position(
    rng: &mut ThreadRng, 
    existing_positions: &[Vec3], 
    config: &WorldObjectConfig,
    terrain_sampler: &TerrainHeightSampler,
) -> Option<Vec3> {
    for _ in 0..50 { // Try up to 50 times to find a valid position
        let angle = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
        let distance = rng.gen::<f32>() * config.spawn_radius;
        
        let x = angle.cos() * distance;
        let z = angle.sin() * distance;
        
        // Sample actual terrain height at this position
        let terrain_height = sample_terrain_height_with_variation(
            terrain_sampler, 
            x, 
            z, 
            rng.gen_range(-0.2..0.2) // Small random height variation
        );
        
        let position = Vec3::new(x, terrain_height, z);
        
        // Check spacing against existing objects
        let mut valid = true;
        for existing_pos in existing_positions {
            if position.distance(*existing_pos) < config.min_spacing {
                valid = false;
                break;
            }
        }
        
        if valid {
            return Some(position);
        }
    }
    None // Couldn't find valid position
}

/// Find a valid position to spawn an object with biome density influence
fn find_valid_spawn_position_with_biome(
    rng: &mut ThreadRng,
    existing_positions: &[Vec3],
    config: &WorldObjectConfig,
    terrain_sampler: &TerrainHeightSampler,
    biome_system: &BiomeSystem,
    object_type: &str, // "tree", "rock", "bush"
) -> Option<Vec3> {
    for _ in 0..100 { // More attempts for biome-aware spawning
        let angle = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
        let distance = rng.gen::<f32>() * config.spawn_radius;
        
        let x = angle.cos() * distance;
        let z = angle.sin() * distance;
        
        // Get biome density multipliers for this position
        let (tree_mult, rock_mult, bush_mult) = get_biome_density_multipliers(
            biome_system, terrain_sampler, Vec2::new(x, z)
        );
        
        // Apply biome influence as spawn probability
        let spawn_probability = match object_type {
            "tree" => tree_mult.min(4.0) / 4.0, // Cap at 4x, normalize to 0-1
            "rock" => rock_mult.min(4.0) / 4.0,
            "bush" => bush_mult.min(4.0) / 4.0,
            _ => 1.0,
        };
        
        // Skip this position if biome makes it unlikely to spawn this object type
        if rng.gen::<f32>() > spawn_probability {
            continue;
        }
        
        // Sample actual terrain height at this position
        let terrain_height = sample_terrain_height_with_variation(
            terrain_sampler,
            x,
            z,
            rng.gen_range(-0.2..0.2) // Small random height variation
        );
        
        let position = Vec3::new(x, terrain_height, z);
        
        // Check spacing against existing objects
        let mut valid = true;
        for existing_pos in existing_positions {
            if position.distance(*existing_pos) < config.min_spacing {
                valid = false;
                break;
            }
        }
        
        if valid {
            return Some(position);
        }
    }
    None // Couldn't find valid position
}

/// Spawn a tree at the given position
fn spawn_tree(
    commands: &mut Commands,
    world_assets: &WorldObjectAssets,
    rng: &mut ThreadRng,
    position: Vec3,
) {
    if world_assets.trees.is_empty() {
        return;
    }
    
    let tree_index = rng.gen_range(0..world_assets.trees.len());
    let tree_handle = world_assets.trees[tree_index].clone();
    
    // Add random rotation (position already includes terrain height and variation)
    let rotation = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
    
    commands.spawn((
        SceneRoot(tree_handle),
        Transform::from_xyz(position.x, position.y, position.z)
            .with_rotation(Quat::from_rotation_y(rotation)),
        
        // Physics - static collision for trees
        RigidBody::Static,
        Collider::cylinder(8.0, 1.0), // Approximate tree collision (height, radius)
        
        // Game components
        WorldObject,
        WorldObjectType::Tree(TreeType::Tree1A), // Simplified for now
    ));
}

/// Spawn a rock at the given position
fn spawn_rock(
    commands: &mut Commands,
    world_assets: &WorldObjectAssets,
    rng: &mut ThreadRng,
    position: Vec3,
) {
    if world_assets.rocks.is_empty() {
        return;
    }
    
    let rock_index = rng.gen_range(0..world_assets.rocks.len());
    let rock_handle = world_assets.rocks[rock_index].clone();
    
    // Add random rotation and scale (position already includes terrain height)
    let rotation = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
    let scale = rng.gen_range(0.8..1.2); // Random size variation
    
    commands.spawn((
        SceneRoot(rock_handle),
        Transform::from_xyz(position.x, position.y, position.z)
            .with_rotation(Quat::from_rotation_y(rotation))
            .with_scale(Vec3::splat(scale)),
        
        // Physics - static collision for rocks
        RigidBody::Static,
        Collider::cuboid(1.0, 1.0, 1.0), // Approximate rock collision
        
        // Game components
        WorldObject,
        WorldObjectType::Rock(RockType::Rock1A), // Simplified for now
    ));
}

/// Spawn a bush at the given position
fn spawn_bush(
    commands: &mut Commands,
    world_assets: &WorldObjectAssets,
    rng: &mut ThreadRng,
    position: Vec3,
) {
    if world_assets.bushes.is_empty() {
        return;
    }
    
    let bush_index = rng.gen_range(0..world_assets.bushes.len());
    let bush_handle = world_assets.bushes[bush_index].clone();
    
    // Add random rotation and scale (position already includes terrain height)
    let rotation = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
    let scale = rng.gen_range(0.9..1.1); // Slight size variation
    
    commands.spawn((
        SceneRoot(bush_handle),
        Transform::from_xyz(position.x, position.y, position.z)
            .with_rotation(Quat::from_rotation_y(rotation))
            .with_scale(Vec3::splat(scale)),
        
        // Physics - small collision for bushes (can walk through)
        RigidBody::Static,
        Collider::cylinder(1.0, 0.5), // Small bush collision
        
        // Game components
        WorldObject,
        WorldObjectType::Bush(BushType::Bush1A), // Simplified for now
    ));
}