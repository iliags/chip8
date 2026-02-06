use core::f32;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use tinyaudio::prelude::*;

use super::audio_settings::AudioSettings;
use super::SoundDevice;

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
}

impl SquareWave {
    pub fn new() -> Self {
        Self {
            bit_pattern: vec![0u8; 128],
            phase_inc: Self::pitch_to_ratio(128),
        }
    }

    pub fn pitch_to_ratio(pitch: u8) -> f32 {
        let base = 2.0f32;
        let sr = 4000.0 * base.powf((f32::from(pitch) - 64.0) / 48.0);
        sr / SAMPLE_RATE as f32
    }

    pub fn set_pattern(&mut self, pitch: u8, pattern: Vec<u8>) {
        // TODO: Apply filtering to the values
        // Map the 16 byte pattern to 128 bits and clamp to 0-1
        let length = 8; // = self.bit_pattern.len() / pattern.len();
        self.bit_pattern = pattern
            .iter()
            //.flat_map(|&x| vec![x.clamp(0, 1); length])
            .flat_map(|&x| vec![x; length])
            .collect();

        self.phase_inc = Self::pitch_to_ratio(pitch);
    }
}

#[derive(Default)]
pub struct TinyAudio {
    device: Option<OutputDevice>,
    playing: Arc<AtomicBool>,
    square_wave: Arc<RwLock<SquareWave>>,
    volume: Arc<RwLock<f32>>,
}

impl TinyAudio {
    pub fn new() -> Self {
        Self {
            device: None,
            playing: Arc::new(AtomicBool::new(false)),
            square_wave: Arc::new(RwLock::new(SquareWave::new())),
            volume: Arc::new(RwLock::new(0.15)),
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
            let device_volume = self.volume.clone();

            let mut phase_bit = 0.0f32;

            move |data| {
                let playing = is_playing.load(Ordering::SeqCst);

                let volume = if playing {
                    if let Ok(ref mutex) = device_volume.try_read() {
                        **mutex
                    } else {
                        0.0
                    }
                } else {
                    0.0
                };

                const PHASE_INC: f32 = 0.15;
                for samples in data.chunks_mut(params.channels_count) {
                    for sample in samples {
                        if let Ok(ref mut mutex) = square_wave.try_read() {
                            *sample = if mutex.bit_pattern[(phase_bit + PHASE_INC) as usize] != 0 {
                                volume
                            } else {
                                -volume
                            };

                            phase_bit += mutex.phase_inc;
                            if (phase_bit + PHASE_INC) as usize > mutex.bit_pattern.len() - 1 {
                                phase_bit = 0.0;
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
    fn play_beep(&mut self, audio_settings: AudioSettings) {
        if self.device.is_none() {
            self.init();
            self.update(audio_settings);
        }

        if let Ok(ref mut mutex) = self.square_wave.try_write() {
            mutex.set_pattern(128, BUFFER.to_vec());
            self.playing.store(true, Ordering::SeqCst);
        } else {
            eprintln!("play_beep: try_write failed");
        }
    }

    fn play_buffer(&mut self, audio_settings: AudioSettings, buffer: Vec<u8>, buffer_pitch: u8) {
        if self.device.is_none() {
            self.init();
            self.update(audio_settings);
        }

        if let Ok(ref mut mutex) = self.square_wave.try_write() {
            mutex.set_pattern(buffer_pitch, buffer.clone());
            self.playing.store(true, Ordering::SeqCst);
        } else {
            eprintln!("play_beep: try_write failed");
        }
    }

    fn pause(&mut self) {
        self.playing.store(false, Ordering::SeqCst);
    }

    fn stop(&mut self) {
        self.playing.store(false, Ordering::SeqCst);
    }

    fn update(&mut self, audio_settings: AudioSettings) {
        if let Ok(ref mut mutex) = self.volume.try_write() {
            **mutex = audio_settings.volume;
        }
        if !audio_settings.is_enabled() {
            self.stop();
        }
    }
}
