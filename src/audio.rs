pub trait AudioFunction: Send {
    fn call(&mut self, out: &mut [f32], frequency: usize, channels: usize);
}

pub trait AudioDevice {
    fn play(&self);
    fn stop(&self);
}
