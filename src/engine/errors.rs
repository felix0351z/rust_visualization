use thiserror::Error;

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