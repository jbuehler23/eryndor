use bevy::prelude::*;
use avian3d::prelude::*;
use super::{config::*, core::*, collision::*};
use crate::resources::{GameDebugConfig, DebugTimer};

/// Debug visualization configuration
#[derive(Resource, Clone, Debug)]
pub struct CharacterControllerDebugConfig {
    pub enabled: bool,
    pub show_velocity_vectors: bool,
    pub show_collision_normals: bool,
    pub show_ground_detection: bool,
    pub show_step_up_checks: bool,
    pub show_slope_analysis: bool,
    pub vector_scale: f32,
    pub line_width: f32,
}

impl Default for CharacterControllerDebugConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            show_velocity_vectors: true,
            show_collision_normals: true,
            show_ground_detection: true,
            show_step_up_checks: false,
            show_slope_analysis: true,
            vector_scale: 2.0,
            line_width: 0.02,
        }
    }
}

/// Debug visualization system for character controller
pub fn debug_character_controller_visualization(
    debug_config: Res<CharacterControllerDebugConfig>,
    config: Res<CharacterControllerConfig>,
    mut gizmos: Gizmos,
    player_query: Query<
        (&Transform, &CharacterControllerState),
        With<crate::components::Player>
    >,
    spatial_query: SpatialQuery,
) {
    if !debug_config.enabled {
        return;
    }

    for (transform, controller_state) in player_query.iter() {
        let position = transform.translation;

        // Draw velocity vector
        if debug_config.show_velocity_vectors && controller_state.velocity.length() > 0.1 {
            let velocity_end = position + controller_state.velocity * debug_config.vector_scale;
            gizmos.line(position, velocity_end, Color::srgb(0.0, 1.0, 0.0)); // Green for velocity
        }

        // Draw ground detection
        if debug_config.show_ground_detection {
            draw_ground_detection(&mut gizmos, position, controller_state, &debug_config);
        }

        // Draw collision normals
        if debug_config.show_collision_normals {
            draw_collision_normals(&mut gizmos, position, &spatial_query, &config, &debug_config);
        }

        // Draw slope analysis
        if debug_config.show_slope_analysis {
            draw_slope_analysis(&mut gizmos, position, controller_state, &config, &debug_config);
        }
    }
}

/// Draw ground detection raycast
fn draw_ground_detection(
    gizmos: &mut Gizmos,
    position: Vec3,
    controller_state: &CharacterControllerState,
    debug_config: &CharacterControllerDebugConfig,
) {
    let ground_check_distance = 2.0;
    let ray_end = position + Vec3::NEG_Y * ground_check_distance;
    
    // Color based on grounded state
    let ray_color = if controller_state.is_grounded {
        Color::srgb(0.0, 1.0, 0.0) // Green if grounded
    } else {
        Color::srgb(1.0, 0.0, 0.0) // Red if not grounded
    };
    
    gizmos.line(position, ray_end, ray_color);
    
    // Draw ground normal if grounded
    if controller_state.is_grounded {
        let normal_end = position + controller_state.ground_normal * debug_config.vector_scale;
        gizmos.line(position, normal_end, Color::srgb(0.0, 0.0, 1.0)); // Blue for normal
    }
}

/// Draw collision normals from nearby surfaces
fn draw_collision_normals(
    gizmos: &mut Gizmos,
    position: Vec3,
    spatial_query: &SpatialQuery,
    config: &CharacterControllerConfig,
    debug_config: &CharacterControllerDebugConfig,
) {
    // Cast rays in multiple directions to find nearby surfaces
    let directions = [
        Vec3::X, Vec3::NEG_X,
        Vec3::Z, Vec3::NEG_Z,
        Vec3::new(1.0, 0.0, 1.0).normalize(),
        Vec3::new(-1.0, 0.0, 1.0).normalize(),
        Vec3::new(1.0, 0.0, -1.0).normalize(),
        Vec3::new(-1.0, 0.0, -1.0).normalize(),
    ];

    for direction in directions.iter() {
        if let Ok(ray_dir) = Dir3::new(*direction) {
            if let Some(hit) = spatial_query.cast_ray(
                position,
                ray_dir,
                1.5, // Check within 1.5 units
                true,
                &SpatialQueryFilter::default(),
            ) {
                // Color code based on surface type
                let surface_type = CollisionSystem::classify_surface(&hit.normal, config);
                let normal_color = match surface_type {
                    SurfaceType::Walkable => Color::srgb(0.0, 1.0, 0.0), // Green for walkable
                    SurfaceType::Slideable => Color::srgb(1.0, 1.0, 0.0), // Yellow for slideable
                    SurfaceType::Wall => Color::srgb(1.0, 0.5, 0.0), // Orange for walls
                    SurfaceType::Ceiling => Color::srgb(1.0, 0.0, 1.0), // Magenta for ceiling
                };

                let hit_point = position + *direction * hit.distance;
                let normal_end = hit_point + hit.normal * debug_config.vector_scale * 0.5;
                gizmos.line(hit_point, normal_end, normal_color);
                
                // Draw small sphere at hit point
                gizmos.sphere(hit_point, 0.05, normal_color);
            }
        }
    }
}

