use std::time::Instant;
use visualization_test::engine::input::DeviceInputSource;

#[test]
fn test_input_speed() {
    let mut input_source = DeviceInputSource::new();

    let start = Instant::now();
    let mut count = 0;

    input_source.build_mono_stream(
        move |data, info| {

            // Count to 50 and print the current time
            count+=1;
            if count == 50 {
                count = 0;
                println!("{:?}", start.elapsed())
            }
        },
        |err| {
            println!("Stream error")
        }
    ).unwrap();

    input_source.start_stream().unwrap();
    loop {}
}