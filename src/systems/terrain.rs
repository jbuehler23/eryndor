use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use avian3d::prelude::*;

/// Terrain generation system for creating rolling hills and varied landscapes
/// Following Single Responsibility: only handles terrain mesh generation and physics

/// Component to mark terrain entities
#[derive(Component)]
pub struct Terrain;

/// Resource for runtime terrain height queries
#[derive(Resource, Clone)]
pub struct TerrainHeightSampler {
    pub config: TerrainConfig,
}

/// Terrain configuration for world generation
#[derive(Component, Clone)]
pub struct TerrainConfig {
    pub size: f32,           // Size of terrain in world units
    pub resolution: u32,     // Number of vertices per side (higher = more detailed)
    pub height_scale: f32,   // Maximum height variation
    pub noise_scale: f32,    // Scale of noise pattern (smaller = more detailed)
}

impl Default for TerrainConfig {
    fn default() -> Self {
        Self {
            size: 200.0,         // 200x200 world units
            resolution: 100,     // 100x100 vertices (reasonable detail)
            height_scale: 20.0,  // Up to 20 units height variation  
            noise_scale: 0.05,   // Good balance of hills and valleys
        }
    }
}

/// Generate terrain mesh with height-based noise for rolling hills
/// Returns both the visual mesh and collision data (vertices, triangles)
pub fn generate_terrain_mesh(config: &TerrainConfig) -> (Mesh, Vec<Vec3>, Vec<[u32; 3]>) {
    let resolution = config.resolution;
    let size = config.size;
    let height_scale = config.height_scale;
    let noise_scale = config.noise_scale;
    
    // Calculate vertex positions with noise-based height
    let mut positions = Vec::with_capacity((resolution * resolution) as usize);
    let mut normals = Vec::with_capacity((resolution * resolution) as usize);
    let mut uvs = Vec::with_capacity((resolution * resolution) as usize);
    let mut indices = Vec::with_capacity(((resolution - 1) * (resolution - 1) * 6) as usize);
    
    // Generate vertices
    for z in 0..resolution {
        for x in 0..resolution {
            let world_x = (x as f32 / (resolution - 1) as f32) * size - size * 0.5;
            let world_z = (z as f32 / (resolution - 1) as f32) * size - size * 0.5;
            
            // Simple noise function for rolling hills (multiple octaves)
            let height = generate_height_at_position(world_x, world_z, noise_scale, height_scale);
            
            positions.push([world_x, height, world_z]);
            normals.push([0.0, 1.0, 0.0]); // Will be recalculated later
            uvs.push([x as f32 / (resolution - 1) as f32, z as f32 / (resolution - 1) as f32]);
        }
    }
    
    // Generate triangle indices for mesh faces
    for z in 0..(resolution - 1) {
        for x in 0..(resolution - 1) {
            let top_left = z * resolution + x;
            let top_right = top_left + 1;
            let bottom_left = (z + 1) * resolution + x;
            let bottom_right = bottom_left + 1;
            
            // Two triangles per quad
            // Triangle 1: top_left -> bottom_left -> top_right
            indices.push(top_left);
            indices.push(bottom_left);
            indices.push(top_right);
            
            // Triangle 2: top_right -> bottom_left -> bottom_right  
            indices.push(top_right);
            indices.push(bottom_left);
            indices.push(bottom_right);
        }
    }
    
    // Calculate proper normals for lighting
    calculate_normals(&mut normals, &positions, &indices, resolution);
    
    // Convert positions to Vec3 for collision
    let collision_vertices: Vec<Vec3> = positions.iter()
        .map(|pos| Vec3::new(pos[0], pos[1], pos[2]))
        .collect();
    
    // Convert flat indices to triangle array format for Avian collision
    let collision_triangles: Vec<[u32; 3]> = indices
        .chunks(3)
        .map(|chunk| [chunk[0], chunk[1], chunk[2]])
        .collect();
    
    // Create Bevy mesh
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    
    (mesh, collision_vertices, collision_triangles)
}

/// Generate heightfield data for Avian physics collider
/// Returns a 2D matrix of height values for heightfield collider
pub fn generate_heightfield_data(config: &TerrainConfig) -> Vec<Vec<f32>> {
    let resolution = config.resolution as usize;
    let mut heights = Vec::with_capacity(resolution);
    
    for z in 0..resolution {
        let mut row = Vec::with_capacity(resolution);
        for x in 0..resolution {
            let world_x = (x as f32 / (resolution - 1) as f32) * config.size - config.size * 0.5;
            let world_z = (z as f32 / (resolution - 1) as f32) * config.size - config.size * 0.5;
            
            let height = generate_height_at_position(world_x, world_z, config.noise_scale, config.height_scale);
            row.push(height);
        }
        heights.push(row);
    }
    
    heights
}

