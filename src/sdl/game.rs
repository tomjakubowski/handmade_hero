use sdl2::Sdl;
use sdl2::event::{Event, EventPump};
use sdl2::keycode::KeyCode;

use input::keys::{self, Keys};
use input::Input;
use sdl::input::from_keycode;

use game;

use std::mem;

pub struct GameLoop<'a> {
    keys: Keys,
    event_pump: EventPump<'a>,
    _context: Sdl,
}

impl<'a> game::GameLoop for GameLoop<'a> {}

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
        self.keys.insert(from_keycode(kc));
    }

    fn key_up(&mut self, kc: KeyCode) {
        self.keys.remove(from_keycode(kc));
    }
}

impl<'a> Iterator for GameLoop<'a> {
    type Item = Input;

    fn next(&mut self) -> Option<Input> {
        'iter: loop {
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
                None => break 'iter,
            }
        }

        if self.keys.intersects(keys::ESCAPE | keys::QUIT) {
            None
        } else {
            Some(Input { keys: self.keys } )
        }
    }
}
