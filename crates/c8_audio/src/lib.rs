//! Audio utilities for the CHIP-8 emulator.

use audio_settings::AudioSettings;
use web_audio::WebAudio;

/// Beeper module for playing sound.
pub mod beeper;

/// Custom audio settings
pub mod audio_settings;

/// Web audio module
//#[cfg(target_arch = "wasm32")]
mod web_audio;

/// Desktop audio module
//#[cfg(not(target_arch = "wasm32"))]
mod desktop_audio;

/// Audio module
#[derive(Debug)]
pub struct AudioDevice {
    //#[cfg(target_arch = "wasm32")]
    web_device: Option<WebAudio>,

    //#[cfg(not(target_arch = "wasm32"))]
    //desktop_audio: Option<DesktopAudio>,
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
            //#[cfg(target_arch = "wasm32")]
            web_device: Some(WebAudio::new()),

            //#[cfg(not(target_arch = "wasm32"))]
            //desktop_audio: Some(DesktopAudio::new()),
            audio_settings: AudioSettings::default(),
        }
    }

    /// Get the current audio settings
    pub fn get_audio_settings(&self) -> &AudioSettings {
        &self.audio_settings
    }

    /// Set the audio settings
    pub fn set_audio_settings(&mut self, settings: AudioSettings) {
        self.audio_settings = settings;
    }

    /// Play a beep sound
    pub fn play_beep(&mut self) {
        // TODO: Desktop

        //#[cfg(target_arch = "wasm32")]
        match &mut self.web_device {
            Some(web_device) => web_device.play_beep(self.audio_settings),
            None => {
                // TODO: Error handling
            }
        }
    }

    /// Play a buffer
    pub fn play_buffer(&mut self) {
        // TODO
    }

    /// Stop the audio
    pub fn stop(&mut self) {
        //#[cfg(target_arch = "wasm32")]
        match &mut self.web_device {
            Some(web_device) => web_device.stop(),
            None => {
                // TODO: Error handling
            }
        }
    }

    /// Update audio settings
    pub fn update_settings(&mut self, settings: AudioSettings) {
        self.audio_settings = settings;
    }
}
