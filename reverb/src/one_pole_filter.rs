use super::clip::Clip;
use std::f32;

pub struct OnePoleFilter {
  sample_rate: f32,
  z: f32,
}

impl OnePoleFilter {
  pub fn new(sample_rate: f64) -> Self {
    Self {
      sample_rate: sample_rate as f32,
      z: 0.,
    }
  }

  fn convert_hertz_to_coefficient(&mut self, freq: f32) -> f32 {
    let coef = (freq * 2. * f32::consts::PI / self.sample_rate).sin();
    Clip::run(coef, 0., 1.)
  }

  fn mix(&mut self, a: f32, b: f32, interp: f32) -> f32 {
    a * (1. - interp) + b * interp
  }

  pub fn run(&mut self, input: f32, cutoff_freq: f32, mode: &str) -> f32 {
    let coefficient = match mode {
      "linear" => 1. - cutoff_freq,
      "Hz" => self.convert_hertz_to_coefficient(cutoff_freq),
      _ => self.convert_hertz_to_coefficient(cutoff_freq),
    };
    let output = self.mix(self.z, input, coefficient);
    self.z = output;
    output
  }
}
