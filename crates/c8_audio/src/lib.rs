//! Audio utilities for the CHIP-8 emulator.

// Remove when the module is more complete.
#![allow(missing_docs, dead_code)]

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BackendSpecificError, BuildStreamError, FromSample, Sample, SizedSample, Stream,
};

use std::{
    cell::Cell,
    rc::Rc,
    sync::mpsc::{Receiver, Sender},
};

#[derive(Debug)]
pub enum Message {
    Play,
    Pause,
    Stop,
}

#[derive(Default)]
pub struct Beeper {
    sender: Option<Sender<Message>>,

    stream_cell: Rc<Cell<Option<Stream>>>,
}

impl std::fmt::Debug for Beeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Beeper {{ sender: {:?} }}", self.sender)
    }
}

impl Beeper {
    pub fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            Beeper::default()
            /*
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
                            console::log_1(&"Creating Beeper".into());
                            stream.set(Some(device));
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

            return beeper;
             */
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::sync::mpsc;
            use std::thread;

            let (sender, receiver): (Sender<Message>, Receiver<Message>) = mpsc::channel();

            thread::spawn(move || {
                let device = Self::create_stream_device();

                Self::stream_audio(receiver, device);
            });

            return Beeper {
                sender: Some(sender),
                stream_cell: Rc::new(Cell::new(None)),
            };
        }
    }

    pub fn play(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        let _ = match self.sender {
            Some(ref sender) => sender.send(Message::Play),
            None => Ok(()),
        };
    }

    pub fn pause(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        let _ = match self.sender {
            Some(ref sender) => sender.send(Message::Pause),
            None => Ok(()),
        };
    }

    pub fn stop(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        let _ = match self.sender {
            Some(ref sender) => sender.send(Message::Stop),
            None => Ok(()),
        };
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
        match self.sender {
            Some(ref sender) => {
                let _ = sender.send(Message::Stop).unwrap_or_else(|e| {
                    eprintln!("Error sending stop message: {}", e);
                });
            }
            None => {}
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
