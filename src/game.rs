use std::iter::Iterator;
use input::keys::Keys;

pub trait GameLoop: Iterator<Item = Keys> {}
