pub mod input;
pub mod sender;
pub mod errors;
pub mod utils;

mod effects;
mod filters;
mod processing;


use input::*;
use utils::AudioBuffer;

use effects::{EffectProcessing, Effect, EffectInfo};
use effects::frequency::FrequencyEffect;
use filters::{FilterProcessing, Filter, FilterInfo};

use cpal::{InputCallbackInfo};
use crate::engine::utils::Domain;
use errors::ApplicationError;
use anyhow::Result;

pub struct Engine {
    input: DeviceInputSource,
    // send handler

    //Led amount of the device
    n_led: usize,

    // All available effects and filters
    effects: Vec<Effect>,
    filters: Vec<Filter>,

    filtering: bool,
    current_effect: usize,
    current_filter: usize,
}


impl Engine {


    /// Generates a new engine with all necessary dependencies
    pub fn new(n_led: usize) -> Engine {
        let input = DeviceInputSource::new();

        let effects: Vec<Effect> = vec![
            Effect::new(
                "Frequency Effect",
                "/fdgfd",
                Domain::FrequencyDomain,
                Box::new(FrequencyEffect)
            )



        ];
        let filters: Vec<Filter> = vec![];

        Engine {
            input,
            n_led,
            effects,
            filters,
            filtering: false,
            current_effect: 0,
            current_filter: 0
        }
    }

    //--------------Public-Methods--------------------------------------------------------

    /// Get all currently available devices which can be used as data input
    pub fn get_available_devices(&self) -> Result<Vec<DeviceInfo>> {
        self.input.available_devices()
    }

    /// Set a specific device as data input
    pub fn set_device(&mut self, position: usize) -> Result<()> {
        self.input.set_device(position)?;

        // Update the stream after the device was changed
        self.update_stream()
    }

    /// Start function
    /// Drops the current stream and start a new stream
    pub fn update_stream(&mut self) -> Result<()> {
        //self.pause_stream()?;
        self.build_stream()?;

        self.start_stream()
    }

    /// Stops the current stream.
    pub fn pause_stream(&self) -> Result<()> {
        self.input.pause_stream()
    }

    /// Get all available effects
    pub fn get_effects(&self) -> Vec<EffectInfo> {
        let mut list: Vec<EffectInfo> = vec![];

        for effect in self.effects.iter() {
            list.push(effect.get_info());
        }

        list
    }

    /// Get all available filters
    pub fn get_filters(&self) -> Vec<FilterInfo> {
        let mut list: Vec<FilterInfo> = vec![];

        for filter in self.filters.iter() {
            list.push(filter.get_info());
        }

        list
    }

    /// Set the current effect
    pub fn set_effect(&mut self, position: usize) -> Result<()> {
        self.current_effect = position;

        self.update_stream()
    }

    // Set the current filter
    pub fn set_filter(&mut self, position: usize) -> Result<()> {
        self.current_filter = position;

        self.update_stream()
    }

    pub fn is_filtering_activated(&self) -> bool {
        self.filtering
    }

    pub fn set_filtering(&mut self, value: bool) {
        self.filtering = value
    }


    //---------------------Private-Methods---------------------------------

    /// Start the current stream
    fn start_stream(&self) -> Result<()> {
        self.input.start_stream()
    }


    fn get_current_effect(&self) -> Result<&Effect> {
        self.effects.get(self.current_effect)
            .ok_or(
                Err(ApplicationError::EffectNotFound {
                    id: self.current_effect
                })?
            )
    }

    fn get_current_filter(&self) -> Option<&Filter> {
        if self.filtering { None? }

        self.filters.get(self.current_filter)
    }

    fn get_frame_length(&self) -> Result<usize> {
        let info = self.input.buffer_info()?;

        Ok(info.frame_length)
    }


    fn build_stream(&mut self) -> Result<()> {

        let frame_length = self.get_frame_length()?;
        let effect = self.get_current_effect()?.create();
        let filter = match self.get_current_filter() {
            Some(value) => Some(value.create()),
            None => None
        };

        // Define callback
        let call =  |data: &[f32]| {
            println!("First: {:?}", data[0]);
        };

        {
            //Build the worker & stream
            let mut worker = Worker::new(
                call, frame_length,
                self.n_led, effect, filter);

            self.input.build_mono_stream(
                move |data, info| {

                    worker.process(data, info)

                },
                move |err| {
                    todo!()
                }

            )?;
        }

        Ok(())
    }


}


//TODO: 1. Framing, 2. Effect, 3. Domain, 4. Filter, N_FFT, N_Melbank, N_LEDs
//TODO:  Hat: Raw audio buffer



/// A worker is an struct which contains all information's to process the audio data to an
/// nice effect. After the effect was created it will be returned to the callback function.
pub struct Worker<C>
    where
    // A Mutable function which gets the effect data is needed.
    // Because of that this Worker struct runs in another thread the callback must have
    // implemented the Send Trait.
        C: FnMut(&[f32]) + Send + 'static,
{
    //Input and Output
    callback: C,
    last_frame: AudioBuffer<f32>,
    fft_buffer: [f32; processing::N_FFT],
    //mel buffer -> heap
    //effect buffer -> heap

    n_led: usize,

    //Framing factor
    effect: Box<dyn EffectProcessing + Send>,
    filter: Option<Box<dyn FilterProcessing + Send>>

}


impl<C> Worker<C>
where
    C: FnMut(&[f32]) + Send + 'static
{

    /// Generates a new Worker struct
    fn new(
        callback: C,
        frame_length: usize,
        n_led: usize,
        effect: Box<dyn EffectProcessing + Send>,
        filter: Option<Box<dyn FilterProcessing + Send>>
    ) -> Self {
        let last_frame = AudioBuffer::new(frame_length, 1);
        let fft_buffer = [0.0 as f32; processing::N_FFT];



        Worker {
            callback, last_frame, fft_buffer,
            n_led, effect, filter
        }
    }

    /// Function which consumes the raw input data and process the effect
    fn process(&mut self, data: &[f32], info: &InputCallbackInfo) {
        //....

        (self.callback)(data)
    }


    /// Process the pre emphasis over the input signal
    fn pre_emphasis(&mut self) {

    }

}
























