#![allow(dead_code)]
/// Font data, the small Chip-8 font is the default and fallback font.
///
/// The non-default Chip-8 font was copied from Octo's font file.

// Deprecated, remains for compatibility until fonts are implemented
//#[deprecated(note = "please use `SMALL_FONTS` instead")]
pub const FONT: &[u8] = FONT_DATA[FontName::CHIP8 as usize].small_data;

/// Static font data
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct FontData {
    pub name: FontName,
    pub small_data: &'static [u8],
    pub large_data: &'static [u8],
}

/// Font name accessors
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FontName {
    CHIP8 = 0,
    VIP = 1,
    DREAM6800 = 2,
    ETI660 = 3,
    FISHIE = 4,
    SUPERCHIP = 5,
}

impl Into<usize> for FontName {
    fn into(self) -> usize {
        self as usize
    }
}

// TODO: Get localized font names
impl Into<String> for FontName {
    fn into(self) -> String {
        match self {
            FontName::CHIP8 => "CHIP-8".to_string(),
            FontName::VIP => "VIP".to_string(),
            FontName::DREAM6800 => "DREAM 6800".to_string(),
            FontName::ETI660 => "ETI 660".to_string(),
            FontName::FISHIE => "FISHIE".to_string(),
            FontName::SUPERCHIP => "SUPER-CHIP".to_string(),
        }
    }
}

impl FontName {}

/// Font sizes
#[allow(missing_docs)]
pub enum FontSize {
    Small,
    Large,
}

