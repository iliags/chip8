use core::f32;
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
            channels_count: 1,
            sample_rate: 44100,
            //channel_sample_count: 4410,
            channel_sample_count: 2048,
        };

        let device = run_output_device(params, {
            // Prepare data here
            let mut clock = 0f32;
            let is_playing = self.playing.clone();
            let quality = (384000 / params.sample_rate) as usize;
            let lowpass_alpha = lowpass_alpha(params.sample_rate as f32 * quality as f32);

            println!("Lowpass alpha: {}", lowpass_alpha);

            const FREQ: f32 = 4000.0;
            const PITCH_BIAS: f32 = 64.0;

            move |data| {
                //println!("Data: {:?}", data.len());
                // TODO: Replace with pitch from device
                let pitch = 103.0;
                let freq = FREQ * (2.0_f32).powf((pitch - PITCH_BIAS) / 48.0);
                let step = freq / params.sample_rate as f32;
                let playing = is_playing.load(Ordering::Relaxed);

                // TODO: Get buffer from running device
                let buffer = if playing { BUFFER } else { SILENCE };

                let mut pos = 0;
                let mut prev_value = 0.0;

                for channel in data.chunks_mut(params.channels_count) {
                    //println!("Channels: {:?}", channel.len());
                    /*
                    //let index = pos as usize % BUFFER.len();
                    let cell = pos >> 3;
                    let shift = pos & 7 ^ 7;
                    //let value = lowpass_filtered_value(lowpass_alpha, buffer[cell] >> shift & 1);
                    let value = buffer[pos] as f32;
                    //println!("Value: {}", value);
                    let value = ((prev_value * 0.4) + value).sin(); // % 0xFF as f32;

                    //prev_value = value as f32;
                    if value != 0.0 {
                        println!("Value: {}", value);
                    }

                    pos = (pos + (step as usize / quality)) % buffer.len();

                    // TODO: Replace with buffer
                    let volume: f32 = if playing { 0.1 } else { 0.0 };

                    for sample in samples {
                        *sample = value * volume;
                    }
                    */
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

fn lowpass_alpha(sampling_freq: f32) -> f32 {
    const CUTOFF: f32 = 18000.0;
    let c = (2.0 * f32::consts::PI * CUTOFF / sampling_freq).cos();
    c - 1.0 + (c * c - 4.0 * c + 3.0).sqrt()
}

fn lowpass_filtered_value(alpha: f32, target: u8) -> f32 {
    const LOWPASS_STEPS: usize = 4;
    let mut lowpass_buffer = vec![0.0; LOWPASS_STEPS + 1];
    lowpass_buffer[0] = target as f32;

    for i in 1..lowpass_buffer.len() {
        lowpass_buffer[i] += (lowpass_buffer[i - 1] - lowpass_buffer[i]) * alpha;
    }
    lowpass_buffer[lowpass_buffer.len() - 1]
}
