mod engine;
mod errors;
mod input;

use cpal::{BufferSize, BuildStreamError, SampleFormat, SampleRate, Stream, StreamConfig, StreamError, SupportedBufferSize};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crate::engine::engine::Engine;

fn main() {
    println!("Hello, world!");

    let engine = Engine::new();

    // On Linux, ALSA will be used as default host
    let host = cpal::default_host();
    let devices = host.input_devices().unwrap();

    for device in devices {
        println!("{}", device.name().expect("Unknown"))
    }



    let device = host.default_input_device().unwrap();
    let audio_config = device.default_input_config().expect("Failed to load audio config");


    println!("DEFAULT_DEVICE: {:?}", device.name());

    let configs = device.supported_input_configs().expect("Failed to get supported configs");
    for config in configs {
        println!("Channels: {}", config.channels());
        println!("Samplerate: {:?}, {:?}", config.min_sample_rate(), config.max_sample_rate());
        println!("BufferSize: {:?}", config.buffer_size());
        println!("-------------")
    }


    println!("Sample Rate: {}", audio_config.config().sample_rate.0);
    println!("Amount channels: {}", audio_config.config().channels);




   match audio_config.buffer_size() {
        SupportedBufferSize::Range { min, max } => {
            println!("Buffer Range: {}-{}", min, max)
        }
        SupportedBufferSize::Unknown => {
            println!("Buffer range unknown")
        }
    }

    let stream = device.build_input_stream(
        &StreamConfig {
            channels: 2,
            sample_rate: SampleRate(48000),
            buffer_size: BufferSize::Fixed(1920)
        },
        move |data: &[f32], error: &cpal::InputCallbackInfo| {
            println!("{:?}", data);
            println!("Length: {}", data.len());
    }, move |error: StreamError| {
            println!("Error")
    }).expect("Failed to create stream");

    stream.play().expect("Failed to play stream");
    loop {

    }
}
