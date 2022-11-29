use dyn_clone::DynClone;
use super::utils::Domain;

// Very abstract: An effect is a algorithm which takes an signal of length x. The length of the output can be different
// A filter instead is a algorithm which takes an signal of length x, but the output signal must have the same length!

dyn_clone::clone_trait_object!(FilterProcessing);
pub trait FilterProcessing: DynClone {

    fn process(&self, data: &mut [f32]);

}

// to send the processing trait to the worker (which is another thread),
// we need to implement the Send trait
type FilterProcessor = dyn FilterProcessing + Send;

pub struct Filter {
    info: FilterInfo,
    processor: Box<FilterProcessor>
}

#[derive(Copy, Clone)]
pub struct FilterInfo {
    pub name: &'static str,
    pub domain: Domain
}

impl Filter {

    pub fn new(name: &'static str, domain: Domain, processor: Box<FilterProcessor>) -> Filter {
        Filter {
            info: FilterInfo {
                name,
                domain
            }
            , processor
        }
    }
    
    /// Name of the effect
    pub fn name(&self) -> &str {
        self.info.name
    }

    /// Frequency or Wave filter?
    pub fn domain(&self) -> Domain { //Copy
        self.info.domain
    }

    /// Create a new boxed Processing trait
    pub fn create(&self) -> Box<FilterProcessor> {
        self.processor.clone()
    }
    
    //Copy the info of the filter
    pub fn get_info(&self) -> FilterInfo {
        self.info
    }
    
}



// Example filter

#[derive(Clone)]
pub struct SimplePreEmphasisFilter;

impl FilterProcessing for SimplePreEmphasisFilter {
    fn process(&self, data: &mut [f32]) {
        todo!()
    }
}
