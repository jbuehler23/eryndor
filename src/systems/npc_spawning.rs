use bevy::prelude::*;
use crate::components::dialogue::*;
use avian3d::prelude::*;

/// System to spawn NPCs in the world
pub fn spawn_demo_npcs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    dialogue_db: Res<DialogueDatabase>,
    existing_npcs: Query<&NpcInfo>,
) {
    // Only spawn NPCs once
    if !existing_npcs.is_empty() {
        return;
    }
    
    info!("👥 Spawning demo NPCs");
    
    // Spawn Merchant Aldric if we have his dialogue
    if dialogue_db.npcs.contains_key("merchant_aldric") {
        spawn_npc(
            &mut commands,
            &mut meshes,
            &mut materials,
            NpcSpawnInfo {
                npc_id: "merchant_aldric".to_string(),
                display_name: "Aldric Goldweaver".to_string(),
                description: "A nervous merchant with mysterious goods".to_string(),
                position: Vec3::new(10.0, 1.0, 5.0),
                npc_type: NpcType::Merchant,
                shape_color: Color::srgb(0.2, 0.8, 0.4), // Green for merchant
                scale: 1.2,
                interaction_range: 4.0,
            }
        );
    }
    
    // Spawn a generic guard NPC
    spawn_npc(
        &mut commands,
        &mut meshes,
        &mut materials,
        NpcSpawnInfo {
            npc_id: "town_guard".to_string(),
            display_name: "Town Guard".to_string(),
            description: "A watchful guard keeping order".to_string(),
            position: Vec3::new(-5.0, 1.0, -8.0),
            npc_type: NpcType::Guard,
            shape_color: Color::srgb(0.4, 0.4, 0.8), // Blue for guard
            scale: 1.0,
            interaction_range: 3.0,
        }
    );
    
    // Spawn a villager NPC
    spawn_npc(
        &mut commands,
        &mut meshes,
        &mut materials,
        NpcSpawnInfo {
            npc_id: "village_elder".to_string(),
            display_name: "Village Elder".to_string(),
            description: "An wise elder with many stories".to_string(),
            position: Vec3::new(0.0, 1.0, 12.0),
            npc_type: NpcType::Villager,
            shape_color: Color::srgb(0.8, 0.6, 0.2), // Yellow for villager
            scale: 0.9,
            interaction_range: 3.5,
        }
    );
    
    info!("✅ Demo NPCs spawned");
}

struct NpcSpawnInfo {
    npc_id: String,
    display_name: String,
    description: String,
    position: Vec3,
    npc_type: NpcType,
    shape_color: Color,
    scale: f32,
    interaction_range: f32,
}

fn spawn_npc(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    spawn_info: NpcSpawnInfo,
) {
    // Create material for the NPC
    let material_handle = materials.add(StandardMaterial {
        base_color: spawn_info.shape_color,
        metallic: 0.1,
        perceptual_roughness: 0.8,
        ..default()
    });
    
    // Store values we need to reuse to avoid move issues
    let npc_id = spawn_info.npc_id.clone();
    let display_name = spawn_info.display_name.clone();
    let description = spawn_info.description.clone();
    let npc_type = spawn_info.npc_type;
    let position = spawn_info.position;
    
    // Create the NPC visual (simple capsule shape)
    let _npc_entity = commands.spawn((
        // Transform and visibility
        Transform::from_translation(spawn_info.position)
            .with_scale(Vec3::splat(spawn_info.scale)),
        Visibility::default(),
        
        // 3D Mesh
        Mesh3d(meshes.add(Capsule3d::new(0.5, 1.8))),
        MeshMaterial3d(material_handle.clone()),
        
        // Physics (static collider so player can't walk through)
        RigidBody::Static,
        Collider::capsule(0.5, 1.8),
        
        // NPC Components
        NpcInfo {
            npc_id: npc_id.clone(),
            display_name: display_name.clone(),
            description,
            npc_type,
        },
        
        DialogueInteractable {
            npc_id: npc_id.clone(),
            interaction_range: spawn_info.interaction_range,
            has_new_dialogue: true,
            priority_level: match npc_type {
                NpcType::Questgiver => 3,
                NpcType::Merchant => 2,
                _ => 1,
            },
        },
        
        DialogueState {
            npc_id: npc_id.clone(),
            current_conversation: None,
            current_node: "start".to_string(),
            conversation_history: Vec::new(),
            flags_set: Vec::new(),
            is_active: false,
            trust_level: 0,
            relationship_modifiers: std::collections::HashMap::new(),
        },
        
        // Material component for visual updates
        NpcMaterial {
            material_handle,
            base_color: spawn_info.shape_color,
        },
    )).id();
    
    // Add floating name tag (simple text above NPC) - TODO: Implement 3D text later
    // For now, we'll skip the name tag as Text3d is not available in the current Bevy version
    // let name_tag = commands.spawn((
    //     Text2d::new(spawn_info.display_name),
    //     Transform::from_translation(Vec3::new(0.0, 2.5, 0.0))
    //         .with_scale(Vec3::splat(0.5)),
    // )).id();
    
    // Make the name tag a child of the NPC
    // commands.entity(npc_entity).add_children(&[name_tag]);
    
    info!("✅ Spawned NPC: {} at {:?}", display_name, position);
}

