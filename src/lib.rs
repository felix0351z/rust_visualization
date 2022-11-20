extern crate core;
mod engine;

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use crate::engine::input::DeviceInputSource;


    //run with --nocapture to see the output
    #[test]
    fn test_input_speed() {
        let mut input_source = DeviceInputSource::new();

        let start = Instant::now();
        let mut count = 0;

        input_source.build_mono_stream(
            move |data, info| {
                //println!("{:?}", data);
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











}