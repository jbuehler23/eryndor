use bevy::prelude::*;

// Input state resource - Single responsibility for WoW-style controls
#[derive(Resource, Default)]
pub struct InputResource {
    // Movement
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    
    // Mouse button states
    pub mouse_left_held: bool,
    pub mouse_right_held: bool,
    pub mouse_left_just_pressed: bool,
    pub mouse_right_just_pressed: bool,
    pub mouse_left_just_released: bool,
    pub mouse_right_just_released: bool,
    
    // Mouse movement during drag operations
    pub mouse_delta: Vec2,
    pub drag_start_position: Option<Vec2>,
    pub is_dragging: bool,
    
    // Mouse wheel for zoom
    pub scroll_delta: f32,
}