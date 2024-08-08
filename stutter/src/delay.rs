use crate::{
  shared::tuple_ext::TupleExt,
  stereo_delay_line::{Interpolation, StereoDelayLine},
};

pub struct Delay {
  delay_time: f32,
  delay_line: StereoDelayLine,
}

impl Delay {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      delay_time: 1.,
      delay_line: StereoDelayLine::new((sample_rate * 12.) as usize, sample_rate),
    }
  }

  pub fn process(
    &mut self,
    input: (f32, f32),
    trigger: bool,
    delay_time: f32,
    fade_a: f32,
    fade_b: f32,
  ) -> (f32, f32) {
    if trigger {
      self.delay_time = delay_time;
    };

    let delay_out = self
      .delay_line
      .read(self.delay_time, Interpolation::Linear)
      .multiply_with(fade_a);

    self
      .delay_line
      .write(input.multiply_with(fade_b).add(delay_out));

    delay_out
  }
}
