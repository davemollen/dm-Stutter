pub struct DcBlock {
  sample_rate: f32,
  z1: f32,
  z2: f32,
}

impl DcBlock {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      sample_rate,
      z1: 0.,
      z2: 0.,
    }
  }

  pub fn run(&mut self, input: f32) -> f32 {
    let coeff = 220.5 / self.sample_rate;
    let output = input - self.z1 + coeff * self.z2;
    self.z1 = input;
    self.z2 = output;
    output
  }
}
