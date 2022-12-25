use std::fmt::{Debug, Error};
use log::error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SenderError {

    /// Maximum packet size, which is defined as 512
    #[error("The maximum packet size of 512 bytes was reached.")]
    WrongPacketSize,

    /// Error occurred in SenderInner.send_packet() function
    #[error("Error occurred while sending: {0}")]
    SendError(String),

    /// Error occurred in new_source() function
    #[error("Error occurred while sender creation {0}")]
    CreationError(String)

}



#[derive(Error, Debug)]
pub enum ApplicationError {

    /// Effect with the given id wasn't found
    #[error("Effect with the id {id} not found.")]
    EffectNotFound {
        id: usize
    },

    /// Filter with the given id wasn't found
    #[error("Filter with the id {id} not found.")]
    FilterNotFound {
        id: usize
    },

    /// No Device is selected as input
    #[error("No input device was selected.")]
    NoDeviceSelected,

    /// Maximum amount of parallel engines was reached
    #[error("Maximum amount of possible engines was reached.")]
    MaximumEngines,

    /// No current input stream is available
    #[error("No input stream selected.")]
    NoInputStream,

    /// Unknown error
    #[error(transparent)]
    Other(
        #[from] Error
    )

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


