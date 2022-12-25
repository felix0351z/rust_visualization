use dyn_clone::DynClone;
use super::utils::Domain;

pub mod frequency;

// Apply the clone trait for every Processing object
dyn_clone::clone_trait_object!(EffectProcessing);

/// Every effect needs a function to process the effect.
/// The input can be the mel domain for Frequency effect or the time domain for wave effects.
pub trait EffectProcessing: DynClone {

    /// Defines how much mel points should be calculated for this effect.
    fn n_mel(&self, n_led: usize) -> usize;

    /// Processes the effect.
    fn process_frequency(&self, mel: &[i16], output: &mut [i16]);

    //fn process_wave(...)
}

// to send the processing trait to the worker (which is another thread),
// we need to implement the Send trait
type EffectProcessor = dyn EffectProcessing + Send;

pub struct Effect {
    info: EffectInfo,
    processor: Box<EffectProcessor>
}

#[derive(Copy, Clone)]
pub struct EffectInfo {
    pub name: &'static str,
    pub icon: &'static str,
    pub domain: Domain,
}

impl Effect {

    pub fn new(name: &'static str, icon: &'static str, domain: Domain, processor: Box<EffectProcessor>) -> Effect {
        Effect { 
            info: EffectInfo {
                name,
                icon,
                domain
            }
            , processor 
        }
    }

    /// Name of the effect
    pub fn name(&self) -> &str {
        self.info.name
    }

    /// Path to the icon of the effect
    pub fn icon(&self) -> &str {
        self.info.icon
    }

    /// Frequency or Wave effect? 
    pub fn domain(&self) -> Domain { //Copy
        self.info.domain
    }

    /// Create a new boxed Processing trait
    pub fn create(&self) -> Box<EffectProcessor> {
        self.processor.clone()
    }

    // Copy the info from the effect
    pub fn get_info(&self) -> EffectInfo {
        self.info
    }

}