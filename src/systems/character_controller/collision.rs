use bevy::prelude::*;
use avian3d::prelude::*;
use super::config::*;

/// Result of a collision detection query
#[derive(Debug, Clone)]
pub struct CollisionResult {
    pub hit: bool,
    pub normal: Vec3,
    pub distance: f32,
    pub point: Vec3,
    pub slide_vector: Vec3,
    pub is_walkable: bool,
}

impl Default for CollisionResult {
    fn default() -> Self {
        Self {
            hit: false,
            normal: Vec3::Y,
            distance: 0.0,
            point: Vec3::ZERO,
            slide_vector: Vec3::ZERO,
            is_walkable: true,
        }
    }
}

/// Surface classification based on collision normal
#[derive(Debug, Clone, PartialEq)]
pub enum SurfaceType {
    Walkable,
    Slideable,
    Wall,
    Ceiling,
}

/// Enhanced collision detection with surface analysis
pub struct CollisionSystem;

impl CollisionSystem {
    /// Enhanced collide-and-slide with proper surface normal handling
    pub fn collide_and_slide(
        current_pos: Vec3,
        desired_movement: Vec3,
        spatial_query: &SpatialQuery,
        config: &CharacterControllerConfig,
        excluded_entities: &[Entity],
    ) -> (Vec3, Vec3, CollisionResult) {
        let movement_length = desired_movement.length();
        if movement_length < 0.001 {
            return (current_pos, Vec3::ZERO, CollisionResult::default());
        }

        let mut position = current_pos;
        let mut remaining_movement = desired_movement;
        let mut final_velocity = Vec3::ZERO;
        let mut collision_result = CollisionResult::default();

        // Multi-step collision resolution
        for iteration in 0..config.collision.max_collision_iterations {
            let step_length = remaining_movement.length();
            if step_length < 0.001 {
                break; // No more movement needed
            }

            let movement_direction = remaining_movement / step_length;
            
            // Perform shape cast for this movement step
            if let Some(hit_result) = Self::cast_character_shape(
                position,
                movement_direction,
                step_length,
                spatial_query,
                config,
                excluded_entities,
            ) {
                // Collision detected
                collision_result = hit_result.clone();
                
                // Move to just before collision point
                let safe_distance = (hit_result.distance - config.collision.collision_margin).max(0.0);
                position += movement_direction * safe_distance;
                
                // Calculate remaining movement after collision
                let consumed_distance = safe_distance;
                let remaining_distance = step_length - consumed_distance;
                
                if remaining_distance <= 0.001 {
                    break; // No significant movement left
                }
                
                // Determine how to handle the collision based on surface type
                match Self::classify_surface(&hit_result.normal, config) {
                    SurfaceType::Walkable => {
                        // Project remaining movement along the walkable surface
                        let projected_movement = Self::project_on_plane(
                            movement_direction * remaining_distance,
                            hit_result.normal,
                        );
                        remaining_movement = projected_movement;
                        final_velocity += projected_movement;
                    },
                    SurfaceType::Slideable => {
                        // Calculate sliding movement with friction
                        let slide_direction = Self::calculate_slide_direction(
                            hit_result.normal,
                            config,
                        );
                        let slide_speed = remaining_distance * config.slopes.slide_friction;
                        remaining_movement = slide_direction * slide_speed;
                        final_velocity += remaining_movement;
                    },
                    SurfaceType::Wall | SurfaceType::Ceiling => {
                        // Project movement along the surface (slide along wall)
                        let slide_movement = Self::project_on_plane(
                            movement_direction * remaining_distance,
                            hit_result.normal,
                        );
                        remaining_movement = slide_movement;
                        final_velocity += slide_movement;
                    },
                }
                
                // Reduce remaining movement to prevent infinite loops
                remaining_movement *= 0.95;
                
            } else {
                // No collision - complete the movement
                position += remaining_movement;
                final_velocity += remaining_movement;
                break;
            }
        }

        (position, final_velocity, collision_result)
    }

