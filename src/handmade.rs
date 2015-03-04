use sdl2::{init, INIT_EVERYTHING};
use renderer::Renderer;
use pixelbuffer::PixelBuffer;
use game::GameLoop;
use audio::{AudioDevice, AudioFunction};

pub struct HandmadeHero<'a> {
    pub renderer: Renderer<'a>,
    pub pixel_buffer: PixelBuffer<u32>,
    pub game_loop: GameLoop<'a>,
    pub audio: AudioDevice,
}

pub fn initialize<'a>(width: i32, height: i32, function: Box<AudioFunction>)
        -> HandmadeHero<'a> {
    let sdl_context = match init(INIT_EVERYTHING) {
        Ok(c) => c,
        Err(e) => panic!("SDL couldn't initialize: {}", e),
    };

    let renderer = Renderer::new(width, height);
    let buffer = PixelBuffer::new(width, height, 0u32);
    let game_loop = GameLoop::new(sdl_context);
    let audio = AudioDevice::new(function);

    HandmadeHero {
        renderer: renderer,
        pixel_buffer: buffer,
        game_loop: game_loop,
        audio: audio,
    }
}
