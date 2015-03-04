use handmade_hero::audio::AudioFunction;
use std::sync::mpsc::{self, Sender, Receiver};

#[allow(dead_code)]
pub enum SineCmd {
    SetHz(f32),
    SetVol(f32),
}

pub struct SineWave {
    hz: f32,
    vol: f32,
    tsin: f32,
    rx: Receiver<SineCmd>,
}

impl SineWave {
    pub fn new(hz: f32, volume: f32) -> (SineWave, Sender<SineCmd>) {
        let (tx, rx) = mpsc::channel();
        (SineWave {
            hz: hz,
            vol: volume,
            tsin: 0.0,
            rx: rx,
        }, tx)
    }
}

impl AudioFunction for SineWave {
    fn call(&mut self, out: &mut [f32], bitrate: usize) {
        use std::num::Float;
        use std::f32::consts::PI;

        match self.rx.try_recv() {
            Ok(SineCmd::SetHz(hz)) => {
                if hz != 0.0 {
                    self.hz = hz;
                }
            },
            Ok(SineCmd::SetVol(vol)) => self.vol = vol,
            Err(_) => {},
        }

        let period = bitrate as f32 / self.hz;

        for n in out.iter_mut() {
            self.tsin += 2.0 * PI / period;

            *n = self.tsin.sin() * self.vol;
        }
        self.tsin -= (out.len() / period as usize) as f32 * 2.0 * PI;
    }
}
