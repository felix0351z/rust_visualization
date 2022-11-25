
// Very abstract: An effect is a algorithm which takes an signal of length x. The length of the output can be different
// A filter instead is a algorithm which takes an signal of length x, but the output signal must have the same length!

//TODO Vlt Differenzierung in VorFilter


pub trait Filter {

    fn process(&self, data: &mut [f32]);

}
