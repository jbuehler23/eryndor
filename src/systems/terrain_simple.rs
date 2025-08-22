/// Simplified terrain system using the clean TerrainGenerator utility
/// Ensures perfect alignment between visual mesh and physics collision
use bevy::prelude::*;
use avian3d::prelude::*;
use bevy::ui::debug;
use crate::systems::Terrain;
use crate::utils::{TerrainGenerator, TerrainHeightSampler};
use crate::systems::biomes::BiomeType;

/// Resource to store biome grid data for world generation
#[derive(Resource)]
pub struct BiomeMap {
    pub grid: Vec<Vec<BiomeType>>,
    pub size: f32,
    pub resolution: u32,
}

impl BiomeMap {
    /// Sample biome at world coordinates
    pub fn sample_biome(&self, world_x: f32, world_z: f32) -> BiomeType {
        let half_size = self.size * 0.5;
        
        // Convert world coordinates to grid coordinates
        let grid_x = ((world_x + half_size) / self.size * (self.resolution - 1) as f32) as usize;
        let grid_z = ((world_z + half_size) / self.size * (self.resolution - 1) as f32) as usize;
        
        // Clamp to grid bounds
        let grid_x = grid_x.min(self.resolution as usize - 1);
        let grid_z = grid_z.min(self.resolution as usize - 1);
        
        self.grid[grid_z][grid_x].clone()
    }
}

/// System to spawn terrain using the clean utility
pub fn setup_simple_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("🌍 Setting up simplified terrain with clean utility...");

    // Create terrain generator with clean parameters
    let terrain_generator = TerrainGenerator::new(
        200.0,    // size: 200x200 world units
        64,       // resolution: 64x64 vertices
        20.0,     // max_height: 20 units of height variation
        0.05,     // noise_scale: 0.05 creates more varied hills  
        42,       // seed: reproducible terrain
    );

    // Generate biome-aware terrain data
    let (height_grid, biome_grid) = terrain_generator.generate_biome_height_grid();
    
    // Debug: Log sample heights from the grid
    info!("🔍 TERRAIN DEBUG - Height grid samples:");
    for (z_idx, z_val) in [0, 16, 32, 48, 63].iter().enumerate() {
        for (x_idx, x_val) in [0, 16, 32, 48, 63].iter().enumerate() {
            let height = height_grid[*z_val][*x_val];
            let world_x = (*x_val as f32 / 63.0) * 200.0 - 100.0;
            let world_z = (*z_val as f32 / 63.0) * 200.0 - 100.0;
            info!("   Grid[{},{}] -> World({:.1},{:.1}) = Height {:.2}", x_val, z_val, world_x, world_z, height);
        }
    }
    
    // Debug: Compare grid heights with direct sampling
    info!("🔍 TERRAIN DEBUG - Direct vs Grid sampling comparison:");
    let test_positions = [(-70.0, -70.0), (0.0, 0.0), (50.0, 30.0)];
    for (world_x, world_z) in test_positions.iter() {
        let direct_height = terrain_generator.sample_height_with_biome(*world_x, *world_z).0;
        
        // Calculate grid position for comparison (same as debug logging)
        let grid_x = ((*world_x + 100.0) / 200.0 * 63.0) as usize;
        let grid_z = ((*world_z + 100.0) / 200.0 * 63.0) as usize;
        let grid_x = grid_x.min(63);
        let grid_z = grid_z.min(63);
        let grid_height = height_grid[grid_z][grid_x];
        
        // Also sample what the grid generation SHOULD have produced at that exact grid cell
        let grid_world_x = (grid_x as f32 / (64 - 1) as f32) * 200.0 - 100.0;
        let grid_world_z = (grid_z as f32 / (64 - 1) as f32) * 200.0 - 100.0;
        let expected_grid_height = terrain_generator.sample_height_with_biome(grid_world_x, grid_world_z).0;
        
        info!("   Pos({:.1},{:.1}): Direct={:.3}, Grid[{},{}]={:.3}, Expected={:.3}", 
              world_x, world_z, direct_height, grid_x, grid_z, grid_height, expected_grid_height);
        info!("   GridCell({:.1},{:.1}): Diff_Direct={:.3}, Diff_Expected={:.3}", 
              grid_world_x, grid_world_z, (direct_height - grid_height).abs(), 
              (expected_grid_height - grid_height).abs());
    }
    
    // Generate visual mesh using biome-influenced heights
    let terrain_mesh = terrain_generator.generate_visual_mesh_from_grid(&height_grid);
    
    // Debug: Matrix layout verification before physics collider generation
    info!("🔍 MATRIX DEBUG - Original heights[z][x] layout:");
    for sample_z in [0, 31, 63] {
        for sample_x in [0, 31, 63] {
            let world_x = (sample_x as f32 / 63.0) * 200.0 - 100.0;
            let world_z = (sample_z as f32 / 63.0) * 200.0 - 100.0;
            info!("   heights[{}][{}] = {:.3} at world({:.1},{:.1})", 
                  sample_z, sample_x, height_grid[sample_z][sample_x], world_x, world_z);
        }
    }
    
    // Generate physics collider using SAME height data with matrix transpose fix
    let terrain_collider = terrain_generator.generate_physics_collider_from_grid(&height_grid);
    
    info!("🔍 MATRIX DEBUG - Physics collider created with transposed matrix (heightfield format)");

    // Create biome-aware terrain material that displays vertex colors
    let terrain_material = StandardMaterial {
        base_color: Color::WHITE, // White base to show vertex colors properly
        perceptual_roughness: 0.9,
        metallic: 0.0,
        ..default()
    };

    info!("Generated terrain mesh with physics collider");

    // Spawn terrain entity with both visual and physics components
    commands.spawn((
        Mesh3d(meshes.add(terrain_mesh)),
        MeshMaterial3d(materials.add(terrain_material)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        terrain_collider,
        RigidBody::Static,
        Name::new("Terrain"),
        Terrain,
    ));

    // Create height sampler resource for runtime queries
    commands.insert_resource(TerrainHeightSampler::new(terrain_generator));
    
    // Store biome data as a resource for other systems to use
    commands.insert_resource(BiomeMap { 
        grid: biome_grid, 
        size: 200.0, 
        resolution: 64 
    });

    info!("✅ Biome-aware terrain spawned with unified height system!");
    info!("   Size: 200x200 world units");
    info!("   Resolution: 64x64 vertices");
    info!("   Max height: 20 units");
    info!("   Biomes: Mountains, Hills, Plains, Forest, Desert");
}

/// Sample terrain height at world coordinates using the height sampler resource
pub fn sample_terrain_height(sampler: &TerrainHeightSampler, world_x: f32, world_z: f32) -> f32 {
    sampler.sample_height(world_x, world_z)
}

/// Debug system to verify terrain height alignment
pub fn debug_terrain_alignment(
    time: Res<Time>,
    sampler: Res<TerrainHeightSampler>,
) {
    // Only run debug check every 2 seconds
    if time.elapsed_secs() % 2.0 < time.delta_secs() {
        // Test several positions across the terrain
        let test_positions = [
            (-70.0, -70.0), // Player spawn area
            (0.0, 0.0),     // Center
            (50.0, 30.0),   // Random position
            (-30.0, 60.0),  // Another random position
        ];

        info!("🔍 TERRAIN HEIGHT DEBUG:");
        for (x, z) in test_positions.iter() {
            let height = sample_terrain_height(&sampler, *x, *z);
            info!("   Position ({:6.1}, {:6.1}): height = {:6.3}", x, z, height);
        }
    }
}