/// Font data collection
pub const FONT_DATA: &[FontData] = &[
    FontData {
        name: FontName::CHIP8,
        small_data: &[
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ],
        large_data: &[
            0xFF, 0xFF, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xFF, 0xFF, // 0
            0x18, 0x78, 0x78, 0x18, 0x18, 0x18, 0x18, 0x18, 0xFF, 0xFF, // 1
            0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, // 2
            0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, // 3
            0xC3, 0xC3, 0xC3, 0xC3, 0xFF, 0xFF, 0x03, 0x03, 0x03, 0x03, // 4
            0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, // 5
            0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, // 6
            0xFF, 0xFF, 0x03, 0x03, 0x06, 0x0C, 0x18, 0x18, 0x18, 0x18, // 7
            0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, // 8
            0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, // 9
            0x7E, 0xFF, 0xC3, 0xC3, 0xC3, 0xFF, 0xFF, 0xC3, 0xC3, 0xC3, // A
            0xFC, 0xFC, 0xC3, 0xC3, 0xFC, 0xFC, 0xC3, 0xC3, 0xFC, 0xFC, // B
            0x3C, 0xFF, 0xC3, 0xC0, 0xC0, 0xC0, 0xC0, 0xC3, 0xFF, 0x3C, // C
            0xFC, 0xFE, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xFE, 0xFC, // D
            0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, // E
            0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC0, 0xC0, 0xC0, 0xC0, // F
        ],
    },
    FontData {
        name: FontName::VIP,
        small_data: &[
            0xF0, 0x90, 0x90, 0x90, 0xF0, 0x60, 0x20, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80,
            0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0xA0, 0xA0, 0xF0, 0x20, 0x20, 0xF0, 0x80, 0xF0,
            0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x10, 0x10, 0x10, 0xF0, 0x90,
            0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xF0,
            0x50, 0x70, 0x50, 0xF0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xF0, 0x50, 0x50, 0x50, 0xF0,
            0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
        ],
        large_data: &[],
    },
    FontData {
        name: FontName::DREAM6800,
        small_data: &[
            0xE0, 0xA0, 0xA0, 0xA0, 0xE0, 0x40, 0x40, 0x40, 0x40, 0x40, 0xE0, 0x20, 0xE0, 0x80,
            0xE0, 0xE0, 0x20, 0xE0, 0x20, 0xE0, 0x80, 0xA0, 0xA0, 0xE0, 0x20, 0xE0, 0x80, 0xE0,
            0x20, 0xE0, 0xE0, 0x80, 0xE0, 0xA0, 0xE0, 0xE0, 0x20, 0x20, 0x20, 0x20, 0xE0, 0xA0,
            0xE0, 0xA0, 0xE0, 0xE0, 0xA0, 0xE0, 0x20, 0xE0, 0xE0, 0xA0, 0xE0, 0xA0, 0xA0, 0xC0,
            0xA0, 0xE0, 0xA0, 0xC0, 0xE0, 0x80, 0x80, 0x80, 0xE0, 0xC0, 0xA0, 0xA0, 0xA0, 0xC0,
            0xE0, 0x80, 0xE0, 0x80, 0xE0, 0xE0, 0x80, 0xC0, 0x80, 0x80,
        ],
        large_data: &[],
    },
    FontData {
        name: FontName::ETI660,
        small_data: &[
            0xE0, 0xA0, 0xA0, 0xA0, 0xE0, 0x20, 0x20, 0x20, 0x20, 0x20, 0xE0, 0x20, 0xE0, 0x80,
            0xE0, 0xE0, 0x20, 0xE0, 0x20, 0xE0, 0xA0, 0xA0, 0xE0, 0x20, 0x20, 0xE0, 0x80, 0xE0,
            0x20, 0xE0, 0xE0, 0x80, 0xE0, 0xA0, 0xE0, 0xE0, 0x20, 0x20, 0x20, 0x20, 0xE0, 0xA0,
            0xE0, 0xA0, 0xE0, 0xE0, 0xA0, 0xE0, 0x20, 0xE0, 0xE0, 0xA0, 0xE0, 0xA0, 0xA0, 0x80,
            0x80, 0xE0, 0xA0, 0xE0, 0xE0, 0x80, 0x80, 0x80, 0xE0, 0x20, 0x20, 0xE0, 0xA0, 0xE0,
            0xE0, 0x80, 0xE0, 0x80, 0xE0, 0xE0, 0x80, 0xC0, 0x80, 0x80,
        ],
        large_data: &[],
    },
    FontData {
        name: FontName::FISHIE,
        small_data: &[
            0x60, 0xA0, 0xA0, 0xA0, 0xC0, 0x40, 0xC0, 0x40, 0x40, 0xE0, 0xC0, 0x20, 0x40, 0x80,
            0xE0, 0xC0, 0x20, 0x40, 0x20, 0xC0, 0x20, 0xA0, 0xE0, 0x20, 0x20, 0xE0, 0x80, 0xC0,
            0x20, 0xC0, 0x40, 0x80, 0xC0, 0xA0, 0x40, 0xE0, 0x20, 0x60, 0x40, 0x40, 0x40, 0xA0,
            0x40, 0xA0, 0x40, 0x40, 0xA0, 0x60, 0x20, 0x40, 0x40, 0xA0, 0xE0, 0xA0, 0xA0, 0xC0,
            0xA0, 0xC0, 0xA0, 0xC0, 0x60, 0x80, 0x80, 0x80, 0x60, 0xC0, 0xA0, 0xA0, 0xA0, 0xC0,
            0xE0, 0x80, 0xC0, 0x80, 0xE0, 0xE0, 0x80, 0xC0, 0x80, 0x80,
        ],
        // At most 7x9 pixels
        large_data: &[
            0x7C, 0xC6, 0xCE, 0xDE, 0xD6, 0xF6, 0xE6, 0xC6, 0x7C, 0x00, 0x10, 0x30, 0xF0, 0x30,
            0x30, 0x30, 0x30, 0x30, 0xFC, 0x00, 0x78, 0xCC, 0xCC, 0x0C, 0x18, 0x30, 0x60, 0xCC,
            0xFC, 0x00, 0x78, 0xCC, 0x0C, 0x0C, 0x38, 0x0C, 0x0C, 0xCC, 0x78, 0x00, 0x0C, 0x1C,
            0x3C, 0x6C, 0xCC, 0xFE, 0x0C, 0x0C, 0x1E, 0x00, 0xFC, 0xC0, 0xC0, 0xC0, 0xF8, 0x0C,
            0x0C, 0xCC, 0x78, 0x00, 0x38, 0x60, 0xC0, 0xC0, 0xF8, 0xCC, 0xCC, 0xCC, 0x78, 0x00,
            0xFE, 0xC6, 0xC6, 0x06, 0x0C, 0x18, 0x30, 0x30, 0x30, 0x00, 0x78, 0xCC, 0xCC, 0xEC,
            0x78, 0xDC, 0xCC, 0xCC, 0x78, 0x00, 0x7C, 0xC6, 0xC6, 0xC6, 0x7C, 0x18, 0x18, 0x30,
            0x70, 0x00, 0x30, 0x78, 0xCC, 0xCC, 0xCC, 0xFC, 0xCC, 0xCC, 0xCC, 0x00, 0xFC, 0x66,
            0x66, 0x66, 0x7C, 0x66, 0x66, 0x66, 0xFC, 0x00, 0x3C, 0x66, 0xC6, 0xC0, 0xC0, 0xC0,
            0xC6, 0x66, 0x3C, 0x00, 0xF8, 0x6C, 0x66, 0x66, 0x66, 0x66, 0x66, 0x6C, 0xF8, 0x00,
            0xFE, 0x62, 0x60, 0x64, 0x7C, 0x64, 0x60, 0x62, 0xFE, 0x00, 0xFE, 0x66, 0x62, 0x64,
            0x7C, 0x64, 0x60, 0x60, 0xF0, 0x00,
        ],
    },
    FontData {
        name: FontName::SUPERCHIP,
        small_data: &[],
        large_data: &[
            0x3C, 0x7E, 0xE7, 0xC3, 0xC3, 0xC3, 0xC3, 0xE7, 0x7E, 0x3C, 0x18, 0x38, 0x58, 0x18,
            0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x3E, 0x7F, 0xC3, 0x06, 0x0C, 0x18, 0x30, 0x60,
            0xFF, 0xFF, 0x3C, 0x7E, 0xC3, 0x03, 0x0E, 0x0E, 0x03, 0xC3, 0x7E, 0x3C, 0x06, 0x0E,
            0x1E, 0x36, 0x66, 0xC6, 0xFF, 0xFF, 0x06, 0x06, 0xFF, 0xFF, 0xC0, 0xC0, 0xFC, 0xFE,
            0x03, 0xC3, 0x7E, 0x3C, 0x3E, 0x7C, 0xE0, 0xC0, 0xFC, 0xFE, 0xC3, 0xC3, 0x7E, 0x3C,
            0xFF, 0xFF, 0x03, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x60, 0x60, 0x3C, 0x7E, 0xC3, 0xC3,
            0x7E, 0x7E, 0xC3, 0xC3, 0x7E, 0x3C, 0x3C, 0x7E, 0xC3, 0xC3, 0x7F, 0x3F, 0x03, 0x03,
            0x3E, 0x7C, // No hex chars
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],
    },
];
