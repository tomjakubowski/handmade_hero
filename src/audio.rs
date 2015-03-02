use sdl2::audio::{AudioCallback};
pub use sdl2::audio::AudioDevice;

pub struct AudioBuffer {
    pub volume: f32,
    pub buffer: Box<[f32]>,
}

impl AudioBuffer {
    pub fn new(volume: f32) -> AudioBuffer {
        AudioBuffer {
            volume: volume,
            buffer: Box::new([0.0; 1024]),
        }
    }
}

impl AudioCallback for AudioBuffer {
    pub type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        let len = self.buffer.len();
        for (i, x) in out.iter_mut().enumerate() {
            *x = self.buffer[i % len] * self.volume;
        }
    }
}
