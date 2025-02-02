use crate::audio_settings::AudioSettings;
use crate::SoundDevice;
use core::f32;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tinyaudio::prelude::*;

const SAMPLE_RATE: usize = 44100;

// Default buffer used for beeps
const BUFFER: [u8; 16] = [
    0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00,
];

#[derive(Default)]
struct SquareWave {
    bit_pattern: Vec<u8>, // 128 1-bit samples
    phase_inc: f32,       // (4000*2^((vx-64)/48)) / device_sample_rate
    phase_bit: f32,       // looping index in bit_pattern
}

impl SquareWave {
    pub fn new() -> Self {
        Self {
            bit_pattern: vec![0u8; 128],
            phase_inc: Self::pitch_to_ratio(128),
            phase_bit: 0.0,
        }
    }

    pub fn pitch_to_ratio(pitch: u8) -> f32 {
        let base = 2.0f32;
        let sr = 4000.0 * base.powf((pitch as f32 - 64.0) / 48.0);
        sr / SAMPLE_RATE as f32
    }

    pub fn set_pattern(&mut self, pitch: u8, pattern: Vec<u8>) {
        // Map the 16 byte pattern to 128 bits and clamp to 0-1
        let length = 8; // = self.bit_pattern.len() / pattern.len();
        self.bit_pattern = pattern
            .iter()
            .flat_map(|&x| vec![x.clamp(0, 1); length])
            .collect();

        self.phase_inc = Self::pitch_to_ratio(pitch);

        println!(
            "Pitch: {:?}, pattern: {:?}",
            self.phase_inc, self.bit_pattern
        )
    }
}

#[derive(Default)]
pub struct TinyAudio {
    device: Option<OutputDevice>,
    playing: Arc<AtomicBool>,
    square_wave: Arc<Mutex<SquareWave>>,
}

impl TinyAudio {
    pub fn new() -> Self {
        Self {
            device: None,
            playing: Arc::new(AtomicBool::new(false)),
            square_wave: Arc::new(Mutex::new(SquareWave::new())),
        }
    }

    fn init(&mut self) {
        if self.device.is_some() {
            return;
        }

        let params = OutputDeviceParameters {
            channels_count: 1,
            sample_rate: SAMPLE_RATE,
            channel_sample_count: 2048,
        };

        let device = run_output_device(params, {
            // Prepare data here
            let is_playing = self.playing.clone();
            let square_wave = self.square_wave.clone();

            move |data| {
                let playing = is_playing.load(Ordering::SeqCst);

                // TODO: Get volume from device
                let volume = if playing { 0.5 } else { 0.0 };

                for samples in data.chunks_mut(params.channels_count) {
                    for sample in samples {
                        // TODO: Move to try_lock
                        let mut sw = square_wave.lock().unwrap();

                        *sample = if sw.bit_pattern[(sw.phase_bit + 0.5) as usize] == 1 {
                            volume
                        } else {
                            -volume
                        };

                        sw.phase_bit += sw.phase_inc;
                        if (sw.phase_bit + 0.5) as usize > sw.bit_pattern.len() - 1 {
                            sw.phase_bit = 0.0;
                        }
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
    fn play_beep(&mut self, audio_settings: AudioSettings) {
        if self.device.is_none() {
            self.init();
        }

        let mut sw = self.square_wave.lock().unwrap();
        sw.set_pattern(128, BUFFER.to_vec());

        self.playing.store(true, Ordering::SeqCst);
    }

    // TODO: Fix the f32->u8 conversion
    fn play_buffer(&mut self, audio_settings: AudioSettings, buffer: Vec<u8>, buffer_pitch: f32) {
        let mut sw = self.square_wave.lock().unwrap();
        sw.set_pattern(buffer_pitch as u8, buffer.to_vec());

        self.playing.store(true, Ordering::SeqCst);
    }

    fn pause(&mut self) {
        self.playing.store(false, Ordering::SeqCst);
    }

    fn stop(&mut self) {
        self.playing.store(false, Ordering::SeqCst);
    }

    fn update(&mut self, audio_settings: crate::audio_settings::AudioSettings) {
        todo!()
    }
}
