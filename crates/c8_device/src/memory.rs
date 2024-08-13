use std::vec;

use crate::{
    fonts::{FontData, FontName, FontSize, FONT_DATA},
    PROGRAM_START,
};

/// Maximum memory size
pub const MAX_MEMORY: usize = 4096;

/// Maximum ROM size
pub const MAX_ROM_SIZE: usize = MAX_MEMORY - PROGRAM_START as usize;

/// Device memory
#[derive(Debug)]
pub struct Memory(pub Vec<u8>, pub FontName);

impl Default for Memory {
    fn default() -> Self {
        let mut new_self = Self(vec![0; MAX_MEMORY], FontName::CHIP8);

        new_self.load_font_small(FONT_DATA[FontName::CHIP8 as usize].clone());

        new_self
    }
}

impl Memory {
    /// Load small font data into memory
    pub fn load_font_small(&mut self, data: FontData) {
        let start = 0;
        let end = data.small_data.len();

        self.0.splice(start..end, data.small_data.iter().cloned());
        self.1 = data.name;
    }

    /// Load large font data into memory
    pub fn load_font_large(&mut self, _data: FontData) {
        println!("Large font data not implemented");
        //self.load_font_raw(data.large_data);
    }

    /// Load font data into memory
    pub fn load_font(&mut self, data: FontData, size: FontSize) {
        match size {
            FontSize::Small => self.load_font_small(data),
            FontSize::Large => self.load_font_large(data),
        }
    }

    /// Load font data into memory by name
    pub fn load_font_name(&mut self, name: FontName, size: FontSize) {
        self.load_font(FONT_DATA[name as usize].clone(), size);
    }

    /// Load ROM data into memory
    pub fn load_rom(&mut self, data: Vec<u8>) {
        // Make sure the ROM data is valid
        match data.len() {
            0 => {
                println!("No ROM data provided");
                return;
            }
            len if len > MAX_ROM_SIZE => {
                println!("ROM data is too large: {} bytes", len);
                return;
            }
            _ => {}
        }

        let start = PROGRAM_START as usize;
        let end = start + data.len();

        self.0.splice(start..end, data.iter().cloned());
    }
}

#[cfg(test)]
mod tests {
    use crate::fonts::FONT;

    use super::*;

    #[test]
    fn test_default_memory() {
        let memory = Memory::default();

        // Check that the memory is the correct size
        assert_eq!(memory.0.len(), MAX_MEMORY);

        // Check that the font data is loaded
        for (i, &byte) in FONT.iter().enumerate() {
            assert_eq!(memory.0[i], byte);
        }

        // Check that the rest of the memory is zero
        for i in FONT.len()..MAX_MEMORY {
            assert_eq!(memory.0[i], 0);
        }
    }

    #[test]
    fn test_load_rom() {
        let mut memory = Memory::default();
        memory.load_rom(vec![0x00, 0xE0, 0x00, 0xEE]);

        assert_eq!(memory.0[0x200], 0x00);
        assert_eq!(memory.0[0x201], 0xE0);
        assert_eq!(memory.0[0x202], 0x00);
        assert_eq!(memory.0[0x203], 0xEE);
    }
}
