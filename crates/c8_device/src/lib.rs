//! Chip-8 device library

/// Chip-8 audio
pub mod audio;

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

/// System messages
pub mod message;

/// Program entry point
pub const PROGRAM_START: u16 = 0x200;

#[allow(missing_docs)]
#[macro_export]
macro_rules! profile_scope {
    () => {
        #[cfg(feature = "enable_puffin")]
        puffin::profile_scope!("scope");
    };
    ($name:expr) => {
        #[cfg(feature = "enable_puffin")]
        puffin::profile_scope!($name);
    };
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! profile_function {
    () => {
        #[cfg(feature = "enable_puffin")]
        puffin::profile_function!();
    };
}
