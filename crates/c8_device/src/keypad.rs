/// Keypad definitions for the Chip 8 buttons
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeypadKey {
    Num1 = 0x1,
    Num2 = 0x2,
    Num3 = 0x3,
    C = 0xC,
    Num4 = 0x4,
    Num5 = 0x5,
    Num6 = 0x6,
    D = 0xD,
    Num7 = 0x7,
    Num8 = 0x8,
    Num9 = 0x9,
    E = 0xE,
    A = 0xA,
    Num0 = 0x0,
    B = 0xB,
    F = 0xF,
}

/// Keypad keys on the chip 8
pub const KEYPAD_KEYS: [KeypadKey; 16] = [
    KeypadKey::Num1,
    KeypadKey::Num2,
    KeypadKey::Num3,
    KeypadKey::C,
    KeypadKey::Num4,
    KeypadKey::Num5,
    KeypadKey::Num6,
    KeypadKey::D,
    KeypadKey::Num7,
    KeypadKey::Num8,
    KeypadKey::Num9,
    KeypadKey::E,
    KeypadKey::A,
    KeypadKey::Num0,
    KeypadKey::B,
    KeypadKey::F,
];

impl KeypadKey {
    /// Get the index of the key
    pub const fn key_index(&self) -> usize {
        *self as usize
    }

    /// Get the name of the key for UI purposes
    pub const fn name(&self) -> &str {
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

impl From<usize> for KeypadKey {
    fn from(value: usize) -> Self {
        match value.clamp(0, 15) {
            0x1 => KeypadKey::Num1,
            0x2 => KeypadKey::Num2,
            0x3 => KeypadKey::Num3,
            0xC => KeypadKey::C,
            0x4 => KeypadKey::Num4,
            0x5 => KeypadKey::Num5,
            0x6 => KeypadKey::Num6,
            0xD => KeypadKey::D,
            0x7 => KeypadKey::Num7,
            0x8 => KeypadKey::Num8,
            0x9 => KeypadKey::Num9,
            0xE => KeypadKey::E,
            0xA => KeypadKey::A,
            0x0 => KeypadKey::Num0,
            0xB => KeypadKey::B,
            0xF => KeypadKey::F,
            _ => unreachable!(),
        }
    }
}

/// Current state of the keypad
#[derive(Debug, PartialEq, Default)]
pub struct Keypad([u8; 16]);

impl Keypad {
    /// Set the state of a key
    pub fn set_key(&mut self, key: &KeypadKey, state: bool) {
        self.0[key.key_index()] = state as u8;
    }

    /// Get the state of a key
    pub fn key(&self, key: &KeypadKey) -> u8 {
        self.0[key.key_index()]
    }

    /// Get the state of a key as a boolean
    pub fn is_key_pressed(&self, key: &KeypadKey) -> bool {
        self.key(key) == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getset_key() {
        let mut keypad = Keypad::default();

        // Test key down
        keypad.set_key(&KeypadKey::Num1, true);
        assert_eq!(keypad.is_key_pressed(&KeypadKey::Num1), true);

        // Test key up
        keypad.set_key(&KeypadKey::Num1, false);
        assert_eq!(keypad.is_key_pressed(&KeypadKey::Num1), false);
    }

    #[test]
    fn test_key_index() {
        assert_eq!(KeypadKey::Num1.key_index(), 0x1);
        assert_eq!(KeypadKey::Num2.key_index(), 0x2);
        assert_eq!(KeypadKey::Num3.key_index(), 0x3);
        assert_eq!(KeypadKey::C.key_index(), 0xC);
        assert_eq!(KeypadKey::Num4.key_index(), 0x4);
        assert_eq!(KeypadKey::Num5.key_index(), 0x5);
        assert_eq!(KeypadKey::Num6.key_index(), 0x6);
        assert_eq!(KeypadKey::D.key_index(), 0xD);
        assert_eq!(KeypadKey::Num7.key_index(), 0x7);
        assert_eq!(KeypadKey::Num8.key_index(), 0x8);
        assert_eq!(KeypadKey::Num9.key_index(), 0x9);
        assert_eq!(KeypadKey::E.key_index(), 0xE);
        assert_eq!(KeypadKey::A.key_index(), 0xA);
        assert_eq!(KeypadKey::Num0.key_index(), 0x0);
        assert_eq!(KeypadKey::B.key_index(), 0xB);
        assert_eq!(KeypadKey::F.key_index(), 0xF);
    }

    #[test]
    fn test_from_guard() {
        assert_eq!(KeypadKey::from(0) == KeypadKey::Num0, true);
        assert_eq!(KeypadKey::from(66) == KeypadKey::F, true);
    }
}
