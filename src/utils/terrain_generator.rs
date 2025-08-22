/// Clean, focused terrain generation utility using noise-rs
/// Provides single source of truth for height data shared between visual and physics systems
use noise::{NoiseFn, Fbm, Perlin, MultiFractal};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use avian3d::prelude::*;
use crate::systems::biomes::BiomeType;

/// Simple terrain configuration with clean parameters
#[derive(Debug, Clone)]
pub struct TerrainGenerator {
    /// Terrain size in world units (e.g., 200.0 for 200x200 world units)
    pub size: f32,
    /// Number of vertices per side (e.g., 64 for 64x64 grid)
    pub resolution: u32,
    /// Maximum terrain height (e.g., 20.0 for 20 units of height variation)
    pub max_height: f32,
    /// Noise scale - smaller values = larger terrain features
    pub noise_scale: f32,
    /// Random seed for reproducible terrain
    pub seed: u32,
}

impl Default for TerrainGenerator {
    fn default() -> Self {
        Self {
            size: 200.0,
            resolution: 64,
            max_height: 20.0,
            noise_scale: 0.01, // 0.01 creates nice rolling hills
            seed: 42,
        }
    }
}

impl TerrainGenerator {
    pub fn new(size: f32, resolution: u32, max_height: f32, noise_scale: f32, seed: u32) -> Self {
        Self { size, resolution, max_height, noise_scale, seed }
    }

    /// Generate height at a specific world coordinate (x, z)
    /// This is the single source of truth for height data
    pub fn sample_height(&self, world_x: f32, world_z: f32) -> f32 {
        // Layer 1: Base terrain with large features
        let base_fbm = Fbm::<Perlin>::new(self.seed)
            .set_frequency(0.01) // Large terrain features
            .set_octaves(4)
            .set_persistence(0.5)
            .set_lacunarity(2.0);

        // Layer 2: Medium details using noise_scale parameter
        let detail_fbm = Fbm::<Perlin>::new(self.seed + 1000)
            .set_frequency(self.noise_scale as f64)
            .set_octaves(3)
            .set_persistence(0.3)
            .set_lacunarity(2.5);

        // Layer 3: Fine surface details
        let surface_fbm = Fbm::<Perlin>::new(self.seed + 2000)
            .set_frequency((self.noise_scale * 4.0) as f64)
            .set_octaves(2)
            .set_persistence(0.2)
            .set_lacunarity(3.0);

        // Sample all layers
        let base_noise = base_fbm.get([world_x as f64, world_z as f64]);
        let detail_noise = detail_fbm.get([world_x as f64, world_z as f64]);
        let surface_noise = surface_fbm.get([world_x as f64, world_z as f64]);

        // Combine layers with different weights
        let combined_noise = base_noise * 0.6 + detail_noise * 0.3 + surface_noise * 0.1;
        
        // Convert noise (-1.0 to 1.0) to height (0.0 to max_height)
        (combined_noise as f32 + 1.0) * 0.5 * self.max_height
    }

    /// Generate biome-influenced height at a specific world coordinate
    /// Uses different noise patterns based on biome characteristics
    pub fn sample_height_with_biome(&self, world_x: f32, world_z: f32) -> (f32, BiomeType) {
        // Determine biome based on noise patterns
        let biome = self.determine_biome(world_x, world_z);
        
        // Get base height from layered noise
        let base_height = self.sample_height(world_x, world_z);
        
        // Apply biome-specific height modifications
        let biome_height = match biome {
            BiomeType::Mountains => {
                // Mountains: Add dramatic height variation
                let mountain_noise = self.generate_mountain_noise(world_x, world_z);
                base_height + mountain_noise * self.max_height * 0.8
            },
            BiomeType::Hills => {
                // Hills: Moderate height variation
                let hill_noise = self.generate_hill_noise(world_x, world_z);
                base_height + hill_noise * self.max_height * 0.4
            },
            BiomeType::Plains => {
                // Plains: Gentle rolling terrain
                base_height * 0.3 + self.max_height * 0.1
            },
            BiomeType::Forest => {
                // Forest: Similar to hills with slightly more variation
                let forest_noise = self.generate_hill_noise(world_x, world_z);
                base_height + forest_noise * self.max_height * 0.5
            },
            BiomeType::Desert => {
                // Desert: Dune-like patterns
                let dune_noise = self.generate_dune_noise(world_x, world_z);
                base_height * 0.4 + dune_noise * self.max_height * 0.3
            },
            BiomeType::Rocky => {
                // Rocky: Similar to hills with more jagged terrain
                let hill_noise = self.generate_hill_noise(world_x, world_z);
                base_height + hill_noise * self.max_height * 0.6
            },
            BiomeType::Wetland => {
                // Wetland: Low-lying areas with minimal elevation
                base_height * 0.2 + self.max_height * 0.05
            },
        };
        
        (biome_height.max(0.0), biome)
    }

