
/// Enum to categories the different Effect types
enum EffectType {
    FrequencyDomain,
    TimeDomain
}

/// An effect is an object which takes an input signal and produces a new output signal.
///
/// This trait is only for the basic functions which every effect needs.
/// A *FrequencyEffect* or an *TimeEffect* are the ones which implements the logic.
pub trait Effect: FrequencyDomain + TimeDomain  {

    /// Name of the effect
    fn name(&self) -> String;

    /// Icon source for the effect
    fn icon(&self) -> String;

    /// Frequency effect or time effect?
    fn effect_type(&self) -> EffectType;
    
}

/// A Frequency Effect is an effect which takes the frequency domain as input signal
/// and process a new effect out of it.
pub trait FrequencyDomain {

    /// Defines how much mel points should be calculated for this effect.
    fn n_mel(&self, n_led: usize);

    /// Processes the effect. Take mel as input.
    fn process(&self, mel: &[f32], output: &mut [f32]);

}

/// A Time Effect is an effect which takes the standard wave form as input signal
/// and process a new effect out of it.
pub trait TimeDomain {
    //TODO
}