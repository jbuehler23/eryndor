use bevy::prelude::*;
use crate::resources::{GameConfig, InputResource};

// Camera component - Single responsibility with spherical coordinates
#[derive(Component)]
pub struct GameCamera {
    pub sensitivity: f32,
    pub speed: f32,
    
    // Spherical coordinates around player
    pub distance: f32,      // Radius from player
    pub yaw: f32,          // Horizontal rotation around player
    pub pitch: f32,        // Vertical angle (up/down)
    
    // Constraints
    pub min_distance: f32,
    pub max_distance: f32,
    pub min_pitch: f32,    // Maximum look down angle
    pub max_pitch: f32,    // Maximum look up angle
    
    // Smooth movement
    pub follow_speed: f32,
    pub zoom_speed: f32,
}

impl Default for GameCamera {
    fn default() -> Self {
        Self {
            sensitivity: 0.005,   // Reduced for smoother control
            speed: 10.0,
            
            // Default camera position: behind and above player
            distance: 12.0,
            yaw: 0.0,             // Behind player
            pitch: 0.3,           // Slightly looking down
            
            // Reasonable constraints for MMO camera
            min_distance: 3.0,
            max_distance: 20.0,
            min_pitch: -1.4,      // ~80 degrees down
            max_pitch: 1.2,       // ~70 degrees up
            
            follow_speed: 10.0,
            zoom_speed: 5.0,
        }
    }
}

// Camera setup system
pub fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn the player entity first
    commands.spawn((
        // Visual representation - simple capsule for now
        Mesh3d(meshes.add(Capsule3d::new(0.5, 1.8))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.4, 0.8), // Blue player
            ..default()
        })),
        Transform::from_xyz(0.0, 1.0, 0.0), // Standing on ground
        // Player components
        crate::components::Player,
        crate::components::PlayerMovement::default(),
        crate::components::PlayerStats::default(),
    ));

    // Spawn the camera positioned behind and above the player
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 8.0, 12.0)
            .looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y), // Look at player height
        GameCamera::default(),
    ));

    // Create a ground plane - Basic 3D scene
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3), // Green ground
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Add a simple cube for reference
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.7, 0.6),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));

    // Add a light source
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1.0, -0.5, 0.0)),
    ));
}

// WoW-style camera system with proper orbit controls
pub fn update_camera(
    time: Res<Time>,
    config: Res<GameConfig>,
    input: Res<InputResource>,
    mut camera_query: Query<(&mut Transform, &mut GameCamera)>,
    mut player_query: Query<&mut Transform, (With<crate::components::Player>, Without<GameCamera>)>,
) {
    let Ok((mut camera_transform, mut camera)) = camera_query.single_mut() else {
        return;
    };
    
    let Ok(mut player_transform) = player_query.single_mut() else {
        return;
    };

    let sensitivity = camera.sensitivity * config.input.mouse_sensitivity;
    let player_pos = player_transform.translation;
    
    // Handle mouse input based on drag type
    if input.mouse_delta.length() > 0.0 {
        if input.mouse_left_held && !input.mouse_right_held {
            // LEFT DRAG: Orbit camera around player (player doesn't rotate)
            camera.yaw -= input.mouse_delta.x * sensitivity;
            camera.pitch += input.mouse_delta.y * sensitivity;
            camera.pitch = camera.pitch.clamp(camera.min_pitch, camera.max_pitch);
            
        } else if input.mouse_right_held && !input.mouse_left_held {
            // RIGHT DRAG: Rotate player AND camera together (mouselook)
            let yaw_delta = -input.mouse_delta.x * sensitivity;
            let pitch_delta = input.mouse_delta.y * sensitivity;
            
            // Rotate player character around Y axis
            player_transform.rotate_y(yaw_delta);
            
            // Update camera yaw to match player rotation
            camera.yaw += yaw_delta;
            
            // Update camera pitch
            camera.pitch += pitch_delta;
            camera.pitch = camera.pitch.clamp(camera.min_pitch, camera.max_pitch);
            
        } else if input.mouse_left_held && input.mouse_right_held {
            // BOTH BUTTONS: Move forward/backward (classic WoW behavior)
            // This would be handled by the movement system, not camera
        }
    }
    
    // Handle zoom with mouse wheel
    if input.scroll_delta != 0.0 {
        camera.distance -= input.scroll_delta * camera.zoom_speed * time.delta_secs();
        camera.distance = camera.distance.clamp(camera.min_distance, camera.max_distance);
    }
    
    // Calculate camera position using spherical coordinates
    let camera_offset = Vec3::new(
        camera.distance * camera.pitch.cos() * camera.yaw.sin(),
        camera.distance * camera.pitch.sin(),
        camera.distance * camera.pitch.cos() * camera.yaw.cos(),
    );
    
    // Position camera relative to player
    let target_pos = player_pos + Vec3::new(0.0, 1.5, 0.0); // Look at player's head height
    let desired_camera_pos = target_pos + camera_offset;
    
    // Smoothly move camera to desired position
    camera_transform.translation = camera_transform.translation.lerp(
        desired_camera_pos, 
        camera.follow_speed * time.delta_secs()
    );
    
    // Always look at player
    camera_transform.look_at(target_pos, Vec3::Y);
}