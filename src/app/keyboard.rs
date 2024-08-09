use c8_device::keypad::KeypadKey;
use egui::Key;

/// The keys on the keyboard that correspond to the Chip-8 keys
pub const KEYBOARD: &[Key] = &[
    Key::Num1,
    Key::Num2,
    Key::Num3,
    Key::Num4,
    Key::Q,
    Key::W,
    Key::E,
    Key::R,
    Key::A,
    Key::S,
    Key::D,
    Key::F,
    Key::Z,
    Key::X,
    Key::C,
    Key::V,
];

/// Get the key mapping for the current input mapping
pub const fn get_key_mapping(key: &Key) -> Option<KeypadKey> {
    let key_match = match key {
        Key::Num1 => KeypadKey::Num1,
        Key::Num2 => KeypadKey::Num2,
        Key::Num3 => KeypadKey::Num3,
        Key::Num4 => KeypadKey::C,
        Key::Q => KeypadKey::Num4,
        Key::W => KeypadKey::Num5,
        Key::E => KeypadKey::Num6,
        Key::R => KeypadKey::D,
        Key::A => KeypadKey::Num7,
        Key::S => KeypadKey::Num8,
        Key::D => KeypadKey::Num9,
        Key::F => KeypadKey::E,
        Key::Z => KeypadKey::A,
        Key::X => KeypadKey::Num0,
        Key::C => KeypadKey::B,
        Key::V => KeypadKey::F,
        _ => return None,
    };

    Some(key_match)
}
