extern crate core;
mod input;

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use crate::input::MonoDeviceInputSource;


    //run with --nocapture to see the output
    #[test]
    fn test_input_speed() {
        let mut input_source = MonoDeviceInputSource::new();

        let start = Instant::now();
        let mut count = 0;

        input_source.build_stream(
            480,
            2,
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