use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use sacn_unofficial::source::SacnSource;
use sacn_unofficial::error::errors::{Error, ErrorKind};
use sacn_unofficial::packet::{ACN_SDT_MULTICAST_PORT, UNIVERSE_CHANNEL_CAPACITY};

use error_stack::{Report, Result};
use super::errors::SenderError;
use super::utils::count_true;

pub struct Sender {
    // Inner sender which sends the data to the network
    inner: Arc<Mutex<SenderInner>>,
    // Reference id of this object for the inner
    owner_id: usize
}


impl Sender {
    const SENDER_NAME: &'static str = "sender";

    /// Create a new Sender
    /// Could throw a IOError if the underlying UDP Socket can't be created
    pub fn new() -> Result<Self, SenderError> {
        let inner = SenderInner::new(Self::SENDER_NAME)?;
        let arc = Arc::new(Mutex::new(inner));

        Ok(
            Sender {
                inner: arc,
                owner_id: 0
            }
        )

    }

    /// Send the data
    /// Note that the length can't be larger then 512 (*SenderInner::PacketCapacity*)
    /// Could thrown an IOError or a WrongPacketSize Error
    pub fn send(&self, data: &[u8]) -> Result<(), SenderError> {
        let mut inner = self.inner.lock().unwrap();
        inner.add(self.owner_id, data)?;

        Ok(())
    }

    /// Clone the sender and register the cloned object as new sender
    /// Could throw an error if the underlying inner already reached the maximum of owners
    pub fn clone(&self) -> Result<Self, SenderError> {
        // Add new owner to the inner, then
        // copy the arc to the inner and construct a new sender

        let id = self.add_owner()?;
        let inner_clone = self.inner.clone();

        Ok(Sender {
            inner: inner_clone,
            owner_id: id
        })
    }

    /// Add a new owner to the inner
    fn add_owner(&self) -> Result<usize, SenderError> {
        // Reference to the inner of the sender. If another engine(thread) uses it, we have to wait
        // until the last owner has finished
        // If an error will be returned another thread panicked, so unwrap will be necessary
        let mut inner = self.inner.lock().unwrap();
        let owner_id = inner.add_owner()?;

        Ok(owner_id)
    }

}

/// Create a new SacnSource
/// Could throw an IOError if the underlying udp socket won't
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




struct SenderInner {
    // Underlying sacn sender source
    source: SacnSource,
    // Numbers of references which can use this struct
    owners: usize,

    // Number of owners, which already wrote their data to the current packet
    packet_complete: [bool; SenderInner::MAX_OWNERS],

    //array which stores all data to send
    packet: [u8; UNIVERSE_CHANNEL_CAPACITY*SenderInner::MAX_OWNERS],

    // Registered universes
    universes: [u16; SenderInner::MAX_OWNERS]
}

impl SenderInner {
    /// Maximum of allowed owners to send
    const MAX_OWNERS: usize = 3;
    /// Maximum size of the packet capacity
    const PACKET_CAPACITY: usize = UNIVERSE_CHANNEL_CAPACITY-1;

    ///Create a new SenderInner
    fn new(name: &'static str) -> Result<Self, SenderError> {
        let mut source = new_source(name)?;
        let owners = 1;
        let matrix = [0 as u8; UNIVERSE_CHANNEL_CAPACITY*SenderInner::MAX_OWNERS];
        let packet_complete = [false; SenderInner::MAX_OWNERS];

        //Generate necessary universes
        let mut universes = [1 as u16; Self::MAX_OWNERS];
        for i in 0..Self::MAX_OWNERS  {
            //Universes starts at 1
            universes[i] = (i+1) as u16;
        }
        // Register all necessary universes
        source.register_universes(universes.as_slice()).unwrap();

        Ok(
            SenderInner {
                source,
                owners,
                packet: matrix,
                packet_complete,
                universes
            }
        )

    }

    /// Increase the number of owners to the inner and returns the next owner_id
    /// Throw MaximumOwner if the maximum of owners is reached
    fn add_owner(&mut self) -> Result<usize, SenderError> {
        if self.owners == Self::MAX_OWNERS {
            Err(Report::new(SenderError::MaximumOwner))?
        }

        // Increase the number of owners
        // Because the length (owners) is 1 more than the highest id, we can use the old length as new id.
        let new_owner_id = self.owners;

        // Increase the number of owners
        self.owners+=1;
        self.source.register_universe(self.owners as u16).unwrap();

        // Return next owner id
        Ok(new_owner_id)
    }


    /// Add the data to the next packet
    fn add(&mut self, owner_id: usize, data: &[u8]) -> Result<(), SenderError> {
        // If not all owners have written their data, the packet will not be send.
        if count_true(self.packet_complete.as_slice()) < self.owners {
            self.add_to_packet(owner_id, data)?;

            self.packet_complete[owner_id] = true;
        }


        // Send the packet if now all users have written their data
        if count_true(self.packet_complete.as_slice()) == self.owners {
            // Send the packet
            self.send_packet()?;
            self.packet_complete = [false; SenderInner::MAX_OWNERS];
        }

        Ok(())
    }

    fn add_to_packet(&mut self, id: usize, data: &[u8]) -> Result<(), SenderError> {
        if data.len() > Self::PACKET_CAPACITY  {
            Err(Report::new(SenderError::WrongPacketSize))?
        }

        // Go the chunk which is reserved for the owner with the *id*
        let chunk = id*UNIVERSE_CHANNEL_CAPACITY;

        // ID = Chunk of owner + Start code + Iteration
        for (i, x) in data.iter().enumerate() {
            self.packet[chunk+1+i] = *x;
        }

        Ok(())
    }

    /// Send the current packet
    fn send_packet(&mut self) -> Result<(), SenderError> {
        match self.source.send(
            &self.universes.as_slice(),
            self.packet.as_slice(),
            None,
            None,
            None
        ) {
            Ok(_) => (),
            Err(err) => {
                match err {
                    Error(ErrorKind::Io(..), _) => Err(SenderError::IOError)?,
                    //Error(ErrorKind::SenderAlreadyTerminated(..), _) => Err(SenderError::AlreadyTerminated)?,
                    _ => {
                        // If there is a error which isn't the users fault the program has to panic
                        panic!()
                    }
                }
            }
        };

        Ok(())
    }


}
