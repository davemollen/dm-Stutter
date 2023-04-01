use crate::{
  phasor::Phasor,
  stereo_delay_line::{Interpolation, StereoDelayLine},
};
use std::f32::consts::PI;

pub struct Shimmer {
  delay_line: StereoDelayLine,
  phasor: Phasor,
}

impl Shimmer {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      delay_line: StereoDelayLine::new((sample_rate * 0.2) as usize, sample_rate),
      phasor: Phasor::new(sample_rate),
    }
  }

  fn write(&mut self, input: (f32, f32)) {
    self.delay_line.write(input);
  }

  fn mix(&self, a: (f32, f32), b: (f32, f32), factor: f32) -> (f32, f32) {
    (
      a.0 * (1. - factor) + b.0 * factor,
      a.1 * (1. - factor) + b.1 * factor,
    )
  }

  fn apply_shimmer(&mut self) -> (f32, f32) {
    let main_phase = self.phasor.run(-5.);

    (0..2)
      .map(|index| {
        let phase = (main_phase + index as f32 * 0.5) % 1.;
        let window = (phase * PI).sin();
        let window = window * window;
        let read = self.delay_line.read(phase * 200., Interpolation::Linear);
        (read.0 * window, read.1 * window)
      })
      .fold((0., 0.), |sum, shimmer| {
        (sum.0 + shimmer.0, sum.1 + shimmer.1)
      })
  }

  pub fn run(&mut self, input: (f32, f32), mix: f32) -> (f32, f32) {
    let out = if mix > 0. {
      let grains_out = self.apply_shimmer();
      self.mix(input, grains_out, mix)
    } else {
      input
    };
    self.write(input);
    out
  }
}
