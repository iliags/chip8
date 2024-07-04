use bevy::prelude::{ButtonInput, KeyCode, Res, *};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::KeysPressed;

// Key mappings for the CHIP-8 keypad
// 1 2 3 4 -> 1 2 3 C
// Q W E R -> 4 5 6 D
// A S D F -> 7 8 9 E
// Z X C V -> A 0 B F
#[derive(Debug, EnumIter)]
pub enum InputEvent {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_events);
    }
}

fn keyboard_events(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut keys_pressed: ResMut<KeysPressed>,
) {
    for input_event in InputEvent::iter() {
        keys_pressed.keys[input_event as usize] = input_event.pressed(&keyboard_input);
    }
}

impl InputEvent {
    pub fn pressed(&self, keyboard_input: &Res<ButtonInput<KeyCode>>) -> bool {
        match self {
            InputEvent::Key0 => keyboard_input.pressed(KeyCode::KeyX),
            InputEvent::Key1 => keyboard_input.pressed(KeyCode::Digit1),
            InputEvent::Key2 => keyboard_input.pressed(KeyCode::Digit2),
            InputEvent::Key3 => keyboard_input.pressed(KeyCode::Digit3),
            InputEvent::Key4 => keyboard_input.pressed(KeyCode::KeyQ),
            InputEvent::Key5 => keyboard_input.pressed(KeyCode::KeyW),
            InputEvent::Key6 => keyboard_input.pressed(KeyCode::KeyE),
            InputEvent::Key7 => keyboard_input.pressed(KeyCode::KeyA),
            InputEvent::Key8 => keyboard_input.pressed(KeyCode::KeyS),
            InputEvent::Key9 => keyboard_input.pressed(KeyCode::KeyD),
            InputEvent::KeyA => keyboard_input.pressed(KeyCode::KeyZ),
            InputEvent::KeyB => keyboard_input.pressed(KeyCode::KeyC),
            InputEvent::KeyC => keyboard_input.pressed(KeyCode::Digit4),
            InputEvent::KeyD => keyboard_input.pressed(KeyCode::KeyR),
            InputEvent::KeyE => keyboard_input.pressed(KeyCode::KeyF),
            InputEvent::KeyF => keyboard_input.pressed(KeyCode::KeyV),
        }
    }
}