/// Draw slope analysis visualization
fn draw_slope_analysis(
    gizmos: &mut Gizmos,
    position: Vec3,
    controller_state: &CharacterControllerState,
    config: &CharacterControllerConfig,
    debug_config: &CharacterControllerDebugConfig,
) {
    if !controller_state.is_grounded {
        return;
    }

    let ground_normal = controller_state.ground_normal;
    let slope_angle = ground_normal.dot(Vec3::Y).acos();

    // Draw slope angle visualization
    let slope_color = if slope_angle <= config.slopes.max_walkable_angle {
        Color::srgb(0.0, 1.0, 0.0) // Green for walkable
    } else if slope_angle <= config.slopes.slide_threshold_angle {
        Color::srgb(1.0, 1.0, 0.0) // Yellow for slideable
    } else {
        Color::srgb(1.0, 0.0, 0.0) // Red for too steep
    };

    // Draw slope plane indication
    let slope_right = ground_normal.cross(Vec3::Y).normalize();
    if slope_right.length() > 0.1 {
        let slope_forward = ground_normal.cross(slope_right).normalize();
        
        let plane_size = 1.0;
        let corners = [
            position + slope_right * plane_size + slope_forward * plane_size,
            position - slope_right * plane_size + slope_forward * plane_size,
            position - slope_right * plane_size - slope_forward * plane_size,
            position + slope_right * plane_size - slope_forward * plane_size,
        ];

        // Draw slope plane outline
        for i in 0..4 {
            let next_i = (i + 1) % 4;
            gizmos.line(corners[i], corners[next_i], slope_color);
        }
    }

    // Draw slope angle arc
    draw_angle_arc(gizmos, position, Vec3::Y, ground_normal, slope_angle, slope_color);
}

/// Draw an arc to visualize angles
fn draw_angle_arc(
    gizmos: &mut Gizmos,
    center: Vec3,
    from_vector: Vec3,
    to_vector: Vec3,
    angle: f32,
    color: Color,
) {
    let arc_radius = 0.5;
    let arc_segments = 10;
    
    let axis = from_vector.cross(to_vector).normalize();
    if axis.length() < 0.1 {
        return; // Vectors are parallel
    }

    let mut previous_point = center + from_vector * arc_radius;
    
    for i in 1..=arc_segments {
        let t = i as f32 / arc_segments as f32;
        let current_angle = angle * t;
        
        let rotation = Quat::from_axis_angle(axis, current_angle);
        let current_point = center + rotation.mul_vec3(from_vector) * arc_radius;
        
        gizmos.line(previous_point, current_point, color);
        previous_point = current_point;
    }
}

/// Debug information display system
pub fn debug_character_controller_info(
    debug_config: Res<CharacterControllerDebugConfig>,
    config: Res<CharacterControllerConfig>,
    player_query: Query<
        (&Transform, &CharacterControllerState),
        With<crate::components::Player>
    >,
) {
    if !debug_config.enabled {
        return;
    }

    for (transform, controller_state) in player_query.iter() {
        // Print debug information to console
        let slope_angle = controller_state.ground_normal.dot(Vec3::Y).acos() * 180.0 / std::f32::consts::PI;
        let velocity_magnitude = controller_state.velocity.length();

        println!("=== CHARACTER CONTROLLER DEBUG ===");
        println!("Position: {:.2}, {:.2}, {:.2}", 
                 transform.translation.x, transform.translation.y, transform.translation.z);
        println!("Velocity: {:.2} units/sec", velocity_magnitude);
        println!("Movement State: {:?}", controller_state.movement_state);
        println!("Grounded: {}", controller_state.is_grounded);
        println!("Slope Angle: {:.1}°", slope_angle);
        println!("Ground Normal: {:.2}, {:.2}, {:.2}", 
                 controller_state.ground_normal.x, 
                 controller_state.ground_normal.y, 
                 controller_state.ground_normal.z);
        
        if controller_state.is_grounded {
            let surface_type = CollisionSystem::classify_surface(&controller_state.ground_normal, &config);
            println!("Surface Type: {:?}", surface_type);
            
            let is_walkable = CollisionSystem::is_surface_walkable(&controller_state.ground_normal, &config);
            println!("Walkable: {}", is_walkable);
        }
        
        println!("===================================\n");
    }
}