    /// Determine biome based on noise patterns for temperature and moisture
    fn determine_biome(&self, world_x: f32, world_z: f32) -> BiomeType {
        // Temperature noise (hot in south, cold in north)
        let temp_noise = Fbm::<Perlin>::new(self.seed + 3000)
            .set_frequency(0.005)
            .set_octaves(2);
        let temperature = temp_noise.get([world_x as f64, world_z as f64]) + (world_z / 200.0) as f64;
        
        // Moisture noise (varies across the map)
        let moisture_noise = Fbm::<Perlin>::new(self.seed + 4000)
            .set_frequency(0.008)
            .set_octaves(3);
        let moisture = moisture_noise.get([world_x as f64, world_z as f64]);
        
        // Classify biome based on temperature and moisture
        match (temperature > 0.0, moisture > 0.0) {
            (true, true) => BiomeType::Forest,   // Hot & Wet
            (true, false) => BiomeType::Desert,  // Hot & Dry
            (false, true) => BiomeType::Hills,   // Cold & Wet
            (false, false) => {                  // Cold & Dry
                if temperature < -0.5 {
                    BiomeType::Mountains         // Very cold = Mountains
                } else {
                    BiomeType::Plains           // Moderately cold = Plains
                }
            }
        }
    }

    /// Generate mountain-specific noise patterns
    fn generate_mountain_noise(&self, world_x: f32, world_z: f32) -> f32 {
        let mountain_fbm = Fbm::<Perlin>::new(self.seed + 5000)
            .set_frequency(0.02)
            .set_octaves(6)
            .set_persistence(0.7)
            .set_lacunarity(2.2);
        
        mountain_fbm.get([world_x as f64, world_z as f64]) as f32
    }

    /// Generate hill-specific noise patterns
    fn generate_hill_noise(&self, world_x: f32, world_z: f32) -> f32 {
        let hill_fbm = Fbm::<Perlin>::new(self.seed + 6000)
            .set_frequency(0.03)
            .set_octaves(4)
            .set_persistence(0.5)
            .set_lacunarity(2.0);
        
        hill_fbm.get([world_x as f64, world_z as f64]) as f32
    }

    /// Generate dune-specific noise patterns
    fn generate_dune_noise(&self, world_x: f32, world_z: f32) -> f32 {
        let dune_fbm = Fbm::<Perlin>::new(self.seed + 7000)
            .set_frequency(0.04)
            .set_octaves(3)
            .set_persistence(0.4)
            .set_lacunarity(2.5);
        
        dune_fbm.get([world_x as f64, world_z as f64]) as f32
    }

    /// Generate 2D height grid with biome awareness for the entire terrain
    /// Returns heights in row-major order (heights[z][x]) and biome map
    pub fn generate_biome_height_grid(&self) -> (Vec<Vec<f32>>, Vec<Vec<BiomeType>>) {
        let res = self.resolution as usize;
        let mut heights = Vec::with_capacity(res);
        let mut biomes = Vec::with_capacity(res);
        let half_size = self.size * 0.5;

        for z in 0..res {
            let mut height_row = Vec::with_capacity(res);
            let mut biome_row = Vec::with_capacity(res);
            for x in 0..res {
                // Convert grid coordinates to world coordinates
                let world_x = (x as f32 / (res - 1) as f32) * self.size - half_size;
                let world_z = (z as f32 / (res - 1) as f32) * self.size - half_size;

                // Sample height and biome using biome-aware generation
                let (height, biome) = self.sample_height_with_biome(world_x, world_z);
                height_row.push(height);
                biome_row.push(biome);
            }
            heights.push(height_row);
            biomes.push(biome_row);
        }

        (heights, biomes)
    }

    /// Generate 2D height grid for the entire terrain
    /// Returns heights in row-major order (heights[z][x])
    pub fn generate_height_grid(&self) -> Vec<Vec<f32>> {
        let res = self.resolution as usize;
        let mut heights = Vec::with_capacity(res);
        let half_size = self.size * 0.5;

        for z in 0..res {
            let mut row = Vec::with_capacity(res);
            for x in 0..res {
                // Convert grid coordinates to world coordinates
                let world_x = (x as f32 / (res - 1) as f32) * self.size - half_size;
                let world_z = (z as f32 / (res - 1) as f32) * self.size - half_size;

                // Sample height using single source of truth
                let height = self.sample_height(world_x, world_z);
                row.push(height);
            }
            heights.push(row);
        }

        heights
    }

