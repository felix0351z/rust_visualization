use std::error::Error;
    use cpal::{Device, Host, InputCallbackInfo, Stream, StreamConfig, StreamError};
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use thiserror::Error;
    use crate::input::InputSourceError::*;


    #[derive(Error, Debug)]
    pub enum InputSourceError {
        #[error("No input device is currently selected")]
        NoDeviceSelected,
        #[error("Device not found")]
        NoDeviceFound,
        #[error("No input stream found")]
        NoStream,
        #[error("Failed to load the device configuration")]
        FailedConfigurationLoad,
        #[error("CPAL: {0}")]
        CPALError(String)
    }


    pub struct DeviceInfo {
        position: usize,
        name: String,
        channels: u16,
        sample_rate: u32
    }

    pub struct MonoDeviceInputSource {
        host: Host,
        device: Option<Device>,
        input_stream: Option<Stream>,
    }

    impl MonoDeviceInputSource {

        pub fn new() -> Self {
            let host = cpal::default_host();
            let device = host.default_input_device();

            MonoDeviceInputSource {
                host,
                device,
                input_stream: None,
            }
        }

        /// Get the current device name
        pub fn current_device_name(&self) -> Result<String, InputSourceError> {

            match &self.device {
                None => Err(NoDeviceSelected),
                Some(device) => Ok(device.own_name())
            }

        }


        /// Gets all available devices
        ///
        ///
        pub fn available_devices(&self) -> Result<Vec<DeviceInfo>, Box<dyn Error>> {
            let mut vec: Vec<DeviceInfo> = vec![];
            let devices= self.host
                .input_devices()?;

            // Iterate over all available input devices
            for (i, device) in devices.enumerate() {
                let name = device.own_name();
                let config = device.own_input_config()?;

                vec.push(DeviceInfo {
                    position: i,
                    name,
                    channels: config.channels,
                    sample_rate: config.sample_rate.0
                })
            }

            Ok(vec)
        }

        pub fn set_default_device(&mut self) {
            self.device = self.host.default_input_device()
        }

        pub fn set_device(&mut self, position: usize) -> Result<(), Box<dyn Error>> {
            let mut devices = self.host.input_devices()?;
            let device = devices.nth(position);

            Ok(self.device = device)
        }

        /// Starts a new input stream the collected data in 50fps
        ///
        ///
        ///
        ///
        pub fn build_stream<C, E>(
            &mut self,
            frame_length: usize,
            frame_capture_size: usize,
            mut callback: C,
            mut error_callback: E
        ) -> Result<(), InputSourceError>
            where
                C: FnMut(&[f32], &InputCallbackInfo) + Send + 'static,
                E: FnMut(StreamError) + Send + 'static
         {
            // Return a NoDevice error when no device will be found
            let device = self.device.as_ref().ok_or(NoDeviceSelected)?;

            // Return a DefaultStreamConfigError when no configuration will be found
            let configuration = device.own_input_config()?;

             // Buffer with a fixed size containing as much frames as declared in FRAME_CAPTURE_SIZE
             //let vec: ArrayVec<f32, INPUT_BUFFER_SIZE> = ArrayVec::new();
             let mut buffer = vec![0 as f32; frame_length*frame_capture_size];
             let mut buffer_step = false;

            self.input_stream = Some(device.build_input_stream(
                &configuration,
                move |data: &[f32], info: &InputCallbackInfo| {

                    // Mono iterator
                    let iter = data.iter().step_by(2).cloned();
                    match buffer_step {
                        false => {
                            buffer.splice(frame_length.., iter);
                        }
                        true => {
                            buffer.splice(..frame_length, iter);
                            callback(buffer.as_slice(), info)
                        }
                    }

                    buffer_step = !buffer_step;
                },
                move |error: StreamError| {
                    error_callback(error)
                }).or_else(|err| Err(CPALError(err.to_string())))
            ?);

            Ok(())
        }

        pub fn start_stream(&self) -> Result<(), Box<dyn Error>> {
            match &self.input_stream {
                None => Err(Box::new(NoStream)),
                Some(it) => Ok(it.play()?)
            }
        }


    }





    trait ErrorWrapper {
        fn own_name(&self) -> String;
        fn own_input_config(&self) -> Result<StreamConfig, InputSourceError>;
    }

    impl ErrorWrapper for Device {

        fn own_name(&self) -> String {
            match self.name() {
                Ok(name) => name,
                Err(_) => String::from("<Unknown>")
            }
        }

        fn own_input_config(&self) -> Result<StreamConfig, InputSourceError> {
            match self.default_input_config() {
                Ok(config) => Ok(config.config()),
                Err(_) => Err(FailedConfigurationLoad)
            }

        }

    }

