use core::f32;
use std::{time::Duration, vec};

use crate::{audio_settings::AudioSettings, SoundDevice};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BackendSpecificError, BuildStreamError, FromSample, Sample, SizedSample, Stream,
};

pub struct DeviceAudio {
    stream_buffer: Option<Stream>,
    device: cpal::Device,
    config: cpal::SupportedStreamConfig,
}

impl Default for DeviceAudio {
    fn default() -> Self {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("no output device available");

        let config = device
            .default_output_config()
            .expect("no default output config");

        DeviceAudio {
            stream_buffer: None,
            device,
            config,
        }
    }
}

impl std::fmt::Debug for DeviceAudio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Desktop Audio")
    }
}

impl SoundDevice for DeviceAudio {
    fn play_beep(&mut self, audio_settings: AudioSettings) {
        // Constant clean tone
        const BUFFER: [u8; 16] = [
            0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00,
            0xFF, 0x00,
        ];

        self.play_buffer(audio_settings, BUFFER.to_vec(), audio_settings.frequency());
    }
    fn play_buffer(&mut self, audio_settings: AudioSettings, buffer: Vec<u8>, buffer_pitch: f32) {
        //println!("Playing buffer with pitch: {}", buffer_pitch);
        if self.stream_buffer.is_none() {
            let new_stream = match self.config.sample_format() {
                cpal::SampleFormat::F32 => Self::create_stream_buffer::<f32>(
                    &self.device,
                    &self.config.clone().into(),
                    &audio_settings,
                    buffer,
                    buffer_pitch,
                ),
                sample_format => Err(BuildStreamError::BackendSpecific {
                    err: BackendSpecificError {
                        description: format!("Unsupported sample format '{sample_format}'"),
                    },
                }),
            };

            self.stream_buffer = Some(new_stream.unwrap());
        }

        match self.stream_buffer.as_ref() {
            Some(stream) => {
                if stream.play().is_err() {
                    eprintln!("Failed to play stream");
                }
            }
            None => {
                eprintln!("No stream available to play");
            }
        }
    }
    fn pause(&mut self) {
        let _ = self.stream_buffer.take();
    }
    fn stop(&mut self) {
        let _ = self.stream_buffer.take();
    }
    fn update(&mut self, _audio_settings: AudioSettings) {
        // TODO
    }
}

impl DeviceAudio {
    pub fn new() -> Self {
        DeviceAudio::default()
    }

    fn create_stream_buffer<T>(
        device: &cpal::Device,
        config: &cpal::StreamConfig,
        settings: &AudioSettings,
        buffer: Vec<u8>,
        buffer_pitch: f32,
    ) -> Result<Stream, BuildStreamError>
    where
        T: SizedSample + FromSample<f32>,
    {
        let sample_rate = config.sample_rate.0 as f32;

        let volume = if settings.is_enabled() {
            settings.volume() * 0.5
        } else {
            0.0
        };

        const BUFFER_FREQ: f32 = 4000.0;
        const PITCH_BIAS: f32 = 64.0;
        let pitch = buffer_pitch;
        let freq = BUFFER_FREQ * (2.0_f32).powf((pitch - PITCH_BIAS) / 48.0);

        //let calculated_pitch = BUFFER_FREQ * (2.0_f32).powf((pitch - 64.0) / 48.0);
        //let repetitions = (sample_rate / calculated_pitch) as usize;

        //let mut samples: Vec<f32> = Vec::with_capacity(buffer.len() * 8 * repetitions);
        let mut samples: Vec<f32> = vec![0.0; 512];

        let mut index = 0;
        for byte in &buffer {
            let mut mask: u8 = 128;
            while mask != 0 {
                let val = if byte & mask != 0 { 1.0 } else { 0.0 };
                samples[index] = val;
                index += 1;
                samples[index] = val;
                index += 1;
                samples[index] = val;
                index += 1;
                samples[index] = val;
                index += 1;

                mask >>= 1;
            }

            /*
            for idx_bit in 0..8 {
                let bit = byte >> (7 - idx_bit) & 0b1 == 0b1;
                let val = if bit { 1.0 } else { 0.0 };
                for _ in 0..repetitions {
                    samples.push(val * pitch);
                }
            }
             */
        }

        println!("Samples: {:?}", samples);

        let channels = config.channels as usize;
        let mut sample_clock = 0;
        let mut next_value = move || {
            sample_clock = (sample_clock + 1) % samples.len();

            //(samples[sample_clock as usize]).sin() * volume
            (samples[sample_clock]) * volume
        };

        let err_fn = |err| eprintln!("Stream error: {}", err);

        device.build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                Self::write_data(data, channels, &mut next_value)
            },
            err_fn,
            Some(Duration::from_secs_f32(10.0 / 60.0)),
        )
    }

    fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
    where
        T: Sample + FromSample<f32>,
    {
        for frame in output.chunks_mut(channels) {
            let value: T = T::from_sample(next_sample());
            for sample in frame.iter_mut() {
                *sample = value;
            }
        }
    }
}

impl Drop for DeviceAudio {
    fn drop(&mut self) {
        self.stop();
    }
}

/* fn lowpass_alpha(sampling_freq: f32) -> f32 {
    const CUTOFF: f32 = 18000.0;
    let c = (2.0 * f32::consts::PI * CUTOFF / sampling_freq).cos();
    c - 1.0 + (c * c - 4.0 * c + 3.0).sqrt()
}

fn lowpass_filtered_value(alpha: f32, target: u8) -> f32 {
    const LOWPASS_STEPS: usize = 4;
    let mut lowpass_buffer = vec![0.0; LOWPASS_STEPS + 1];
    lowpass_buffer[0] = target as f32;

    for i in 1..lowpass_buffer.len() {
        lowpass_buffer[i] += alpha * (lowpass_buffer[i - 1] - lowpass_buffer[i]);
    }
    lowpass_buffer[lowpass_buffer.len() - 1]
}
 */
