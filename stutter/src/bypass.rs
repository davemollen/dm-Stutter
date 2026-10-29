mod smooth;
use crate::shared::float_ext::FloatExt;
use smooth::{LinearSmooth, Smoother};

const FADE_TIME_IN_MS: f32 = 1.;

pub struct Bypass {
  mix_factor: LinearSmooth,
  prev_enabled: bool,
}

impl Bypass {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      mix_factor: LinearSmooth::new(sample_rate, 1000. / FADE_TIME_IN_MS),
      prev_enabled: false,
    }
  }

  pub fn reset(&mut self) {
    self.mix_factor.reset(1.);
  }

  pub fn set_enabled<F: FnMut()>(&mut self, enabled: bool, mut on_disabled: F) {
    if self.prev_enabled && !enabled {
      on_disabled();
    }
    self.mix_factor.set_target(if enabled { 0. } else { 1. });
    self.prev_enabled = enabled;
  }

  pub fn is_done_activating(&self) -> bool {
    self.mix_factor.target == 0. && self.mix_factor.current == 0.
  }

  pub fn process(&mut self, dry: (f32, f32), fx: (f32, f32, bool, bool)) -> (f32, f32, bool, bool) {
    let mix_factor = self.mix_factor.next().cubic_spline_curve();
    let is_active = mix_factor > 0.;
    (
      dry.0.mix(fx.0, mix_factor),
      dry.1.mix(fx.1, mix_factor),
      is_active && fx.2,
      is_active && fx.3,
    )
  }
}
