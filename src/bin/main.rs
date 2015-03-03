#![feature(core)]

extern crate handmade_hero;

use handmade_hero::pixelbuffer::PixelBuffer;
use handmade_hero::audio::AudioDevice;
use handmade_hero::util::CircularIterMut;

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

fn sine_wave(device: &mut AudioDevice, hz: usize) {
    use std::num::Float;
    use std::f32::consts::PI;

    let half_period = 44100 / hz;

    let mut lock = device.lock();
    let mut index = lock.index();
    for n in CircularIterMut::new(&mut lock.buffer, index) {
        let rad = index as f32 / half_period as f32 * PI;
        *n = rad.sin();
        index += 1;
    }
}

fn main() {
    let hz = 25;
    let mut hh = handmade_hero::initialize(640, 480, 0.2);

    let mut x_offset = 0i32;
    let mut y_offset = 0i32;

    sine_wave(&mut hh.audio, hz);
    hh.audio.play();
    for e in hh.game_loop {
        x_offset += (e.direction.0 / 10) as i32;
        y_offset += (e.direction.1 / 10) as i32;

        let hz_actual = hz + ((e.direction.1 + e.direction.0) / 2) as usize;
        sine_wave(&mut hh.audio, hz_actual);

        weird_gradient_pattern(&mut hh.pixel_buffer, (x_offset, y_offset));
        hh.renderer.blit(&hh.pixel_buffer);
    }
}
