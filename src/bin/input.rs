use handmade_hero::game::keys;

pub fn direction(keys: keys::Keys) -> (isize, isize) {
    let mut x = 0;
    let mut y = 0;

    if keys.contains(keys::UP) {
        y += 100;
    }
    if keys.contains(keys::DOWN) {
        y -= 100;
    }
    if keys.contains(keys::LEFT) {
        x += 100;
    }
    if keys.contains(keys::RIGHT) {
        x -= 100;
    }

    (x, y)
}
