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
            const BUFFER_FREQ: f32 = 4000.0;
            const PITCH_BIAS: f32 = 64.0;

            let mut clock = 0f32;
            let playing = self.playing.clone();
            move |data| {
                //println!("Data: {:?}", data.len());
                let buffer = if playing.load(Ordering::SeqCst) {
                    &BUFFER
                } else {
                    &SILENCE
                };
                for samples in data.chunks_mut(params.channels_count) {
                    //clock = (clock + 1.0) % params.sample_rate as f32;
                    clock = (clock + 1.0) % buffer.len() as f32;
                    /* let value = (clock * 440.0 * 2.0 * std::f32::consts::PI
                    / params.sample_rate as f32)
                    .sin(); */
                    let pitch = 64.0;
                    let freq = BUFFER_FREQ * (2.0_f32).powf((pitch - PITCH_BIAS) / 48.0);
                    let value = clock * freq * buffer[clock as usize] as f32;
                    //println!("Samples: {:?}", samples.len());
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
