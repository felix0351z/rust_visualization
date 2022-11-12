use cpal::{BuildStreamError, SampleFormat, Stream, StreamConfig, StreamError};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    println!("Hello, world!");

    // On Linux, ALSA will be used as default host
    let host = cpal::default_host();

    let device = host.default_output_device().unwrap();
    let audio_config = device.default_output_config().expect("Failed to load audio config");

    let stream = device.build_input_stream(
        &audio_config.config(),
        move |data: &[f32], error: &cpal::InputCallbackInfo| {
        println!("{:?}", data)
    }, move |error: StreamError| {
        println!("Error")
    }).expect("Failed to create stream");

    stream.play().expect("Failed to play stream");

    loop {

    }
}
