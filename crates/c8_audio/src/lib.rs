//! Audio utilities for the CHIP-8 emulator.

use audio_settings::AudioSettings;

/// Custom audio settings
pub mod audio_settings;
mod device_audio;

/// Trait for sound devices
pub(crate) trait SoundDevice: std::fmt::Debug {
    fn play_beep(&mut self, audio_settings: AudioSettings);
    fn play_buffer(&mut self, audio_settings: AudioSettings, buffer: Vec<u8>, buffer_pitch: f32);
    fn pause(&mut self);
    fn stop(&mut self);
    fn update(&mut self, audio_settings: AudioSettings);
}

/// Audio module
#[derive(Debug)]
pub struct AudioDevice {
    audio_device: Box<dyn SoundDevice>,

    audio_settings: AudioSettings,
}

impl Default for AudioDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioDevice {
    /// Create a new audio device
    pub fn new() -> Self {
        Self {
            audio_device: Box::new(device_audio::DeviceAudio::new()),

            audio_settings: AudioSettings::default(),
        }
    }

    /// Get the current audio settings
    pub fn audio_settings(&self) -> &AudioSettings {
        &self.audio_settings
    }

    /// Set the audio settings
    pub fn set_audio_settings(&mut self, settings: AudioSettings) {
        self.audio_settings = settings;
    }

    /// Play a beep sound
    pub fn play_beep(&mut self) {
        self.audio_device.play_beep(self.audio_settings);
    }

    /// Play a buffer
    pub fn play_buffer(&mut self, buffer: Vec<u8>, buffer_pitch: f32) {
        self.audio_device
            .play_buffer(self.audio_settings, buffer, buffer_pitch);
    }

    /// Pause the audio, if supported
    pub fn pause(&mut self) {
        self.audio_device.pause();
    }

    /// Stop the audio
    pub fn stop(&mut self) {
        self.audio_device.stop();
    }

    /// Update audio settings
    pub fn update_settings(&mut self, settings: AudioSettings) {
        if self.audio_settings != settings {
            self.audio_settings = settings;
            self.audio_device.update(self.audio_settings);
        }
    }
}
