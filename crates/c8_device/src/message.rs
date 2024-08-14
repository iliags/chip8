use crate::display::DisplayResolution;

/// Device messages
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceMessage {
    /// Request to change the display resolution
    ChangeResolution(DisplayResolution),
    /// Exit the device
    Exit,
}
