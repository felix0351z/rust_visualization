use crate::engine::effects::EffectProcessing;

#[derive(Clone)]
pub struct FrequencyEffect;

impl EffectProcessing for FrequencyEffect {

    fn n_mel(&self, n_led: usize) -> usize{
        n_led/2
    }

    fn process_frequency(&self, input: &[f32], output: &mut [f32]) {
        todo!()
    }
}

