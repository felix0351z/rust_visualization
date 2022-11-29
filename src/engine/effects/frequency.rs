use crate::engine::effects::EffectProcessing;

#[derive(Clone)]
pub struct FrequencyEffect;

impl EffectProcessing for FrequencyEffect {
    fn n_mel(&self, n_led: usize) {
        todo!()
    }

    fn process_frequency(&self, input: &[f32], output: &mut [f32]) {
        todo!()
    }
}

