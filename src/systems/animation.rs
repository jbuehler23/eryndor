use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_animation::graph::{AnimationGraph, AnimationGraphHandle};
use bevy::gltf::GltfAssetLabel;
use crate::components::{Player, AnimationController, AnimationAssets, KnightAnimationSetup, PlayerMovementState};
use crate::resources::InputResource;

/// Animation system - updates character animation states based on kinematic movement
/// Following Single Responsibility: only handles animation state updates
pub fn update_animation_states(
    time: Res<Time>,
    input: Res<InputResource>,
    mut animation_query: Query<(&mut AnimationController, &PlayerMovementState, &Transform, &Children), With<Player>>,
    spatial_query: SpatialQuery,
) {
    for (mut anim_controller, movement_state, transform, children) in animation_query.iter_mut() {
        // Ground detection using spatial query (same as kinematic character controller)
        let is_grounded = is_grounded_for_animation(transform.translation, &spatial_query, children);
        
        // Determine input-based movement state
        let is_moving = input.forward || input.backward || input.left || input.right || 
                       (input.mouse_left_held && input.mouse_right_held); // WoW both-button forward
        let is_running = is_moving && input.up; // Shift key for running
        let is_jumping = input.down; // Space key for jumping (now handled with landing logic)
        
        // Create velocity vector from kinematic movement state for animation system
        let current_velocity = Vec3::new(
            movement_state.current_direction.x * movement_state.current_speed,
            0.0, // Y velocity handled separately by gravity/jumping
            movement_state.current_direction.z * movement_state.current_speed
        );
        
        // Update animation state based on kinematic movement
        let state_changed = anim_controller.update_state(
            current_velocity, 
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

/// Ground detection for animation system - matches kinematic character controller
fn is_grounded_for_animation(pos: Vec3, spatial_query: &SpatialQuery, children: &Children) -> bool {
    use avian3d::prelude::*;
    
    let ray_origin = pos;
    let ray_direction = Dir3::NEG_Y;
    let max_distance = 3.0; // Increased to match player controller
    
    // Create filter to exclude all child colliders (we don't have player entity here, just children)
    let mut excluded_entities = Vec::new();
    for child in children.iter() {
        excluded_entities.push(child);
    }
    let filter = SpatialQueryFilter::default().with_excluded_entities(excluded_entities);
    
    if let Some(hit) = spatial_query.cast_ray(
        ray_origin, 
        ray_direction, 
        max_distance, 
        true, 
        &filter
    ) {
        // Consider grounded only if we hit something at a reasonable distance
        // Distance should be > 0.1 (to avoid self-collision) and < 1.2 (reasonable ground distance)
        hit.distance > 0.1 && hit.distance <= 1.2
    } else {
        false
    }
}

/// Animation asset loading system - simplified to just store empty resource
/// Following Bevy patterns: Animation graph created when scene loads
pub fn setup_animation_assets(
    mut commands: Commands,
) {
    // Initialize empty animation assets resource - will be populated when scene loads
    let animation_assets = AnimationAssets::default();
    commands.insert_resource(animation_assets);
    info!("Animation assets resource initialized - waiting for scene to load");
}

/// System to setup Knight animations when scene is ready
/// Following Bevy patterns from animated_mesh.rs example
pub fn setup_knight_animations_when_ready(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    _children: Query<&Children>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    mut knight_setup_query: Query<&mut KnightAnimationSetup, With<Player>>,
) {
    // Check if we have a newly added AnimationPlayer (from GLTF scene loading)
    for (animation_player_entity, mut animation_player) in players.iter_mut() {
        // Find the player entity with KnightAnimationSetup
        if let Ok(mut knight_setup) = knight_setup_query.single_mut() {
            if knight_setup.graph_handle.is_none() {
                // Create animation graph with Knight animations - fix order to match actual node indices
                let (graph, animation_nodes) = AnimationGraph::from_clips([
                    asset_server.load(GltfAssetLabel::Animation(72).from_asset("KayKit_Adventurers_1.0_FREE/Characters/gltf/Knight.glb")), // Walking_A (will be NodeIndex(0))
                    asset_server.load(GltfAssetLabel::Animation(36).from_asset("KayKit_Adventurers_1.0_FREE/Characters/gltf/Knight.glb")), // Idle (will be NodeIndex(1))  
                    asset_server.load(GltfAssetLabel::Animation(48).from_asset("KayKit_Adventurers_1.0_FREE/Characters/gltf/Knight.glb")), // Running_A (will be NodeIndex(2))
                    asset_server.load(GltfAssetLabel::Animation(42).from_asset("KayKit_Adventurers_1.0_FREE/Characters/gltf/Knight.glb")), // Jump_Start (will be NodeIndex(3))
                    asset_server.load(GltfAssetLabel::Animation(40).from_asset("KayKit_Adventurers_1.0_FREE/Characters/gltf/Knight.glb")), // Jump_Idle/fall (will be NodeIndex(4))
                    asset_server.load(GltfAssetLabel::Animation(41).from_asset("KayKit_Adventurers_1.0_FREE/Characters/gltf/Knight.glb")), // Jump_Land (will be NodeIndex(5))
                ]);
                
                let graph_handle = animation_graphs.add(graph);
                
                // Store animation setup
                knight_setup.graph_handle = Some(graph_handle.clone());
                knight_setup.animation_nodes = [
                    Some(animation_nodes[1]), // idle (now at index 1 in clips array)
                    Some(animation_nodes[0]), // walk (now at index 0 in clips array)
                    Some(animation_nodes[2]), // run (still at index 2)
                    Some(animation_nodes[3]), // jump (still at index 3) 
                    Some(animation_nodes[4]), // fall (still at index 4)
                    Some(animation_nodes[5]), // land (still at index 5)
                ];
                
                // Debug: Log the actual animation node indices (after reordering)
                info!("Animation node mapping created (reordered):");
                info!("  Walk (clips[0]): {:?}", animation_nodes[0]);
                info!("  Idle (clips[1]): {:?}", animation_nodes[1]);
                info!("  Run (clips[2]): {:?}", animation_nodes[2]);
                info!("  Jump (clips[3]): {:?}", animation_nodes[3]);
                info!("  Fall (clips[4]): {:?}", animation_nodes[4]);
                info!("  Land (clips[5]): {:?}", animation_nodes[5]);
                
                // Store the entity with the AnimationPlayer
                knight_setup.animation_player_entity = Some(animation_player_entity);
                
                // Add the animation graph to the entity with the AnimationPlayer
                commands.entity(animation_player_entity)
                    .insert(AnimationGraphHandle(graph_handle));
                
                // Start with idle animation - now at index 1 in clips array
                info!("Starting with initial idle animation: {:?}", animation_nodes[1]);
                animation_player.play(animation_nodes[1]).repeat();
                
                info!("Knight animations setup complete with 6 animation nodes!");
            }
        }
    }
}

/// Animation player system - plays Knight animations based on current state
/// Following corrected Bevy pattern: target the entity with AnimationPlayer
pub fn play_animations(
    mut knight_setup_query: Query<(&AnimationController, &KnightAnimationSetup), With<Player>>,
    mut animation_players: Query<&mut AnimationPlayer>,
) {
    for (anim_controller, knight_setup) in knight_setup_query.iter_mut() {
        // Only proceed if we have the animation player entity set up
        if let (Some(animation_player_entity), Some(_)) = (knight_setup.animation_player_entity, &knight_setup.graph_handle) {
            if let Ok(mut animation_player) = animation_players.get_mut(animation_player_entity) {
                // Only change animation if state just changed this frame
                if anim_controller.state_just_changed {
                    // Get the animation node index for current state
                    let (node_index, should_repeat) = match anim_controller.current_state {
                        crate::components::AnimationState::Idle => (knight_setup.animation_nodes[0], true),
                        crate::components::AnimationState::Walking => (knight_setup.animation_nodes[1], true),
                        crate::components::AnimationState::Running => (knight_setup.animation_nodes[2], true),
                        crate::components::AnimationState::Jumping => (knight_setup.animation_nodes[3], false),
                        crate::components::AnimationState::Falling => (knight_setup.animation_nodes[4], true),
                        crate::components::AnimationState::Landing => (knight_setup.animation_nodes[5], false),
                    };
                    
                    // Play the animation if we have the node
                    if let Some(node_index) = node_index {
                        // Use play_and_fade for smooth transitions
                        animation_player.stop_all();
                        let animation = animation_player.play(node_index);

                        if should_repeat {
                            animation.repeat();
                        }

                        info!(
                            "Playing Knight animation node {:?} for state {:?} (repeat: {})", 
                            node_index, 
                            anim_controller.current_state,
                            should_repeat
                        );
                    } else {
                        warn!(
                            "No animation node loaded for state {:?}", 
                            anim_controller.current_state
                        );
                    }
                }
            }
        }
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