/// Simple noise function for terrain height generation
/// Uses multiple octaves for more natural-looking hills
pub fn generate_height_at_position(x: f32, z: f32, noise_scale: f32, height_scale: f32) -> f32 {
    // Primary noise layer - large rolling hills
    let noise1 = (x * noise_scale).sin() * (z * noise_scale).cos() * 0.7;
    
    // Secondary noise layer - medium details  
    let noise2 = (x * noise_scale * 2.0).sin() * (z * noise_scale * 2.0).cos() * 0.2;
    
    // Tertiary noise layer - fine details
    let noise3 = (x * noise_scale * 4.0).sin() * (z * noise_scale * 4.0).cos() * 0.1;
    
    // Combine noise layers and apply height scaling
    (noise1 + noise2 + noise3) * height_scale
}

/// Calculate proper vertex normals for smooth lighting
fn calculate_normals(normals: &mut Vec<[f32; 3]>, positions: &[[f32; 3]], indices: &[u32], _resolution: u32) {
    // Reset all normals to zero
    for normal in normals.iter_mut() {
        *normal = [0.0, 0.0, 0.0];
    }
    
    // Calculate face normals and accumulate to vertex normals
    for triangle in indices.chunks(3) {
        let i0 = triangle[0] as usize;
        let i1 = triangle[1] as usize;  
        let i2 = triangle[2] as usize;
        
        let v0 = Vec3::from(positions[i0]);
        let v1 = Vec3::from(positions[i1]);
        let v2 = Vec3::from(positions[i2]);
        
        // Calculate face normal using cross product
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let face_normal = edge1.cross(edge2).normalize();
        
        // Add face normal to all three vertices
        for &idx in &[i0, i1, i2] {
            normals[idx][0] += face_normal.x;
            normals[idx][1] += face_normal.y;
            normals[idx][2] += face_normal.z;
        }
    }
    
    // Normalize all vertex normals
    for normal in normals.iter_mut() {
        let length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
        if length > 0.0 {
            normal[0] /= length;
            normal[1] /= length;
            normal[2] /= length;
        }
    }
}

/// System to spawn the initial terrain
pub fn setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let config = TerrainConfig::default();
    
    // Generate terrain mesh and collision data
    let (terrain_mesh, collision_vertices, collision_triangles) = generate_terrain_mesh(&config);
    
    // Store triangle count before move
    let triangle_count = collision_triangles.len();
    
    // Create basic grass-like material
    let terrain_material = StandardMaterial {
        base_color: Color::srgb(0.3, 0.7, 0.2), // Green grass color
        perceptual_roughness: 0.8, // Rough surface for realistic grass
        metallic: 0.0,  // Non-metallic
        ..default()
    };
    
    
    // Spawn terrain entity with physics collision
    commands.spawn((
        Mesh3d(meshes.add(terrain_mesh.clone())),
        MeshMaterial3d(materials.add(terrain_material)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        
        // Physics components - Direct trimesh collision from mesh data
        RigidBody::Static, // Static rigidbody for terrain
        Collider::trimesh(collision_vertices, collision_triangles),
        CollisionMargin(0.01), // Collision margin for trimesh stability
        // Game components
        Terrain,
        config.clone(),
    ));
    
    // Terrain collision now working with direct trimesh approach!
    // Debug flat ground plane removed since terrain collision is successful
    
    // Create and insert height sampler resource for runtime queries
    let height_sampler = TerrainHeightSampler {
        config: config.clone(),
    };
    commands.insert_resource(height_sampler);
    
    
    info!("Terrain generated: {}x{} units with {} vertices", 
          config.size, config.size, config.resolution * config.resolution);
    info!("Terrain height range: -{} to +{} units", config.height_scale, config.height_scale);
    info!("Terrain collision: Direct trimesh with {} triangles", triangle_count);
    info!("Terrain height sampler initialized for runtime queries");
}

/// Sample terrain height at given world coordinates
/// Returns terrain height at (x, z) or 0.0 if outside terrain bounds
pub fn sample_terrain_height(sampler: &TerrainHeightSampler, x: f32, z: f32) -> f32 {
    let config = &sampler.config;
    let half_size = config.size * 0.5;
    
    // Check if position is within terrain bounds
    if x < -half_size || x > half_size || z < -half_size || z > half_size {
        return 0.0; // Return ground level for out-of-bounds positions
    }
    
    // Sample height using same noise function as terrain generation
    generate_height_at_position(x, z, config.noise_scale, config.height_scale)
}

/// Sample terrain height with small random variation for natural placement
/// Pass in a random value from the calling system to avoid importing rand here
pub fn sample_terrain_height_with_variation(
    sampler: &TerrainHeightSampler, 
    x: f32, 
    z: f32, 
    random_offset: f32
) -> f32 {
    let base_height = sample_terrain_height(sampler, x, z);
    base_height + random_offset
}