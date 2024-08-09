use crate::keypad::KeypadKey;

use super::{cpu::CPU, display, quirks::Quirks, MAX_MEMORY};

/// Chip-8 Device
#[derive(Debug)]
pub struct C8 {
    /// The RAM (4kb)
    memory: Vec<u8>,

    /// The display of the device (64x32)
    display: display::Display,

    /// Chip-8 CPU
    cpu: CPU,

    /// Stack memory
    stack: Vec<u16>,

    /// Whether the device is running
    is_running: bool,

    /// Keyboard state
    keyboard: [u8; 16],

    /// Quirks
    pub quirks: Quirks,
}

impl Default for C8 {
    fn default() -> Self {
        Self {
            // 4kb of memory
            memory: vec![0; MAX_MEMORY],

            // 64x32 display
            display: display::Display::default(),
            cpu: CPU::default(),
            stack: vec![],
            is_running: false,
            keyboard: [0; 16],
            quirks: Quirks::default(),
        }
    }
}

impl C8 {
    /// Get the display of the device
    pub fn get_display(&self) -> &display::Display {
        &self.display
    }

    /// Load font in the first 512 bytes of memory
    fn load_font(&mut self) {
        for (i, &byte) in super::FONT.iter().enumerate() {
            self.memory[i] = byte;
        }
    }

    /// Resets the device, loads ROM and font data into memory, and starts the device
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        // Make sure the ROM data is valid
        match rom.len() {
            0 => {
                println!("No ROM data provided");
                return;
            }
            len if len > 3584 => {
                println!("ROM data is too large: {} bytes", len);
                return;
            }
            _ => {}
        }

        self.reset_device();
        self.load_font();
        for (i, &byte) in rom.iter().enumerate() {
            self.memory[i + 512] = byte;
        }
        self.is_running = true;
    }

    /// Resets the device
    fn reset_device(&mut self) {
        self.memory = vec![0; 4096];
        self.display = display::Display::default();
        self.cpu.reset_cpu();
        self.stack = vec![];
        self.is_running = false;
        self.keyboard = [0; 16];
    }

    /// Set the state of a key
    pub fn set_key(&mut self, key: &KeypadKey, pressed: bool) {
        self.keyboard[key.get_key_index()] = pressed as u8;
    }

    /// Get the state of a key
    pub fn get_key(&self, key: &KeypadKey) -> bool {
        self.keyboard[key.get_key_index()] == 1
    }

    /// Step the device
    pub fn step(&mut self, cpu_speed: u32) {
        if self.is_running {
            // TODO: Move timers to CPU with events

            // Update timers
            if self.cpu.delay_timer > 0 {
                self.cpu.delay_timer = self.cpu.delay_timer.saturating_sub(1);
            }

            if self.cpu.sound_timer > 0 {
                self.cpu.sound_timer = self.cpu.sound_timer.saturating_sub(1);

                // TODO: Play sound
            }

            // Execute instructions
            for _ in 0..cpu_speed {
                self.cpu.step(
                    &mut self.memory,
                    &mut self.display,
                    &mut self.stack,
                    &self.quirks,
                    self.keyboard,
                );
            }
        }
    }
}
