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
pub fn setup_camera(mut commands: Commands) {
    // Calculate initial camera position using spherical coordinates to match dynamic system
    let player_spawn = Vec3::new(-70.0, 20.0, -70.0); // Updated to match new player spawn
    let camera_defaults = GameCamera::default();
    
    // Calculate camera offset using same spherical coordinate math as update_camera
    let camera_offset = Vec3::new(
        camera_defaults.distance * camera_defaults.pitch.cos() * camera_defaults.yaw.sin(),
        camera_defaults.distance * camera_defaults.pitch.sin(),
        camera_defaults.distance * camera_defaults.pitch.cos() * camera_defaults.yaw.cos(),
    );
    
    let target_pos = player_spawn + Vec3::new(0.0, 1.5, 0.0); // Same head height as update_camera
    let initial_camera_pos = target_pos + camera_offset;
    
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(initial_camera_pos).looking_at(target_pos, Vec3::Y),
        camera_defaults,
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

// WoW-style camera system with proper orbit controls for physics player
pub fn update_camera(
    time: Res<Time>,
    config: Res<GameConfig>,
    input: Res<InputResource>,
    mut camera_query: Query<(&mut Transform, &mut GameCamera)>,
    player_query: Query<&Transform, (With<crate::components::Player>, Without<GameCamera>)>,
) {
    let Ok((mut camera_transform, mut camera)) = camera_query.single_mut() else {
        return;
    };
    
    let Ok(player_transform) = player_query.single() else {
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
            // RIGHT DRAG: Rotate camera together with player direction (mouselook)
            // Note: Player rotation with physics needs to be handled in the player system
            let yaw_delta = -input.mouse_delta.x * sensitivity;
            let pitch_delta = input.mouse_delta.y * sensitivity;
            
            // Update camera yaw and pitch together
            camera.yaw += yaw_delta;
            camera.pitch += pitch_delta;
            camera.pitch = camera.pitch.clamp(camera.min_pitch, camera.max_pitch);
            
        } else if input.mouse_left_held && input.mouse_right_held {
            // BOTH BUTTONS: Move forward/backward (classic WoW behavior)
            // This would be handled by the movement system, not camera
        }
    }
    
    // Handle zoom with mouse wheel
    if input.scroll_delta != 0.0 {
        camera.distance -= input.scroll_delta * camera.zoom_speed;
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
    
    
    // Smoothly move camera to desired position - use clamped lerp factor to prevent feedback loops
    let lerp_factor = (camera.follow_speed * time.delta_secs()).clamp(0.0, 1.0);
    camera_transform.translation = camera_transform.translation.lerp(
        desired_camera_pos, 
        lerp_factor
    );
    
    // Always look at player
    camera_transform.look_at(target_pos, Vec3::Y);
}