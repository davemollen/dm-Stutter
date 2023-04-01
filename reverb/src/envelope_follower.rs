use crate::slide::Slide;

pub struct EnvelopeFollower {
  slide: Slide,
}

impl EnvelopeFollower {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      slide: Slide::new(sample_rate),
    }
  }

  pub fn get_value(&self) -> f32 {
    self.slide.get_value()
  }

  pub fn run(&mut self, input: f32) -> f32 {
    self.slide.run(input, 2., 80.).abs()
  }
}
