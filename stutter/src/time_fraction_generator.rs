pub struct TimeFractionGenerator {
  fraction: Option<f32>,
  probability: Vec<(f32, f32)>,
}

impl TimeFractionGenerator {
  pub fn new() -> Self {
    Self {
      fraction: None,
      probability: Vec::with_capacity(15),
    }
  }

  pub fn set_probability(&mut self, probability: [(f32, f32); 15]) {
    let accumulated_probability = probability
      .into_iter()
      .scan((0., 0.), |acc, (chance, value)| {
        (*acc).0 += chance;
        (*acc).1 = value;
        Some(*acc)
      });
    let total = accumulated_probability.clone().last().unwrap().0;

    self.probability = accumulated_probability
      .map(|(chance, value)| (chance / total, value))
      .collect();
  }

  pub fn process(&mut self, trigger: bool) -> Option<f32> {
    if trigger {
      let random_num = fastrand::f32();
      self.fraction = self.get_fraction(random_num);
    }

    self.fraction
  }

  fn get_fraction(&self, random_num: f32) -> Option<f32> {
    match self.probability.iter().find(|item| random_num < item.0) {
      Some(f) => Some(f.1),
      None => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::TimeFractionGenerator;

  #[test]
  fn normalize() {
    let fraction_gen = &mut TimeFractionGenerator::new();

    fraction_gen.set_probability([
      (0., 2.),
      (0.5, 1.75),
      (0.5, 1.5),
      (0.5, 1.33333333),
      (0., 1.25),
      (0., 1.),
      (0., 0.75),
      (0., 1.5_f32.recip()),
      (0., 2_f32.recip()),
      (0., 3_f32.recip()),
      (0., 4_f32.recip()),
      (0., 6_f32.recip()),
      (0., 8_f32.recip()),
      (0., 12_f32.recip()),
      (0., 16_f32.recip()),
    ]);

    assert_eq!(
      fraction_gen.probability,
      vec![
        (0., 2.),
        (0.33333334, 1.75),
        (0.6666667, 1.5),
        (1., 1.33333333),
        (1., 1.25),
        (1., 1.),
        (1., 0.75),
        (1., 1.5_f32.recip()),
        (1., 2_f32.recip()),
        (1., 3_f32.recip()),
        (1., 4_f32.recip()),
        (1., 6_f32.recip()),
        (1., 8_f32.recip()),
        (1., 12_f32.recip()),
        (1., 16_f32.recip()),
      ]
    );
  }

  #[test]
  fn get_fraction() {
    let fraction_gen = &mut TimeFractionGenerator::new();

    fraction_gen.set_probability([
      (0., 2.),
      (0.5, 1.75),
      (0.5, 1.5),
      (0.5, 1.33333333),
      (0., 1.25),
      (0., 1.),
      (0., 0.75),
      (0., 1.5_f32.recip()),
      (0., 2_f32.recip()),
      (0., 3_f32.recip()),
      (0., 4_f32.recip()),
      (0., 6_f32.recip()),
      (0., 8_f32.recip()),
      (0., 12_f32.recip()),
      (0., 16_f32.recip()),
    ]);

    assert_eq!(
      fraction_gen.probability,
      vec![
        (0., 2.),
        (0.33333334, 1.75),
        (0.6666667, 1.5),
        (1., 1.33333333),
        (1., 1.25),
        (1., 1.),
        (1., 0.75),
        (1., 1.5_f32.recip()),
        (1., 2_f32.recip()),
        (1., 3_f32.recip()),
        (1., 4_f32.recip()),
        (1., 6_f32.recip()),
        (1., 8_f32.recip()),
        (1., 12_f32.recip()),
        (1., 16_f32.recip()),
      ]
    );
    assert_eq!(fraction_gen.get_fraction(0.2), Some(1.75));
    assert_eq!(fraction_gen.get_fraction(0.4), Some(1.5));
    assert_eq!(fraction_gen.get_fraction(0.7), Some(1.3333334));
  }
}
