#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyboardKey {
    Num1,
    Num2,
    Num3,
    C,
    Num4,
    Num5,
    Num6,
    D,
    Num7,
    Num8,
    Num9,
    E,
    A,
    Num0,
    B,
    F,
}

/// Get the numeric index of a key
pub fn get_key_index(key: &KeyboardKey) -> usize {
    match key {
        KeyboardKey::Num1 => 0x1,
        KeyboardKey::Num2 => 0x2,
        KeyboardKey::Num3 => 0x3,
        KeyboardKey::C => 0xC,
        KeyboardKey::Num4 => 0x4,
        KeyboardKey::Num5 => 0x5,
        KeyboardKey::Num6 => 0x6,
        KeyboardKey::D => 0xD,
        KeyboardKey::Num7 => 0x7,
        KeyboardKey::Num8 => 0x8,
        KeyboardKey::Num9 => 0x9,
        KeyboardKey::E => 0xE,
        KeyboardKey::A => 0xA,
        KeyboardKey::Num0 => 0x0,
        KeyboardKey::B => 0xB,
        KeyboardKey::F => 0xF,
    }
}
