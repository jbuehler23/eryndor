use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::input::mouse::{MouseWheel, MouseMotion};
use crate::resources::InputResource;

// Input handling system - WoW-style camera controls
pub fn handle_input(
    mut input_resource: ResMut<InputResource>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut scroll_events: EventReader<MouseWheel>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    // Reset frame-specific input
    input_resource.mouse_delta = Vec2::ZERO;
    input_resource.scroll_delta = 0.0;
    
    // Keyboard input - KISS approach
    input_resource.forward = keyboard.pressed(KeyCode::KeyW);
    input_resource.backward = keyboard.pressed(KeyCode::KeyS);
    input_resource.left = keyboard.pressed(KeyCode::KeyA);
    input_resource.right = keyboard.pressed(KeyCode::KeyD);
    input_resource.up = keyboard.pressed(KeyCode::ShiftLeft); // Shift for running
    input_resource.down = keyboard.pressed(KeyCode::ControlLeft);
    
    // Mouse button state tracking
    input_resource.mouse_left_just_pressed = mouse.just_pressed(MouseButton::Left);
    input_resource.mouse_right_just_pressed = mouse.just_pressed(MouseButton::Right);
    input_resource.mouse_left_just_released = mouse.just_released(MouseButton::Left);
    input_resource.mouse_right_just_released = mouse.just_released(MouseButton::Right);
    input_resource.mouse_left_held = mouse.pressed(MouseButton::Left);
    input_resource.mouse_right_held = mouse.pressed(MouseButton::Right);
    
    // Handle drag state and cursor visibility
    let was_dragging = input_resource.is_dragging;
    input_resource.is_dragging = input_resource.mouse_left_held || input_resource.mouse_right_held;
    
    if let Ok(mut window) = windows.single_mut() {
        if input_resource.is_dragging && !was_dragging {
            // Start dragging - hide cursor and grab it
            window.cursor_options.visible = false;
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
        } else if !input_resource.is_dragging && was_dragging {
            // Stop dragging - show cursor and release it
            window.cursor_options.visible = true;
            window.cursor_options.grab_mode = CursorGrabMode::None;
        }
    }
    
    // Mouse movement - only during drag operations, use relative motion
    if input_resource.is_dragging {
        // Accumulate all mouse motion events this frame
        for mouse_motion in mouse_motion_events.read() {
            input_resource.mouse_delta += mouse_motion.delta;
        }
    } else {
        // Clear mouse motion events when not dragging
        mouse_motion_events.clear();
    }
    
    // Mouse wheel for zoom
    for scroll_event in scroll_events.read() {
        input_resource.scroll_delta += scroll_event.y;
    }
}