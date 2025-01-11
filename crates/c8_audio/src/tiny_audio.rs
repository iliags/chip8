use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::SoundDevice;
use tinyaudio::prelude::*;

// TODO: Atomic buffer
const BUFFER: [u8; 16] = [
    0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00,
];

const SILENCE: [u8; 16] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

#[derive(Default)]
pub struct TinyAudio {
    device: Option<OutputDevice>,
    playing: Arc<AtomicBool>,
}

impl TinyAudio {
    pub fn new() -> Self {
        Self {
            device: None,
            playing: Arc::new(AtomicBool::new(false)),
        }
    }

    fn init(&mut self) {
        if self.device.is_some() {
            return;
        }

        let params = OutputDeviceParameters {
            channels_count: 2,
            sample_rate: 44100,
            channel_sample_count: 4410,
        };

        let device = run_output_device(params, {
            // Do stuff
            let mut clock = 0f32;
            let playing = self.playing.clone();
            move |data| {
                for samples in data.chunks_mut(params.channels_count) {
                    if playing.load(Ordering::SeqCst) == false {
                        for (i, sample) in samples.iter_mut().enumerate() {
                            *sample = SILENCE[i] as f32 / 255.0;
                        }
                        continue;
                    }
                    clock = (clock + 1.0) % params.sample_rate as f32;
                    let value = (clock * 440.0 * 2.0 * std::f32::consts::PI
                        / params.sample_rate as f32)
                        .sin();
                    for sample in samples {
                        *sample = value;
                    }
                }
            }
        })
        .unwrap();

        self.device = Some(device);
    }
}

impl std::fmt::Debug for TinyAudio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tiny Audio")
    }
}

impl SoundDevice for TinyAudio {
    fn play_beep(&mut self, audio_settings: crate::audio_settings::AudioSettings) {
        if self.device.is_none() {
            self.init();
        }

        self.playing
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    fn play_buffer(
        &mut self,
        audio_settings: crate::audio_settings::AudioSettings,
        buffer: Vec<u8>,
        buffer_pitch: f32,
    ) {
        todo!()
    }

    fn pause(&mut self) {
        self.playing
            .store(false, std::sync::atomic::Ordering::Relaxed);
    }

    fn stop(&mut self) {
        self.playing
            .store(false, std::sync::atomic::Ordering::Relaxed);
    }

    fn update(&mut self, audio_settings: crate::audio_settings::AudioSettings) {
        todo!()
    }
}
