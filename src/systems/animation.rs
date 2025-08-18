use bevy::prelude::*;
use avian3d::prelude::*;
use crate::components::{Player, AnimationController, AnimationAssets};
use crate::resources::InputResource;

/// Animation system - updates character animation states based on input and physics
/// Following Single Responsibility: only handles animation state updates
pub fn update_animation_states(
    time: Res<Time>,
    input: Res<InputResource>,
    mut animation_query: Query<(&mut AnimationController, &LinearVelocity), With<Player>>,
) {
    for (mut anim_controller, velocity) in animation_query.iter_mut() {
        // Ground detection - simple physics check for now
        let is_grounded = velocity.y.abs() < 1.0;
        
        // Determine input-based movement state
        let is_moving = input.forward || input.backward || input.left || input.right || 
                       (input.mouse_left_held && input.mouse_right_held); // WoW both-button forward
        let is_running = is_moving && input.up; // Shift key for running
        let is_jumping = input.down; // Space key for jumping
        
        // Update animation state based on input and physics
        let state_changed = anim_controller.update_state(
            **velocity, 
            is_grounded,
            is_moving,
            is_running, 
            is_jumping,
            time.delta_secs()
        );
        
        if state_changed {
            info!(
                "Animation state changed: {:?} -> {:?}", 
                anim_controller.previous_state, 
                anim_controller.current_state
            );
        }
    }
}

/// Animation asset loading system - loads animation clips for characters
/// Following DRY: Centralized animation asset management
pub fn setup_animation_assets(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
) {
    // Initialize animation assets resource
    let animation_assets = AnimationAssets {
        // TODO: Load actual animation files when we have character models
        // For now, we'll set up the resource structure
        idle: None, // Some(asset_server.load("animations/idle.gltf#Animation0")),
        walk: None, // Some(asset_server.load("animations/walk.gltf#Animation0")),
        run: None,  // Some(asset_server.load("animations/run.gltf#Animation0")),
        jump: None, // Some(asset_server.load("animations/jump.gltf#Animation0")),
        fall: None, // Some(asset_server.load("animations/fall.gltf#Animation0")),
        land: None, // Some(asset_server.load("animations/land.gltf#Animation0")),
    };
    
    commands.insert_resource(animation_assets);
    info!("Animation assets resource initialized");
}

/// Animation player system - plays animations based on current state
/// Following Open/Closed: Easy to extend with new animation types
pub fn play_animations(
    _animation_assets: Res<AnimationAssets>,
    animation_query: Query<
        &AnimationController, 
        (With<Player>, Changed<AnimationController>)
    >,
) {
    for anim_controller in animation_query.iter() {
        // TODO: Implement actual animation playing once we have character models
        // For now, just log the state changes for debugging
        debug!(
            "Animation state: {:?}", 
            anim_controller.current_state
        );
        
        // The animation system will be completed in a future phase when we add:
        // - Character models with animation rigs
        // - Animation clips loaded from assets
        // - AnimationPlayer components on entities
    }
}

/// Debug animation system - displays current animation state in UI
/// Following YAGNI: Simple debug display for development
pub fn debug_animation_state(
    animation_query: Query<&AnimationController, With<Player>>,
    mut debug_text_query: Query<&mut Text, With<crate::systems::ui::FPSText>>,
) {
    if let Ok(anim_controller) = animation_query.single() {
        if let Ok(mut text) = debug_text_query.single_mut() {
            // Update debug text to include animation state (Bevy 0.16 Text API)
            let current_text = &text.0;
            let fps_part = current_text.split('\n').next().unwrap_or(current_text);
            
            text.0 = format!(
                "{}\nAnim: {:?}", 
                fps_part,
                anim_controller.current_state
            );
        }
    }
}