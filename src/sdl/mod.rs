pub mod renderer;
pub mod game;
pub mod audio;
mod input;

use sdl2::{init, INIT_EVERYTHING};

use super::renderer::Renderer;
use super::pixelbuffer::PixelBuffer;
use super::game::GameLoop;
use super::audio::{AudioDevice, AudioFunction};

use FatalResult;

pub struct HandmadeHero<'a> {
    pub renderer: renderer::Renderer<'a>,
    pub pixel_buffer: PixelBuffer<u32>,
    pub game_loop: game::GameLoop<'a>,
    pub audio: audio::AudioDevice,
}

pub fn initialize<'a>(width: i32, height: i32, function: Box<AudioFunction>)
        -> FatalResult<HandmadeHero<'a>> {
    let sdl_context = match init(INIT_EVERYTHING) {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    let renderer = renderer::Renderer::new(width, height);
    let buffer = match PixelBuffer::new(width, height, 0u32) {
        Ok(b) => b,
        Err(e) => return Err(e),
    };
    let game_loop = game::GameLoop::new(sdl_context);
    let audio = match audio::AudioDevice::new(function) {
        Ok(a) => a,
        Err(e) => return Err(e),
    };

    Ok(HandmadeHero {
        renderer: renderer,
        pixel_buffer: buffer,
        game_loop: game_loop,
        audio: audio,
    })
}
