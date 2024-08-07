//! Chip-8 emulator

/// The UI and entry point of the emulator
pub mod app;

/// The Chip-8 device
pub mod device;

/// ROMs included with the emulator
pub mod roms;

/// Localization for the emulator
pub mod localization;

/// Assembler for Chip-8
pub mod asm;

/// Disassembler for Chip-8
pub mod dasm;
