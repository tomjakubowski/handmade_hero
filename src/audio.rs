use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::audio::AudioDevice as SdlAudioDevice;

pub trait AudioFunction: Send {
    fn call(&mut self, out: &mut [f32], bitrate: usize);
}

struct AudioWrap {
    pub function: Box<AudioFunction>,
    bitrate: usize,
}

impl AudioWrap {
    /// Creates a new AudioBuffer
    ///
    /// 0.0 <= volume <= 1.0
    /// per_sec should be channels * frequency
    pub fn new(function: Box<AudioFunction>, bitrate: usize) -> AudioWrap {
        AudioWrap {
            function: function,
            bitrate: bitrate,
        }
    }
}

impl AudioCallback for AudioWrap {
    pub type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        self.function.call(out, self.bitrate);
    }
}

pub struct AudioDevice {
    sdl_device: SdlAudioDevice<AudioWrap>,
}

impl AudioDevice {
    pub fn new(function: Box<AudioFunction>) -> AudioDevice {
        let freq = 44100;
        let channels = 2;
        let spec = AudioSpecDesired {
            freq: freq,
            channels: channels,
            samples: 0,
            callback: AudioWrap::new(function, freq as usize * channels as usize),
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
}
