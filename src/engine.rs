pub mod input;
pub mod errors;
pub mod utils;

mod effects;
mod filters;
mod processing;


use input::*;
use utils::AudioBuffer;

use std::error::Error;
use cpal::{InputCallbackInfo};




/// Direct interaction with
pub struct Engine {
    input: DeviceInputSource,

    // send handler
}


impl Engine {


    /// Generates a new engine with all necessary dependencies
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let input = DeviceInputSource::new();

        Ok(
            Engine { input }
        )
    }

    //--------------Public-Methods--------------------------------------------------------

    /// Get all currently available devices which can be used as data input
    /// *DevicesError* or *DefaultStreamConfigError*
    pub fn get_available_devices(&self) -> Result<Vec<DeviceInfo>, Box<dyn Error>> {
        self.input.available_devices()
    }

    /// Set a specific device as data input
    /// Errors: *DevicesError*
    pub fn set_device(&mut self, position: usize) -> Result<(), Box<dyn Error>> {
        self.input.set_device(position)?;

        // Update the stream after the device was changed
        self.update_stream()
    }


    /// Start the current stream. Returns an error if no stream was build
    /// Errors: *NoStream* or *PlayStreamError*
    pub fn start_stream(&self) -> Result<(), Box<dyn Error>> {
        self.input.start_stream()
    }

    /// Stops the current stream.
    /// Errors: *NoStream* or *PauseStreamError*
    pub fn pause_stream(&self) -> Result<(), Box<dyn Error>> {
        self.input.pause_stream()
    }

    pub fn get_effects(&self) {
        todo!()
    }

    pub fn set_effect(&mut self, position: usize) -> Result<(), Box<dyn Error>> {
        ///....

        self.update_stream()
    }


    //---------------------Private-Methods---------------------------------

    /// Drops the current stream and start a new stream
    fn update_stream(&mut self) -> Result<(), Box<dyn Error>> {
        //self.pause_stream()?;
        self.build_stream()?;


        self.start_stream()
    }


    fn build_stream(&mut self) -> Result<(), Box<dyn Error>> {

        // The input contains 1 buffer and for framing we want so save 2 frames.
        let frame_length = self.input.buffer_info()?.buffer_size();
        let buffer = AudioBuffer::new(frame_length, 2);

        let call =  |data: &[f32]| {
            println!("First: {:?}", data[0]);
        };
        let mut worker = Worker::new(buffer, call);

        self.input.build_mono_stream(
            move |data, info| {
                worker.process(data, info)
            },
            move |err| {
                todo!()
            }

        )?;

        Ok(())
    }


}





/// A worker is an struct which contains all information's to process the audio data to an
/// nice effect. After the effect was created it will be returned to the callback function.
pub struct Worker<C>
    where
    // A Mutable function which gets the effect data is needed.
    // Because of that this Worker struct runs in another thread the callback must have
    // implemented the Send Trait.
        C: FnMut(&[f32]) + Send + 'static,
{
    callback: C,
    buffer: AudioBuffer<f32>,
    //effect
    //filter
}


impl<C> Worker<C>
where
    C: FnMut(&[f32]) + Send + 'static
{

    /// Generates a new Worker struct with an callback
    fn new(buffer: AudioBuffer<f32>, callback: C) -> Self {
        Worker { callback, buffer }
    }

    /// Function which consumes the raw input data and process the effect
    fn process(&mut self, data: &[f32], info: &InputCallbackInfo) {
        //....

        (self.callback)(data)
    }

}
























