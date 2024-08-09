/// Keypad definitions for the Chip 8 buttons
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeypadKey {
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

impl KeypadKey {
    /// Get the index of the key
    pub const fn get_key_index(&self) -> usize {
        match &self {
            KeypadKey::Num1 => 0x1,
            KeypadKey::Num2 => 0x2,
            KeypadKey::Num3 => 0x3,
            KeypadKey::C => 0xC,
            KeypadKey::Num4 => 0x4,
            KeypadKey::Num5 => 0x5,
            KeypadKey::Num6 => 0x6,
            KeypadKey::D => 0xD,
            KeypadKey::Num7 => 0x7,
            KeypadKey::Num8 => 0x8,
            KeypadKey::Num9 => 0x9,
            KeypadKey::E => 0xE,
            KeypadKey::A => 0xA,
            KeypadKey::Num0 => 0x0,
            KeypadKey::B => 0xB,
            KeypadKey::F => 0xF,
        }
    }

    /// Get the name of the key for UI purposes
    pub const fn get_name(&self) -> &str {
        match &self {
            KeypadKey::Num1 => "1",
            KeypadKey::Num2 => "2",
            KeypadKey::Num3 => "3",
            KeypadKey::C => "C",
            KeypadKey::Num4 => "4",
            KeypadKey::Num5 => "5",
            KeypadKey::Num6 => "6",
            KeypadKey::D => "D",
            KeypadKey::Num7 => "7",
            KeypadKey::Num8 => "8",
            KeypadKey::Num9 => "9",
            KeypadKey::E => "E",
            KeypadKey::A => "A",
            KeypadKey::Num0 => "0",
            KeypadKey::B => "B",
            KeypadKey::F => "F",
        }
    }
}
