use crate::shared::float_ext::FloatExt;

pub struct Phasor {
  sample_rate: f32,
  phase: f32,
  prev_phase: f32,
}

impl Phasor {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      sample_rate,
      phase: 0.,
      prev_phase: 0.,
    }
  }

  pub fn reset(&mut self) {
    self.phase = 0.;
    self.prev_phase = 0.;
  }

  pub fn process(&mut self, duration: f32) -> bool {
    let step_size = duration.mstosamps(self.sample_rate).recip();
    self.phase += step_size;

    if self.phase >= 1. {
      self.phase -= 1.;
    }

    let trigger = self.phase - self.prev_phase < 0.;
    self.prev_phase = self.phase;

    trigger
  }
}
