use sdl2::{init, INIT_VIDEO};
use renderer::Renderer;
use pixelbuffer::PixelBuffer;
use game::GameLoop;

pub fn initialize<'a>(width: i32, height: i32) ->
        (Renderer<'a>, PixelBuffer<u32>, GameLoop<'a>) {
    let sdl_context = match init(INIT_VIDEO) {
        Ok(c) => c,
        Err(e) => panic!("SDL couldn't initialize: {}", e),
    };

    let renderer = Renderer::new(width, height);
    let buffer = PixelBuffer::new(width, height, 0u32);
    let game_loop = GameLoop::new(sdl_context);

    (renderer, buffer, game_loop)
}
