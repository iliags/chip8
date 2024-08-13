//! Chip-8 device library

/// Chip-8 CPU
pub mod cpu;

/// Chip-8 display
pub mod display;

/// Chip-8 device
pub mod device;

/// Chip-8 quirks
pub mod quirks;

/// Chip-8 keypad
pub mod keypad;

/// Font data
pub mod fonts;

/// Maximum memory size
pub const MAX_MEMORY: usize = 4096;

/// Program entry point
pub const PROGRAM_START: u16 = 0x200;

/// Maximum ROM size
pub const MAX_ROM_SIZE: usize = MAX_MEMORY - PROGRAM_START as usize;