    /// Cast character's collision shape along movement direction
    fn cast_character_shape(
        position: Vec3,
        direction: Vec3,
        distance: f32,
        spatial_query: &SpatialQuery,
        config: &CharacterControllerConfig,
        excluded_entities: &[Entity],
    ) -> Option<CollisionResult> {
        // Create character capsule collider
        let capsule = Collider::capsule(
            config.collision.capsule_radius,
            config.collision.capsule_height,
        );

        // Cast from character center (capsule center height)
        let cast_origin = position + Vec3::new(0.0, config.collision.capsule_height * 0.5, 0.0);
        
        let shape_direction = Dir3::new(direction).unwrap_or(Dir3::NEG_Y);
        let max_distance = distance + config.collision.collision_margin;

        let shape_cast_config = ShapeCastConfig {
            max_distance,
            ..default()
        };

        let filter = SpatialQueryFilter::default()
            .with_excluded_entities(excluded_entities.to_vec());

        if let Some(hit) = spatial_query.cast_shape(
            &capsule,
            cast_origin,
            Quat::IDENTITY,
            shape_direction,
            &shape_cast_config,
            &filter,
        ) {
            let normal = hit.normal1;
            let slide_vector = Self::project_on_plane(direction, normal);
            let is_walkable = Self::is_surface_walkable(&normal, config);

            Some(CollisionResult {
                hit: true,
                normal,
                distance: hit.distance,
                point: cast_origin + direction * hit.distance,
                slide_vector,
                is_walkable,
            })
        } else {
            None
        }
    }

    /// Classify surface type based on normal vector
    pub fn classify_surface(normal: &Vec3, config: &CharacterControllerConfig) -> SurfaceType {
        let angle_with_up = normal.dot(Vec3::Y).acos();
        
        if angle_with_up <= config.slopes.max_walkable_angle {
            SurfaceType::Walkable
        } else if angle_with_up <= config.slopes.slide_threshold_angle {
            SurfaceType::Slideable  
        } else if angle_with_up < std::f32::consts::PI * 0.75 {
            SurfaceType::Wall
        } else {
            SurfaceType::Ceiling
        }
    }

    /// Check if a surface is walkable based on slope angle
    pub fn is_surface_walkable(normal: &Vec3, config: &CharacterControllerConfig) -> bool {
        let angle_with_up = normal.dot(Vec3::Y).acos();
        angle_with_up <= config.slopes.max_walkable_angle
    }

    /// Project a vector onto a plane defined by its normal
    pub fn project_on_plane(vector: Vec3, plane_normal: Vec3) -> Vec3 {
        vector - plane_normal * vector.dot(plane_normal)
    }

    /// Calculate sliding direction on a slope
    fn calculate_slide_direction(normal: Vec3, config: &CharacterControllerConfig) -> Vec3 {
        // Find the steepest downward direction on the slope
        let horizontal_normal = Vec3::new(normal.x, 0.0, normal.z).normalize();
        let slide_direction = Vec3::new(-horizontal_normal.x, 0.0, -horizontal_normal.z);
        
        // Project onto the slope surface
        Self::project_on_plane(slide_direction, normal).normalize()
    }

    /// Ground detection using downward raycast
    pub fn is_grounded(
        position: Vec3,
        spatial_query: &SpatialQuery,
        config: &CharacterControllerConfig,
        excluded_entities: &[Entity],
    ) -> (bool, Option<CollisionResult>) {
        let ray_origin = position;
        let ray_direction = Dir3::NEG_Y;
        let max_distance = config.collision.capsule_height * 0.5 + 0.1; // Just below character

        let filter = SpatialQueryFilter::default()
            .with_excluded_entities(excluded_entities.to_vec());

        if let Some(hit) = spatial_query.cast_ray(
            ray_origin,
            ray_direction,
            max_distance,
            true,
            &filter,
        ) {
            let collision_result = CollisionResult {
                hit: true,
                normal: hit.normal,
                distance: hit.distance,
                point: ray_origin + ray_direction.as_vec3() * hit.distance,
                slide_vector: Vec3::ZERO,
                is_walkable: Self::is_surface_walkable(&hit.normal, config),
            };

            // Consider grounded if we hit something close enough and it's walkable
            let is_grounded = hit.distance <= config.collision.capsule_height * 0.5 + 0.05
                && collision_result.is_walkable;

            (is_grounded, Some(collision_result))
        } else {
            (false, None)
        }
    }

