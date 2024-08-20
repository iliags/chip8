#![allow(dead_code, unused_variables)]

use crate::{audio_settings::AudioSettings, SoundDevice};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BackendSpecificError, BuildStreamError, FromSample, Sample, SizedSample, Stream,
};

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

/// Messages for the desktop audio system
#[derive(Debug, PartialEq)]
enum Message {
    PlayBeep,
    PlayBuffer,
    Pause,
    Stop,
    Update,
}

#[derive(Default)]
pub struct DesktopAudio {
    sender: Option<Sender<Message>>,
    stream: Option<Stream>,
}

impl std::fmt::Debug for DesktopAudio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sender {{ sender: {:?} }}", self.sender)
    }
}

impl SoundDevice for DesktopAudio {
    fn play_beep(&mut self, audio_settings: AudioSettings) {
        if let Some(sender) = &self.sender {
            let _ = sender.send(Message::PlayBeep);
        }
    }
    fn play_buffer(&mut self, audio_settings: AudioSettings, buffer: Vec<u8>, buffer_pitch: f32) {
        if let Some(sender) = &self.sender {
            let _ = sender.send(Message::PlayBuffer);
        }
    }
    fn pause(&mut self) {
        if let Some(sender) = &self.sender {
            let _ = sender.send(Message::Pause);
        }
    }
    fn stop(&mut self) {
        if let Some(sender) = &self.sender {
            let _ = sender.send(Message::Stop);
        }
    }
}

impl DesktopAudio {
    pub fn new() -> Self {
        let (sender, receiver): (Sender<Message>, Receiver<Message>) = mpsc::channel();

        let mut audio_device = DesktopAudio::default();

        thread::spawn(move || {
            let device = Self::create_stream_device();

            Self::stream_audio(receiver, device);
        });

        audio_device.sender = Some(sender);

        audio_device
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
            cpal::SampleFormat::F32 => Self::create_stream::<f32>(&device, &config.into()),
            cpal::SampleFormat::I16 => Self::create_stream::<i16>(&device, &config.into()),
            cpal::SampleFormat::U16 => Self::create_stream::<u16>(&device, &config.into()),
            sample_format => Err(BuildStreamError::BackendSpecific {
                err: BackendSpecificError {
                    description: format!("Unsupported sample format '{sample_format}'"),
                },
            }),
        }
    }

    fn stream_audio(receiver: Receiver<Message>, stream: Result<Stream, BuildStreamError>) {
        match stream {
            Ok(stream) => {
                loop {
                    match receiver.recv() {
                        Ok(Message::PlayBeep) => {
                            let _ = stream.play();
                        }
                        Ok(Message::PlayBuffer) => {
                            // TODO
                        }
                        Ok(Message::Pause) => {
                            let _ = stream.pause();
                        }
                        Ok(Message::Stop) => {
                            let _ = stream.pause();
                            return;
                        }
                        Ok(Message::Update) => {
                            // TODO: Implement update stream with change detection
                            let _ = stream.pause();
                            //return;
                        }
                        Err(e) => {
                            eprintln!("Receive error: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("BuildStreamError {:?}", e);
            }
        }
    }

    // TODO: Create update stream method
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
            let volume = 0.05;
            let octave = 2.0;

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
        if let Some(ref sender) = self.sender {
            sender.send(Message::Stop).unwrap_or_else(|e| {
                eprintln!("Error sending stop message: {}", e);
            });
        }
    }
}
