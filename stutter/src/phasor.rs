pub struct Phasor {
  sample_rate: f32,
  phase: f32,
  previous_phase: f32,
  flip_flop: bool,
}

impl Phasor {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      sample_rate,
      phase: 0.,
      previous_phase: 0.,
      flip_flop: false,
    }
  }

  pub fn process(&mut self, duration: f32, auto_trigger: bool, manual_trigger: bool) {
    let step_size = self.mstosamps(duration).recip();
    self.phase += step_size;
    if self.phase >= 1. {
      if auto_trigger {
        self.phase = 0.;
      } else {
        self.phase = 1.;
      }
    }

    if !auto_trigger && manual_trigger {
      self.phase = 0.;
    }
  }

  pub fn get_trigger(&mut self) -> (bool, bool) {
    let diff = self.phase - self.previous_phase;
    self.previous_phase = self.phase;

    match (diff < 0., self.flip_flop) {
      (true, false) => {
        self.flip_flop = true;
        (true, false)
      }
      (true, true) => {
        self.flip_flop = false;
        (false, true)
      }
      _ => (false, false),
    }
  }

  pub fn get_flip_flop(&self) -> bool {
    self.flip_flop
  }

  fn mstosamps(&self, time: f32) -> f32 {
    time * 0.001 * self.sample_rate
  }
}
