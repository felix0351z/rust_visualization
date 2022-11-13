pub mod input {
    use std::borrow::{Borrow, BorrowMut};
    use std::error::Error;
    use cpal::{InputCallbackInfo, StreamError};
    use cpal::traits::{DeviceTrait, HostTrait};

    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum InputSourceError {
        #[error("No input device found")]
        NoDevice,
    }


    pub struct DeviceInputSource {
        host: cpal::Host,
        device: Option<cpal::Device>,
        input_stream: Option<cpal::Stream>,

        buffer: Vec<f32>,
    }

    impl DeviceInputSource {

        pub fn new() -> Self {
            let host = cpal::default_host();
            let device = host.default_input_device();
            // Buffer with a size of 960 samples
            let buffer: Vec<f32> = vec![0.0; 960];

            DeviceInputSource {
                host,
                device,
                input_stream: None,
                buffer
            }
        }

        //current_device

        //list_devices

        //set_device

        pub fn start_stream(mut self) -> Result<(), Box<dyn Error>> {
            // Return a NoDevice error when no device will be found
            let device = self.device.ok_or(InputSourceError::NoDevice)?;
            // Return a DefaultStreamConfigError when no configuration will be found
            let configuration = device.default_input_config()?.config();


            self.input_stream = Some(device.build_input_stream(
                &configuration,
                move |data: &[f32], error: &InputCallbackInfo| {

                    //self.buffer.splice(0..480, data.iter().cloned());
                    //TODO Buffering
                },
                move |error: StreamError| {
                    //TODO
                })?
            );

            Ok(())
        }




    }









}