/// Empty sound device. Used when the `audio` feature is disabled.
use super::SoundDevice;

#[derive(Debug, Default)]
pub struct NullAudio;

impl SoundDevice for NullAudio {
    fn play_beep(&mut self, _audio_settings: super::audio_settings::AudioSettings) {}

    fn play_buffer(
        &mut self,
        _audio_settings: super::audio_settings::AudioSettings,
        _buffer: Vec<u8>,
        _buffer_pitch: u8,
    ) {
    }

    fn pause(&mut self) {}

    fn stop(&mut self) {}

    fn update(&mut self, _audio_settings: super::audio_settings::AudioSettings) {}
}
