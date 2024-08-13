use std::vec;

use crate::{
    fonts::{FontData, FontName, FontSize, FONT_DATA},
    PROGRAM_START,
};

// Note: The extended option changes the memory size from 4096 to 65536.
// Changing the underlying container may be necessary if the memory usage is a concern.
const ENABLE_XO: bool = true;

const MAX_MEMORY: usize = if ENABLE_XO { 65536 } else { 4096 };

/// Device memory
#[derive(Debug)]
pub struct Memory {
    /// Memory data
    pub data: Vec<u8>,

    /// Default system font
    pub system_font: FontName,

    /// Enable extensions
    pub enable_xo: bool,
}

impl Default for Memory {
    fn default() -> Self {
        let mut new_self = Self {
            data: vec![0; MAX_MEMORY],
            system_font: FontName::CHIP8,
            enable_xo: false,
        };

        new_self.load_font_small(FONT_DATA[FontName::CHIP8 as usize].clone());

        new_self
    }
}

impl Memory {
    /// Load small font data into memory
    pub fn load_font_small(&mut self, data: FontData) {
        let start = 0;
        let end = data.small_data.len();

        self.data
            .splice(start..end, data.small_data.iter().cloned());
        self.system_font = data.name;
    }

    /// Load large font data into memory
    pub fn load_font_large(&mut self, data: FontData) {
        // Only load large font data if the XO extension is enabled
        if self.enable_xo {
            let small_font_length = FONT_DATA[self.system_font as usize].small_data.len();
            let start = small_font_length;
            let end = start + data.large_data.len();

            self.data
                .splice(start..end, data.large_data.iter().cloned());
        }
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
            _ => {}
        }

        let start = PROGRAM_START as usize;
        let end = start + data.len();

        self.data.splice(start..end, data.iter().cloned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_memory() {
        let memory = Memory::default();
        let font_data = FONT_DATA[FontName::CHIP8 as usize].small_data;

        // Check that the memory is the correct size
        assert_eq!(memory.data.len(), MAX_MEMORY);

        // Check that the font data is loaded
        for (i, &byte) in font_data.iter().enumerate() {
            assert_eq!(memory.data[i], byte);
        }

        // Check that the rest of the memory is zero
        for i in font_data.len()..MAX_MEMORY {
            assert_eq!(memory.data[i], 0);
        }
    }

    #[test]
    fn test_load_rom() {
        let mut memory = Memory::default();
        memory.load_rom(vec![0x00, 0xE0, 0x00, 0xEE]);

        assert_eq!(memory.data[0x200], 0x00);
        assert_eq!(memory.data[0x201], 0xE0);
        assert_eq!(memory.data[0x202], 0x00);
        assert_eq!(memory.data[0x203], 0xEE);
    }
}
