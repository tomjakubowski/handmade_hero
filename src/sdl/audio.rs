use sdl2::audio as sdl;
use audio;

use FatalResult;

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
    pub fn new(function: Box<audio::AudioFunction>) -> FatalResult<AudioDevice> {
        let freq = 44100;
        let channels = 2;
        let spec = sdl::AudioSpecDesired {
            freq: freq,
            channels: channels,
            samples: 0,
            callback: AudioWrap::new(function, freq as usize, channels as usize),
        };
        match spec.open_audio_device(None, false) {
            Ok(audio) => {
                Ok(AudioDevice {
                    sdl_device: audio,
                })
            },
            Err(e) => Err(e),
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
