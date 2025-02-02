use crate::audio_settings::AudioSettings;
use crate::SoundDevice;
use core::f32;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tinyaudio::prelude::*;

// TODO: Check if RWLock has better results

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
                        let mut lock = square_wave.try_lock();
                        if let Ok(ref mut mutex) = lock {
                            *sample = if mutex.bit_pattern[(mutex.phase_bit + 0.5) as usize] == 1 {
                                volume
                            } else {
                                -volume
                            };

                            mutex.phase_bit += mutex.phase_inc;
                            if (mutex.phase_bit + 0.5) as usize > mutex.bit_pattern.len() - 1 {
                                mutex.phase_bit = 0.0;
                            }
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
    // TODO: Audio settings
    fn play_beep(&mut self, _audio_settings: AudioSettings) {
        if self.device.is_none() {
            self.init();
        }

        let mut lock = self.square_wave.try_lock();
        if let Ok(ref mut mutex) = lock {
            mutex.set_pattern(128, BUFFER.to_vec());
            self.playing.store(true, Ordering::SeqCst);
        } else {
            println!("play_beep: try_lock failed");
        }
    }

    // TODO: Audio settings
    fn play_buffer(&mut self, _audio_settings: AudioSettings, buffer: Vec<u8>, buffer_pitch: u8) {
        if self.device.is_none() {
            self.init();
        }

        let mut lock = self.square_wave.try_lock();
        if let Ok(ref mut mutex) = lock {
            mutex.set_pattern(buffer_pitch, buffer.to_vec());
            self.playing.store(true, Ordering::SeqCst);
        } else {
            println!("play_buffer: try_lock failed");
        }
    }

    fn pause(&mut self) {
        self.playing.store(false, Ordering::SeqCst);
    }

    fn stop(&mut self) {
        self.playing.store(false, Ordering::SeqCst);
    }

    fn update(&mut self, _audio_settings: crate::audio_settings::AudioSettings) {
        // TODO: Check if this is still needed
        println!("update: Not implemented");
    }
}
