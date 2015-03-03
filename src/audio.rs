use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioDeviceLockGuard};
use sdl2::audio::AudioDevice as SdlAudioDevice;

use std::ops::{Deref, DerefMut};

pub struct AudioBuffer {
    pub volume: f32,
    pub buffer: Box<[f32]>,
    sample_len: usize,
    index: usize,
}

impl AudioBuffer {
    /// Creates a new AudioBuffer
    ///
    /// 0.0 <= volume <= 1.0
    /// per_sec should be channels * frequency
    pub fn new(volume: f32, per_sec: usize) -> AudioBuffer {
        AudioBuffer {
            volume: volume,
            buffer: vec![0.0; per_sec].into_boxed_slice(),
            sample_len: per_sec,
            index: 0,
        }
    }

    pub fn sample_len(&self) -> usize {
        self.sample_len
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl AudioCallback for AudioBuffer {
    pub type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = self.buffer[self.index % self.sample_len] * self.volume;
            self.index += 1;
        }
    }
}

pub struct AudioDevice {
    sdl_device: SdlAudioDevice<AudioBuffer>,
}

impl AudioDevice {
    pub fn new(volume: f32) -> AudioDevice {
        let freq = 44100;
        let channels = 2;
        let spec = AudioSpecDesired {
            freq: freq,
            channels: channels,
            samples: 0,
            callback: AudioBuffer::new(volume, freq as usize * channels as usize),
        };
        let audio = match spec.open_audio_device(None, false) {
            Ok(d) => d,
            Err(e) => panic!("{:?}", e)
        };

        AudioDevice {
            sdl_device: audio,
        }
    }

    pub fn play(&self) {
        self.sdl_device.resume()
    }

    pub fn lock<'a>(&'a mut self) -> LockGuard<'a> {
        LockGuard {
            sdl_lockguard: self.sdl_device.lock()
        }
    }
}

pub struct LockGuard<'a> {
    sdl_lockguard: AudioDeviceLockGuard<'a, AudioBuffer>,
}

impl<'a> !Send for LockGuard<'a> {}

impl<'a> Deref for LockGuard<'a> {
    type Target = AudioBuffer;
    fn deref(&self) -> &AudioBuffer {
        &self.sdl_lockguard.deref()
    }
}

impl<'a> DerefMut for LockGuard<'a> {
    fn deref_mut(&mut self) -> &mut AudioBuffer {
        self.sdl_lockguard.deref_mut()
    }
}
