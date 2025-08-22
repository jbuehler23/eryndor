use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use avian3d::prelude::*;
use noise::{NoiseFn, Perlin};
use std::collections::HashMap;

/// Advanced procedural terrain generation system with multi-biome support
/// Uses multiple noise layers for temperature, humidity, and elevation to create diverse biomes
/// Following Single Responsibility: handles terrain mesh generation, biome classification, and physics

/// Component to mark terrain entities
#[derive(Component)]
pub struct Terrain;

/// Biome types available for procedural generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BiomeType {
    DeepWater,    // Deep ocean areas
    Water,        // Shallow water, lakes, rivers
    Beach,        // Sandy shores and coastal areas
    Plains,       // Grasslands and open areas
    Forest,       // Dense woodland areas
    Mountains,    // Rocky peaks and highlands
    Desert,       // Arid regions (future expansion)
}

/// Environmental parameters for biome determination
/// Based on Whittaker biome classification approach
#[derive(Debug, Clone, Copy)]
pub struct BiomeParameters {
    pub temperature: f32,      // -1.0 (cold) to 1.0 (hot)
    pub humidity: f32,         // 0.0 (dry) to 1.0 (wet)
    pub elevation: f32,        // 0.0 (sea level) to 1.0 (mountain peaks)
    pub continentalness: f32,  // 0.0 (oceanic) to 1.0 (continental)
    pub erosion: f32,          // 0.0 (eroded) to 1.0 (sharp features)
}

/// Biome-specific height modification settings
#[derive(Debug, Clone, Copy)]
pub struct BiomeHeightProfile {
    pub height_multiplier: f32,    // How much to amplify/flatten the base height
    pub height_offset: f32,        // Base height offset for this biome
    pub roughness: f32,           // Additional noise detail for this biome
}

/// Resource for runtime terrain height queries
#[derive(Resource, Clone)]
pub struct TerrainHeightSampler {
    pub config: TerrainConfig,
}

/// Advanced terrain configuration for multi-biome world generation
#[derive(Component, Clone)]
pub struct TerrainConfig {
    // Basic terrain properties
    pub size: f32,                    // Size of terrain in world units
    pub resolution: u32,              // Number of vertices per side (higher = more detailed)
    pub height_scale: f32,            // Maximum height variation
    
    // Multi-parameter noise configuration
    pub elevation_scale: f32,         // Scale for elevation noise (smaller = more detailed)
    pub temperature_scale: f32,       // Scale for temperature noise
    pub humidity_scale: f32,          // Scale for humidity noise
    pub continentalness_scale: f32,   // Scale for continentalness noise
    pub erosion_scale: f32,           // Scale for erosion noise
    
    // Noise seeds for reproducible generation
    pub elevation_seed: u32,
    pub temperature_seed: u32,
    pub humidity_seed: u32,
    pub continentalness_seed: u32,
    pub erosion_seed: u32,
    
    // Water level configuration
    pub water_level: f32,             // Height at which water appears
    pub beach_width: f32,             // Width of beach transition zones
    
    // Biome height profiles
    pub biome_profiles: HashMap<BiomeType, BiomeHeightProfile>,
}

