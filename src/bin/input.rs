use handmade_hero::input::keys;

pub fn direction(keys: keys::Keys) -> (isize, isize) {
    let mut x = 0;
    let mut y = 0;

    if keys.intersects(keys::UP | keys::W) {
        y += 100;
    }
    if keys.intersects(keys::DOWN | keys::S) {
        y -= 100;
    }
    if keys.intersects(keys::LEFT | keys::A) {
        x += 100;
    }
    if keys.intersects(keys::RIGHT | keys::D) {
        x -= 100;
    }

    (x, y)
}
