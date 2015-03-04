use std::iter::Iterator;

use sdl2::Sdl;
use sdl2::event::{Event, EventPump};
use sdl2::keyboard::get_scancode_from_key;
use sdl2::scancode::ScanCode;
use sdl2::keycode::KeyCode;

use std::mem;

#[allow(dead_code)]
/// Based on scancodes, not keycodes
/// So the same button on any keyboard will say where that would be on a qwerty
pub mod keys {
    bitflags! {
        flags Keys: u64 {
            const NONE = 0,
            const SHIFT = 1 << 0,
            const Q = 1 << 1,
            const W = 1 << 2,
            const E = 1 << 3,
            const R = 1 << 4,
            const T = 1 << 5,
            const Y = 1 << 6,
            const U = 1 << 7,
            const I = 1 << 8,
            const O = 1 << 9,
            const P = 1 << 10,
            const LEFTBRACKET = 1 << 11,
            const RIGHTBRACKET = 1 << 12,
            const BACKSLASH = 1 << 13,

            const A = 1 << 14,
            const S = 1 << 15,
            const D = 1 << 16,
            const F = 1 << 17,
            const G = 1 << 18,
            const H = 1 << 19,
            const J = 1 << 20,
            const K = 1 << 21,
            const L = 1 << 22,
            const COLON = 1 << 23,
            const QUOTE = 1 << 24,

            const Z = 1 << 25,
            const X = 1 << 26,
            const C = 1 << 27,
            const V = 1 << 28,
            const B = 1 << 29,
            const N = 1 << 30,
            const M = 1 << 31,
            const COMMA = 1 << 32,
            const PERIOD = 1 << 33,
            const SLASH = 1 << 34,

            const NUM1 = 1 << 35,
            const NUM2 = 1 << 36,
            const NUM3 = 1 << 37,
            const NUM4 = 1 << 38,
            const NUM5 = 1 << 39,
            const NUM6 = 1 << 40,
            const NUM7 = 1 << 41,
            const NUM8 = 1 << 42,
            const NUM9 = 1 << 43,
            const NUM0 = 1 << 44,
            const MINUS = 1 << 45,
            const PLUS = 1 << 46,
            const TILDE = 1 << 47,

            const UP = 1 << 48,
            const DOWN = 1 << 49,
            const LEFT = 1 << 50,
            const RIGHT = 1 << 51,

            const ESCAPE = 1 << 52,
            const TAB = 1 << 53,
            const CTRL = 1 << 54,
            const ALT = 1 << 55,
            const BACKSPACE = 1 << 56,
            const ENTER = 1 << 57,
            const SPACE = 1 << 58,
            const QUIT = 1 << 59,
        }
    }
}
use self::keys::Keys;

impl Keys {
    pub fn from_keycode(keycode: KeyCode) -> Keys {
        let scancode = get_scancode_from_key(keycode);
        match scancode {
            ScanCode::A => keys::A,
            ScanCode::B => keys::B,
            ScanCode::C => keys::C,
            ScanCode::D => keys::D,
            ScanCode::E => keys::E,
            ScanCode::F => keys::F,
            ScanCode::G => keys::G,
            ScanCode::H => keys::H,
            ScanCode::I => keys::I,
            ScanCode::J => keys::J,
            ScanCode::K => keys::K,
            ScanCode::L => keys::L,
            ScanCode::M => keys::M,
            ScanCode::N => keys::N,
            ScanCode::O => keys::O,
            ScanCode::P => keys::P,
            ScanCode::Q => keys::Q,
            ScanCode::R => keys::R,
            ScanCode::S => keys::S,
            ScanCode::T => keys::T,
            ScanCode::U => keys::U,
            ScanCode::V => keys::V,
            ScanCode::W => keys::W,
            ScanCode::X => keys::X,
            ScanCode::Y => keys::Y,
            ScanCode::Z => keys::Z,
            ScanCode::Num1 => keys::NUM1,
            ScanCode::Num2 => keys::NUM2,
            ScanCode::Num3 => keys::NUM3,
            ScanCode::Num4 => keys::NUM4,
            ScanCode::Num5 => keys::NUM5,
            ScanCode::Num6 => keys::NUM6,
            ScanCode::Num7 => keys::NUM7,
            ScanCode::Num8 => keys::NUM8,
            ScanCode::Num9 => keys::NUM9,
            ScanCode::Num0 => keys::NUM0,
            ScanCode::Return => keys::ENTER,
            ScanCode::Escape => keys::ESCAPE,
            ScanCode::Backspace => keys::BACKSPACE,
            ScanCode::Tab => keys::TAB,
            ScanCode::Space => keys::SPACE,
            ScanCode::Minus => keys::MINUS,
            ScanCode::Equals => keys::PLUS,
            ScanCode::LeftBracket => keys::LEFTBRACKET,
            ScanCode::RightBracket => keys::RIGHTBRACKET,
            ScanCode::Backslash => keys::BACKSLASH,
            ScanCode::Semicolon => keys::COLON,
            ScanCode::Apostrophe => keys::QUOTE,
            ScanCode::Grave => keys::TILDE,
            ScanCode::Comma => keys::COMMA,
            ScanCode::Period => keys::PERIOD,
            ScanCode::Slash => keys::SLASH,
            ScanCode::Up => keys::UP,
            ScanCode::Down => keys::DOWN,
            ScanCode::Left => keys::LEFT,
            ScanCode::Right => keys::RIGHT,
            _ => keys::NONE,
        }
    }
}

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