impl Default for TerrainConfig {
    fn default() -> Self {
        let mut biome_profiles = HashMap::new();
        
        // Define height profiles for each biome type
        biome_profiles.insert(BiomeType::DeepWater, BiomeHeightProfile {
            height_multiplier: 0.0,
            height_offset: -5.0,
            roughness: 0.1,
        });
        biome_profiles.insert(BiomeType::Water, BiomeHeightProfile {
            height_multiplier: 0.0,
            height_offset: -1.0,
            roughness: 0.2,
        });
        biome_profiles.insert(BiomeType::Beach, BiomeHeightProfile {
            height_multiplier: 0.3,
            height_offset: 0.5,
            roughness: 0.1,
        });
        biome_profiles.insert(BiomeType::Plains, BiomeHeightProfile {
            height_multiplier: 0.5,
            height_offset: 2.0,
            roughness: 0.3,
        });
        biome_profiles.insert(BiomeType::Forest, BiomeHeightProfile {
            height_multiplier: 0.8,
            height_offset: 3.0,
            roughness: 0.5,
        });
        biome_profiles.insert(BiomeType::Mountains, BiomeHeightProfile {
            height_multiplier: 2.5,
            height_offset: 8.0,
            roughness: 0.8,
        });
        biome_profiles.insert(BiomeType::Desert, BiomeHeightProfile {
            height_multiplier: 0.4,
            height_offset: 1.5,
            roughness: 0.2,
        });
        
        Self {
            // Basic terrain properties
            size: 200.0,
            resolution: 64,              // Increased resolution for better biome detail
            height_scale: 15.0,          // Significant height variation for interesting terrain
            
            // Multi-parameter noise scales (lower = more detailed)
            elevation_scale: 0.01,       // Large-scale elevation features
            temperature_scale: 0.005,    // Very large temperature zones
            humidity_scale: 0.008,       // Medium-scale humidity patterns
            continentalness_scale: 0.003, // Massive continental features
            erosion_scale: 0.02,         // Fine erosion details
            
            // Reproducible noise seeds
            elevation_seed: 12345,
            temperature_seed: 67890,
            humidity_seed: 54321,
            continentalness_seed: 98765,
            erosion_seed: 13579,
            
            // Water configuration
            water_level: 0.0,            // Sea level at height 0
            beach_width: 5.0,            // 5-unit beach transition zones
            
            biome_profiles,
        }
    }
}

/// Generate environmental parameters for a world position using multi-layered noise
/// This creates the foundation for biome classification
pub fn generate_biome_parameters(x: f32, z: f32, config: &TerrainConfig) -> BiomeParameters {
    // Create separate Perlin noise generators for each parameter
    let elevation_noise = Perlin::new(config.elevation_seed);
    let temperature_noise = Perlin::new(config.temperature_seed);
    let humidity_noise = Perlin::new(config.humidity_seed);
    let continentalness_noise = Perlin::new(config.continentalness_seed);
    let erosion_noise = Perlin::new(config.erosion_seed);
    
    // Calculate base noise values with different frequencies
    let elevation = sample_fractal_noise(&elevation_noise, x, z, config.elevation_scale, 4);
    let temperature = sample_fractal_noise(&temperature_noise, x, z, config.temperature_scale, 3);
    let humidity = sample_fractal_noise(&humidity_noise, x, z, config.humidity_scale, 3);
    let continentalness = sample_fractal_noise(&continentalness_noise, x, z, config.continentalness_scale, 2);
    let erosion = sample_fractal_noise(&erosion_noise, x, z, config.erosion_scale, 5);
    
    // DEBUG: Log noise values for key positions
    if (x + 70.0).abs() < 1.0 && (z + 70.0).abs() < 1.0 {
        info!("🔍 PERLIN NOISE DEBUG at ({:.1}, {:.1}):", x, z);
        info!("   Raw elevation: {:.3} -> normalized: {:.3}", elevation, (elevation + 1.0) * 0.5);
        info!("   Raw temperature: {:.3} -> kept: {:.3}", temperature, temperature);
        info!("   Raw humidity: {:.3} -> normalized: {:.3}", humidity, (humidity + 1.0) * 0.5);
        info!("   Raw continentalness: {:.3} -> normalized: {:.3}", continentalness, (continentalness + 1.0) * 0.5);
        info!("   Raw erosion: {:.3} -> normalized: {:.3}", erosion, (erosion + 1.0) * 0.5);
    }
    
    BiomeParameters {
        elevation: (elevation + 1.0) * 0.5,           // Normalize from [-1,1] to [0,1]
        temperature: temperature,                      // Keep in [-1,1] for hot/cold
        humidity: (humidity + 1.0) * 0.5,             // Normalize from [-1,1] to [0,1]
        continentalness: (continentalness + 1.0) * 0.5, // Normalize from [-1,1] to [0,1]
        erosion: (erosion + 1.0) * 0.5,               // Normalize from [-1,1] to [0,1]
    }
}

