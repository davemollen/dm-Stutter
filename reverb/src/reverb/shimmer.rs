use crate::shared::{
  delay_line::{DelayLine, Interpolation},
  float_ext::FloatExt,
  phasor::Phasor,
};
use std::f32::consts::PI;

pub struct Shimmer {
  delay_line: DelayLine,
  phasor: Phasor,
}

impl Shimmer {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      delay_line: DelayLine::new((sample_rate * 0.2) as usize, sample_rate),
      phasor: Phasor::new(sample_rate),
    }
  }

  fn write(&mut self, input: f32) {
    self.delay_line.write(input);
  }

  fn mix(&self, a: f32, b: f32, factor: f32) -> f32 {
    a * (1. - factor) + b * factor
  }

  fn apply_shimmer(&mut self) -> f32 {
    let main_phase = self.phasor.run(-5.);

    (0..2)
      .map(|index| {
        let phase = (main_phase + index as f32 * 0.5) % 1.;
        let window = (phase * PI).fast_sin();
        self.delay_line.read(phase * 200., Interpolation::Linear) * window * window
      })
      .sum()
  }

  pub fn run(&mut self, input: f32, mix: f32) -> f32 {
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
