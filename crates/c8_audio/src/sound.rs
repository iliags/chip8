#![allow(missing_docs)]
#![allow(dead_code)]

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BackendSpecificError, BuildStreamError, FromSample, Sample, SizedSample, Stream,
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn play_beep() {
    let context = web_sys::AudioContext::new().unwrap();
    let buffer_size = context.sample_rate() as usize * 2; // Adjust buffer size as needed

    // Fill an audio buffer with beep data (similar to create_stream)

    //let buffer = web_sys::AudioBuffer::new(1, buffer_size as u32, context.sample_rate()).unwrap();
    let buffer = context
        .create_buffer(1, buffer_size as u32, context.sample_rate())
        .unwrap();

    let frequency = 440.0;
    let volume = 0.1;
    let duration = 0.5;

    let mut data = buffer.get_channel_data(0).unwrap();
    for i in 0..buffer_size {
        let t = i as f32 / context.sample_rate() as f32;
        data[i] = (volume * (2.0 * std::f32::consts::PI * frequency * t).sin()) as f32;
    }

    let _ = buffer.copy_to_channel(&data, 0);

    //let source = web_sys::AudioBufferSourceNode::new(&buffer).unwrap();
    let source = context.create_buffer_source().unwrap();
    source.set_buffer(Some(&buffer));

    // Connect the source to the audio context's destination (speakers)
    source
        .connect_with_audio_node(&context.destination())
        .unwrap();

    // Start playback
    source.start().unwrap();
}
/*
#[derive(Default)]
pub struct SoundDevice {
    stream: Option<Stream>,
}

impl SoundDevice {
    pub fn new() -> Self {
        SoundDevice::default()
    }

    pub fn play(&mut self) {
        match self.stream {
            Some(ref stream) => {
                stream.play().unwrap();
            }
            None => match create_stream_device() {
                Ok(stream) => {
                    self.stream = Some(stream);
                }
                Err(err) => {
                    eprintln!("An error occurred: {}", err);
                }
            },
        }
    }

    pub fn pause(&mut self) {
        if let Some(stream) = &self.stream {
            stream.pause().unwrap();
        }
    }
}

impl std::fmt::Debug for SoundDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Device",)
    }
}

fn create_stream_device() -> Result<Stream, BuildStreamError> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("no output device available");

    let config = device
        .default_output_config()
        .expect("no default output config");

    match config.sample_format() {
        cpal::SampleFormat::F32 => create_stream::<f32>(&device, &config.into()),
        cpal::SampleFormat::I16 => create_stream::<i16>(&device, &config.into()),
        cpal::SampleFormat::U16 => create_stream::<u16>(&device, &config.into()),
        sample_format => Err(BuildStreamError::BackendSpecific {
            err: BackendSpecificError {
                description: format!("Unsupported sample format '{sample_format}'"),
            },
        }),
    }
}

fn create_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
) -> Result<Stream, BuildStreamError>
where
    T: SizedSample + FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        let pitch = 440.0;
        let octave = 2.0;
        let volume = 0.1;

        (sample_clock * pitch * octave * std::f32::consts::PI / sample_rate).sin() * volume
    };

    let err_fn = |err| eprintln!("Stream error: {}", err);

    device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
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
 */
