use std::thread::sleep;
use std::time::Duration;
use visualization_test::engine::Engine;
use visualization_test::engine::input::DeviceInfo;

use anyhow::Result;
const LEDS: usize = 60;


fn print_device(device: &DeviceInfo) {
    println!("Device {} at position {} detected. With {} channels, and sample rate of {} | Standard: {}", device.name, device.position, device.channels, device.sample_rate, device.standard);
}

#[test]
fn test_available_devices() -> Result<()> {
    let engine = Engine::new(LEDS);
    let devices = engine.get_available_devices();

    match devices {
        Ok(value) => {
            for device in value.iter() {
                print_device(device)
            }
            Ok(())
        }
        Err(err) => {
            Err(err)
        }
    }

}

#[test]
fn test_set_device() -> Result<()>{
    let mut engine = Engine::new(LEDS);
    let devices = engine.get_available_devices();


    match devices {
        Ok(value) => {
            for (i, device) in value.iter().enumerate() {
                // Falls es nicht das Standard gerät ist:
                if !device.standard {
                    print_device(device);
                    engine.set_device(i)?
                }

            }
            Ok(())


        }
        Err(err) =>Err(err),
    }




}

#[test]
fn test_get_effects() -> () {
    let engine = Engine::new(LEDS);

    let effects = engine.get_effects();

    for effect in effects.iter() {
        println!("Effect {} with {} | {}", effect.name, effect.icon, effect.domain);
    }

}

#[test]
fn test_set_effect() {

}


#[test]
fn test_get_filters() {
    let engine = Engine::new(LEDS);

    let filters = engine.get_filters();

    for filter in  filters.iter() {
        println!("Filter {} | {}", filter.name, filter.domain)
    }
}

#[test]
fn test_set_filter() {


}


#[test]
fn test_runtime() -> Result<()> {
    let mut engine = Engine::new(LEDS);
    engine.update_stream()?; //Start the stream!

    sleep(Duration::from_secs(5));
    Ok(())
}

#[test]
fn test_pause() -> Result<()> {
    let mut engine = Engine::new(LEDS);
    engine.update_stream()?;

    sleep(Duration::from_secs(2));
    engine.pause_stream()?;
    sleep(Duration::from_secs(2));

    Ok(())
}





