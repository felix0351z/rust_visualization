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
                println!("{:?}", start.elapsed());

                let mut max = 0;
                let mut min = 0;
                for (index, &value) in data.iter().enumerate() {
                    if max > value { max = value  }
                    if min < value { min = value }
                }
                println!("Length of current frame: {:?}", data.len());

                println!("Max amplitude: {:?}", max);
                println!("Min amplitude: {:?}", min);

            }
        },
        |err| {
            println!("Stream error")
        }
    ).unwrap();

    input_source.start_stream().unwrap();
    loop {}
}