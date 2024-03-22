use crate::delay_line::{DelayLine, Interpolation};

pub struct Delay {
  delay_time: f32,
  delay_line: DelayLine,
}

impl Delay {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      delay_time: 1.,
      delay_line: DelayLine::new((sample_rate * 2.) as usize, sample_rate),
    }
  }

  pub fn process(
    &mut self,
    input: f32,
    trigger: bool,
    delay_time: f32,
    fade_a: f32,
    fade_b: f32,
  ) -> f32 {
    if trigger {
      self.delay_time = delay_time;
    };
    let delay_out = self.delay_line.read(self.delay_time, Interpolation::Linear) * fade_a;

    self.delay_line.write(input * fade_b + delay_out);

    delay_out
  }
}
