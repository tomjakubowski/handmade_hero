use std::iter::Iterator;

use sdl2::Sdl;
use sdl2::event::{Event, EventPump};
use sdl2::keycode::KeyCode;

use input::keys::{self, Keys};

use std::mem;

pub struct GameLoop<'a> {
    keys: Keys,
    event_pump: EventPump<'a>,
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
            keys: Keys::empty(),
            event_pump: event_pump,
            _context: context,
        }
    }

    fn key_down(&mut self, kc: KeyCode) {
        self.keys.insert(Keys::from_keycode(kc));
    }

    fn key_up(&mut self, kc: KeyCode) {
        self.keys.remove(Keys::from_keycode(kc));
    }
}

impl<'a> Iterator for GameLoop<'a> {
    type Item = Keys;

    fn next(&mut self) -> Option<Keys> {
        match self.event_pump.poll_event() {
            Some(e) => match e {
                Event::Quit {..} => self.keys.insert(keys::QUIT),
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
                _ => {},
            },
            None => {},
        }

        if self.keys.intersects(keys::ESCAPE | keys::QUIT) {
            None
        } else {
            Some(self.keys)
        }
    }
}
