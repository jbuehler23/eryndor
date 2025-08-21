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
            resolution: 120,     // Increased resolution for smoother collision
            height_scale: 4.0,   // Reduced height variation for gentler slopes
            noise_scale: 0.025,  // Slightly reduced noise scale for smoother transitions
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
    
    // Smooth vertices to reduce collision artifacts
    smooth_terrain_vertices(&mut positions, resolution);
    
    // Calculate proper normals for lighting after smoothing
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

/// Improved smooth noise function for terrain height generation
/// Uses smoothed sine waves and interpolation for continuous collision surfaces
pub fn generate_height_at_position(x: f32, z: f32, noise_scale: f32, height_scale: f32) -> f32 {
    // Use smoother noise functions to prevent collision artifacts
    
    // Primary layer - smooth rolling hills using combined sine/cosine
    let phase1_x = x * noise_scale;
    let phase1_z = z * noise_scale;
    let noise1 = (phase1_x.sin() + phase1_z.cos()) * 0.35; // Reduced amplitude
    
    // Secondary layer - gentler variation with offset phases
    let phase2_x = x * noise_scale * 1.7 + 1.3; // Offset phase to avoid alignment
    let phase2_z = z * noise_scale * 1.7 + 2.1;
    let noise2 = (phase2_x.sin() + phase2_z.cos()) * 0.15;
    
    // Tertiary layer - very subtle detail with smooth transitions
    let phase3_x = x * noise_scale * 2.3 + 0.7;
    let phase3_z = z * noise_scale * 2.3 + 1.9;
    let noise3 = (phase3_x.sin() + phase3_z.cos()) * 0.05;
    
    // Apply smoothstep-like function to reduce sharp transitions
    let combined_noise = noise1 + noise2 + noise3;
    let smoothed = combined_noise * combined_noise * combined_noise; // Cubic smoothing
    
    // Scale and ensure continuous derivatives
    smoothed * height_scale * 0.8 // Reduced overall height variation for smoother collision
}

/// Smooth terrain vertices to reduce collision artifacts and sharp edges
/// Uses neighbor averaging to create gentler height transitions
fn smooth_terrain_vertices(positions: &mut [[f32; 3]], resolution: u32) {
    let res = resolution as usize;
    let mut smoothed_heights = vec![0.0; res * res];
    
    // Apply smoothing kernel to height values
    for z in 0..res {
        for x in 0..res {
            let idx = z * res + x;
            let current_height = positions[idx][1];
            
            let mut total_height = current_height;
            let mut count = 1;
            
            // Sample neighboring vertices for smoothing
            let neighbors = [
                (-1, -1), (0, -1), (1, -1),
                (-1,  0),          (1,  0),
                (-1,  1), (0,  1), (1,  1),
            ];
            
            for (dx, dz) in neighbors.iter() {
                let nx = x as i32 + dx;
                let nz = z as i32 + dz;
                
                if nx >= 0 && nx < res as i32 && nz >= 0 && nz < res as i32 {
                    let neighbor_idx = (nz as usize) * res + (nx as usize);
                    total_height += positions[neighbor_idx][1];
                    count += 1;
                }
            }
            
            // Apply gentle smoothing (blend 70% smoothed with 30% original)
            let smoothed = total_height / count as f32;
            smoothed_heights[idx] = current_height * 0.3 + smoothed * 0.7;
        }
    }
    
    // Apply smoothed heights back to positions
    for i in 0..smoothed_heights.len() {
        positions[i][1] = smoothed_heights[i];
    }
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
    let (terrain_mesh, _collision_vertices, _collision_triangles) = generate_terrain_mesh(&config);
    let _heights = generate_heightfield_data(&config);
    
    // Create basic grass-like material
    let terrain_material = StandardMaterial {
        base_color: Color::srgb(0.3, 0.7, 0.2), // Green grass color
        perceptual_roughness: 0.8, // Rough surface for realistic grass
        metallic: 0.0,  // Non-metallic
        ..default()
    };


// // Build a heightfield collider instead of a trimesh
// Collider::heightfield(
//     heights.into_iter().flatten().collect(),
//     resolution,
//     resolution,
//     Vec3::new(config.size, config.height_scale * 2.0, config.size),
// )
    // Spawn terrain entity with physics collision
    commands.spawn((
        Mesh3d(meshes.add(terrain_mesh.clone())),
        MeshMaterial3d(materials.add(terrain_material)),
        Transform::from_xyz(0.0, 0.0, 0.0), // Heightfield origin at center
        
        // Physics components - Direct trimesh from visual mesh (exact match with collision margin)
        RigidBody::Static, // Static rigidbody for terrain
        Collider::trimesh_from_mesh(&terrain_mesh).unwrap(),
        CollisionMargin(0.05), // Reduced collision margin for smoother terrain contact
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
    info!("Terrain collision: Direct trimesh from visual mesh (exact match with larger margin)");
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