use std::fmt::{Debug, Display, Error, Formatter};
use cpal::StreamError;
use error_stack::Context;

#[derive(Debug)]
pub enum ProgramError {

    InputError,

    EffectNotFound(usize),

    FilterNotFound(usize)

}

impl Display for ProgramError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramError::InputError => {
                f.write_str("Input related error occurred")
            }
            ProgramError::EffectNotFound(id) => {
                f.write_fmt(format_args!("Effect with the id {id} not found"))
            }
            ProgramError::FilterNotFound(id) => {
                f.write_fmt(format_args!("Filter with the id {id} not found"))
            }
        }
    }
}

impl Context for ProgramError {

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

pub enum SenderError {
    IOError,
    UnsupportedName,
    Unknown,
}

impl Display for SenderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Debug for SenderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Context for SenderError {

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


