use bevy::prelude::*;
use super::{config::*, core::*};

/// Movement behavior trait for extensible character controllers
pub trait MovementBehavior: Send + Sync {
    fn apply(&self, input: &MovementInput, state: &mut CharacterControllerState, dt: f32) -> Vec3;
}

/// Input data for movement calculations
#[derive(Debug, Clone)]
pub struct MovementInput {
    pub direction: Vec3,
    pub speed: f32,
    pub is_running: bool,
    pub jump_pressed: bool,
    pub ground_normal: Vec3,
}

/// Standard ground movement behavior
pub struct GroundMovementBehavior {
    pub config: GroundMovementConfig,
}

impl MovementBehavior for GroundMovementBehavior {
    fn apply(&self, input: &MovementInput, state: &mut CharacterControllerState, dt: f32) -> Vec3 {
        if input.direction.length() < 0.1 {
            // Deceleration
            let current_speed = state.velocity.length();
            let decel_amount = self.config.deceleration * dt;
            let new_speed = (current_speed - decel_amount).max(0.0);
            
            if new_speed > 0.001 {
                state.velocity = state.velocity.normalize() * new_speed;
            } else {
                state.velocity = Vec3::ZERO;
            }
        } else {
            // Acceleration towards target
            let target_speed = if input.is_running {
                self.config.run_speed
            } else {
                self.config.walk_speed
            };

            let target_velocity = input.direction.normalize() * target_speed;
            let velocity_diff = target_velocity - state.velocity;
            let accel_amount = self.config.acceleration * dt;
            
            if velocity_diff.length() > accel_amount {
                let accel_direction = velocity_diff.normalize();
                state.velocity += accel_direction * accel_amount;
            } else {
                state.velocity = target_velocity;
            }
        }

        state.velocity * dt
    }
}

/// Air movement behavior with reduced control
pub struct AirMovementBehavior {
    pub config: AirMovementConfig,
}

impl MovementBehavior for AirMovementBehavior {
    fn apply(&self, input: &MovementInput, state: &mut CharacterControllerState, dt: f32) -> Vec3 {
        // Limited air control
        if input.direction.length() > 0.1 {
            let air_influence = input.direction.normalize() * input.speed * self.config.air_control;
            let air_acceleration = air_influence * dt;
            
            // Limit air acceleration to prevent infinite speed gain
            let max_air_accel = 10.0 * dt;
            if air_acceleration.length() > max_air_accel {
                state.velocity += air_acceleration.normalize() * max_air_accel;
            } else {
                state.velocity += air_acceleration;
            }
        }

        // Apply gravity
        state.vertical_velocity += -9.81 * self.config.gravity_scale * dt;
        state.vertical_velocity = state.vertical_velocity.max(-self.config.fall_speed_limit);

        // Combine horizontal and vertical movement
        let horizontal_movement = Vec3::new(state.velocity.x, 0.0, state.velocity.z) * dt;
        let vertical_movement = Vec3::Y * state.vertical_velocity * dt;

        horizontal_movement + vertical_movement
    }
}

/// Surface sliding behavior for steep slopes
pub struct SlidingBehavior {
    pub config: SlopeConfig,
}

impl MovementBehavior for SlidingBehavior {
    fn apply(&self, input: &MovementInput, state: &mut CharacterControllerState, dt: f32) -> Vec3 {
        // Calculate sliding direction (down the slope)
        let slide_direction = calculate_slide_direction(input.ground_normal);
        
        // Apply friction to reduce sliding speed
        let friction_force = self.config.slide_friction;
        let slide_acceleration = 9.81 * (1.0 - friction_force); // Gravity minus friction
        
        // Update sliding velocity
        let slide_velocity = slide_direction * slide_acceleration * dt;
        state.velocity = state.velocity * (1.0 - friction_force * dt) + slide_velocity;
        
        // Limit sliding speed
        let max_slide_speed = 8.0;
        if state.velocity.length() > max_slide_speed {
            state.velocity = state.velocity.normalize() * max_slide_speed;
        }

        state.velocity * dt
    }
}

