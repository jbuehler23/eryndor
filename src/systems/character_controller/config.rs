use bevy::prelude::*;

/// Comprehensive configuration for modular character controller
/// Designed for easy tuning and different game types
#[derive(Resource, Clone, Debug)]
pub struct CharacterControllerConfig {
    pub ground: GroundMovementConfig,
    pub air: AirMovementConfig,
    pub slopes: SlopeConfig,
    pub step_up: StepUpConfig,
    pub collision: CollisionConfig,
}

impl Default for CharacterControllerConfig {
    fn default() -> Self {
        Self {
            ground: GroundMovementConfig::default(),
            air: AirMovementConfig::default(),
            slopes: SlopeConfig::default(),
            step_up: StepUpConfig::default(),
            collision: CollisionConfig::default(),
        }
    }
}

/// Ground movement configuration
#[derive(Clone, Debug)]
pub struct GroundMovementConfig {
    pub walk_speed: f32,
    pub run_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub turn_speed: f32,
}

impl Default for GroundMovementConfig {
    fn default() -> Self {
        Self {
            walk_speed: 3.0,
            run_speed: 6.0,
            acceleration: 30.0,
            deceleration: 40.0,
            turn_speed: 25.0,
        }
    }
}

/// Air movement configuration
#[derive(Clone, Debug)]
pub struct AirMovementConfig {
    pub air_control: f32,
    pub gravity_scale: f32,
    pub jump_height: f32,
    pub fall_speed_limit: f32,
}

impl Default for AirMovementConfig {
    fn default() -> Self {
        Self {
            air_control: 0.3,
            gravity_scale: 1.0,
            jump_height: 1.5,
            fall_speed_limit: 15.0,
        }
    }
}

/// Slope traversal configuration
#[derive(Clone, Debug)]
pub struct SlopeConfig {
    /// Maximum walkable slope angle in radians (π/4 = 45°)
    pub max_walkable_angle: f32,
    /// Angle at which character starts sliding (π/3 = 60°)
    pub slide_threshold_angle: f32,
    /// Friction when sliding on slopes (0.0 = ice, 1.0 = sandpaper)
    pub slide_friction: f32,
    /// Speed multiplier when going uphill
    pub uphill_speed_multiplier: f32,
    /// Speed multiplier when going downhill  
    pub downhill_speed_multiplier: f32,
}

impl Default for SlopeConfig {
    fn default() -> Self {
        Self {
            max_walkable_angle: std::f32::consts::PI / 4.0, // 45°
            slide_threshold_angle: std::f32::consts::PI / 3.0, // 60°
            slide_friction: 0.1, // Moderate sliding
            uphill_speed_multiplier: 0.8,
            downhill_speed_multiplier: 1.2,
        }
    }
}

/// Step-up mechanics configuration
#[derive(Clone, Debug)]
pub struct StepUpConfig {
    /// Maximum height character can step up automatically
    pub max_step_height: f32,
    /// Minimum horizontal width to be considered a valid step
    pub min_step_width: f32,
    /// Distance to cast ahead when checking for steps
    pub step_check_distance: f32,
    /// Whether step-up is enabled
    pub enabled: bool,
}

impl Default for StepUpConfig {
    fn default() -> Self {
        Self {
            max_step_height: 0.3, // 30cm step-up
            min_step_width: 0.1,
            step_check_distance: 0.6,
            enabled: true,
        }
    }
}

/// Collision detection configuration
#[derive(Clone, Debug)]
pub struct CollisionConfig {
    /// Collision margin to prevent getting stuck in geometry
    pub collision_margin: f32,
    /// Maximum number of collision resolution iterations
    pub max_collision_iterations: u32,
    /// Distance tolerance for considering surfaces "flat"
    pub surface_tolerance: f32,
    /// Character capsule radius
    pub capsule_radius: f32,
    /// Character capsule height
    pub capsule_height: f32,
}

impl Default for CollisionConfig {
    fn default() -> Self {
        Self {
            collision_margin: 0.05, // 5cm safety margin
            max_collision_iterations: 4,
            surface_tolerance: 0.01,
            capsule_radius: 0.4,
            capsule_height: 1.8,
        }
    }
}

/// Character controller capabilities for different game types
impl CharacterControllerConfig {
    /// Configuration optimized for MMO-style gameplay
    pub fn mmo_optimized() -> Self {
        Self {
            ground: GroundMovementConfig {
                walk_speed: 3.5,
                run_speed: 7.0,
                acceleration: 35.0,
                deceleration: 45.0,
                turn_speed: 30.0,
            },
            slopes: SlopeConfig {
                max_walkable_angle: std::f32::consts::PI / 4.5, // Slightly steeper: ~40°
                uphill_speed_multiplier: 0.9, // Less penalty for better MMO feel
                downhill_speed_multiplier: 1.1, // Less boost for stability
                ..Default::default()
            },
            step_up: StepUpConfig {
                max_step_height: 0.4, // Higher step-up for varied terrain
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Configuration for platformer-style gameplay
    pub fn platformer() -> Self {
        Self {
            air: AirMovementConfig {
                air_control: 0.8, // High air control
                jump_height: 2.0, // Higher jumps
                ..Default::default()
            },
            slopes: SlopeConfig {
                max_walkable_angle: std::f32::consts::PI / 3.0, // Steeper: 60°
                slide_friction: 0.3, // More sliding for platformer feel
                ..Default::default()
            },
            step_up: StepUpConfig {
                max_step_height: 0.5, // High step-up for platforms
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Configuration for realistic/simulation gameplay
    pub fn realistic() -> Self {
        Self {
            ground: GroundMovementConfig {
                walk_speed: 1.5,
                run_speed: 4.0,
                acceleration: 15.0,
                deceleration: 20.0,
                turn_speed: 10.0,
            },
            slopes: SlopeConfig {
                max_walkable_angle: std::f32::consts::PI / 6.0, // Conservative: 30°
                slide_friction: 0.05, // More realistic sliding
                uphill_speed_multiplier: 0.5,
                downhill_speed_multiplier: 1.1,
                ..Default::default()
            },
            step_up: StepUpConfig {
                max_step_height: 0.2, // Realistic step height
                ..Default::default()
            },
            ..Default::default()
        }
    }
}