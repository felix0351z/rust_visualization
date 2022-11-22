use std::fmt::{Debug, Display, Formatter};
use error_stack::Context;

#[derive(Debug)]
pub enum ProgramError {
    InputError(InputError),

    /// Error which isn't fault of the user
    /// CPAL errors are also included here
    Other(String)
}


impl Context for ProgramError{}

impl Display for ProgramError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramError::InputError(err) => {
                write!(f, "InputError occurred: {}", err.to_string())
            }
            ProgramError::Other(err) => {
                write!(f, "Unknown error: {}", err)
            }
        }
    }
}


#[derive(Debug)]
pub enum InputError {
    /// Will be returned if there is no Device currently selected
    NoDeviceSelected,
    /// Will be returned when function was called which needs an InputStream, but no InputStream is available
    NoStream,


    /// Errors which isn't fault of the user
    CPAL,
}

impl Context for InputError {

}

impl Display for InputError {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::NoDeviceSelected => {
                f.write_str("No input device is currently selected.")
            }
            InputError::NoStream => {
                f.write_str("No input stream was found.")
            }
            InputError::CPAL => {
                f.write_str( "CPAL Error occurred")
            }
        }
    }

}

#[derive(Debug)]
pub struct CPALError;

impl Context for CPALError {}

impl Display for CPALError {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str( "CPAL Error occurred")
    }

}



/// Simple macro for iterations when a list should be returned, but one object creates an error
#[macro_export] macro_rules! ok_or_skip {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(err) => {
                warn!("Error: {:?} skipped.", err);
                continue;
            }
        }
    };
}


