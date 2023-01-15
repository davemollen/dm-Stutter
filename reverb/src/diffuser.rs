use super::allpass_filter::AllpassFilter;

pub struct Diffuser {
  allpass_filter: AllpassFilter,
  time: f32,
}

impl Diffuser {
  pub fn new(sample_rate: f64, time: f32) -> Self {
    Self {
      allpass_filter: AllpassFilter::new(sample_rate),
      time,
    }
  }

  pub fn run(&mut self, input: f32, diffuse: f32) -> f32 {
    self.allpass_filter.run(input, self.time, diffuse)
  }
}