/// Toggle debug visualization with key input
pub fn toggle_character_controller_debug(
    keys: Res<ButtonInput<KeyCode>>,
    mut debug_config: ResMut<CharacterControllerDebugConfig>,
) {
    if keys.just_pressed(KeyCode::F4) {
        debug_config.enabled = !debug_config.enabled;
        info!("Character Controller Debug {}", 
              if debug_config.enabled { "ENABLED" } else { "DISABLED" });
    }
    
    if keys.just_pressed(KeyCode::F5) {
        debug_config.show_velocity_vectors = !debug_config.show_velocity_vectors;
        info!("Velocity Vectors {}", 
              if debug_config.show_velocity_vectors { "ON" } else { "OFF" });
    }
    
    if keys.just_pressed(KeyCode::F6) {
        debug_config.show_collision_normals = !debug_config.show_collision_normals;
        info!("Collision Normals {}", 
              if debug_config.show_collision_normals { "ON" } else { "OFF" });
    }
    
    if keys.just_pressed(KeyCode::F7) {
        debug_config.show_slope_analysis = !debug_config.show_slope_analysis;
        info!("Slope Analysis {}", 
              if debug_config.show_slope_analysis { "ON" } else { "OFF" });
    }
}

/// Performance monitoring for character controller
#[derive(Resource, Default)]
pub struct CharacterControllerPerformanceMetrics {
    pub collision_queries_per_frame: u32,
    pub average_collision_time: f32,
    pub step_up_attempts_per_second: f32,
}

/// System to collect performance metrics
pub fn collect_performance_metrics(
    mut metrics: ResMut<CharacterControllerPerformanceMetrics>,
    time: Res<Time>,
    player_query: Query<&CharacterControllerState, With<crate::components::Player>>,
) {
    // Reset frame counters
    metrics.collision_queries_per_frame = 0;
    
    for controller_state in player_query.iter() {
        // Count active collision queries (simplified)
        if controller_state.velocity.length() > 0.1 {
            metrics.collision_queries_per_frame += 1;
        }
        
        if controller_state.movement_state == MovementState::SteppingUp {
            metrics.step_up_attempts_per_second += 1.0 * time.delta_secs();
        }
    }
}

/// Debug UI overlay (optional, requires UI framework)
#[cfg(feature = "debug_ui")]
pub fn debug_ui_overlay(
    debug_config: Res<CharacterControllerDebugConfig>,
    metrics: Res<CharacterControllerPerformanceMetrics>,
    mut contexts: bevy_egui::EguiContexts,
    player_query: Query<
        (&Transform, &CharacterControllerState),
        With<crate::components::Player>
    >,
) {
    if !debug_config.enabled {
        return;
    }

    bevy_egui::egui::Window::new("Character Controller Debug")
        .show(contexts.ctx_mut(), |ui| {
            for (transform, controller_state) in player_query.iter() {
                ui.label(format!("Position: {:.2?}", transform.translation));
                ui.label(format!("Velocity: {:.2}", controller_state.velocity.length()));
                ui.label(format!("State: {:?}", controller_state.movement_state));
                ui.label(format!("Grounded: {}", controller_state.is_grounded));
                
                let slope_angle = controller_state.ground_normal.dot(Vec3::Y).acos() * 180.0 / std::f32::consts::PI;
                ui.label(format!("Slope: {:.1}°", slope_angle));
                
                ui.separator();
                ui.label(format!("Collision Queries/Frame: {}", metrics.collision_queries_per_frame));
                ui.label(format!("Step-up Attempts/s: {:.1}", metrics.step_up_attempts_per_second));
            }
        });
}