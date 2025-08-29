//! Console UI and input handling systems.

use bevy::prelude::*;
use crate::{ConsoleState, ConsoleHistory, ConsoleLine, ConsoleLineType, ConsoleCommand};

/// System to handle console toggle with backtick key
pub fn console_toggle_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut console_state: ResMut<ConsoleState>,
) {
    if keyboard.just_pressed(KeyCode::Backquote) {
        console_state.is_visible = !console_state.is_visible;
        info!(
            "Dev console {}",
            if console_state.is_visible { "opened" } else { "closed" }
        );
        
        // Clear current input when opening
        if console_state.is_visible {
            console_state.current_input.clear();
            console_state.cursor_position = 0;
        }
    }
}

/// System to handle console input when visible
pub fn console_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut console_state: ResMut<ConsoleState>,
    mut console_history: ResMut<ConsoleHistory>,
    mut command_writer: EventWriter<ConsoleCommand>,
    mut text_input: EventReader<bevy::input::keyboard::KeyboardInput>,
    time: Res<Time>,
) {
    if !console_state.is_visible {
        return;
    }
    
    // Handle special keys
    if keyboard.just_pressed(KeyCode::Enter) {
        execute_command(
            &mut console_state,
            &mut console_history,
            &mut command_writer,
            time.elapsed_secs_f64(),
        );
        return;
    }
    
    if keyboard.just_pressed(KeyCode::Backspace) {
        if console_state.cursor_position > 0 {
            console_state.cursor_position -= 1;
            let pos = console_state.cursor_position;
            console_state.current_input.remove(pos);
        }
        return;
    }
    
    if keyboard.just_pressed(KeyCode::Delete) {
        let pos = console_state.cursor_position;
        if pos < console_state.current_input.len() {
            console_state.current_input.remove(pos);
        }
        return;
    }
    
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        if console_state.cursor_position > 0 {
            console_state.cursor_position -= 1;
        }
        return;
    }
    
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        if console_state.cursor_position < console_state.current_input.len() {
            console_state.cursor_position += 1;
        }
        return;
    }
    
    if keyboard.just_pressed(KeyCode::Home) {
        console_state.cursor_position = 0;
        return;
    }
    
    if keyboard.just_pressed(KeyCode::End) {
        console_state.cursor_position = console_state.current_input.len();
        return;
    }
    
    // Handle command history navigation
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        if let Some(previous) = console_history.get_previous() {
            console_state.current_input = previous.clone();
            console_state.cursor_position = console_state.current_input.len();
        }
        return;
    }
    
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        if let Some(next) = console_history.get_next() {
            console_state.current_input = next.clone();
        } else {
            console_state.current_input.clear();
        }
        console_state.cursor_position = console_state.current_input.len();
        return;
    }
    
    // Handle text input events for character input
    for event in text_input.read() {
        if event.state == bevy::input::ButtonState::Pressed {
            // Convert key code to character if it's a printable character
            if let Some(character) = key_code_to_char(event.key_code, keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight)) {
                // Insert character at cursor position
                let pos = console_state.cursor_position;
                console_state.current_input.insert(pos, character);
                console_state.cursor_position += 1;
            }
        }
    }
}

/// Execute the current command
fn execute_command(
    console_state: &mut ConsoleState,
    console_history: &mut ConsoleHistory,
    command_writer: &mut EventWriter<ConsoleCommand>,
    timestamp: f64,
) {
    let command_text = console_state.current_input.trim();
    
    if command_text.is_empty() {
        return;
    }
    
    // Add to console output
    let mut command_line = ConsoleLine::command(command_text);
    command_line.timestamp = timestamp;
    console_state.output_lines.push(command_line);
    
    // Add to history
    console_history.add_command(command_text.to_string());
    
    // Parse and send command event
    let parts: Vec<String> = command_text
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    
    if !parts.is_empty() {
        let command = parts[0].clone();
        let args = parts[1..].to_vec();
        
        command_writer.write(ConsoleCommand { command, args });
    }
    
    // Clear input
    console_state.current_input.clear();
    console_state.cursor_position = 0;
    
    // Limit output lines to prevent memory bloat
    if console_state.output_lines.len() > 1000 {
        console_state.output_lines.drain(0..100);
    }
}

/// System to render the console UI
pub fn console_ui_system(
    mut commands: Commands,
    console_state: Res<ConsoleState>,
    existing_console: Query<Entity, With<DevConsoleRoot>>,
) {
    // Remove existing console UI if visibility changed
    for entity in &existing_console {
        commands.entity(entity).despawn();
    }
    
    if !console_state.is_visible {
        return;
    }
    
    // Create console UI
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Percent(50.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        DevConsoleRoot,
    ))
    .with_children(|parent| {
        // Output area
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(85.0),
                flex_direction: FlexDirection::Column,
                overflow: Overflow::clip_y(),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
        ))
        .with_children(|parent| {
            // Console output lines
            for line in console_state.output_lines.iter().rev().take(50).rev() {
                let color = match line.line_type {
                    ConsoleLineType::Command => Color::srgb(0.8, 0.8, 1.0),
                    ConsoleLineType::Output => Color::srgb(1.0, 1.0, 1.0),
                    ConsoleLineType::Error => Color::srgb(1.0, 0.4, 0.4),
                    ConsoleLineType::Warning => Color::srgb(1.0, 1.0, 0.4),
                    ConsoleLineType::System => Color::srgb(0.4, 1.0, 0.4),
                };
                
                parent.spawn((
                    Text::new(&line.text),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(color),
                ));
            }
        });
        
        // Input area
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(15.0),
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.9)),
        ))
        .with_children(|parent| {
            // Input prompt
            parent.spawn((
                Text::new("> "),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 1.0)),
            ));
            
            // Input text with cursor
            let input_with_cursor = if console_state.cursor_position <= console_state.current_input.len() {
                let mut text = console_state.current_input.clone();
                text.insert(console_state.cursor_position, '|');
                text
            } else {
                format!("{}|", console_state.current_input)
            };
            
            parent.spawn((
                Text::new(&input_with_cursor),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
            ));
        });
    });
}

