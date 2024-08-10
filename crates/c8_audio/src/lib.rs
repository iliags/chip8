//! Audio utilities for the CHIP-8 emulator.

// Remove when the module is more complete.
#![allow(missing_docs, dead_code)]

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BackendSpecificError, BuildStreamError, FromSample, Sample, SizedSample, Stream,
};

use std::sync::mpsc::{self, Receiver, Sender};

pub enum Message {
    Play,
    Pause,
    Stop,
}

pub struct Beeper {
    sender: Sender<Message>,
}

impl Beeper {
    pub fn new() -> Self {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("no output device available");

        let config = device
            .default_output_config()
            .expect("no default output config");

        let (sender, receiver): (Sender<Message>, Receiver<Message>) = mpsc::channel();

        //#[cfg(not(target_arch = "wasm32"))]
        {
            use std::thread;
            thread::spawn(move || {
                Self::stream_audio(receiver, device, config);
            });
        }

        //#[cfg(target_arch = "wasm32")]
        {}

        Beeper { sender }
    }

    pub fn play(&self) {
        let _ = self.sender.send(Message::Play);
    }

    pub fn pause(&self) {
        let _ = self.sender.send(Message::Pause);
    }

    pub fn stop(&self) {
        let _ = self.sender.send(Message::Stop);
    }

    fn stream_audio(
        receiver: Receiver<Message>,
        device: cpal::Device,
        config: cpal::SupportedStreamConfig,
    ) {
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => Self::create_stream::<f32>(&device, &config.into()),
            cpal::SampleFormat::I16 => Self::create_stream::<i16>(&device, &config.into()),
            cpal::SampleFormat::U16 => Self::create_stream::<u16>(&device, &config.into()),
            sample_format => Err(BuildStreamError::BackendSpecific {
                err: BackendSpecificError {
                    description: format!("Unsupported sample format '{sample_format}'"),
                },
            }),
        };

        match stream {
            Ok(stream) => {
                let _ = stream.pause();

                // TODO: Implement message passing.

                loop {
                    match receiver.recv() {
                        Ok(Message::Play) => {
                            let _ = stream.play();
                        }
                        Ok(Message::Pause) => {
                            let _ = stream.pause();
                        }
                        Ok(Message::Stop) => {
                            let _ = stream.pause();
                            return;
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

        // TODO
        //Ok(Self { sender: s })
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
            (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
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

impl Drop for Beeper {
    fn drop(&mut self) {
        self.sender.send(Message::Stop).unwrap_or_else(|e| {
            eprintln!("Error sending stop message: {}", e);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_beeper() {
        let beeper = Beeper::new();

        beeper.play();
        std::thread::sleep(std::time::Duration::from_secs(1));
        beeper.pause();
        std::thread::sleep(std::time::Duration::from_secs(1));
        beeper.play();
        std::thread::sleep(std::time::Duration::from_secs(1));
        beeper.stop();
    }
}
