use std::iter::Iterator;

use sdl2::Sdl;
use sdl2::event::{Event, EventPump};
use sdl2::keyboard::get_scancode_from_key;
use sdl2::scancode::ScanCode;
use sdl2::keycode::KeyCode;
use sdl2::controller::Axis;

use std::mem;

#[allow(dead_code)]
mod keys {
    bitflags! {
        flags Keys: u64 {
            const UP    = 0b0001,
            const DOWN  = 0b0010,
            const LEFT  = 0b0100,
            const RIGHT = 0b1000,
        }
    }
}

enum Control {
    Keys(keys::Keys),
    Controller(i16, i16),
}

/// direction: Value from -100 to 100
pub struct Input {
    pub direction: (i16, i16),
}

pub struct GameLoop<'a> {
    control: Control,
    event_pump: EventPump<'a>,
    quit: bool,
    _context: Sdl,
}

impl<'a> GameLoop<'a> {
    pub fn new(context: Sdl) -> GameLoop<'a> {
        // This is ugly, but necessary to put the context and event_pump in the
        // same struct
        let event_pump = unsafe {
            mem::transmute::<&Sdl, &Sdl>(&context).event_pump()
        };

        GameLoop {
            control: Control::Keys(keys::Keys::empty()),
            event_pump: event_pump,
            quit: false,
            _context: context,
        }
    }

    fn key_down(&mut self, kc: KeyCode) {
        match self.control {
            Control::Keys(ref mut keys) => {
                match get_scancode_from_key(kc) {
                    ScanCode::Up | ScanCode::W =>
                        keys.insert(keys::UP),
                    ScanCode::Down | ScanCode::S =>
                        keys.insert(keys::DOWN),
                    ScanCode::Left | ScanCode::A =>
                        keys.insert(keys::LEFT),
                    ScanCode::Right | ScanCode::D =>
                        keys.insert(keys::RIGHT),
                    ScanCode::Escape =>
                        self.quit = true,
                    _ => {},
                }
            },
            Control::Controller(x, y) => {
                self.control =
                match get_scancode_from_key(kc) {
                    ScanCode::Up | ScanCode::W =>
                        Control::Keys(
                            keys::Keys::from_bits_truncate(keys::UP.bits())
                        ),
                    ScanCode::Down | ScanCode::S =>
                        Control::Keys(
                            keys::Keys::from_bits_truncate(keys::DOWN.bits())
                        ),
                    ScanCode::Left | ScanCode::A =>
                        Control::Keys(
                            keys::Keys::from_bits_truncate(keys::LEFT.bits())
                        ),
                    ScanCode::Right | ScanCode::D =>
                        Control::Keys(
                            keys::Keys::from_bits_truncate(keys::RIGHT.bits())
                        ),
                    ScanCode::Escape => {
                        self.quit = true;
                        Control::Keys(keys::Keys::empty())
                    },
                    _ => {Control::Controller(x, y)},
                }
            }
        }
    }

    fn key_up(&mut self, kc: KeyCode) {
        match self.control {
            Control::Keys(ref mut keys) => {
                match get_scancode_from_key(kc) {
                    ScanCode::Up | ScanCode::W =>
                        keys.remove(keys::UP),
                    ScanCode::Down | ScanCode::S =>
                        keys.remove(keys::DOWN),
                    ScanCode::Left | ScanCode::A =>
                        keys.remove(keys::LEFT),
                    ScanCode::Right | ScanCode::D =>
                        keys.remove(keys::RIGHT),
                    _ => {},
                }
            },
            Control::Controller(_, _) => {},
        }
    }

    fn set_controller(&mut self, axis: Axis, value: i16) {
        match self.control {
            Control::Keys(keys) => {
                self.control =
                match axis {
                    Axis::LeftX => {
                        Control::Controller(value, 0)
                    },
                    Axis::LeftY => {
                        Control::Controller(0, value)
                    },
                    _ => Control::Keys(keys),
                };
            },
            Control::Controller(x, y) => {
                self.control =
                match axis {
                    Axis::LeftX => {
                        Control::Controller(value, y)
                    },
                    Axis::LeftY => {
                        Control::Controller(x, value)
                    },
                    _ => Control::Controller(x, y),
                }
            },
        }
    }

    fn get_direction(&self) -> (i16, i16) {
        match self.control {
            Control::Keys(keys) => (
                if keys.intersects(keys::RIGHT | keys::LEFT) {
                    if keys.contains(keys::RIGHT | keys::LEFT) {
                        0
                    } else if keys.contains(keys::LEFT) {
                        100
                    } else {
                        -100
                    }
                } else {
                    0
                },
                if keys.intersects(keys::UP | keys::DOWN) {
                    if keys.contains(keys::UP | keys::DOWN) {
                        0
                    } else if keys.contains(keys::UP) {
                        100
                    } else {
                        -100
                    }
                } else {
                    0
                }
            ),
            Control::Controller(x, y) => (x, y),
        }
    }
}

impl<'a> Iterator for GameLoop<'a> {
    type Item = Input;

    fn next(&mut self) -> Option<Input> {
        match self.event_pump.poll_event() {
            Some(e) => match e {
                Event::Quit {..} => self.quit = true,
                Event::KeyDown {keycode, repeat, ..} => {
                    if !repeat {
                        self.key_down(keycode);
                    }
                },
                Event::KeyUp {keycode, repeat, ..} => {
                    if !repeat {
                        self.key_up(keycode);
                    }
                },
                Event::ControllerAxisMotion {which, axis, value, ..} => {
                    if which == 0 {
                        println!("{}", which);
                        self.set_controller(axis, value)
                    } else {
                        println!("{}", which);
                    }
                },
                _ => {},
            },
            None => {},
        }

        if self.quit {
            None
        } else {
            Some( Input {
                direction: self.get_direction(),
            })
        }
    }
}
