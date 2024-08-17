/// Struct to hold ROM data
pub struct ROM {
    name: &'static str,
    data: &'static [u8],
}

impl ROM {
    /// Get the name of the ROM
    pub fn get_name(&self) -> &str {
        self.name.strip_suffix(".ch8").unwrap_or(self.name)
    }

    /// Get the data of the ROM
    pub fn get_data(&self) -> &[u8] {
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
    ROM {
        name: "scrolling.ch8",
        data: include_bytes!("../assets/test_roms/8-scrolling.ch8"),
    },
];

pub const GAME_ROMS: &[ROM] = &[
    ROM {
        name: "An Evening to Die For",
        data: include_bytes!("../assets/games/anEveningToDieFor.ch8"),
    },
    ROM {
        name: "Cave Explorer",
        data: include_bytes!("../assets/games/cave-explorer/cave-explorer.ch8"),
    },
    ROM {
        name: "Flight Runner",
        data: include_bytes!("../assets/games/flightrunner.ch8"),
    },
    ROM {
        name: "Glitch Ghost",
        data: include_bytes!("../assets/games/glitch-ghost/glitch-ghost.ch8"),
    },
    ROM {
        name: "Octoma",
        data: include_bytes!("../assets/games/octoma.ch8"),
    },
    ROM {
        name: "Octo Rancher",
        data: include_bytes!("../assets/games/octorancher.ch8"),
    },
    ROM {
        name: "Octo Sample",
        data: include_bytes!("../assets/games/octo-sample/octo-sample.ch8"),
    },
    ROM {
        name: "Rockto",
        data: include_bytes!("../assets/games/rockto.ch8"),
    },
    ROM {
        name: "Skyward",
        data: include_bytes!("../assets/games/skyward/skyward.ch8"),
    },
];
