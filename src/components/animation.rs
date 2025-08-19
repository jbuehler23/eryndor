use bevy::prelude::*;
use bevy_animation::graph::{AnimationGraph, AnimationNodeIndex};

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
    pub state_just_changed: bool,   // Flag to track if state changed this frame
}

impl Default for AnimationController {
    fn default() -> Self {
        Self {
            current_state: AnimationState::Idle,
            previous_state: AnimationState::Idle,
            state_timer: 0.0,
            transition_time: 0.1,      // 100ms transitions
            velocity_threshold: 0.5,    // Kept for backward compatibility (unused)
            run_threshold: 8.0,         // Kept for backward compatibility (unused)  
            air_time_threshold: 0.2,    // 200ms before falling animation
            state_just_changed: false,  // Initially no state change
        }
    }
}

/// Animation assets resource - holds references to animation clips and graphs
/// Following DRY: Centralized animation asset management
#[derive(Resource, Default)]
pub struct AnimationAssets {
    pub idle: Option<Handle<AnimationClip>>,
    pub walk: Option<Handle<AnimationClip>>,
    pub run: Option<Handle<AnimationClip>>,
    pub jump: Option<Handle<AnimationClip>>,
    pub fall: Option<Handle<AnimationClip>>,
    pub land: Option<Handle<AnimationClip>>,
    // Animation graph and node indices for Bevy 0.16
    pub animation_graph: Option<Handle<AnimationGraph>>,
    pub idle_node: Option<AnimationNodeIndex>,
    pub walk_node: Option<AnimationNodeIndex>, 
    pub run_node: Option<AnimationNodeIndex>,
    pub jump_node: Option<AnimationNodeIndex>,
    pub fall_node: Option<AnimationNodeIndex>,
    pub land_node: Option<AnimationNodeIndex>,
}

/// Knight animation setup component - tracks animation player entity and graph
#[derive(Component)]
pub struct KnightAnimationSetup {
    pub animation_player_entity: Option<Entity>,
    pub graph_handle: Option<Handle<AnimationGraph>>,
    pub animation_nodes: [Option<AnimationNodeIndex>; 6], // idle, walk, run, jump, fall, land
}

impl Default for KnightAnimationSetup {
    fn default() -> Self {
        Self {
            animation_player_entity: None,
            graph_handle: None,
            animation_nodes: [None; 6],
        }
    }
}

impl AnimationController {
    /// Update animation state based on input and physics
    /// Following KISS: Input-driven for ground states, physics for air states, proper landing transitions
    pub fn update_state(&mut self, velocity: Vec3, is_grounded: bool, is_moving: bool, is_running: bool, is_jumping: bool, dt: f32) -> bool {
        let vertical_speed = velocity.y;
        let was_grounded = matches!(self.current_state, AnimationState::Idle | AnimationState::Walking | AnimationState::Running | AnimationState::Landing);
        
        // Determine new state based on input, physics, and transitions
        let new_state = if !is_grounded {
            // Airborne states
            if vertical_speed > 0.5 {
                AnimationState::Jumping
            } else if self.state_timer > self.air_time_threshold {
                AnimationState::Falling
            } else {
                self.current_state // Keep current state briefly
            }
        } else {
            // Just landed - transition through landing state if coming from air
            if !was_grounded && matches!(self.current_state, AnimationState::Jumping | AnimationState::Falling) {
                AnimationState::Landing
            }
            // Landing state auto-transitions after short duration (0.3s for landing animation)
            else if self.current_state == AnimationState::Landing && self.state_timer > 0.3 {
                // Transition to appropriate movement state after landing
                if !is_moving {
                    AnimationState::Idle
                } else if is_running {
                    AnimationState::Running
                } else {
                    AnimationState::Walking
                }
            }
            // Normal grounded states - only allow jump if not in landing transition
            else if is_jumping && self.current_state != AnimationState::Landing {
                AnimationState::Jumping  // Immediate jump response (but not during landing)
            } else if self.current_state == AnimationState::Landing {
                self.current_state // Stay in landing until timer expires
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
            self.state_just_changed = true;
        } else {
            self.state_timer += dt;
            self.state_just_changed = false;
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