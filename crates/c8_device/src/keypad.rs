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
        *self as usize
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
        self.0[key.get_key_index()] = state as u8;
    }

    /// Get the state of a key
    pub fn get_key(&self, key: &KeypadKey) -> u8 {
        self.0[key.get_key_index()]
    }

    /// Get the state of a key as a boolean
    pub fn is_key_pressed(&self, key: &KeypadKey) -> bool {
        self.get_key(key) == 1
    }

    /// Get the underlying key array
    pub fn get_keys(&self) -> &[u8; 16] {
        &self.0
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
        assert_eq!(KeypadKey::Num1.get_key_index(), 0);
        assert_eq!(KeypadKey::Num2.get_key_index(), 1);
        assert_eq!(KeypadKey::Num3.get_key_index(), 2);
        assert_eq!(KeypadKey::C.get_key_index(), 3);
        assert_eq!(KeypadKey::Num4.get_key_index(), 4);
        assert_eq!(KeypadKey::Num5.get_key_index(), 5);
        assert_eq!(KeypadKey::Num6.get_key_index(), 6);
        assert_eq!(KeypadKey::D.get_key_index(), 7);
        assert_eq!(KeypadKey::Num7.get_key_index(), 8);
        assert_eq!(KeypadKey::Num8.get_key_index(), 9);
        assert_eq!(KeypadKey::Num9.get_key_index(), 10);
        assert_eq!(KeypadKey::E.get_key_index(), 11);
        assert_eq!(KeypadKey::A.get_key_index(), 12);
        assert_eq!(KeypadKey::Num0.get_key_index(), 13);
        assert_eq!(KeypadKey::B.get_key_index(), 14);
        assert_eq!(KeypadKey::F.get_key_index(), 15);
    }

    #[test]
    fn test_from_guard() {
        assert_eq!(KeypadKey::from(0) == KeypadKey::Num0, true);
        assert_eq!(KeypadKey::from(66) == KeypadKey::F, true);
    }
}
