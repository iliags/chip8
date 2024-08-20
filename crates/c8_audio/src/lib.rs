//! Audio utilities for the CHIP-8 emulator.

use audio_settings::AudioSettings;

#[cfg(target_arch = "wasm32")]
use web_audio::WebAudio;

/// Beeper module for playing sound.
pub mod beeper;

/// Custom audio settings
pub mod audio_settings;

/// Web audio module
#[cfg(target_arch = "wasm32")]
mod web_audio;

/// Desktop audio module
//#[cfg(not(target_arch = "wasm32"))]
mod desktop_audio;

/// Audio module
#[derive(Debug)]
pub struct AudioDevice {
    #[cfg(target_arch = "wasm32")]
    web_device: Option<WebAudio>,

    //#[cfg(not(target_arch = "wasm32"))]
    //desktop_audio: Option<DesktopAudio>,
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
            web_device: Some(WebAudio::new()),

            //#[cfg(not(target_arch = "wasm32"))]
            //desktop_audio: Some(DesktopAudio::new()),
            audio_settings: AudioSettings::default(),

            pitch: 440.0,
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

    /// Set the buffer pitch
    pub fn set_buffer_pitch(&mut self, pitch: f32) {
        self.pitch = pitch;
    }

    /// Play a beep sound
    pub fn play_beep(&mut self) {
        //#[cfg(not(target_arch = "wasm32"))]
        // TODO: Desktop

        #[cfg(target_arch = "wasm32")]
        match &mut self.web_device {
            Some(web_device) => web_device.play_beep(self.audio_settings),
            None => {
                use web_sys::console;
                let message = "Play Beep: Failed to get web audio device";
                console::error_1(&message.into());
            }
        }
    }

    /// Play a buffer
    #[allow(unused_variables)]
    pub fn play_buffer(&mut self, buffer: Vec<u8>) {
        //#[cfg(not(target_arch = "wasm32"))]
        // TODO: Desktop

        #[cfg(target_arch = "wasm32")]
        match &mut self.web_device {
            Some(web_device) => {
                web_device.play_buffer(self.audio_settings, buffer, self.pitch);
            }
            None => {
                use web_sys::console;
                let message = "Stop: Failed to get web audio device";
                console::error_1(&message.into());
            }
        }
    }

    /// Stop the audio
    pub fn stop(&mut self) {
        //#[cfg(not(target_arch = "wasm32"))]
        // TODO: Desktop

        #[cfg(target_arch = "wasm32")]
        match &mut self.web_device {
            Some(web_device) => web_device.stop(),
            None => {
                use web_sys::console;
                let message = "Stop: Failed to get web audio device";
                console::error_1(&message.into());
            }
        }
    }

    /// Update audio settings
    pub fn update_settings(&mut self, settings: AudioSettings) {
        self.audio_settings = settings;
    }
}
