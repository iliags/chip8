//! Audio utilities for the CHIP-8 emulator.

use audio_settings::AudioSettings;

/// Custom audio settings
pub mod audio_settings;

/// Web audio module
#[cfg(target_arch = "wasm32")]
mod web_audio;

/// Desktop audio module
#[cfg(not(target_arch = "wasm32"))]
mod desktop_audio;

/// Trait for sound devices
pub(crate) trait SoundDevice: std::fmt::Debug {
    fn play_beep(&mut self, audio_settings: AudioSettings);
    fn play_buffer(&mut self, audio_settings: AudioSettings, buffer: Vec<u8>, buffer_pitch: f32);
    fn pause(&mut self);
    fn stop(&mut self);
}

/// Audio module
#[derive(Debug)]
pub struct AudioDevice {
    audio_device: Box<dyn SoundDevice>,

    audio_settings: AudioSettings,

    // Used for buffer playback
    pitch: f32,
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
            #[cfg(target_arch = "wasm32")]
            audio_device: Box::new(web_audio::WebAudio::new()),

            #[cfg(not(target_arch = "wasm32"))]
            audio_device: Box::new(desktop_audio::DesktopAudio::new()),

            audio_settings: AudioSettings::default(),

            pitch: 440.0,
        }
    }

    /// Get the current audio settings
    pub fn get_audio_settings(&self) -> &AudioSettings {
        &self.audio_settings
    }

    /// Get the current audio settings mutable
    pub fn get_audio_settings_mut(&mut self) -> &mut AudioSettings {
        &mut self.audio_settings
    }

    /// Set the audio settings
    pub fn set_audio_settings(&mut self, settings: AudioSettings) {
        self.audio_settings = settings;
    }

    /// Set the buffer pitch
    pub fn set_buffer_pitch(&mut self, pitch: f32) {
        self.pitch = pitch;
    }

    /// Play a beep sound
    pub fn play_beep(&mut self) {
        self.audio_device.play_beep(self.audio_settings);
    }

    /// Play a buffer
    #[allow(unused_variables)]
    pub fn play_buffer(&mut self, buffer: Vec<u8>) {
        self.audio_device
            .play_buffer(self.audio_settings, buffer, self.pitch);
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
        self.audio_settings = settings;
    }
}
