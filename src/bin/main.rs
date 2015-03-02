#![feature(core)]

extern crate handmade_hero;

use handmade_hero::pixelbuffer::PixelBuffer;
use handmade_hero::audio::{AudioBuffer, AudioDevice};

fn weird_gradient_pattern(buff: &mut PixelBuffer<u32>, (xoff, yoff): (i32, i32)) {
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

fn sine_wave(device: &mut AudioDevice<AudioBuffer>, hz: usize) {
    use std::num::Float;
    use std::f32::consts::PI;

    let period = 44100 / hz;

    let mut lock = device.lock();
    for (i, n) in lock.buffer.iter_mut().enumerate() {
        let rad = i as f32 / period as f32 * PI;
        *n = rad.sin();
    }
}

fn main() {
    let hz = 262;
    let mut hh = handmade_hero::initialize(640, 480, 0.5);

    let mut x_offset = 0i32;
    let mut y_offset = 0i32;

    sine_wave(&mut hh.audio, hz);
    hh.audio.resume();
    for e in hh.game_loop {
        x_offset += e.direction.0 as i32;
        y_offset += e.direction.1 as i32;

        weird_gradient_pattern(&mut hh.pixel_buffer, (x_offset, y_offset));
        hh.renderer.blit(&hh.pixel_buffer);
    }
}
