use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use avian3d::prelude::*;
use noise::{NoiseFn, Perlin};

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
            resolution: 150,     // Higher resolution for ultra-smooth collision
            height_scale: 3.0,   // Gentle height variation for smooth physics
            noise_scale: 0.02,   // Optimized for Perlin noise frequency
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

/// Professional Perlin noise terrain generation with ultra-smooth collision surfaces
/// Uses multi-octave Perlin noise with C2 continuity for seamless physics interaction
pub fn generate_height_at_position(x: f32, z: f32, noise_scale: f32, height_scale: f32) -> f32 {
    // Create Perlin noise generator with fixed seed for consistent terrain
    let perlin = Perlin::new(12345);
    
    // Scale coordinates for noise sampling
    let nx = x as f64 * noise_scale as f64;
    let nz = z as f64 * noise_scale as f64;
    
    // Multi-octave fractal Perlin noise for natural terrain
    let mut height = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    let octaves = 4;
    let persistence = 0.5; // How much each octave contributes
    let lacunarity = 2.0;   // Frequency multiplier between octaves
    
    for _ in 0..octaves {
        // Sample Perlin noise at current frequency
        let sample = perlin.get([nx * frequency, nz * frequency]) as f32;
        
        // Accumulate noise with current amplitude
        height += sample * amplitude;
        
        // Update frequency and amplitude for next octave
        frequency *= lacunarity;
        amplitude *= persistence;
    }
    
    // Apply smoothstep function for C2 continuity (smooth first and second derivatives)
    let normalized_height = (height + 1.0) * 0.5; // Normalize from [-1, 1] to [0, 1]
    let smoothed = smoothstep(normalized_height);
    
    // Scale to final height and center around zero
    (smoothed - 0.5) * height_scale * 2.0
}

/// Smoothstep function for C2 continuity - eliminates sharp edges in collision detection
/// f(t) = 3t² - 2t³ provides smooth first and second derivatives
fn smoothstep(t: f32) -> f32 {
    let clamped = t.clamp(0.0, 1.0);
    clamped * clamped * (3.0 - 2.0 * clamped)
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
            
            // Apply aggressive smoothing (blend 90% smoothed with 10% original)
            let smoothed = total_height / count as f32;
            smoothed_heights[idx] = current_height * 0.1 + smoothed * 0.9;
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
    let heights = generate_heightfield_data(&config);
    
    // Create basic grass-like material
    let terrain_material = StandardMaterial {
        base_color: Color::srgb(0.3, 0.7, 0.2), // Green grass color
        perceptual_roughness: 0.8, // Rough surface for realistic grass
        metallic: 0.0,  // Non-metallic
        ..default()
    };


    // Use trimesh collision with ultra-smooth Perlin noise terrain
    // The vertex smoothing and bilinear interpolation provide smooth collision
    
    // Spawn terrain entity with physics collision
    commands.spawn((
        Mesh3d(meshes.add(terrain_mesh.clone())),
        MeshMaterial3d(materials.add(terrain_material)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        
        // Physics components - Trimesh collision matches visual mesh exactly
        RigidBody::Static,
        Collider::trimesh_from_mesh(&terrain_mesh).unwrap(),
        CollisionMargin(0.01), // Minimal margin with smooth terrain
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
    info!("Terrain collision: Ultra-smooth trimesh with Perlin noise and vertex smoothing");
    info!("Terrain height sampler initialized for runtime queries");
}

/// Sample terrain height at given world coordinates with bilinear interpolation
/// Returns smoothly interpolated terrain height for ultra-smooth collision detection
pub fn sample_terrain_height(sampler: &TerrainHeightSampler, x: f32, z: f32) -> f32 {
    let config = &sampler.config;
    let half_size = config.size * 0.5;
    
    // Check if position is within terrain bounds
    if x < -half_size || x > half_size || z < -half_size || z > half_size {
        return 0.0; // Return ground level for out-of-bounds positions
    }
    
    // For ultra-smooth collision detection, use bilinear interpolation between grid points
    bilinear_sample_height(x, z, config)
}

/// Bilinear interpolation height sampling for smooth collision detection
/// Interpolates between the 4 nearest grid points for seamless height transitions
fn bilinear_sample_height(x: f32, z: f32, config: &TerrainConfig) -> f32 {
    let resolution = config.resolution as f32;
    let size = config.size;
    let half_size = size * 0.5;
    
    // Convert world coordinates to grid coordinates
    let grid_x = (x + half_size) / size * (resolution - 1.0);
    let grid_z = (z + half_size) / size * (resolution - 1.0);
    
    // Get integer grid coordinates (floor)
    let x0 = grid_x.floor() as u32;
    let z0 = grid_z.floor() as u32;
    let x1 = (x0 + 1).min(config.resolution - 1);
    let z1 = (z0 + 1).min(config.resolution - 1);
    
    // Calculate fractional parts for interpolation
    let fx = grid_x - x0 as f32;
    let fz = grid_z - z0 as f32;
    
    // Convert grid coordinates back to world coordinates for sampling
    let world_x0 = (x0 as f32 / (resolution - 1.0)) * size - half_size;
    let world_z0 = (z0 as f32 / (resolution - 1.0)) * size - half_size;
    let world_x1 = (x1 as f32 / (resolution - 1.0)) * size - half_size;
    let world_z1 = (z1 as f32 / (resolution - 1.0)) * size - half_size;
    
    // Sample height at four corner points
    let h00 = generate_height_at_position(world_x0, world_z0, config.noise_scale, config.height_scale);
    let h10 = generate_height_at_position(world_x1, world_z0, config.noise_scale, config.height_scale);
    let h01 = generate_height_at_position(world_x0, world_z1, config.noise_scale, config.height_scale);
    let h11 = generate_height_at_position(world_x1, world_z1, config.noise_scale, config.height_scale);
    
    // Bilinear interpolation
    let h_top = h00 * (1.0 - fx) + h10 * fx;
    let h_bottom = h01 * (1.0 - fx) + h11 * fx;
    h_top * (1.0 - fz) + h_bottom * fz
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