/// Marker component for console root
#[derive(Component)]
pub struct DevConsoleRoot;

/// Convert key code to character for text input
fn key_code_to_char(key_code: KeyCode, shift_pressed: bool) -> Option<char> {
    match key_code {
        // Letters
        KeyCode::KeyA => Some(if shift_pressed { 'A' } else { 'a' }),
        KeyCode::KeyB => Some(if shift_pressed { 'B' } else { 'b' }),
        KeyCode::KeyC => Some(if shift_pressed { 'C' } else { 'c' }),
        KeyCode::KeyD => Some(if shift_pressed { 'D' } else { 'd' }),
        KeyCode::KeyE => Some(if shift_pressed { 'E' } else { 'e' }),
        KeyCode::KeyF => Some(if shift_pressed { 'F' } else { 'f' }),
        KeyCode::KeyG => Some(if shift_pressed { 'G' } else { 'g' }),
        KeyCode::KeyH => Some(if shift_pressed { 'H' } else { 'h' }),
        KeyCode::KeyI => Some(if shift_pressed { 'I' } else { 'i' }),
        KeyCode::KeyJ => Some(if shift_pressed { 'J' } else { 'j' }),
        KeyCode::KeyK => Some(if shift_pressed { 'K' } else { 'k' }),
        KeyCode::KeyL => Some(if shift_pressed { 'L' } else { 'l' }),
        KeyCode::KeyM => Some(if shift_pressed { 'M' } else { 'm' }),
        KeyCode::KeyN => Some(if shift_pressed { 'N' } else { 'n' }),
        KeyCode::KeyO => Some(if shift_pressed { 'O' } else { 'o' }),
        KeyCode::KeyP => Some(if shift_pressed { 'P' } else { 'p' }),
        KeyCode::KeyQ => Some(if shift_pressed { 'Q' } else { 'q' }),
        KeyCode::KeyR => Some(if shift_pressed { 'R' } else { 'r' }),
        KeyCode::KeyS => Some(if shift_pressed { 'S' } else { 's' }),
        KeyCode::KeyT => Some(if shift_pressed { 'T' } else { 't' }),
        KeyCode::KeyU => Some(if shift_pressed { 'U' } else { 'u' }),
        KeyCode::KeyV => Some(if shift_pressed { 'V' } else { 'v' }),
        KeyCode::KeyW => Some(if shift_pressed { 'W' } else { 'w' }),
        KeyCode::KeyX => Some(if shift_pressed { 'X' } else { 'x' }),
        KeyCode::KeyY => Some(if shift_pressed { 'Y' } else { 'y' }),
        KeyCode::KeyZ => Some(if shift_pressed { 'Z' } else { 'z' }),
        
        // Numbers
        KeyCode::Digit0 => Some(if shift_pressed { ')' } else { '0' }),
        KeyCode::Digit1 => Some(if shift_pressed { '!' } else { '1' }),
        KeyCode::Digit2 => Some(if shift_pressed { '@' } else { '2' }),
        KeyCode::Digit3 => Some(if shift_pressed { '#' } else { '3' }),
        KeyCode::Digit4 => Some(if shift_pressed { '$' } else { '4' }),
        KeyCode::Digit5 => Some(if shift_pressed { '%' } else { '5' }),
        KeyCode::Digit6 => Some(if shift_pressed { '^' } else { '6' }),
        KeyCode::Digit7 => Some(if shift_pressed { '&' } else { '7' }),
        KeyCode::Digit8 => Some(if shift_pressed { '*' } else { '8' }),
        KeyCode::Digit9 => Some(if shift_pressed { '(' } else { '9' }),
        
        // Special characters
        KeyCode::Space => Some(' '),
        KeyCode::Minus => Some(if shift_pressed { '_' } else { '-' }),
        KeyCode::Equal => Some(if shift_pressed { '+' } else { '=' }),
        KeyCode::BracketLeft => Some(if shift_pressed { '{' } else { '[' }),
        KeyCode::BracketRight => Some(if shift_pressed { '}' } else { ']' }),
        KeyCode::Backslash => Some(if shift_pressed { '|' } else { '\\' }),
        KeyCode::Semicolon => Some(if shift_pressed { ':' } else { ';' }),
        KeyCode::Quote => Some(if shift_pressed { '"' } else { '\'' }),
        KeyCode::Comma => Some(if shift_pressed { '<' } else { ',' }),
        KeyCode::Period => Some(if shift_pressed { '>' } else { '.' }),
        KeyCode::Slash => Some(if shift_pressed { '?' } else { '/' }),
        
        _ => None,
    }
}