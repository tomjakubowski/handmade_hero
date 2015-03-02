extern crate handmade_hero;

use handmade_hero::pixelbuffer::PixelBuffer;
use handmade_hero::audio::{AudioBuffer, AudioDevice};

fn weird_gradient_pattern(buff: &mut PixelBuffer<u32>,
        x_offset: i32, y_offset: i32) {
    let mut pixel = 0;
    for y in 0..buff.height() {
        for x in 0..buff.width() {
            let green = (x + x_offset) as u8;
            let blue = (y + y_offset) as u8;

            buff.buffer[pixel] = ((green as u32) << 8) | (blue as u32);
            pixel += 1;
        }
    }
}

fn square_wave(device: &mut AudioDevice<AudioBuffer>) {
    let mut lock = device.lock();
    for (i, n) in (*lock).buffer.iter_mut().enumerate() {
        *n = (i / 128 % 2 * 16) as f32;
    }
}

fn main() {
    let mut hh = handmade_hero::initialize(640, 480, 0.5);

    let mut x_offset = 0i32;
    let mut y_offset = 0i32;

    square_wave(&mut hh.audio);
    hh.audio.resume();
    for e in hh.game_loop {
        x_offset += e.direction.0 as i32;
        y_offset += e.direction.1 as i32;

        weird_gradient_pattern(&mut hh.pixel_buffer, x_offset, y_offset);
        hh.renderer.blit(&hh.pixel_buffer);
    }
}
