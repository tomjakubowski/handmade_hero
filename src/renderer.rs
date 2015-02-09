use sdl2::render::{self, Renderer, Texture};
use sdl2::video;
use sdl2::pixels;

use std::mem;

use pixelbuffer::PixelBuffer;

pub struct Renderer<'a> {
    renderer: Renderer,
    texture: Texture<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(width: i32, height: i32) -> HHRenderer<'a> {
        let renderer = Renderer::new_with_window(width, height,
             video::SHOWN).unwrap();

        // This is ugly, but necessary
        let texture = unsafe {
            mem::transmute::<&Renderer, &Renderer>(&renderer).create_texture(
                pixels::PixelFormatEnum::ARGB8888,
                render::TextureAccess::Streaming, (width, height)).unwrap()
        };

        HHRenderer {
            renderer: renderer,
            texture: texture,
        }
    }

    pub fn blit<T>(&mut self, buffer: &PixelBuffer<T>) {
        let _ = 
        unsafe { self.texture.update(None, mem::transmute(&*buffer.buffer),
            buffer.pitch() as i32)
        };

        self.renderer.drawer().clear();
        self.renderer.drawer().copy(&self.texture, None, None);
        self.renderer.drawer().present();
    }
}

#[unsafe_destructor]
impl<'a> Drop for Renderer<'a> {
    fn drop(&mut self) {
        use sdl2_sys::render::{SDL_DestroyRenderer, SDL_DestroyTexture};
        unsafe {
            SDL_DestroyTexture(self.texture.raw());
            SDL_DestroyRenderer(self.renderer.raw());
        }
    }
}

