use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use sacn_unofficial::source::SacnSource;

use error_stack::Result;

use sacn_unofficial::error::errors::{Error, ErrorKind};
use sacn_unofficial::packet::{ACN_SDT_MULTICAST_PORT, UNIVERSE_CHANNEL_CAPACITY};
use crate::engine::errors::SenderError;


pub struct Sender {
    inner: Arc<Mutex<SenderInner>>,
    owner_id: u8
}


impl Sender {
    const SENDER_NAME: &'static str = "sender";

    fn new() -> Result<Self, SenderError> {
        let inner = SenderInner::new(Self::SENDER_NAME)?;
        let arc = Arc::new(Mutex::new(inner));

        Ok(
            Sender {
                inner: arc,
                owner_id: 0
            }
        )

    }

    fn add_owner(&self) -> u8 {
        // Reference to the inner of the sender. If another engine(thread) uses it, we have to wait
        // until the last owner has finished
        // If an error will be returned another thread panicked, so unwrap will be necessary
        let mut inner = self.inner.lock().unwrap();

        // Increase the number of owners
        // Because the length (owners) is 1 more than the highest id, we can use the old length as new id.
        let owner_id = inner.owners;
        inner.owners+=1;

        owner_id
    }

    //TODO: send(data) / send_sync(data)

}

impl Clone for Sender {

    fn clone(&self) -> Self {
        // Add new owner to the inner, then
        // copy the arc to the inner and construct a new sender

        let id = self.add_owner();
        let inner_clone = self.inner.clone();

        Sender {
            inner: inner_clone,
            owner_id: id
        }
    }

}



struct SenderInner {
    source: SacnSource,
    owners: u8,
    // An array for every owner
    matrix: [[u8; UNIVERSE_CHANNEL_CAPACITY]; SenderInner::MAX_OWNERS],
}

impl SenderInner {
    /// Maximum of allowed owners to send
    const MAX_OWNERS: usize = 3;

    fn new_source(name: &'static str) -> Result<SacnSource, SenderError> {
        // Own socket address
        let v4 = IpAddr::V4("0.0.0.0".parse().unwrap());
        // Own socket port
        let v4port = ACN_SDT_MULTICAST_PORT;

        let socket =  SocketAddr::new(v4, v4port);

        // Create sacn source and check for possible errors
        let source = match SacnSource::with_ip(name, socket) {
            Ok(value) => value,
            Err(err) => {
                match err {
                    Error(ErrorKind::Io(..), _) => Err(SenderError::IOError)?,
                    _ => Err(SenderError::Unknown)?
                }
            }
        };

        Ok(source)
    }

    fn new(name: &'static str) -> Result<Self, SenderError> {
        let source = Self::new_source(name)?;
        let owners = 1;
        let matrix = [[0 as u8; UNIVERSE_CHANNEL_CAPACITY]; SenderInner::MAX_OWNERS];

        Ok(
            SenderInner {
                source,
                owners,
                matrix
            }
        )

    }

}