    /// Step-up detection and execution
    pub fn attempt_step_up(
        position: Vec3,
        movement_direction: Vec3,
        spatial_query: &SpatialQuery,
        config: &CharacterControllerConfig,
        excluded_entities: &[Entity],
    ) -> Option<Vec3> {
        if !config.step_up.enabled {
            return None;
        }

        let step_check_distance = config.step_up.step_check_distance;
        let max_step_height = config.step_up.max_step_height;
        let min_step_width = config.step_up.min_step_width;

        // 1. Cast forward to detect obstacle
        let forward_cast_origin = position;
        let forward_direction = Dir3::new(movement_direction).ok()?;
        
        let filter = SpatialQueryFilter::default()
            .with_excluded_entities(excluded_entities.to_vec());

        let forward_hit = spatial_query.cast_ray(
            forward_cast_origin,
            forward_direction,
            step_check_distance,
            true,
            &filter,
        )?;

        // 2. Cast upward to find step height
        let up_cast_origin = forward_cast_origin + forward_direction.as_vec3() * forward_hit.distance;
        let up_direction = Dir3::Y;
        
        let up_hit = spatial_query.cast_ray(
            up_cast_origin,
            up_direction,
            max_step_height,
            true,
            &filter,
        );

        let step_height = up_hit.map_or(max_step_height, |hit| hit.distance);
        
        if step_height > max_step_height {
            return None; // Step too high
        }

        // 3. Cast forward again at step height to check for clearance
        let elevated_origin = position + Vec3::Y * (step_height + 0.1);
        let clearance_hit = spatial_query.cast_ray(
            elevated_origin,
            forward_direction,
            step_check_distance,
            true,
            &filter,
        );

        if let Some(hit) = clearance_hit {
            if hit.distance < min_step_width {
                return None; // Not enough clearance
            }
        }

        // 4. Cast downward to find landing surface
        let landing_cast_origin = elevated_origin + movement_direction * step_check_distance;
        let down_direction = Dir3::NEG_Y;
        
        if let Some(landing_hit) = spatial_query.cast_ray(
            landing_cast_origin,
            down_direction,
            step_height + 0.2,
            true,
            &filter,
        ) {
            if Self::is_surface_walkable(&landing_hit.normal, config) {
                // Valid step-up location found
                let landing_point = up_cast_origin + Dir3::NEG_Y.as_vec3() * landing_hit.distance;
                return Some(landing_point + Vec3::Y * 0.1); // Slight elevation for safety
            }
        }

        None
    }
}

/// Utility functions for collision math
pub mod collision_utils {
    use super::*;

    /// Calculate angle between two vectors in radians
    pub fn angle_between(v1: Vec3, v2: Vec3) -> f32 {
        v1.dot(v2).clamp(-1.0, 1.0).acos()
    }

    /// Check if two normals represent similar surfaces
    pub fn normals_similar(n1: Vec3, n2: Vec3, tolerance: f32) -> bool {
        n1.dot(n2) > (1.0 - tolerance)
    }

    /// Smooth normal interpolation for better visual results
    pub fn smooth_normal_transition(
        from_normal: Vec3,
        to_normal: Vec3,
        transition_factor: f32,
    ) -> Vec3 {
        from_normal.lerp(to_normal, transition_factor).normalize()
    }
}