    /// Generate Bevy mesh for visual terrain
    pub fn generate_visual_mesh(&self) -> Mesh {
        let heights = self.generate_height_grid();
        let res = self.resolution as usize;
        let half_size = self.size * 0.5;

        // Generate vertices
        let mut positions = Vec::with_capacity(res * res);
        let mut normals = Vec::with_capacity(res * res);
        let mut uvs = Vec::with_capacity(res * res);

        for z in 0..res {
            for x in 0..res {
                // World position
                let world_x = (x as f32 / (res - 1) as f32) * self.size - half_size;
                let world_z = (z as f32 / (res - 1) as f32) * self.size - half_size;
                let world_y = heights[z][x];

                positions.push([world_x, world_y, world_z]);
                normals.push([0.0, 1.0, 0.0]); // Will calculate proper normals later
                uvs.push([x as f32 / (res - 1) as f32, z as f32 / (res - 1) as f32]);
            }
        }

        // Generate triangle indices
        let mut indices = Vec::with_capacity((res - 1) * (res - 1) * 6);
        for z in 0..(res - 1) {
            for x in 0..(res - 1) {
                let top_left = (z * res + x) as u32;
                let top_right = top_left + 1;
                let bottom_left = ((z + 1) * res + x) as u32;
                let bottom_right = bottom_left + 1;

                // Two triangles per quad (counter-clockwise winding)
                indices.extend_from_slice(&[
                    top_left, bottom_left, top_right,
                    top_right, bottom_left, bottom_right,
                ]);
            }
        }

        // Calculate proper normals
        self.calculate_normals(&mut normals, &positions, &indices);

        // Create Bevy mesh
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_indices(Indices::U32(indices));

        mesh
    }

    /// Generate Bevy mesh from pre-computed height grid
    pub fn generate_visual_mesh_from_grid(&self, heights: &[Vec<f32>]) -> Mesh {
        let res = heights.len();
        let half_size = self.size * 0.5;

        // Generate vertices with biome-based coloring
        let mut positions = Vec::with_capacity(res * res);
        let mut normals = Vec::with_capacity(res * res);
        let mut uvs = Vec::with_capacity(res * res);
        let mut colors = Vec::with_capacity(res * res);

        for z in 0..res {
            for x in 0..res {
                // World position
                let world_x = (x as f32 / (res - 1) as f32) * self.size - half_size;
                let world_z = (z as f32 / (res - 1) as f32) * self.size - half_size;
                let world_y = heights[z][x];

                // Sample biome type for this vertex to determine color
                let (_biome_height, biome_type) = self.sample_height_with_biome(world_x, world_z);
                
                // Assign color based on biome type for rich visual diversity
                let color = match biome_type {
                    BiomeType::Forest => [0.2, 0.6, 0.1, 1.0],      // Dark green forest
                    BiomeType::Plains => [0.4, 0.8, 0.2, 1.0],      // Light green plains
                    BiomeType::Hills => [0.5, 0.7, 0.3, 1.0],       // Olive green hills
                    BiomeType::Mountains => [0.6, 0.5, 0.4, 1.0],   // Rocky brown mountains
                    BiomeType::Desert => [0.9, 0.7, 0.4, 1.0],      // Sandy tan desert
                    BiomeType::Rocky => [0.7, 0.6, 0.5, 1.0],       // Gray rocky terrain
                    BiomeType::Wetland => [0.3, 0.5, 0.4, 1.0],     // Dark swamp green
                };

                positions.push([world_x, world_y, world_z]);
                normals.push([0.0, 1.0, 0.0]); // Will calculate proper normals later
                uvs.push([x as f32 / (res - 1) as f32, z as f32 / (res - 1) as f32]);
                colors.push(color);
            }
        }

        // Generate triangle indices
        let mut indices = Vec::with_capacity((res - 1) * (res - 1) * 6);
        for z in 0..(res - 1) {
            for x in 0..(res - 1) {
                let top_left = (z * res + x) as u32;
                let top_right = top_left + 1;
                let bottom_left = ((z + 1) * res + x) as u32;
                let bottom_right = bottom_left + 1;

                // Two triangles per quad (counter-clockwise winding)
                indices.extend_from_slice(&[
                    top_left, bottom_left, top_right,
                    top_right, bottom_left, bottom_right,
                ]);
            }
        }

        // Calculate proper normals
        self.calculate_normals(&mut normals, &positions, &indices);

        // Create Bevy mesh with biome-based vertex colors
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh.insert_indices(Indices::U32(indices));

        mesh
    }

    /// Generate physics heightfield collider from pre-computed height grid
    pub fn generate_physics_collider_from_grid(&self, heights: &[Vec<f32>]) -> Collider {
        // CRITICAL FIX: Transpose matrix for Avian3D heightfield coordinate system
        // Our heights[z][x] (rows=Z, cols=X) → Avian3D expects heights[x][z] (rows=X, cols=Z)
        let res = heights.len();
        let mut transposed_heights = vec![vec![0.0; res]; res];
        
        for z in 0..res {
            for x in 0..res {
                transposed_heights[x][z] = heights[z][x];
            }
        }
        
        // Convert transposed matrix to format expected by Avian3D
        let height_data = transposed_heights.iter().map(|row| row.clone()).collect();
        
        // Create heightfield collider with terrain dimensions
        Collider::heightfield(
            height_data,
            Vec3::new(self.size, 1.0, self.size), // Horizontal scale and unit height scale
        )
    }

