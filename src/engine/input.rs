use log::warn;
use anyhow::{Context, Result};
use super::errors::ApplicationError;

use crate::engine::utils::{AudioBuffer, BufferInfo};
use crate::ok_or_skip;

use cpal::{DefaultStreamConfigError, Device, Host, InputCallbackInfo, Stream, StreamConfig, StreamError};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};


// The default frame on WASAPI is 100 FPS.
const CAPTURE_FRAME_RATE: u32 = 100;
// The default streaming rate
const DISPLAY_FRAME_RATE: u32 = 50;
//the difference has to be an int.



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

        /// Build new InputSource with default audio device
        /// and automatically select the default input device.
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
        pub fn current_device_name(&self) -> Result<String> {
            match &self.device {
                None => Err(ApplicationError::NoDeviceSelected)?,
                Some(device) => Ok(device.safe_name())
            }
        }

        /// Gets all current available devices
        pub fn available_devices(&self) -> Result<Vec<DeviceInfo>> {
            let mut vec: Vec<DeviceInfo> = vec![];

            let devices = self.host.input_devices()?;
            let default_device_name = self.default_device_name();

            // Iterate over all available input devices
            for (i, device) in devices.enumerate() {

                let name = ok_or_skip!(device.name());
                let config = ok_or_skip!(device.default_input_config())
                    .config();

                // Check if the current device is the default device
                let default_device = match default_device_name.as_ref() {
                    None => false, // If there is no default device, no device will it be
                    Some(default_name) => { default_name.as_str() == name.as_str() } // Compare the device name to check for standard device
                };


                vec.push(DeviceInfo {
                    position: i,
                    name,
                    channels: config.channels,
                    sample_rate: config.sample_rate.0,
                    standard: default_device
                })
            }

            Ok(vec)
        }

        /// Set device at position as current device
        pub fn set_device(&mut self, position: usize) -> Result<()> {
            let mut devices = self.host.input_devices()?;

            let device = devices.nth(position);
            Ok(self.device = device)
        }

        pub fn buffer_info(&self) -> Result<BufferInfo> {
            let device = self.device.as_ref()
                .ok_or(ApplicationError::NoDeviceSelected)?;

            let config = device.supported_stream_configuration()?;

            // The frame length defines a pack of samples. We need as much frames as in FRAME_RATE declared.
            // So we split the Samples to the FRAME_RATE
            let frame_length = (config.sample_rate.0 / CAPTURE_FRAME_RATE) as usize;

            // Frame capture_size defines how many frames will be captured in 1 period
            // The display rate is set to 100fps and the streaming rate is 50fps. So we need 2 frames for each period.
            let frame_capture_size = (CAPTURE_FRAME_RATE/DISPLAY_FRAME_RATE) as usize;

            Ok(
                BufferInfo { frame_length, frame_capture_size }
            )
        }


        /// Start the current stream
        pub fn start_stream(&self) -> Result<()> {
            match &self.input_stream {
                // Return NoStream if no stream is selected
                None => Err(ApplicationError::NoInputStream)?,
                Some(it) => Ok(it.play()?)
            }
        }

        /// Stop the current stream
        pub fn pause_stream(&self) -> Result<()> {
            match &self.input_stream {
                // Return NoStream if no stream is selected
                None => Err(ApplicationError::NoInputStream)?,
                Some(it) => Ok(it.pause()?)
            }
        }


        /// Build a new input stream with a fixed configuration, frame_length and frame_capture_size
        /// Returns the size of one frame
        pub fn build_mono_stream<C, E>(
            &mut self,
            mut callback: C,
            mut error_callback: E
        ) -> Result<()>
            where
                C: FnMut(&[i16], &InputCallbackInfo) + Send + 'static,
                E: FnMut(StreamError) + Send + 'static
         {
             // Return a NoDevice error when no device will be found or a DefaultStreamConfigError if no configuration will be found
             let device = self.current_device()?;
             let configuration = device.supported_stream_configuration()?;


             let mut buffer = AudioBuffer::from_info(self.buffer_info()?);
             let mut buffer_step = false;

            self.input_stream = Some(device.build_input_stream(
                &configuration,
                move |data: &[i16], info: &InputCallbackInfo| {

                    // Creates an iterator which only collects one channel
                    let iter = data.iter().step_by(2).cloned();

                    match buffer_step {
                        false => {
                            buffer.data.splice(..buffer.frame_length(), iter);
                        }
                        true => {
                            buffer.data.splice(buffer.frame_length().., iter);
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
                })?
            
            );

            Ok(())
        }

        /// Get the current device
        fn current_device(&self) -> Result<&Device> {
            self.device.as_ref()
                .context(ApplicationError::NoDeviceSelected)
        }

        /// Get the default name for the input device.
        fn default_device_name(&self) -> Option<String> {
            match self.host.default_input_device() {
                None => None,
                Some(device) => Some(device.safe_name())
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


/// Describes a device with all necessary information's to decide,
/// which device should be used.
pub struct DeviceInfo {
    /// The position of the device referred to the host.
    pub position: usize,

    /// Display name of the device
    pub name: String,

    /// Amount of channels the device has. Mostly stereo or mono
    pub channels: u16,

    /// SampleRate of the device. Important for visualization.
    /// Lower sample rate means less information to display or a smaller refresh rate
    pub sample_rate: u32,

    pub standard: bool
}


