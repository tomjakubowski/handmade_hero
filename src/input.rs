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

pub struct Input {
    pub keys: keys::Keys,
    // TODO: Add clock
}
