use std::time::Duration;

use crate::{audio_settings::AudioSettings, SoundDevice};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BackendSpecificError, BuildStreamError, FromSample, Sample, SizedSample, Stream,
};

pub struct DesktopAudio {
    stream: Option<Stream>,
    stream_buffer: Option<Stream>,
    //host: cpal::Host,
    device: cpal::Device,
    config: cpal::SupportedStreamConfig,
}

impl Default for DesktopAudio {
    fn default() -> Self {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("no output device available");

        let config = device
            .default_output_config()
            .expect("no default output config");

        DesktopAudio {
            stream: None,
            stream_buffer: None,
            //host,
            device,
            config,
        }
    }
}

impl std::fmt::Debug for DesktopAudio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Desktop Audio")
    }
}

impl SoundDevice for DesktopAudio {
    fn play_beep(&mut self, audio_settings: AudioSettings) {
        // Creating a new stream for each beep is bad, refactor later
        if self.stream.is_none() {
            let new_stream = match self.config.sample_format() {
                cpal::SampleFormat::F32 => Self::create_stream_beep::<f32>(
                    &self.device,
                    &self.config.clone().into(),
                    &audio_settings,
                ),
                sample_format => Err(BuildStreamError::BackendSpecific {
                    err: BackendSpecificError {
                        description: format!("Unsupported sample format '{sample_format}'"),
                    },
                }),
            };

            // TODO: Error handling
            self.stream = Some(new_stream.unwrap());
        }

        match self.stream.as_ref() {
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
    fn play_buffer(&mut self, audio_settings: AudioSettings, buffer: Vec<u8>, buffer_pitch: f32) {
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
        match self.stream.as_ref() {
            Some(stream) => {
                if stream.pause().is_err() {
                    println!("Failed to pause stream");
                }
            }
            None => {}
        }

        let _ = self.stream_buffer.take();
    }
    fn stop(&mut self) {
        let _ = self.stream.take();
        let _ = self.stream_buffer.take();
    }
    fn update(&mut self, _audio_settings: AudioSettings) {
        // TODO
    }
}

impl DesktopAudio {
    pub fn new() -> Self {
        DesktopAudio::default()
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
        let channels = config.channels as usize;

        // Produce a sinusoid of maximum amplitude.
        let mut sample_clock = 0f32;

        let volume = if settings.is_enabled() {
            settings.get_volume()
        } else {
            0.0
        };

        const BUFFER_FREQ: f32 = 4000.0;

        let pitch = buffer_pitch;

        let calculated_pitch = BUFFER_FREQ * (2.0_f32).powf((pitch - 64.0) / 48.0);
        let repetitions = (sample_rate / calculated_pitch) as usize;

        let mut samples: Vec<f32> = Vec::with_capacity(buffer.len() * 8 * repetitions);

        for byte in &buffer {
            for idx_bit in 0..8 {
                let bit = byte >> (7 - idx_bit) & 0b1 == 0b1;
                let val = if bit { volume } else { 0.0 };
                for _ in 0..repetitions {
                    samples.push(val * pitch);
                }
            }
        }

        let mut next_value = move || {
            sample_clock = (sample_clock + 1.0) % samples.len() as f32;

            samples[sample_clock as usize] * volume
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

    fn create_stream_beep<T>(
        device: &cpal::Device,
        config: &cpal::StreamConfig,
        settings: &AudioSettings,
    ) -> Result<Stream, BuildStreamError>
    where
        T: SizedSample + FromSample<f32>,
    {
        let sample_rate = config.sample_rate.0 as f32;
        let channels = config.channels as usize;

        // Produce a sinusoid of maximum amplitude.
        let mut sample_clock = 0f32;

        let pitch = settings.get_frequency();

        // Zero volume if audio is disabled
        let volume = if settings.is_enabled() {
            settings.get_volume()
        } else {
            0.0
        };

        let octave = 2.0;

        let mut next_value = move || {
            sample_clock = (sample_clock + 1.0) % sample_rate;

            (sample_clock * pitch * octave * std::f32::consts::PI / sample_rate).sin() * volume
        };

        let err_fn = |err| eprintln!("Stream error: {}", err);

        device.build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                Self::write_data(data, channels, &mut next_value)
            },
            err_fn,
            None,
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

impl Drop for DesktopAudio {
    fn drop(&mut self) {
        self.stop();
    }
}
