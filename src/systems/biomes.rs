use bevy::prelude::*;
use crate::systems::terrain::TerrainHeightSampler;

/// Biome system for creating distinct environmental zones
/// Following Single Responsibility: handles biome generation and zone management

/// Different biome types for world generation
#[derive(Component, Clone, Debug, PartialEq)]
pub enum BiomeType {
    Forest,      // Dense trees, thick vegetation
    Plains,      // Open grassland, scattered objects  
    Rocky,       // More rocks, sparse vegetation
    Wetland,     // Future: marsh areas with water features
}

/// Biome configuration for world zones
#[derive(Component, Clone)]
pub struct BiomeConfig {
    pub center: Vec2,          // Center point of biome zone
    pub radius: f32,           // Influence radius
    pub biome_type: BiomeType,
    pub tree_multiplier: f32,  // Modify base tree density
    pub rock_multiplier: f32,  // Modify base rock density
    pub bush_multiplier: f32,  // Modify base bush density
    pub elevation_preference: f32, // Preferred terrain height (-1.0 to 1.0)
}

impl BiomeConfig {
    /// Create a forest biome configuration
    pub fn forest(center: Vec2, radius: f32) -> Self {
        Self {
            center,
            radius,
            biome_type: BiomeType::Forest,
            tree_multiplier: 3.0,   // 3x more trees
            rock_multiplier: 0.3,   // Fewer rocks
            bush_multiplier: 2.0,   // 2x more bushes
            elevation_preference: 0.2, // Slightly elevated areas
        }
    }
    
    /// Create a plains biome configuration
    pub fn plains(center: Vec2, radius: f32) -> Self {
        Self {
            center,
            radius,
            biome_type: BiomeType::Plains,
            tree_multiplier: 0.2,   // Very few trees
            rock_multiplier: 0.5,   // Some rocks
            bush_multiplier: 0.8,   // Fewer bushes
            elevation_preference: -0.3, // Lower, flatter areas
        }
    }
    
    /// Create a rocky biome configuration
    pub fn rocky(center: Vec2, radius: f32) -> Self {
        Self {
            center,
            radius,
            biome_type: BiomeType::Rocky,
            tree_multiplier: 0.1,   // Almost no trees
            rock_multiplier: 4.0,   // 4x more rocks
            bush_multiplier: 0.3,   // Few bushes
            elevation_preference: 0.6, // Higher, mountainous areas
        }
    }
}

/// Resource to hold all biome configurations
#[derive(Resource, Default)]
pub struct BiomeSystem {
    pub biomes: Vec<BiomeConfig>,
}

/// Component to mark entities with biome information
#[derive(Component)]
pub struct BiomeInfluence {
    pub biome_type: BiomeType,
    pub influence_strength: f32, // 0.0 to 1.0
}

/// System to setup initial biome zones
pub fn setup_biomes(
    mut commands: Commands,
) {
    let mut biome_system = BiomeSystem::default();
    
    // Create overlapping biome zones for natural transitions
    // Forest zones - multiple smaller forests
    biome_system.biomes.push(BiomeConfig::forest(Vec2::new(-40.0, 30.0), 35.0));
    biome_system.biomes.push(BiomeConfig::forest(Vec2::new(45.0, -20.0), 40.0));
    biome_system.biomes.push(BiomeConfig::forest(Vec2::new(-10.0, -60.0), 25.0));
    
    // Plains zones - larger open areas
    biome_system.biomes.push(BiomeConfig::plains(Vec2::new(20.0, 40.0), 30.0));
    biome_system.biomes.push(BiomeConfig::plains(Vec2::new(-50.0, -10.0), 35.0));
    
    // Rocky zones - mountainous areas
    biome_system.biomes.push(BiomeConfig::rocky(Vec2::new(60.0, 15.0), 25.0));
    biome_system.biomes.push(BiomeConfig::rocky(Vec2::new(-30.0, 50.0), 20.0));
    biome_system.biomes.push(BiomeConfig::rocky(Vec2::new(10.0, -40.0), 30.0));
    
    commands.insert_resource(biome_system);
    
    info!("Biome system initialized: {} forest, {} plains, {} rocky zones", 
          3, 2, 3);
}

