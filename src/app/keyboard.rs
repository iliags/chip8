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

/// Get the numeric index of a key
pub const fn get_key_index(key: &Key) -> Option<i32> {
    let key_index = match key {
        Key::Num1 => 0x1,
        Key::Num2 => 0x2,
        Key::Num3 => 0x3,
        Key::Num4 => 0xC,
        Key::Q => 0x4,
        Key::W => 0x5,
        Key::E => 0x6,
        Key::R => 0xD,
        Key::A => 0x7,
        Key::S => 0x8,
        Key::D => 0x9,
        Key::F => 0xE,
        Key::Z => 0xA,
        Key::X => 0x0,
        Key::C => 0xB,
        Key::V => 0xF,
        _ => return None,
    };

    Some(key_index)
}

/// Get the name of the key for the UI
pub const fn get_key_name(key: &Key) -> &str {
    match key {
        Key::Num1 => "1",
        Key::Num2 => "2",
        Key::Num3 => "3",
        Key::Num4 => "C",
        Key::Q => "4",
        Key::W => "5",
        Key::E => "6",
        Key::R => "D",
        Key::A => "7",
        Key::S => "8",
        Key::D => "9",
        Key::F => "E",
        Key::Z => "A",
        Key::X => "0",
        Key::C => "B",
        Key::V => "F",
        _ => "None",
    }
}
