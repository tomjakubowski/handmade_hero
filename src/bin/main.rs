#![feature(core)]

extern crate handmade_hero;

mod audio;
mod input;

use audio::{SineCmd, SineWave};

use handmade_hero::pixelbuffer::PixelBuffer;
use handmade_hero::audio::AudioDevice;
use handmade_hero::renderer::Renderer;

fn weird_gradient_pattern(buff: &mut PixelBuffer<u32>,
        (xoff, yoff): (i32, i32)) {
    let mut pixel = 0;
    for y in 0..buff.height() {
        for x in 0..buff.width() {
            let green = (x + xoff) as u8;
            let blue = (y + yoff) as u8;

            buff.buffer[pixel] = ((green as u32) << 8) | (blue as u32);
            pixel += 1;
        }
    }
}

fn main() {
    let hz = 231.6;
    let (audio_func, audio_hz) = SineWave::new(hz, 0.2);
    let mut hh = handmade_hero::sdl::initialize(640, 480,
        Box::new(audio_func));

    let mut x_offset = 0i32;
    let mut y_offset = 0i32;

    let mut hz_old = hz;
    hh.audio.play();
    for e in hh.game_loop {
        let (x, y) = input::direction(e);
        x_offset += x as i32 / 20;
        y_offset += y as i32 / 20;

        let hz = hz + ((x + y) / 2) as f32;
        if hz != hz_old {
            hz_old = hz;
            let _ = audio_hz.send(SineCmd::SetHz(hz));
        }

        weird_gradient_pattern(&mut hh.pixel_buffer, (x_offset, y_offset));
        hh.renderer.blit(&hh.pixel_buffer);
    }
}