/// Get biome influence at a specific world position
/// Returns the dominant biome type and combined influence strength
pub fn get_biome_at_position(
    biome_system: &BiomeSystem, 
    terrain_sampler: &TerrainHeightSampler,
    position: Vec2
) -> (BiomeType, f32) {
    let mut max_influence = 0.0;
    let mut dominant_biome = BiomeType::Plains; // Default fallback
    
    // Sample terrain height for elevation-based biome preferences
    let terrain_height = crate::systems::terrain::sample_terrain_height(
        terrain_sampler, position.x, position.y
    );
    let normalized_height = (terrain_height / terrain_sampler.config.height_scale).clamp(-1.0, 1.0);
    
    for biome in &biome_system.biomes {
        let distance = position.distance(biome.center);
        
        // Calculate base influence based on distance
        let distance_influence = if distance < biome.radius {
            1.0 - (distance / biome.radius).powi(2) // Smooth falloff
        } else {
            0.0
        };
        
        // Apply elevation preference modifier
        let elevation_match = 1.0 - (normalized_height - biome.elevation_preference).abs();
        let elevation_modifier = 0.7 + (elevation_match * 0.3); // 0.7 to 1.0 range
        
        let total_influence = distance_influence * elevation_modifier;
        
        if total_influence > max_influence {
            max_influence = total_influence;
            dominant_biome = biome.biome_type.clone();
        }
    }
    
    (dominant_biome, max_influence)
}

/// Get density multipliers for object spawning based on biome influence
pub fn get_biome_density_multipliers(
    biome_system: &BiomeSystem,
    terrain_sampler: &TerrainHeightSampler,
    position: Vec2
) -> (f32, f32, f32) { // (tree, rock, bush)
    let mut tree_mult = 1.0;
    let mut rock_mult = 1.0; 
    let mut bush_mult = 1.0;
    let mut total_influence = 0.0;
    
    // Sample terrain height for elevation-based preferences
    let terrain_height = crate::systems::terrain::sample_terrain_height(
        terrain_sampler, position.x, position.y
    );
    let normalized_height = (terrain_height / terrain_sampler.config.height_scale).clamp(-1.0, 1.0);
    
    // Accumulate weighted influence from all biomes
    for biome in &biome_system.biomes {
        let distance = position.distance(biome.center);
        
        let distance_influence = if distance < biome.radius {
            1.0 - (distance / biome.radius).powi(2)
        } else {
            0.0
        };
        
        if distance_influence > 0.0 {
            // Apply elevation preference
            let elevation_match = 1.0 - (normalized_height - biome.elevation_preference).abs();
            let elevation_modifier = 0.7 + (elevation_match * 0.3);
            
            let influence = distance_influence * elevation_modifier;
            
            // Weight the multipliers by influence
            tree_mult += (biome.tree_multiplier - 1.0) * influence;
            rock_mult += (biome.rock_multiplier - 1.0) * influence;
            bush_mult += (biome.bush_multiplier - 1.0) * influence;
            total_influence += influence;
        }
    }
    
    // Normalize if we have any biome influence, otherwise use defaults
    if total_influence > 0.0 {
        (tree_mult.max(0.0), rock_mult.max(0.0), bush_mult.max(0.0))
    } else {
        (1.0, 1.0, 1.0) // Default multipliers for areas outside biomes
    }
}

/// Debug system to visualize biome zones (optional - can be disabled)
pub fn debug_biome_visualization(
    mut gizmos: Gizmos,
    biome_system: Res<BiomeSystem>,
) {
    for biome in &biome_system.biomes {
        let color = match biome.biome_type {
            BiomeType::Forest => Color::srgb(0.2, 0.8, 0.2), // Green
            BiomeType::Plains => Color::srgb(0.8, 0.8, 0.2), // Yellow
            BiomeType::Rocky => Color::srgb(0.6, 0.6, 0.6),  // Gray
            BiomeType::Wetland => Color::srgb(0.2, 0.2, 0.8), // Blue
        };
        
        // Draw biome influence circles at ground level using Bevy 0.16 API
        let isometry = bevy::math::Isometry3d::from_translation(
            Vec3::new(biome.center.x, 1.0, biome.center.y)
        );
        gizmos.circle(isometry, biome.radius, color);
    }
}