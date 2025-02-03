//! Chip-8 emulator

/// The UI and entry point of the emulator
pub mod app;

/// ROMs included with the emulator
pub mod roms;

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

#[macro_export]
macro_rules! profile_function {
    () => {
        #[cfg(feature = "enable_puffin")]
        puffin::profile_function!();
    };
}
