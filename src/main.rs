use cpal::{BufferSize, BuildStreamError, SampleFormat, SampleRate, Stream, StreamConfig, StreamError, SupportedBufferSize};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    println!("Hello, world!");

    // On Linux, ALSA will be used as default host
    let host = cpal::default_host();

    let device = host.default_output_device().unwrap();
    let audio_config = device.default_output_config().expect("Failed to load audio config");

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
            buffer_size: BufferSize::Default
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
