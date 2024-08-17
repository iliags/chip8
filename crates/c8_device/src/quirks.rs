/// Quirks for the Chip-8 device
// Note: this could be implemented as a bitfield in the future
#[derive(Debug, Clone, Copy)]
pub struct Quirks {
    /// Quirk: Some programs expect VF to be 0
    pub vf_zero: bool,

    /// Quirk: Some programs expect I to be incremented when performing certain operations
    pub i_incremented: bool,

    /// Quirk: Some programs expect VX to be shifted directly without assigning VY
    pub vx_shifted_directly: bool,

    /// Quirk: Wait for the display to finish drawing before continuing, caps drawing at 60 sprites per second
    /// Not implemented
    pub display_waiting: bool,

    /// Quirk: Clip sprites vertically to the display, instead of wrapping around the edges
    /// Not implemented
    pub clip_sprites: bool,
}

impl Default for Quirks {
    fn default() -> Self {
        Self {
            vf_zero: true,
            //i_incremented: true,
            i_incremented: false,
            vx_shifted_directly: true,
            display_waiting: false,
            clip_sprites: false,
        }
    }
}
