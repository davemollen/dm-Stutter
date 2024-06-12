use crate::shared::float_ext::FloatExt;

pub struct Crossfade {
  prev: f32,
  index: usize,
  step_size: f32,
  z: f32,
  sample_rate: f32,
}

impl Crossfade {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      prev: 0.,
      index: 0,
      step_size: 0.,
      z: 0.,
      sample_rate,
    }
  }

  pub fn process(&mut self, input: f32, ramp_time: f32) -> (f32, f32) {
    let difference = input - self.z;

    if difference.is_equal_to(0.) {
      (input, 1. - input)
    } else {
      let ramp = self.ramp(input, self.mstosamps(ramp_time), difference);
      (ramp, 1. - ramp)
    }
  }

  fn ramp(&mut self, input: f32, ramp_time: f32, difference: f32) -> f32 {
    if input != self.prev {
      self.index = ramp_time as usize;
      self.step_size = difference / ramp_time;
      self.prev = input;
    }

    if self.index > 0 {
      self.index -= 1;
      self.z += self.step_size;
    }
    self.z
  }

  fn mstosamps(&self, time: f32) -> f32 {
    time * 0.001 * self.sample_rate
  }
}

#[cfg(test)]
mod tests {
  use super::Crossfade;

  #[test]
  fn should_ramp_up_and_down_in_time() {
    let ramp_time = 8.0;
    let mut smoother = Crossfade::new(1000.);

    assert_eq!(smoother.process(1., ramp_time), (0.125, 1. - 0.125));
    assert_eq!(smoother.process(1., ramp_time), (0.25, 1. - 0.25));
    assert_eq!(smoother.process(1., ramp_time), (0.375, 1. - 0.375));
    assert_eq!(smoother.process(1., ramp_time), (0.5, 1. - 0.5));
    assert_eq!(smoother.process(1., ramp_time), (0.625, 1. - 0.625));
    assert_eq!(smoother.process(1., ramp_time), (0.75, 1. - 0.75));
    assert_eq!(smoother.process(1., ramp_time), (0.875, 1. - 0.875));
    assert_eq!(smoother.process(1., ramp_time), (1.0, 1. - 1.0));
    assert_eq!(smoother.process(1., ramp_time), (1.0, 1. - 1.0));
    assert_eq!(smoother.process(0., ramp_time), (0.875, 1. - 0.875));
    assert_eq!(smoother.process(0., ramp_time), (0.75, 1. - 0.75));
    assert_eq!(smoother.process(0., ramp_time), (0.625, 1. - 0.625));
    assert_eq!(smoother.process(0., ramp_time), (0.5, 1. - 0.5));
    assert_eq!(smoother.process(1., ramp_time), (0.5625, 1. - 0.5625));
  }
}
