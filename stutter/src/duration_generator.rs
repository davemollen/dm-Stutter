pub struct DurationGenerator {
  fraction: f32,
}

impl DurationGenerator {
  pub fn new() -> Self {
    Self { fraction: 1. }
  }

  pub fn process(
    &mut self,
    delay_time: f32,
    time_fraction: f32,
    duration: f32,
    trigger: bool,
  ) -> f32 {
    if trigger {
      let random = fastrand::f32();
      let random = random * random;
      self.fraction = (random * duration * 8. * time_fraction).ceil().max(1.);
    }

    delay_time * self.fraction
  }
}
