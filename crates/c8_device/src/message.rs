use crate::display::DisplayResolution;

/// Device messages
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceMessage {
    /// Request to change the display resolution
    ChangeResolution(DisplayResolution),

    /// Unknown OpCode
    UnknownOpCode(u16),

    /// Waiting for a key to be pressed
    WaitingForKey(Option<usize>),
}
