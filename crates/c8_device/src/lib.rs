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

/// Chip-8 Memory
pub mod memory;

/// Program entry point
pub const PROGRAM_START: u16 = 0x200;
