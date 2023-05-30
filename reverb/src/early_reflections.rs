use crate::{float_ext::FloatExt, tap::Tap, MAX_SIZE, MIN_SIZE};

const MINUS_TWELVE_DB: f32 = 0.251189;

pub struct EarlyReflections {
  reflections_left: [f32; 6],
  reflections_right: [f32; 6],
}

impl EarlyReflections {
  pub fn new() -> Self {
    Self {
      reflections_left: [0., 0.188, 0.278, 0.38, 0.482, 0.584],
      reflections_right: [0.018, 0.086, 0.29, 0.392, 0.494, 0.597],
    }
  }

  fn read_early_reflection(&self, size: f32, time_fraction: &f32, tap: &mut Tap) -> f32 {
    tap.read_early_reflection(size, *time_fraction)
  }

  fn process_channel(&mut self, reflections: [f32; 6], size: f32, tap: &mut Tap, gain: f32) -> f32 {
    reflections
      .iter()
      .map(|time_fraction| self.read_early_reflection(size, time_fraction, tap))
      .sum::<f32>()
      * gain
  }

  pub fn run(&mut self, size: f32, taps: &mut [Tap; 4]) -> (f32, f32) {
    let gain = size.scale(MIN_SIZE, MAX_SIZE, 1., MINUS_TWELVE_DB);
    let out_left = self.process_channel(self.reflections_left, size, &mut taps[0], gain);
    let out_right = self.process_channel(self.reflections_right, size, &mut taps[1], gain);
    (out_left, out_right)
  }
}
