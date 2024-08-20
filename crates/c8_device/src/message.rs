use crate::display::DisplayResolution;

/// Device messages
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceMessage {
    /// Request to change the display resolution
    ChangeResolution(DisplayResolution),
    /// Exit the device
    Exit,
    /// Unknown OpCode
    UnknownOpCode(u16),

    /// Waiting for a key to be pressed
    WaitingForKey(Option<usize>),

    /// Beep
    Beep(u8),

    /// Set the pitch of the beeper
    SetPitch(f64),

    /// New audio buffer created
    NewAudioBuffer,
}
