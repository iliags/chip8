use c8::keypad::KeypadKey;
use egui::Key;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Deserialize, serde::Serialize)]
pub enum KeyMapping {
    #[default]
    Qwerty,
    Azerty,
}

impl KeyMapping {
    pub fn name(&self) -> &str {
        match self {
            KeyMapping::Qwerty => "Qwerty",
            KeyMapping::Azerty => "Azerty",
        }
    }
}

pub const KEY_MAPPINGS: &[KeyMapping] = &[KeyMapping::Qwerty, KeyMapping::Azerty];

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
    Key::ArrowUp,
    Key::ArrowDown,
    Key::ArrowLeft,
    Key::ArrowRight,
    Key::Space,
];

#[derive(Debug, Clone, Copy, PartialEq, Default, serde::Deserialize, serde::Serialize)]
pub struct KeyboardMapping {
    mapping: KeyMapping,
}

impl KeyboardMapping {
    /// Get the key mapping for the current input mapping
    pub fn key_from_mapping(&self, key: &Key) -> Option<KeypadKey> {
        match self.mapping {
            KeyMapping::Qwerty => self.key_mapping_qwerty(key),
            KeyMapping::Azerty => self.key_mapping_azerty(key),
        }
    }

    pub fn set_key_mapping(&mut self, mapping: KeyMapping) {
        self.mapping = mapping;
    }

    pub fn key_mapping(&self) -> KeyMapping {
        self.mapping
    }

    pub fn key_mapping_mut(&mut self) -> &mut KeyMapping {
        &mut self.mapping
    }

    pub fn key_mapping_name(&self) -> String {
        self.mapping.name().to_string()
    }

    pub fn is_extra_key(&self, key: &Key) -> bool {
        matches!(
            key,
            Key::ArrowUp | Key::ArrowDown | Key::ArrowLeft | Key::ArrowRight | Key::Space
        )
    }

    pub fn regular_key_from_extra_key(&self, key: &Key) -> Option<Key> {
        match key {
            Key::ArrowUp => Some(Key::W),
            Key::ArrowDown => Some(Key::S),
            Key::ArrowLeft => Some(Key::A),
            Key::ArrowRight => Some(Key::D),
            Key::Space => Some(Key::E),
            _ => None,
        }
    }

    const fn key_mapping_qwerty(&self, key: &Key) -> Option<KeypadKey> {
        let key_match = match key {
            Key::Num1 => KeypadKey::Num1,
            Key::Num2 => KeypadKey::Num2,
            Key::Num3 => KeypadKey::Num3,
            Key::Num4 => KeypadKey::C,
            Key::Q => KeypadKey::Num4,
            Key::W | Key::ArrowUp => KeypadKey::Num5,
            Key::E | Key::Space => KeypadKey::Num6,
            Key::R => KeypadKey::D,
            Key::A | Key::ArrowLeft => KeypadKey::Num7,
            Key::S | Key::ArrowDown => KeypadKey::Num8,
            Key::D | Key::ArrowRight => KeypadKey::Num9,
            Key::F => KeypadKey::E,
            Key::Z => KeypadKey::A,
            Key::X => KeypadKey::Num0,
            Key::C => KeypadKey::B,
            Key::V => KeypadKey::F,
            _ => return None,
        };

        Some(key_match)
    }

    const fn key_mapping_azerty(&self, key: &Key) -> Option<KeypadKey> {
        let key_match = match key {
            Key::Num1 => KeypadKey::Num1,
            Key::Num2 => KeypadKey::Num2,
            Key::Num3 => KeypadKey::Num3,
            Key::Num4 => KeypadKey::C,
            Key::A => KeypadKey::Num4,
            Key::Z | Key::ArrowUp => KeypadKey::Num5,
            Key::E | Key::Space => KeypadKey::Num6,
            Key::R => KeypadKey::D,
            Key::Q | Key::ArrowLeft => KeypadKey::Num7,
            Key::S | Key::ArrowDown => KeypadKey::Num8,
            Key::D | Key::ArrowRight => KeypadKey::Num9,
            Key::F => KeypadKey::E,
            Key::W => KeypadKey::A,
            Key::X => KeypadKey::Num0,
            Key::C => KeypadKey::B,
            Key::V => KeypadKey::F,
            _ => return None,
        };

        Some(key_match)
    }
}
