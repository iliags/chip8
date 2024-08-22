use c8_audio::AudioDevice;

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

    /// Audio device for both web and non-web targets
    pub audio_device: AudioDevice,

    /// Temporary enable/disable audio flag while controls are being implemented
    pub temp_enable_audio: bool,
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
            audio_device: AudioDevice::new(),
            temp_enable_audio: true,
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

    /// Set the quirks of the device
    pub fn set_quirks(&mut self, quirks: Quirks) {
        self.quirks = quirks;
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
    pub fn load_rom(&mut self, rom: &[u8]) {
        self.reset_device();

        self.memory.load_rom(rom);

        self.is_running = true;
    }

    /// Resets the device
    pub fn reset_device(&mut self) {
        self.audio_device.stop();
        let current_font = self.memory.system_font;
        *self = Self::default();

        // Reload font data
        self.memory
            .load_font_name(current_font, crate::fonts::FontSize::Small)
    }

    /// Step the device
    pub fn step(&mut self, cpu_speed: u32) -> Vec<DeviceMessage> {
        crate::profile_function!();
        let mut messages: Vec<DeviceMessage> = Vec::new();

        if self.is_running {
            // TODO: Move timers to CPU with events

            // Update timers
            if self.cpu.delay_timer > 0 {
                self.cpu.delay_timer = self.cpu.delay_timer.saturating_sub(1);
            }

            if self.cpu.sound_timer > 0 {
                self.cpu.sound_timer = self.cpu.sound_timer.saturating_sub(1);

                if self.audio_device.get_audio_settings().is_enabled() {
                    if self.cpu.get_audio_buffer().is_empty() {
                        self.audio_device.play_beep();
                    } else {
                        self.audio_device.play_buffer(
                            self.cpu.get_audio_buffer().clone(),
                            self.cpu.get_buffer_pitch(),
                        );
                    }
                }
            } else {
                self.cpu.clear_audio_buffer();
                self.audio_device.pause();
            }

            // Execute instructions
            for _ in 0..cpu_speed {
                let mut new_messages = self.cpu.step(
                    &mut self.memory,
                    &mut self.display,
                    &mut self.stack,
                    &self.quirks,
                    &self.keypad,
                );

                messages.append(new_messages.as_mut());
            }
        }

        // TODO: Change to observer pattern
        for message in messages.iter() {
            match message {
                DeviceMessage::ChangeResolution(resolution) => {
                    self.display.set_resolution(*resolution);
                }
                DeviceMessage::Beep(_duration) => {
                    if self.audio_device.get_audio_settings().is_enabled() {
                        self.audio_device.play_beep();
                    }
                }
                DeviceMessage::Exit => {
                    self.reset_device();
                }
                _ => {}
            }
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
        c8.load_rom(&vec![0x00, 0xE0, 0x00, 0xEE]);
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
        c8.load_rom(&vec![0x00, 0xE0, 0x00, 0xEE]);
        c8.cpu.delay_timer = 1;
        c8.cpu.sound_timer = 1;
        c8.step(1);
        assert_eq!(c8.cpu.delay_timer, 0);
        assert_eq!(c8.cpu.sound_timer, 0);
    }
}
