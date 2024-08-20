use crate::{audio_settings::AudioSettings, SoundDevice};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BackendSpecificError, BuildStreamError, FromSample, Sample, SizedSample, Stream,
};

use std::sync::mpsc::{Receiver, Sender};

/// Messages for the desktop audio system
#[derive(Debug, PartialEq)]
pub enum Message {
    /// Play the audio
    Play,
    /// Pause the audio
    Pause,
    /// Stop the audio
    Stop,
    /// Update the audio settings
    Update,
}

#[derive(Default)]
pub struct DesktopAudio {
    sender: Option<Sender<Message>>,
    stream: Option<Stream>,
}

impl std::fmt::Debug for DesktopAudio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sender {{ sender: {:?} }}", self.sender)
    }
}

impl DesktopAudio {
    pub fn new() -> Self {
        Self::default()
    }
}

impl SoundDevice for DesktopAudio {
    fn play_beep(&mut self, audio_settings: AudioSettings) {}
    fn play_buffer(&mut self, audio_settings: AudioSettings, buffer: Vec<u8>, buffer_pitch: f32) {}
    fn pause(&mut self) {}
    fn stop(&mut self) {}
}
