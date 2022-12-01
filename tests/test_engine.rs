use visualization_test::engine::Engine;

#[test]
fn test_available_devices() -> Result<(), String> {
    let engine = Engine::new(60);
    let devices = engine.get_available_devices();

    match devices {
        Ok(value) => {
            for device in value.iter() {
                println!("Device {} at position {} detected. With {} channels, and sample rate of {} | Standard: {}", device.name, device.position, device.channels, device.sample_rate, device.standard);
            }
            Ok(())
        }
        Err(err) => {
            Err(err.to_string())
        }
    }

}




