use c8_audio::Beeper;

use crate::{
    cpu::CPU, display::Display, keypad::Keypad, memory::Memory, message::DeviceMessage,
    quirks::Quirks,
};

/// Chip-8 Device
#[derive(Debug)]
pub struct C8 {
    /// The RAM (4kb)
    memory: Memory,

    /// The display of the device (64x32)
    display: Display,

    /// Chip-8 CPU
    cpu: CPU,

    /// Stack memory
    stack: Vec<u16>,

    /// Whether the device is running
    is_running: bool,

    /// Keyboard state
    keypad: Keypad,

    /// Quirks
    quirks: Quirks,

    /// Audio
    pub beeper: Beeper,
}

impl Default for C8 {
    fn default() -> Self {
        Self {
            memory: Memory::default(),
            display: Display::default(),
            cpu: CPU::default(),
            stack: vec![],
            is_running: false,
            keypad: Keypad::default(),
            quirks: Quirks::default(),
            beeper: Beeper::new(),
        }
    }
}

impl C8 {
    /// Get the memory of the device
    pub fn get_memory(&self) -> &Memory {
        &self.memory
    }

    /// Get the memory of the device (mutable)
    pub fn get_memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
    }

    /// Get the quirks of the device
    pub fn get_quirks(&self) -> &Quirks {
        &self.quirks
    }

    /// Get the quirks of the device (mutable)
    pub fn get_quirks_mut(&mut self) -> &mut Quirks {
        &mut self.quirks
    }

    /// Get the display of the device
    pub fn get_display(&self) -> &Display {
        &self.display
    }

    /// Get the display of the device (mutable)
    pub fn get_display_mut(&mut self) -> &mut Display {
        &mut self.display
    }

    /// Get the keypad of the device
    pub fn get_keypad(&self) -> &Keypad {
        &self.keypad
    }

    /// Get the keypad of the device (mutable)
    pub fn get_keypad_mut(&mut self) -> &mut Keypad {
        &mut self.keypad
    }

    /// Get if the device is running
    pub fn get_is_running(&self) -> bool {
        self.is_running
    }

    /// Resets the device, loads ROM and font data into memory, and starts the device
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.reset_device();

        self.memory.load_rom(rom);

        self.is_running = true;
    }

    /// Resets the device
    pub fn reset_device(&mut self) {
        self.beeper.stop();
        let current_font = self.memory.system_font;
        *self = Self::default();

        // Reload font data
        self.memory
            .load_font_name(current_font, crate::fonts::FontSize::Small)
    }

    /// Step the device
    pub fn step(&mut self, cpu_speed: u32) -> Vec<DeviceMessage> {
        if !self.is_running {
            return Vec::new();
        }

        // TODO: Move timers to CPU with events

        // Update timers
        if self.cpu.delay_timer > 0 {
            self.cpu.delay_timer = self.cpu.delay_timer.saturating_sub(1);
        }

        if self.cpu.sound_timer > 0 {
            self.cpu.sound_timer = self.cpu.sound_timer.saturating_sub(1);

            // TODO: Enable when WASM audio is supported
            #[cfg(debug_assertions)]
            {
                self.beeper.play();
            }
        } else {
            // TODO: Make this more ergonomic (i.e. only pause if it's playing)
            self.beeper.pause();
        }

        let mut messages: Vec<DeviceMessage> = Vec::new();

        // Execute instructions
        for _ in 0..cpu_speed {
            let mut new_messages = self.cpu.step(
                &mut self.memory,
                &mut self.display,
                &mut self.stack,
                &self.quirks,
                &self.keypad,
            );

            for message in new_messages.iter().clone() {
                match message {
                    DeviceMessage::ChangeResolution(resolution) => {
                        self.display.set_resolution(*resolution);
                    }
                    DeviceMessage::Exit => {
                        //self.is_running = false;
                        self.reset_device();
                    }
                    _ => {}
                }
            }

            messages.append(new_messages.as_mut());
        }

        messages
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests were generated by GitHub Copilot

    #[test]
    fn test_load_rom() {
        let mut c8 = C8::default();
        c8.load_rom(vec![0x00, 0xE0, 0x00, 0xEE]);
        assert_eq!(c8.memory.data[0x200], 0x00);
        assert_eq!(c8.memory.data[0x201], 0xE0);
        assert_eq!(c8.memory.data[0x202], 0x00);
        assert_eq!(c8.memory.data[0x203], 0xEE);
    }

    #[test]
    fn test_reset_device() {
        let mut c8 = C8::default();
        c8.memory.data[0x200] = 0x01;
        c8.memory.data[0x201] = 0x02;
        c8.memory.data[0x202] = 0x03;
        c8.memory.data[0x203] = 0x04;
        c8.reset_device();
        assert_eq!(c8.memory.data[0x200], 0x00);
        assert_eq!(c8.memory.data[0x201], 0x00);
        assert_eq!(c8.memory.data[0x202], 0x00);
        assert_eq!(c8.memory.data[0x203], 0x00);
    }

    #[test]
    fn test_step_timers() {
        let mut c8 = C8::default();
        c8.load_rom(vec![0x00, 0xE0, 0x00, 0xEE]);
        c8.cpu.delay_timer = 1;
        c8.cpu.sound_timer = 1;
        c8.step(1);
        assert_eq!(c8.cpu.delay_timer, 0);
        assert_eq!(c8.cpu.sound_timer, 0);
    }
}
