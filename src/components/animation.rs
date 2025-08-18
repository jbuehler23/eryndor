use bevy::prelude::*;

/// Animation state enum - defines different character animation states
/// Following SOLID: Single responsibility for animation state management
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AnimationState {
    Idle,
    Walking,
    Running,
    Jumping,
    Falling,
    Landing,
}

impl Default for AnimationState {
    fn default() -> Self {
        AnimationState::Idle
    }
}

/// Animation controller component - manages character animation state transitions
/// Following Single Responsibility: only handles animation state logic
#[derive(Component)]
pub struct AnimationController {
    pub current_state: AnimationState,
    pub previous_state: AnimationState,
    pub state_timer: f32,           // Time in current state
    pub transition_time: f32,       // How long transitions take
    pub velocity_threshold: f32,    // Velocity to trigger walking
    pub run_threshold: f32,         // Velocity to trigger running
    pub air_time_threshold: f32,    // Time in air before falling animation
}

impl Default for AnimationController {
    fn default() -> Self {
        Self {
            current_state: AnimationState::Idle,
            previous_state: AnimationState::Idle,
            state_timer: 0.0,
            transition_time: 0.1,      // 100ms transitions
            velocity_threshold: 0.5,    // Walking speed threshold
            run_threshold: 8.0,         // Running speed threshold  
            air_time_threshold: 0.2,    // 200ms before falling animation
        }
    }
}

/// Animation assets resource - holds references to animation clips
/// Following DRY: Centralized animation asset management
#[derive(Resource, Default)]
pub struct AnimationAssets {
    pub idle: Option<Handle<AnimationClip>>,
    pub walk: Option<Handle<AnimationClip>>,
    pub run: Option<Handle<AnimationClip>>,
    pub jump: Option<Handle<AnimationClip>>,
    pub fall: Option<Handle<AnimationClip>>,
    pub land: Option<Handle<AnimationClip>>,
}

impl AnimationController {
    /// Update animation state based on input and physics
    /// Following KISS: Input-driven for ground states, physics for air states
    pub fn update_state(&mut self, velocity: Vec3, is_grounded: bool, is_moving: bool, is_running: bool, is_jumping: bool, dt: f32) -> bool {
        let vertical_speed = velocity.y;
        
        // Determine new state based on input and physics
        let new_state = if !is_grounded {
            if vertical_speed > 0.5 {
                AnimationState::Jumping
            } else if self.state_timer > self.air_time_threshold {
                AnimationState::Falling
            } else {
                self.current_state // Keep current state briefly
            }
        } else {
            // Grounded states - input-based for immediate response
            if is_jumping {
                AnimationState::Jumping  // Immediate jump response
            } else if !is_moving {
                AnimationState::Idle
            } else if is_running {
                AnimationState::Running
            } else {
                AnimationState::Walking
            }
        };
        
        // Update state if changed
        let state_changed = new_state != self.current_state;
        if state_changed {
            self.previous_state = self.current_state;
            self.current_state = new_state;
            self.state_timer = 0.0;
        } else {
            self.state_timer += dt;
        }
        
        state_changed
    }
    
    /// Check if currently in a specific state
    pub fn is_state(&self, state: AnimationState) -> bool {
        self.current_state == state
    }
    
    /// Check if transitioning between states
    pub fn is_transitioning(&self) -> bool {
        self.state_timer < self.transition_time
    }
}