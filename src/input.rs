//use bevy::prelude::{ButtonInput, KeyCode, Res};

// Key mappings for the CHIP-8 keypad
// 1 2 3 4 -> 1 2 3 C
// Q W E R -> 4 5 6 D
// A S D F -> 7 8 9 E
// Z X C V -> A 0 B F

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
