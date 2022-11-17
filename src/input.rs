use std::error::Error;
    use cpal::{ChannelCount, DefaultStreamConfigError, Device, Host, InputCallbackInfo, Stream, StreamConfig, StreamError};
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use thiserror::Error;
    use crate::input::InputSourceError::*;

    //TODO: Bufferalgorithmus auslagern


    // The default frame on WASAPI is 100 FPS.
    const CAPTURE_FRAME_RATE: u32 = 100;
    // The default streaming rate
    const DISPLAY_FRAME_RATE: u32 = 50;
    //the difference has to be an int.


    /// Important errors for interaction with an *DeviceInputSource*
    #[derive(Error, Debug)]
    pub enum InputSourceError {

        /// Will be returned if there is no Device currently selected(*None*) in the InputSource
        #[error("No input device is currently selected")]
        NoDeviceSelected,

        /// Will be returned when function was called which needs an InputStream, but no InputStream is available
        #[error("No input stream found")]
        NoStream,

        // Will be returned if a stereo stream want to be build, but there is no stereo channel available
        #[error("No Stereo channel available")]
        NoStereoChannel,
    }


    /// Describes a device with all necessary information's to decide,
    /// which device should be used.
    pub struct DeviceInfo {
        /// The position of the device referred to the host.
        position: usize,

        /// Display name of the device
        name: String,

        /// Amount of channels the device has. Mostly stereo or mono
        channels: u16,

        /// SampleRate of the device. Important for visualization.
        /// Lower sample rate means less information to display or a smaller refresh rate
        sample_rate: u32
    }

    /// Create an interaction with the pc's audio input
    /// Provides all necessary audio device information's
    /// and callbacks to react on the audio data.
    pub struct DeviceInputSource {
        /// Audio host of the pc. Windows => WASAPI
        host: Host,

        /// Selected audio device for interaction
        device: Option<Device>,

        /// Own input stream.
        input_stream: Option<Stream>,
    }


    impl DeviceInputSource {

        /// Build new InputSource with default audio device.
        pub fn new() -> Self {
            let host = cpal::default_host();
            let device = host.default_input_device();

            DeviceInputSource {
                host,
                device,
                input_stream: None,
            }
        }

        /// Get the current device name
        pub fn current_device_name(&self) -> Result<String, InputSourceError> {
            match &self.device {
                None => Err(NoDeviceSelected),
                Some(device) => Ok(device.safe_name())
            }
        }

        /// Gets all available devices
        pub fn available_devices(&self) -> Result<Vec<DeviceInfo>, Box<dyn Error>> {
            let mut vec: Vec<DeviceInfo> = vec![];
            let devices= self.host
                .input_devices()?;

            // Iterate over all available input devices
            for (i, device) in devices.enumerate() {
                let name = device.safe_name();
                let config = device.default_input_config()?.config();

                vec.push(DeviceInfo {
                    position: i,
                    name,
                    channels: config.channels,
                    sample_rate: config.sample_rate.0
                })
            }

            Ok(vec)
        }

        /// Set the default device as input device
        pub fn set_default_device(&mut self) {
            self.device = self.host.default_input_device()
        }

        /// Set device at position as current device
        pub fn set_device(&mut self, position: usize) -> Result<(), Box<dyn Error>> {
            let mut devices = self.host.input_devices()?;
            let device = devices.nth(position);

            Ok(self.device = device)
        }

        /// Get the current device
        pub fn current_device(&self) -> Result<&Device, InputSourceError> {
            self.device.as_ref().ok_or(NoDeviceSelected)
        }

        /*fn build_stereo_stream<C, E>(
            &mut self,
            mut callback: C,
            mut error_callback: E
        ) -> Result<(), Box<dyn Error>>
            where
                C: FnMut(&[f32], &InputCallbackInfo) + Send + 'static,
                E: FnMut(StreamError) + Send + 'static
        {
        }*/


        /// Build a new input stream with a fixed configuration, frame_length and frame_capture_size
        pub fn build_mono_stream<C, E>(
            &mut self,
            mut callback: C,
            mut error_callback: E
        ) -> Result<(), Box<dyn Error>>
            where
                C: FnMut(&[f32], &InputCallbackInfo) + Send + 'static,
                E: FnMut(StreamError) + Send + 'static
         {
             // Return a NoDevice error when no device will be found or a DefaultStreamConfigError if no configuration will be found
             let device = self.current_device()?;
             let configuration = device.supported_stream_configuration()?;

             // The frame length defines a pack of samples. We need as much frames as in FRAME_RATE declared.
             // So we split the Samples to the FRAME_RATE
             let frame_length = (configuration.sample_rate.0 / CAPTURE_FRAME_RATE) as usize;

             // Frame capture_size defines how many frames will be captured in 1 period
             // The display rate is set to 100fps and the streaming rate is 50fps. So we need 2 frames for each period.
             let frame_capture_size = (CAPTURE_FRAME_RATE/DISPLAY_FRAME_RATE) as usize;




             // Create a new buffer which needs to contain as much frames as frame_capture_size
             let mut buffer = vec![0 as f32; frame_length*frame_capture_size];
             let mut buffer_step = false;

            self.input_stream = Some(device.build_input_stream(
                &configuration,
                move |data: &[f32], info: &InputCallbackInfo| {

                    // Creates an iterator which only collects one channel
                    let iter = data.iter().step_by(2).cloned();

                    match buffer_step {
                        false => {
                            buffer.splice(..frame_length, iter);
                        }
                        true => {
                            buffer.splice(frame_length.., iter);
                            callback(buffer.as_slice(), info)
                        }
                    }


                    //TODO New buffer algorithm
                    //for (i, value) in data.iter().step_by(2).enumerate() {
                    //    buffer[i/2] = *value
                    //}

                    buffer_step = !buffer_step;
                },
                move |error: StreamError| {
                    error_callback(error)
                })?);

            Ok(())
        }

        /// Start the current stream
        pub fn start_stream(&self) -> Result<(), Box<dyn Error>> {
            match &self.input_stream {
                // Return NoStream if no stream is selected
                None => Err(Box::new(NoStream)),
                Some(it) => Ok(it.play()?)
            }
        }




    }



    /// Utilities to interact better with the device.
    trait DeviceUtilities {
        /// Get the name of with <Unknown> instead of an error
        fn safe_name(&self) -> String;
        /// Get the supported stream config
        fn supported_stream_configuration(&self) -> Result<StreamConfig, DefaultStreamConfigError>;
    }

    impl DeviceUtilities for Device {

        fn safe_name(&self) -> String {
            match self.name() {
                Ok(name) => name,
                Err(_) => String::from("<Unknown>")
            }
        }

        fn supported_stream_configuration(&self) -> Result<StreamConfig, DefaultStreamConfigError> {
            let configuration = self.default_input_config()?;
            Ok(configuration.config())
        }

    }

