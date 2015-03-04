use pixelbuffer::PixelBuffer;

pub trait Renderer {
    fn blit<T>(&mut self, buffer: &PixelBuffer<T>);
}
