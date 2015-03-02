use sdl2::audio::{AudioCallback};
pub use sdl2::audio::AudioDevice;

pub struct AudioBuffer {
    pub volume: f32,
    pub buffer: Box<[f32]>,
    index: usize,
}

impl AudioBuffer {
    /// Creates a new AudioBuffer
    ///
    /// 0.0 <= volume <= 1.0
    /// per_sec should be channels * frequency
    pub fn new(volume: f32, per_sec: i32) -> AudioBuffer {
        AudioBuffer {
            volume: volume,
            buffer: vec![0.0; per_sec as usize].into_boxed_slice(),
            index: 0,
        }
    }
}

impl AudioCallback for AudioBuffer {
    pub type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        let len = self.buffer.len();
        for x in out.iter_mut() {
            *x = self.buffer[self.index % len] * self.volume;
            self.index += 1;
        }

        self.index %= len;
    }
}
