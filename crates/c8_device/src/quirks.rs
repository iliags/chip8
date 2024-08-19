/// Quirks for the Chip-8 device
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Quirks {
    /// Quirk: Some programs expect VF to be 0
    /// Octo: compat-logic
    pub vf_zero: bool,

    /// Quirk: Some programs expect I to be incremented when performing certain operations
    /// Octo: compat-load
    pub i_incremented: bool,

    /// Quirk: Some programs expect VX to be shifted directly without assigning VY
    /// Octo: compat-shift
    pub vx_shifted_directly: bool,

    /// Quirk: Wait for the display to finish drawing before continuing, caps drawing at 60 sprites per second
    /// Not implemented
    /// Octo: compat-vblank
    pub v_blank: bool,

    /// Quirk: Clip sprites vertically to the display, instead of wrapping around the edges
    /// Octo: compat-clip
    pub clip_sprites: bool,

    /// Quirk: The 4 high bits of target address determines the offset register instead of V0
    /// Octo: compat-jump0
    pub jump_bits: bool,
}

impl Default for Quirks {
    fn default() -> Self {
        Self {
            vf_zero: false,
            i_incremented: false,
            vx_shifted_directly: false,
            v_blank: false,
            clip_sprites: false,
            jump_bits: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub enum CompatibilityDevice {
    // Also the Octo default
    #[default]
    Default,
    Chip8,
    SuperChip,
    //XOChip,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub struct CompatibilityProfile {
    pub device: CompatibilityDevice,
    pub quirks: Quirks,
}

impl CompatibilityProfile {
    /// Get the locale string key of the compatibility profile
    pub fn get_name_key(&self) -> &str {
        match self.device {
            CompatibilityDevice::Default => "default",
            CompatibilityDevice::Chip8 => "chip8",
            CompatibilityDevice::SuperChip => "super_chip",
            //CompatibilityDevice::XOChip => "xo_chip",
        }
    }

    /// Find the profile name based on the quirks
    pub fn find_profile_name_key(quirks: Quirks) -> &'static str {
        for profile in COMPATIBILITY_PROFILES.iter() {
            if profile.quirks == quirks {
                return profile.get_name_key();
            }
        }
        "custom"
    }
}

/// Compatibility profiles for different devices
pub const COMPATIBILITY_PROFILES: [CompatibilityProfile; 3] = [
    CompatibilityProfile {
        device: CompatibilityDevice::Default,
        quirks: Quirks {
            vf_zero: false,
            i_incremented: false,
            vx_shifted_directly: false,
            v_blank: false,
            clip_sprites: false,
            jump_bits: false,
        },
    },
    CompatibilityProfile {
        device: CompatibilityDevice::Chip8,
        quirks: Quirks {
            vf_zero: true,
            i_incremented: false,
            vx_shifted_directly: false,
            v_blank: true,
            clip_sprites: true,
            jump_bits: false,
        },
    },
    CompatibilityProfile {
        device: CompatibilityDevice::SuperChip,
        quirks: Quirks {
            vf_zero: false,
            i_incremented: true,
            vx_shifted_directly: false,
            v_blank: false,
            clip_sprites: true,
            jump_bits: true,
        },
    },
    /*
    CompatibilityProfile {
        device: CompatibilityDevice::XOChip,
        quirks: Quirks {
            vf_zero: false,
            i_incremented: false,
            vx_shifted_directly: false,
            v_blank: false,
            clip_sprites: false,
            jump_bits: false,
        },
    },
     */
];
