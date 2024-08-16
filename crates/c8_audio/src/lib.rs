//! Audio utilities for the CHIP-8 emulator.

#![allow(dead_code)]

// TODO: WASM audio has popping when pausing then playing the audio.

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BackendSpecificError, BuildStreamError, FromSample, Sample, SizedSample, Stream,
};

use std::{
    cell::OnceCell,
    rc::Rc,
    sync::mpsc::{Receiver, Sender},
};

/// Beeper settings
#[derive(Debug, Clone, Copy)]
pub struct BeeperSettings {
    /// Tone pitch
    pub pitch: f32,

    /// Tone octave
    pub octave: f32,

    /// Tone volume
    pub volume: f32,
}

impl Default for BeeperSettings {
    fn default() -> Self {
        BeeperSettings {
            pitch: 440.0,
            octave: 2.0,
            volume: 0.05,
        }
    }
}

/// Messages for the beeper
#[derive(Debug, PartialEq)]
pub enum Message {
    /// Play the audio
    Play,
    /// Pause the audio
    Pause,
    /// Stop the audio
    Stop,
    /// Update the audio settings
    Update,
}

/// Beeper
#[derive(Default)]
pub struct Beeper {
    /// Sender used on non-wasm32 targets
    sender: Option<Sender<Message>>,

    /// Stream cell used on wasm32 targets
    stream_cell: Rc<OnceCell<Option<Stream>>>,

    // TODO: Make thread safe
    /// Beeper settings
    pub settings: BeeperSettings,
}

impl std::fmt::Debug for Beeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Beeper {{ sender: {:?} }}", self.sender)
    }
}

impl Beeper {
    /// Create a new beeper instance
    pub fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::prelude::*;
            use web_sys::{window, HtmlElement};

            use web_sys::console;

            let beeper = Beeper::default();

            let stream = beeper.stream_cell.clone();

            {
                let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
                    let device = Self::create_stream_device();
                    match device {
                        Ok(device) => {
                            match stream.set(Some(device)) {
                                Ok(_) => {
                                    console::log_1(&"Stream set".into());
                                }
                                Err(_) => {
                                    // Trying to set the stream again will always fail because of OnceCell
                                }
                            }
                        }
                        Err(e) => {
                            let error = format!("Error creating stream device: {:?}", e);
                            console::log_1(&error.into());
                        }
                    }
                });

                if let Some(window) = window() {
                    if let Some(document) = window.document() {
                        if let Some(body) = document.body() {
                            let body: HtmlElement = body.into();
                            body.set_onclick(Some(closure.as_ref().unchecked_ref()));
                        }
                    }
                }

                closure.forget();
            }

            beeper
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::sync::mpsc;
            use std::thread;

            let mut beeper = Beeper::default();

            let (sender, receiver): (Sender<Message>, Receiver<Message>) = mpsc::channel();

            thread::spawn(move || {
                let device = Self::create_stream_device();

                Self::stream_audio(receiver, device);
            });

            beeper.sender = Some(sender);

            beeper
        }
    }

    /// Play the audio
    pub fn play(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        let _ = match self.sender {
            Some(ref sender) => sender.send(Message::Play),
            None => Ok(()),
        };

        #[cfg(target_arch = "wasm32")]
        match self.stream_cell.get() {
            Some(stream) => match stream {
                Some(ref stream) => {
                    let _ = stream.play();
                }
                None => {}
            },
            None => {}
        }
    }

    /// Pause the audio
    pub fn pause(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        let _ = match self.sender {
            Some(ref sender) => sender.send(Message::Pause),
            None => Ok(()),
        };

        #[cfg(target_arch = "wasm32")]
        match self.stream_cell.get() {
            Some(stream) => match stream {
                Some(ref stream) => {
                    let _ = stream.pause();
                }
                None => {}
            },
            None => {}
        }
    }

    /// Stop the audio
    pub fn stop(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        let _ = match self.sender {
            Some(ref sender) => sender.send(Message::Stop),
            None => Ok(()),
        };

        #[cfg(target_arch = "wasm32")]
        match self.stream_cell.get() {
            Some(stream) => match stream {
                Some(ref stream) => {
                    let _ = stream.pause();
                }
                None => {}
            },
            None => {}
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

        let settings = BeeperSettings::default();

        match config.sample_format() {
            cpal::SampleFormat::F32 => {
                Self::create_stream::<f32>(&device, &config.into(), settings)
            }
            cpal::SampleFormat::I16 => {
                Self::create_stream::<i16>(&device, &config.into(), settings)
            }
            cpal::SampleFormat::U16 => {
                Self::create_stream::<u16>(&device, &config.into(), settings)
            }
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
                        Ok(Message::Update) => {
                            // TODO: Implement update stream with change detection
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
    }

    // TODO: Create update stream method
    fn create_stream<T>(
        device: &cpal::Device,
        config: &cpal::StreamConfig,
        settings: BeeperSettings,
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
            (sample_clock * settings.pitch * settings.octave * std::f32::consts::PI / sample_rate)
                .sin()
                * settings.volume
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
        if let Some(ref sender) = self.sender {
            sender.send(Message::Stop).unwrap_or_else(|e| {
                eprintln!("Error sending stop message: {}", e);
            });
        }
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
