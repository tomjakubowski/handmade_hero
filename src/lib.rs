#![feature(collections, unsafe_destructor)]
extern crate sdl2;
extern crate "sdl2-sys" as sdl2_sys;
#[macro_use]
extern crate bitflags;

mod handmade;
pub use handmade::*;

pub mod renderer;
pub mod pixelbuffer;
pub mod game;
