use sdl2::{init, INIT_EVERYTHING};
use sdl2::audio::{AudioDevice, AudioSpecDesired};
use renderer::Renderer;
use pixelbuffer::PixelBuffer;
use game::GameLoop;
use audio::AudioBuffer;

pub struct HandmadeHero<'a> {
    pub renderer: Renderer<'a>,
    pub pixel_buffer: PixelBuffer<u32>,
    pub game_loop: GameLoop<'a>,
    pub audio: AudioDevice<AudioBuffer>,
}

pub fn initialize<'a>(width: i32, height: i32, volume: f32) -> HandmadeHero<'a> {
    let sdl_context = match init(INIT_EVERYTHING) {
        Ok(c) => c,
        Err(e) => panic!("SDL couldn't initialize: {}", e),
    };

    let renderer = Renderer::new(width, height);
    let buffer = PixelBuffer::new(width, height, 0u32);
    let game_loop = GameLoop::new(sdl_context);

    let freq = 44100;
    let channels = 2;
    let spec = AudioSpecDesired {
        freq: freq,
        channels: channels,
        samples: 0,
        callback: AudioBuffer::new(volume, freq * channels as i32),
    };
    let audio = match spec.open_audio_device(None, false) {
        Ok(d) => d,
        Err(e) => panic!("{:?}", e)
    };

    HandmadeHero {
        renderer: renderer,
        pixel_buffer: buffer,
        game_loop: game_loop,
        audio: audio,
    }
}
