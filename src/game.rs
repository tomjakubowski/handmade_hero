use std::iter::Iterator;
use input::Input;

pub trait GameLoop: Iterator<Item = Input> {}