/// Sample Fractal Brownian Motion (multiple octaves of noise)
/// This creates more natural, detailed terrain features
fn sample_fractal_noise(noise: &Perlin, x: f32, z: f32, base_scale: f32, octaves: usize) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = base_scale;
    let mut max_value = 0.0;
    
    let persistence = 0.5;  // How much each octave contributes
    let lacunarity = 2.0;   // Frequency multiplier between octaves
    
    for _ in 0..octaves {
        let sample = noise.get([x as f64 * frequency as f64, z as f64 * frequency as f64]) as f32;
        value += sample * amplitude;
        max_value += amplitude;
        
        amplitude *= persistence;
        frequency *= lacunarity;
    }
    
    // Normalize to [-1, 1] range
    value / max_value
}

/// Classify a biome based on environmental parameters using Whittaker approach
/// This creates realistic biome distributions based on temperature and humidity
pub fn classify_biome(params: &BiomeParameters) -> BiomeType {
    // Primary classification based on elevation (water boundaries)
    if params.elevation < 0.3 {
        if params.elevation < 0.15 {
            return BiomeType::DeepWater;
        } else {
            return BiomeType::Water;
        }
    }
    
    // Beach zones near water
    if params.elevation < 0.4 && params.continentalness < 0.6 {
        return BiomeType::Beach;
    }
    
    // High elevation always creates mountains
    if params.elevation > 0.7 {
        return BiomeType::Mountains;
    }
    
    // Secondary classification based on temperature and humidity (Whittaker biome matrix)
    match (params.temperature, params.humidity) {
        // Hot and dry -> Desert
        (temp, humid) if temp > 0.3 && humid < 0.3 => BiomeType::Desert,
        
        // Moderate to high humidity -> Forest
        (_, humid) if humid > 0.6 => BiomeType::Forest,
        
        // Cold or moderate conditions with medium humidity -> Plains
        _ => BiomeType::Plains,
    }
}

/// Calculate height for a specific biome type and environmental parameters
/// Uses spline-based height modification for realistic terrain variation
pub fn calculate_biome_height(biome: BiomeType, params: &BiomeParameters, config: &TerrainConfig) -> f32 {
    let profile = config.biome_profiles.get(&biome)
        .unwrap_or(&BiomeHeightProfile {
            height_multiplier: 1.0,
            height_offset: 0.0,
            roughness: 0.5,
        });
    
    // Base height from elevation parameter
    let base_height = params.elevation * config.height_scale;
    
    // Apply biome-specific modifications
    let modified_height = base_height * profile.height_multiplier + profile.height_offset;
    
    // Add fine detail based on erosion and biome roughness
    let detail_noise = (params.erosion - 0.5) * profile.roughness * 2.0;
    
    let final_height = modified_height + detail_noise;
    
    // DEBUG: Log height calculation for key positions  
    if params.elevation > 0.45 && params.elevation < 0.55 { // Around player spawn area
        info!("🏔️ HEIGHT CALC DEBUG:");
        info!("   Biome: {:?}", biome);
        info!("   Elevation param: {:.3} -> base_height: {:.3}", params.elevation, base_height);
        info!("   Profile: mult={:.2}, offset={:.2}, rough={:.2}", profile.height_multiplier, profile.height_offset, profile.roughness);
        info!("   Modified height: {:.3}, detail: {:.3} -> FINAL: {:.3}", modified_height, detail_noise, final_height);
    }
    
    final_height
}

/// Generate a unified heightmap for both rendering and physics
pub fn generate_unified_heightmap(config: &TerrainConfig) -> Vec<Vec<f32>> {
    let resolution = config.resolution as usize;
    let size = config.size;
    let mut heights = Vec::with_capacity(resolution);

    for z in 0..resolution {
        let mut row = Vec::with_capacity(resolution);
        for x in 0..resolution {
            let world_x = (x as f32 / (resolution - 1) as f32) * size - size * 0.5;
            let world_z = (z as f32 / (resolution - 1) as f32) * size - size * 0.5;

            let biome_params = generate_biome_parameters(world_x, world_z, config);
            let biome_type = classify_biome(&biome_params);
            let height = calculate_biome_height(biome_type, &biome_params, config);
            row.push(height);
        }
        heights.push(row);
    }

    heights
}

