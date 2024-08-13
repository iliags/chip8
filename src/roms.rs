/// Struct to hold ROM data
pub struct ROM {
    name: &'static str,
    data: &'static [u8],
}

impl ROM {
    /// Get the name of the ROM
    pub fn get_name(&self) -> &'static str {
        self.name
    }

    /// Get the data of the ROM
    pub fn get_data(&self) -> &'static [u8] {
        self.data
    }
}

/// List of ROMs used for testing the emulator
pub const TEST_ROMS: &[ROM] = &[
    ROM {
        name: "chip8-logo.ch8",
        data: include_bytes!("../assets/test_roms/1-chip8-logo.ch8"),
    },
    ROM {
        name: "ibm-logo.ch8",
        data: include_bytes!("../assets/test_roms/2-ibm-logo.ch8"),
    },
    ROM {
        name: "corax+.ch8",
        data: include_bytes!("../assets/test_roms/3-corax+.ch8"),
    },
    ROM {
        name: "flags.ch8",
        data: include_bytes!("../assets/test_roms/4-flags.ch8"),
    },
    ROM {
        name: "quirks.ch8",
        data: include_bytes!("../assets/test_roms/5-quirks.ch8"),
    },
    ROM {
        name: "keypad.ch8",
        data: include_bytes!("../assets/test_roms/6-keypad.ch8"),
    },
    ROM {
        name: "beep.ch8",
        data: include_bytes!("../assets/test_roms/7-beep.ch8"),
    },
    // Requires Super Chip-8 support
    //ROM {
    //    name: "scrolling.ch8",
    //    data: include_bytes!("../assets/test_roms/8-scrolling.ch8"),
    //},
];

pub const GAME_ROMS: &[ROM] = &[ROM {
    name: "Octo Sample",
    data: include_bytes!("../assets/games/octo-sample/octo-sample.ch8"),
}];
