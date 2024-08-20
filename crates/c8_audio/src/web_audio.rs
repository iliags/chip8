use wasm_bindgen::prelude::*;
use web_sys::{console, AudioBufferSourceNode};

use crate::audio_settings::AudioSettings;

#[derive(Debug)]
pub(crate) struct WebAudio {
    pub(crate) context: web_sys::AudioContext,
    pub(crate) source: Option<AudioBufferSourceNode>,
}

impl WebAudio {
    pub(crate) fn new() -> Self {
        let context = web_sys::AudioContext::new().unwrap_or_else(|_| {
            let message = "Failed to create an AudioContext";
            console::log_1(&message.into());
            panic!("{}", message);
        });

        Self {
            context,
            source: None,
        }
    }

    pub(crate) fn play_beep(&mut self, audio_settings: AudioSettings) {
        if self.source.is_some() {
            return;
        }

        let result = play_beep(
            self.context.clone(),
            audio_settings.volume,
            audio_settings.frequency,
        );

        self.source = Some(result);
    }

    pub(crate) fn stop(&mut self) {
        stop_source(self.source.take());
    }

    pub(crate) fn play_buffer(
        &mut self,
        audio_settings: AudioSettings,
        buffer: Vec<u8>,
        buffer_pitch: f32,
    ) {
        if self.source.is_some() {
            return;
        }

        let result = play_buffer(
            self.context.clone(),
            audio_settings.volume,
            buffer,
            buffer_pitch,
        );

        self.source = Some(result);
    }
}

#[wasm_bindgen]
pub fn stop_source(source: Option<AudioBufferSourceNode>) {
    if let Some(source) = source {
        source.stop().unwrap();
    }
}

#[wasm_bindgen]
pub fn play_beep(
    context: web_sys::AudioContext,
    volume: f32,
    frequency: f32,
) -> AudioBufferSourceNode {
    // The duration is longer as the sound should be stopped by the CPU timer
    let duration = 2.0;
    let buffer_size = (context.sample_rate() * duration) as usize;

    let buffer = context
        .create_buffer(1, buffer_size as u32, context.sample_rate())
        .unwrap_or_else(|_| {
            let message = "Failed to create an AudioBuffer";
            console::error_1(&message.into());
            panic!("{}", message);
        });

    // TODO: Check if data creation can be done once instead of every time beep is called
    let mut data = buffer.get_channel_data(0).unwrap();

    for (i, block) in data.iter_mut().enumerate().take(buffer_size) {
        let t = i as f32 / context.sample_rate();
        *block = volume * (2.0 * std::f32::consts::PI * frequency * t).sin();
    }

    // TODO: Catch errors
    let _ = buffer.copy_to_channel(&data, 0);

    let source = context.create_buffer_source().unwrap();
    source.set_buffer(Some(&buffer));

    // Connect the source to the audio context's destination (speakers)
    source
        .connect_with_audio_node(&context.destination())
        .unwrap_or_else(|_| {
            let message = "Failed to create an AudioBufferSourceNode";
            console::error_1(&message.into());
            panic!("{}", message);
        });

    // Start playback
    source.start().unwrap_or_else(|_| {
        let message = "Failed to start the audio source";
        console::error_1(&message.into());
        panic!("{}", message);
    });

    source
}

#[wasm_bindgen]
pub fn play_buffer(
    context: web_sys::AudioContext,
    volume: f32,
    audio_buffer: Vec<u8>,
    buffer_pitch: f32,
) -> AudioBufferSourceNode {
    // The duration is longer as the sound should be stopped by the CPU timer
    let buffer_sample = context.sample_rate() as f32 / buffer_pitch;

    // TODO: Check if the 8.0 is necessary
    let duration = audio_buffer.len() as f32 * 8.0;
    let buffer_size = (buffer_sample * duration) as usize;

    let buffer = context
        .create_buffer(1, buffer_size as u32, context.sample_rate())
        .unwrap_or_else(|_| {
            let message = "Failed to create an AudioBuffer";
            console::error_1(&message.into());
            panic!("{}", message);
        });

    let mut data = buffer.get_channel_data(0).unwrap();

    for byte in audio_buffer.iter() {
        // V1
        let value = *byte as f32 / 255.0;
        for block in data.iter_mut().take(buffer_size) {
            *block = value;
        }

        // Yay, audio wizardry!
        // (no idea how this works)
        /*
        for idx_bit in 0..8 {
            let bit = byte >> (7 - idx_bit) & 0b1 == 0b1;
            //let value = if bit { volume } else { 0.0 };
            let value = if bit { 1.0 } else { 0.0 };
            for block in data.iter_mut().take(buffer_size) {
                *block = value;
            }
        }
         */
    }

    // TODO: Catch errors
    let _ = buffer.copy_to_channel(&data, 0);

    let source = context.create_buffer_source().unwrap();
    source.set_buffer(Some(&buffer));

    // Connect the source to the audio context's destination (speakers)
    source
        .connect_with_audio_node(&context.destination())
        .unwrap_or_else(|_| {
            let message = "Failed to create an AudioBufferSourceNode";
            console::error_1(&message.into());
            panic!("{}", message);
        });

    // Start playback
    source.start().unwrap_or_else(|_| {
        let message = "Failed to start the audio source";
        console::error_1(&message.into());
        panic!("{}", message);
    });

    source
}
