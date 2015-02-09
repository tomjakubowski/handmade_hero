use std::mem;
use std::num::Int;
use std::i32;

pub struct PixelBuffer<T: Int> {
    width: i32,
    height: i32,
    pitch: i32,
    pub buffer: Box<[T]>,
}

impl<T: Int> PixelBuffer<T> {
    pub fn new(width: i32, height: i32, setting: T)
            -> PixelBuffer<T> {
        let mut buffer: Vec<T> = Vec::with_capacity((width * height) as usize);
        for _ in 0..(width * height) {
            buffer.push(setting);
        }

        let pitch = width as usize * mem::size_of::<T>();

        if pitch > i32::MAX as usize {
            panic!("width * sizeof(T) (pitch) is too large");
        }

        PixelBuffer {
            width: width,
            height: height,
            pitch: pitch as i32,
            buffer: buffer.into_boxed_slice(),
        }
    }
}

impl<T> PixelBuffer<T> {
    pub fn width(&self) -> i32 {
        self.width
    }
    
    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn pitch(&self) -> i32 {
        self.pitch
    }
}

impl PixelBuffer<u32> {
    pub fn render_weird_gradient(&mut self) {
        let mut pixel = 0;
        for y in 0..self.height {
            for x in 0..self.width {

                let green = x as u8;
                let blue = y as u8;

                self.buffer[pixel] = ((green as u32) << 8) | (blue as u32);
                pixel += 1;
            }
        }
    }
}
