
/// Contains all necessary information's to buffer audio data..
pub struct AudioBuffer<T> {
    pub data: Vec<T>,

    frame_length: usize,
    frame_capture_size: usize,
}

impl<T> AudioBuffer<T> {


    /// To buffer audio data, we need a *frame_length*, to know how long a single audio frame is.
    /// In Addition, wee need to know how many frames are saved in the buffer.
    pub fn new(frame_length: usize, frame_capture_size: usize) -> AudioBuffer<T> {

        AudioBuffer {
            data: Vec::with_capacity(frame_length*frame_capture_size),
            frame_length,
            frame_capture_size,
        }

    }

    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.data.as_mut_slice()
    }

    pub fn frame_length(&self) -> usize {
        self.frame_length
    }

    pub fn frame_capture_size(&self) -> usize {
        self.frame_capture_size
    }

    /// Get the size of the buffer
    pub fn buffer_size(&self) -> usize {
        self.frame_length*self.frame_capture_size
    }


}