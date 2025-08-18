use bevy::prelude::*;
use eryndor::EryndorPlugin;

#[test]
fn test_player_component_creation() {
    use eryndor::components::{Player, PlayerMovement, PlayerStats};
    
    let player = Player;
    let movement = PlayerMovement::default();
    let stats = PlayerStats::default();
    
    // Verify default values
    assert_eq!(movement.speed, 5.0);
    assert_eq!(movement.run_speed, 10.0);
    assert_eq!(movement.is_running, false);
    assert_eq!(stats.health, 100.0);
    assert_eq!(stats.max_health, 100.0);
}

// Integration tests for the Eryndor game engine

#[test]
fn test_full_app_startup() {
    // Create a headless app for testing (no rendering)
    let mut app = App::new();
    app
        .add_plugins(MinimalPlugins) // Use minimal plugins for testing
        .add_plugins(EryndorPlugin);
    
    // Verify the app can startup without panicking
    app.update(); // Run one frame
    
    // Verify player entity was created
    use eryndor::components::Player;
    let mut player_query = app.world_mut().query::<&Player>();
    assert_eq!(player_query.iter(&app.world()).count(), 1, "Player entity should be spawned");
    
    // Verify camera entity was created
    use eryndor::systems::camera::GameCamera;
    let mut camera_query = app.world_mut().query::<&GameCamera>();
    assert_eq!(camera_query.iter(&app.world()).count(), 1, "Camera entity should be spawned");
}

#[test]
fn test_config_loading() {
    use eryndor::resources::config::load_config;
    
    let config = load_config();
    
    // Verify default config values
    assert_eq!(config.graphics.vsync, true);
    assert_eq!(config.graphics.msaa_samples, 4);
    assert_eq!(config.graphics.render_scale, 1.0);
    assert_eq!(config.input.mouse_sensitivity, 0.002);
    assert_eq!(config.input.invert_y, false);
    assert_eq!(config.debug.show_fps, true);
}

#[test]
fn test_input_resource_initialization() {
    use eryndor::resources::input::InputResource;
    
    let input = InputResource::default();
    
    // Verify default input state
    assert!(!input.forward);
    assert!(!input.backward);
    assert!(!input.left);
    assert!(!input.right);
    assert!(!input.up);
    assert!(!input.down);
    
    // Verify mouse states
    assert!(!input.mouse_left_held);
    assert!(!input.mouse_right_held);
    assert!(!input.mouse_left_just_pressed);
    assert!(!input.mouse_right_just_pressed);
    assert!(!input.mouse_left_just_released);
    assert!(!input.mouse_right_just_released);
    assert_eq!(input.mouse_delta, Vec2::ZERO);
    assert_eq!(input.drag_start_position, None);
    assert!(!input.is_dragging);
    assert_eq!(input.scroll_delta, 0.0);
}