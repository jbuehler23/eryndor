use bevy::prelude::*;
use avian3d::prelude::*;
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
    // Spawn the player entity with physics - initially with fallback capsule
    let player_entity = commands.spawn((
        // Visual representation - Start with fallback, will be replaced when assets load
        Mesh3d(meshes.add(Capsule3d::new(0.5, 1.8))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.4, 0.8), // Blue player fallback
            ..default()
        })),
        
        Transform::from_xyz(0.0, 5.0, 0.0), // Start high above terrain to let physics settle
        
        // Physics components - Avian 3D
        RigidBody::Dynamic, // Dynamic for gravity and realistic physics
        // Capsule collider positioned so bottom aligns with feet
        Collider::capsule(0.5, 0.5),
        ColliderTransform {
            translation: Vec3::Y, // Lift collider so bottom = feet level
            rotation: Rotation(Quat::IDENTITY),
            scale: Vec3::ONE,
        },
        
        // Prevent rotation on X and Z axes (character should stay upright)
        LockedAxes::new().lock_rotation_x().lock_rotation_z(),
        
        // Physics properties for character movement
        LinearVelocity::default(),
        Friction::new(0.7), // Ground friction for stopping
        Restitution::new(0.0), // No bounce when hitting things
    )).id();
    
    // Add game components separately to avoid bundle size limits
    commands.entity(player_entity).insert((
        crate::components::Player,
        crate::components::PlayerMovementConfig::default(),
        crate::components::PlayerMovementState::default(), // Smooth movement state tracking
        crate::components::PlayerStats::default(),
        crate::components::AnimationController::default(),
        crate::components::CharacterModel::default(), // Track character model type
        crate::components::KnightAnimationSetup::default(), // Track animation setup
    ));
    
    info!("Player spawned with fallback capsule - will upgrade to 3D model when assets load");

    // Spawn the camera positioned behind and above the player
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 8.0, 12.0)
            .looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y), // Look at player height
        GameCamera::default(),
    ));

    // Note: Ground collision is now provided by the terrain mesh system

    // Add a simple cube for reference with physics
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.7, 0.6),
            ..default()
        })),
        Transform::from_xyz(5.0, 2.0, 0.0), // Move it to the side so player doesn't spawn inside
        
        // Physics for the cube
        RigidBody::Static, // Static reference object
        Collider::cuboid(1.0, 1.0, 1.0), // Match visual size
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