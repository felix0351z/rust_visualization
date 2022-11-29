pub struct BufferInfo {
    pub frame_length: usize,
    pub frame_capture_size: usize
}

/// Enum to categories fir different domains
#[derive(Copy, Clone)]
pub enum Domain {
    FrequencyDomain,
    TimeDomain
}

impl BufferInfo {

    /// Get the size of the buffer
    pub fn buffer_size(&self) -> usize {
        return self.frame_length*self.frame_capture_size
    }

}

/// Contains all necessary information's to buffer audio data..
pub struct AudioBuffer<T> {
    pub data: Vec<T>,
    info: BufferInfo
}

impl AudioBuffer<f32> {

    /// To buffer audio data, we need a *frame_length*, to know how long a single audio frame is.
    /// In Addition, wee need to know how many frames are saved in the buffer.
    pub fn new(frame_length: usize, frame_capture_size: usize) -> AudioBuffer<f32> {
        AudioBuffer {
            data: vec![0.0; frame_length*frame_capture_size],
            info: BufferInfo { frame_length, frame_capture_size }
        }
    }

    pub fn from_info(info: BufferInfo) -> AudioBuffer<f32> {
        AudioBuffer {
            data: vec![0.0; info.frame_length*info.frame_capture_size],
            info
        }
    }

}

impl<T> AudioBuffer<T>  {

    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.data.as_mut_slice()
    }

    pub fn frame_length(&self) -> usize {
        self.info.frame_length
    }

    pub fn frame_capture_size(&self) -> usize {
        self.info.frame_capture_size
    }

    pub fn buffer_size(&self) -> usize {
        self.info.buffer_size()
    }

    pub fn buffer_info(&self) -> &BufferInfo {
        &self.info
    }

}
