#![allow(missing_docs)]
#![allow(dead_code)]

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
