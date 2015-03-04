#![feature(collections, unsafe_destructor)]

extern crate sdl2;
extern crate "sdl2-sys" as sdl2_sys;

#[macro_use]
extern crate bitflags;

pub mod audio;
pub mod game;
pub mod input;
pub mod pixelbuffer;
pub mod renderer;

pub mod sdl;