    /// Generate physics heightfield collider
    /// Uses the same height data as visual mesh for perfect alignment
    pub fn generate_physics_collider(&self) -> Collider {
        let heights = self.generate_height_grid();
        
        // Use the grid-based method which includes the matrix transpose fix
        self.generate_physics_collider_from_grid(&heights)
    }

    /// Calculate proper vertex normals for smooth lighting
    fn calculate_normals(&self, normals: &mut Vec<[f32; 3]>, positions: &[[f32; 3]], indices: &[u32]) {
        // Reset all normals to zero
        for normal in normals.iter_mut() {
            *normal = [0.0, 0.0, 0.0];
        }

        // Calculate face normals and add to vertex normals
        for triangle in indices.chunks(3) {
            let i0 = triangle[0] as usize;
            let i1 = triangle[1] as usize;
            let i2 = triangle[2] as usize;

            let v0 = Vec3::from(positions[i0]);
            let v1 = Vec3::from(positions[i1]);
            let v2 = Vec3::from(positions[i2]);

            // Calculate face normal
            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let face_normal = edge1.cross(edge2).normalize();

            // Add to vertex normals
            for &i in &[i0, i1, i2] {
                let normal = &mut normals[i];
                normal[0] += face_normal.x;
                normal[1] += face_normal.y;
                normal[2] += face_normal.z;
            }
        }

        // Normalize all vertex normals
        for normal in normals.iter_mut() {
            let n = Vec3::from(*normal).normalize();
            *normal = [n.x, n.y, n.z];
        }
    }
}

/// Terrain height sampler for runtime queries
/// Provides consistent height sampling across the game
#[derive(Resource)]
pub struct TerrainHeightSampler {
    pub generator: TerrainGenerator,
}

impl TerrainHeightSampler {
    pub fn new(generator: TerrainGenerator) -> Self {
        Self { generator }
    }

    /// Sample terrain height at world coordinates using grid-based interpolation
    /// This matches exactly what the physics heightfield collider uses
    pub fn sample_height(&self, world_x: f32, world_z: f32) -> f32 {
        // Generate the height grid (same as physics collider)
        let (height_grid, _biome_grid) = self.generator.generate_biome_height_grid();
        
        // Use bilinear interpolation to sample from the grid
        self.sample_height_from_grid(&height_grid, world_x, world_z)
    }

    /// Sample terrain height and biome at world coordinates using direct generation
    pub fn sample_height_and_biome(&self, world_x: f32, world_z: f32) -> (f32, BiomeType) {
        self.generator.sample_height_with_biome(world_x, world_z)
    }

    /// Sample height from a pre-computed grid using bilinear interpolation
    /// This exactly matches what Avian3D heightfield collision uses
    fn sample_height_from_grid(&self, height_grid: &[Vec<f32>], world_x: f32, world_z: f32) -> f32 {
        let half_size = self.generator.size * 0.5;
        let res = self.generator.resolution as f32;
        
        // Convert world coordinates to grid coordinates (floating point)
        let grid_x_f = (world_x + half_size) / self.generator.size * (res - 1.0);
        let grid_z_f = (world_z + half_size) / self.generator.size * (res - 1.0);
        
        // Clamp to grid bounds
        let grid_x_f = grid_x_f.clamp(0.0, res - 1.0);
        let grid_z_f = grid_z_f.clamp(0.0, res - 1.0);
        
        // Get integer grid indices
        let x0 = (grid_x_f.floor() as usize).min(self.generator.resolution as usize - 1);
        let z0 = (grid_z_f.floor() as usize).min(self.generator.resolution as usize - 1);
        let x1 = (x0 + 1).min(self.generator.resolution as usize - 1);
        let z1 = (z0 + 1).min(self.generator.resolution as usize - 1);
        
        // Get fractional parts for interpolation
        let fx = grid_x_f - x0 as f32;
        let fz = grid_z_f - z0 as f32;
        
        // Sample the four corners
        let h00 = height_grid[z0][x0];
        let h10 = height_grid[z0][x1];
        let h01 = height_grid[z1][x0];
        let h11 = height_grid[z1][x1];
        
        // Bilinear interpolation
        let h0 = h00 * (1.0 - fx) + h10 * fx;
        let h1 = h01 * (1.0 - fx) + h11 * fx;
        h0 * (1.0 - fz) + h1 * fz
    }
}