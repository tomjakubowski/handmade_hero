pub mod renderer;
pub mod game;
pub mod audio;
mod input;

use sdl2::{init, INIT_EVERYTHING};

use super::renderer::Renderer;
use super::pixelbuffer::PixelBuffer;
use super::game::GameLoop;
use super::audio::{AudioDevice, AudioFunction};

pub struct HandmadeHero<'a> {
    pub renderer: renderer::Renderer<'a>,
    pub pixel_buffer: PixelBuffer<u32>,
    pub game_loop: game::GameLoop<'a>,
    pub audio: audio::AudioDevice,
}

pub fn initialize<'a>(width: i32, height: i32, function: Box<AudioFunction>)
        -> HandmadeHero<'a> {
    let sdl_context = match init(INIT_EVERYTHING) {
        Ok(c) => c,
        Err(e) => panic!("SDL couldn't initialize: {}", e),
    };

    let renderer = renderer::Renderer::new(width, height);
    let buffer = PixelBuffer::new(width, height, 0u32);
    let game_loop = game::GameLoop::new(sdl_context);
    let audio = audio::AudioDevice::new(function);

    HandmadeHero {
        renderer: renderer,
        pixel_buffer: buffer,
        game_loop: game_loop,
        audio: audio,
    }
}