/// Generate terrain mesh with advanced multi-biome procedural generation
/// Returns both the visual mesh and collision data (vertices, triangles)
pub fn generate_terrain_mesh(config: &TerrainConfig, heightmap: &Vec<Vec<f32>>) -> (Mesh, Vec<Vec3>, Vec<[u32; 3]>) {
    let resolution = config.resolution;
    let size = config.size;
    
    // Calculate vertex positions using advanced biome-based height generation
    let mut positions = Vec::with_capacity((resolution * resolution) as usize);
    let mut normals = Vec::with_capacity((resolution * resolution) as usize);
    let mut uvs = Vec::with_capacity((resolution * resolution) as usize);
    let mut colors = Vec::with_capacity((resolution * resolution) as usize);
    let mut indices = Vec::with_capacity(((resolution - 1) * (resolution - 1) * 6) as usize);
    
    // Generate vertices with biome-aware height calculation
    for z in 0..resolution {
        for x in 0..resolution {
            let world_x = (x as f32 / (resolution - 1) as f32) * size - size * 0.5;
            let world_z = (z as f32 / (resolution - 1) as f32) * size - size * 0.5;
            
            let height = heightmap[z as usize][x as usize];
            
            positions.push([world_x, height, world_z]);
            normals.push([0.0, 1.0, 0.0]); // Will be recalculated later
            uvs.push([x as f32 / (resolution - 1) as f32, z as f32 / (resolution - 1) as f32]);
            
            // Add biome-based vertex colors
            let biome_params = generate_biome_parameters(world_x, world_z, config);
            let biome_type = classify_biome(&biome_params);
            let color = match biome_type {
                BiomeType::DeepWater => [0.1, 0.2, 0.6, 1.0],    // Deep blue
                BiomeType::Water => [0.2, 0.4, 0.8, 1.0],        // Light blue
                BiomeType::Beach => [0.9, 0.8, 0.6, 1.0],        // Sandy tan
                BiomeType::Plains => [0.3, 0.7, 0.2, 1.0],       // Green
                BiomeType::Forest => [0.2, 0.5, 0.1, 1.0],       // Dark green
                BiomeType::Mountains => [0.5, 0.4, 0.3, 1.0],    // Brown/gray
                BiomeType::Desert => [0.8, 0.7, 0.4, 1.0],       // Sandy yellow
            };
            colors.push(color);
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
    
    // TEMPORARILY DISABLED: Smooth vertices to reduce collision artifacts
    // smooth_terrain_vertices(&mut positions, resolution);
    
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
    
    // Create Bevy mesh with biome-based vertex colors
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
    
    (mesh, collision_vertices, collision_triangles)
}

/// Generate simple flat heightfield data for ultra-smooth collision testing
/// Returns a 2D matrix of height values - starting completely flat for testing
pub fn generate_flat_heightfield_data(config: &TerrainConfig) -> Vec<Vec<f32>> {
    let resolution = config.resolution as usize;
    let mut heights = Vec::with_capacity(resolution);
    
    // Generate completely flat terrain for perfect collision testing
    for _z in 0..resolution {
        let mut row = Vec::with_capacity(resolution);
        for _x in 0..resolution {
            // All heights are 0.0 - perfectly flat terrain
            row.push(0.0);
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
            
            // Apply gentle smoothing to preserve terrain features (blend 70% original with 30% smoothed)
            let smoothed = total_height / count as f32;
            smoothed_heights[idx] = current_height * 0.7 + smoothed * 0.3;
        }
    }
    
    // Apply smoothed heights back to positions
    let mut height_changes = Vec::new();
    for i in 0..smoothed_heights.len() {
        let original_height = positions[i][1];
        positions[i][1] = smoothed_heights[i];
        let height_change = (original_height - smoothed_heights[i]).abs();
        if height_change > 0.1 {
            height_changes.push((i, original_height, smoothed_heights[i], height_change));
        }
    }
    
    // DEBUG: Log significant height changes from smoothing
    if !height_changes.is_empty() {
        info!("🔧 SMOOTHING applied {} significant height changes (>0.1 units):", height_changes.len());
        for (i, original, smoothed, change) in height_changes.iter().take(5) {
            info!("   Vertex {}: {:.3} -> {:.3} (change: {:.3})", i, original, smoothed, change);
        }
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

/// Generate biome-based heightfield data for physics collision
/// This matches the visual terrain mesh for perfect collision accuracy
pub fn generate_biome_heightfield_data(heightmap: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    heightmap.clone()
}

/// System to spawn the initial terrain
pub fn setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let config = TerrainConfig::default();
    
    // Generate a unified heightmap for both rendering and physics
    let heightmap = generate_unified_heightmap(&config);

    // Generate advanced biome-based terrain for rendering
    let (terrain_mesh, _, _) = generate_terrain_mesh(&config, &heightmap);
    
    // Generate biome-based heightfield collision matching the visual terrain
    let heights = generate_biome_heightfield_data(&heightmap);
    
    // Create terrain material that uses vertex colors for biome visualization
    let terrain_material = StandardMaterial {
        base_color: Color::WHITE, // White base to show vertex colors properly
        perceptual_roughness: 0.9, 
        metallic: 0.0,
        cull_mode: None, // Show both sides of triangles for debugging
        ..default()
    };

    // Create heightfield collider with biome-based terrain heights
    // Heights are in world coordinates, scale provides terrain dimensions
    let heightfield_collider = Collider::heightfield(
        heights,
        Vec3::new(config.size, 1.0, config.size), // Scale: horizontal size x unit height x horizontal size
    );
    
    // Spawn terrain entity with physics collision
    commands.spawn((
        Mesh3d(meshes.add(terrain_mesh.clone())),
        MeshMaterial3d(materials.add(terrain_material)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        
        // Physics components - Heightfield collider for ultra-smooth collision
        RigidBody::Static,
        heightfield_collider,
        CollisionMargin(0.0), // No margin needed with heightfield
        // Game components
        Terrain,
        config.clone(),
    ));
    
    // Create and insert height sampler resource for runtime queries
    let height_sampler = TerrainHeightSampler {
        config: config.clone(),
    };
    commands.insert_resource(height_sampler);
    
    
    info!("🌍 PROCEDURAL BIOME TERRAIN: {}x{} units with {} vertices", 
          config.size, config.size, config.resolution * config.resolution);
    info!("🏔️ Multi-biome generation: Mountains, Forest, Plains, Beach, Water zones");
    info!("🎲 Noise parameters: elevation={:.3}, temp={:.3}, humidity={:.3}", 
          config.elevation_scale, config.temperature_scale, config.humidity_scale);
    info!("⚡ Terrain collision: Advanced biome-based heightfield collision ACTIVE");
    info!("🎨 Visual terrain: Full biome-based procedural generation active");
    info!("📊 Terrain height sampler initialized for runtime biome queries");
}

/// Sample terrain height using advanced biome-based generation
/// Returns the actual calculated height for the given world position
pub fn sample_terrain_height(sampler: &TerrainHeightSampler, x: f32, z: f32) -> f32 {
    // Generate environmental parameters for this position
    let biome_params = generate_biome_parameters(x, z, &sampler.config);
    
    // Classify the biome and calculate height
    let biome_type = classify_biome(&biome_params);
    let height = calculate_biome_height(biome_type, &biome_params, &sampler.config);
    
    height
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
    
    // Sample height at four corner points using new biome-based system
    let params00 = generate_biome_parameters(world_x0, world_z0, config);
    let biome00 = classify_biome(&params00);
    let h00 = calculate_biome_height(biome00, &params00, config);
    
    let params10 = generate_biome_parameters(world_x1, world_z0, config);
    let biome10 = classify_biome(&params10);
    let h10 = calculate_biome_height(biome10, &params10, config);
    
    let params01 = generate_biome_parameters(world_x0, world_z1, config);
    let biome01 = classify_biome(&params01);
    let h01 = calculate_biome_height(biome01, &params01, config);
    
    let params11 = generate_biome_parameters(world_x1, world_z1, config);
    let biome11 = classify_biome(&params11);
    let h11 = calculate_biome_height(biome11, &params11, config);
    
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