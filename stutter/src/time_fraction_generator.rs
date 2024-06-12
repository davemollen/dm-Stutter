pub struct TimeFractionGenerator {
  fraction: f32,
}

// const FRACTIONS: [f32; 10] = [1. / 2., 2. / 3., 3. / 4., 1., 1.5, 2., 3., 4., 6., 8.];
const FRACTIONS: [f32; 12] = [
  1. / 2.,
  3. / 4.,
  3. / 4.,
  1.,
  1.,
  1.,
  2.,
  2.,
  3.,
  4.,
  4.,
  8.,
];
// const FRACTIONS: [f32; 7] = [1. / 2., 3. / 4., 1., 1., 2., 4., 8.];

impl TimeFractionGenerator {
  pub fn new() -> Self {
    Self { fraction: 1. }
  }

  pub fn process(&mut self, trigger: bool) -> f32 {
    if trigger {
      let index = fastrand::usize(..FRACTIONS.len());
      self.fraction = FRACTIONS[index];
    }

    self.fraction
  }
}