/// Calculate the direction of sliding on a slope
fn calculate_slide_direction(surface_normal: Vec3) -> Vec3 {
    // Find the steepest descent direction
    let horizontal_normal = Vec3::new(surface_normal.x, 0.0, surface_normal.z);
    
    if horizontal_normal.length() < 0.001 {
        return Vec3::ZERO; // Flat surface, no sliding
    }

    let horizontal_normal = horizontal_normal.normalize();
    
    // The slide direction is perpendicular to the contour lines (steepest descent)
    Vec3::new(-horizontal_normal.x, 0.0, -horizontal_normal.z)
}

/// Movement state machine for handling different movement modes
pub struct MovementStateMachine {
    pub ground_behavior: GroundMovementBehavior,
    pub air_behavior: AirMovementBehavior,
    pub sliding_behavior: SlidingBehavior,
}

impl MovementStateMachine {
    pub fn new(config: &CharacterControllerConfig) -> Self {
        Self {
            ground_behavior: GroundMovementBehavior {
                config: config.ground.clone(),
            },
            air_behavior: AirMovementBehavior {
                config: config.air.clone(),
            },
            sliding_behavior: SlidingBehavior {
                config: config.slopes.clone(),
            },
        }
    }

    pub fn update(
        &self,
        input: &MovementInput,
        state: &mut CharacterControllerState,
        dt: f32,
    ) -> Vec3 {
        match state.movement_state {
            MovementState::Walking | MovementState::Running | MovementState::Idle => {
                if state.is_grounded {
                    self.ground_behavior.apply(input, state, dt)
                } else {
                    self.air_behavior.apply(input, state, dt)
                }
            }
            MovementState::Sliding => {
                self.sliding_behavior.apply(input, state, dt)
            }
            MovementState::Jumping | MovementState::Falling => {
                self.air_behavior.apply(input, state, dt)
            }
            MovementState::SteppingUp | MovementState::Landing => {
                // Special states, minimal movement
                Vec3::ZERO
            }
        }
    }
}

/// Animation state mapping for integration with existing animation system
impl From<MovementState> for crate::components::AnimationState {
    fn from(movement_state: MovementState) -> Self {
        match movement_state {
            MovementState::Idle => crate::components::AnimationState::Idle,
            MovementState::Walking => crate::components::AnimationState::Walking,
            MovementState::Running => crate::components::AnimationState::Running,
            MovementState::Jumping => crate::components::AnimationState::Jumping,
            MovementState::Falling => crate::components::AnimationState::Falling,
            MovementState::Landing => crate::components::AnimationState::Landing,
            MovementState::Sliding => crate::components::AnimationState::Walking, // Use walking for sliding
            MovementState::SteppingUp => crate::components::AnimationState::Walking,
        }
    }
}

/// Helper functions for movement calculations
pub mod movement_utils {
    use super::*;

    /// Calculate the angle between movement direction and slope
    pub fn movement_slope_angle(movement_dir: Vec3, surface_normal: Vec3) -> f32 {
        let slope_direction = Vec3::new(-surface_normal.x, 0.0, -surface_normal.z).normalize();
        movement_dir.dot(slope_direction).acos()
    }

    /// Apply surface constraints to movement
    pub fn constrain_to_surface(movement: Vec3, surface_normal: Vec3) -> Vec3 {
        movement - surface_normal * movement.dot(surface_normal)
    }

    /// Calculate effective movement speed on slopes
    pub fn slope_speed_modifier(
        movement_dir: Vec3,
        surface_normal: Vec3,
        uphill_modifier: f32,
        downhill_modifier: f32,
    ) -> f32 {
        let slope_direction = Vec3::new(-surface_normal.x, 0.0, -surface_normal.z).normalize();
        let dot_product = movement_dir.dot(slope_direction);

        if dot_product > 0.1 {
            // Going downhill
            downhill_modifier
        } else if dot_product < -0.1 {
            // Going uphill
            uphill_modifier
        } else {
            // Perpendicular to slope
            1.0
        }
    }

    /// Smooth velocity transitions to prevent jittering
    pub fn smooth_velocity_transition(
        current_velocity: Vec3,
        target_velocity: Vec3,
        smoothing_factor: f32,
        dt: f32,
    ) -> Vec3 {
        let transition_speed = smoothing_factor * dt;
        current_velocity.lerp(target_velocity, transition_speed.clamp(0.0, 1.0))
    }
}