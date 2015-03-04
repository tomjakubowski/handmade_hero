use sdl2::audio as sdl;
use audio;

struct AudioWrap {
    pub function: Box<audio::AudioFunction>,
    frequency: usize,
    channels: usize,
}

impl AudioWrap {
    /// Creates a new AudioBuffer
    ///
    /// 0.0 <= volume <= 1.0
    /// per_sec should be channels * frequency
    pub fn new(function: Box<audio::AudioFunction>, frequency: usize, channels: usize)
            -> AudioWrap {
        AudioWrap {
            function: function,
            frequency: frequency,
            channels: channels,
        }
    }
}

impl sdl::AudioCallback for AudioWrap {
    pub type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        self.function.call(out, self.frequency, self.channels);
    }
}

pub struct AudioDevice {
    sdl_device: sdl::AudioDevice<AudioWrap>,
}

impl AudioDevice {
    pub fn new(function: Box<audio::AudioFunction>) -> AudioDevice {
        let freq = 44100;
        let channels = 2;
        let spec = sdl::AudioSpecDesired {
            freq: freq,
            channels: channels,
            samples: 0,
            callback: AudioWrap::new(function, freq as usize, channels as usize),
        };
        let audio = match spec.open_audio_device(None, false) {
            Ok(d) => d,
            Err(e) => panic!("{:?}", e)
        };

        AudioDevice {
            sdl_device: audio,
        }
    }
}

impl audio::AudioDevice for AudioDevice {
    fn play(&self) {
        self.sdl_device.resume()
    }

    fn stop(&self) {
        self.sdl_device.pause()
    }
}