/// Component to track NPC material for visual updates
#[derive(Component)]
pub struct NpcMaterial {
    pub material_handle: Handle<StandardMaterial>,
    pub base_color: Color,
}

/// System to update NPC visual indicators based on interaction state
pub fn update_npc_indicators(
    npc_query: Query<(&Transform, &DialogueInteractable, &NpcMaterial), Without<crate::components::Player>>,
    player_query: Query<&Transform, With<crate::components::Player>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    
    for (npc_transform, interactable, npc_material) in &npc_query {
        let distance = player_transform.translation.distance(npc_transform.translation);
        
        if let Some(material) = materials.get_mut(&npc_material.material_handle) {
            // Change NPC color when player is in interaction range
            if distance <= interactable.interaction_range {
                // Brighten the NPC when in range
                material.emissive = LinearRgba::new(0.3, 0.3, 0.3, 1.0);
            } else {
                // Normal color when out of range
                material.emissive = LinearRgba::new(0.0, 0.0, 0.0, 1.0);
            }
        }
    }
}

/// System to display interaction prompts for nearby NPCs
pub fn npc_interaction_prompts(
    npc_query: Query<(&Transform, &DialogueInteractable, &NpcInfo), Without<crate::components::Player>>,
    player_query: Query<&Transform, With<crate::components::Player>>,
    active_dialogue: Res<ActiveDialogue>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    
    // Find nearest interactable NPC
    let mut nearest_npc: Option<(&NpcInfo, f32)> = None;
    
    for (npc_transform, interactable, npc_info) in &npc_query {
        let distance = player_transform.translation.distance(npc_transform.translation);
        
        if distance <= interactable.interaction_range {
            match nearest_npc {
                None => nearest_npc = Some((npc_info, distance)),
                Some((_, nearest_distance)) => {
                    if distance < nearest_distance {
                        nearest_npc = Some((npc_info, distance));
                    }
                }
            }
        }
    }
    
    // Display interaction prompt for nearest NPC
    static mut LAST_PROMPT_NPC: Option<String> = None;
    
    unsafe {
        match nearest_npc {
            Some((npc_info, _)) => {
                // Only show prompt if it's a new NPC or we're not in dialogue
                if LAST_PROMPT_NPC.as_ref() != Some(&npc_info.npc_id) && active_dialogue.npc_entity.is_none() {
                    info!("💬 Press E to talk to {}", npc_info.display_name);
                    LAST_PROMPT_NPC = Some(npc_info.npc_id.clone());
                }
            },
            None => {
                LAST_PROMPT_NPC = None;
            }
        }
    }
}

/// Debug system to show NPC information
pub fn debug_npc_info(
    keyboard: Res<ButtonInput<KeyCode>>,
    npc_query: Query<(&NpcInfo, &DialogueState, &DialogueInteractable)>,
) {
    if keyboard.just_pressed(KeyCode::F11) {
        info!("👥 NPC DEBUG INFO:");
        for (npc_info, dialogue_state, interactable) in &npc_query {
            info!("  {} ({})", npc_info.display_name, npc_info.npc_id);
            info!("    Type: {:?}", npc_info.npc_type);
            info!("    Description: {}", npc_info.description);
            info!("    In dialogue: {}", dialogue_state.is_active);
            info!("    Interaction range: {}", interactable.interaction_range);
            info!("    Trust level: {}", dialogue_state.trust_level);
            info!("    Flags set: {:?}", dialogue_state.flags_set);
            info!("    ---");
        }
    }
}