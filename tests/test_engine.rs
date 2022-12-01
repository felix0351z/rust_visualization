use visualization_test::engine::Engine;
use visualization_test::engine::errors::{InputError, ProgramError};
use visualization_test::engine::input::DeviceInfo;

use error_stack::Result;


fn print_device(device: &DeviceInfo) {
    println!("Device {} at position {} detected. With {} channels, and sample rate of {} | Standard: {}", device.name, device.position, device.channels, device.sample_rate, device.standard);
}

#[test]
fn test_available_devices() -> Result<(), InputError> {
    let engine = Engine::new(60);
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
fn test_set_device() -> Result<(), ProgramError>{
    let mut engine = Engine::new(60);
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


        } ,
        Err(err) => Err(err.change_context(ProgramError::InputError)),
    }




}




