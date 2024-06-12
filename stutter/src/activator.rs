use crate::crossfade::Crossfade;

pub struct Activator {
  is_active: bool,
  crossfade: Crossfade,
}

impl Activator {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      is_active: false,
      crossfade: Crossfade::new(sample_rate),
    }
  }

  pub fn process(
    &mut self,
    dry_signal: f32,
    wet_signal: f32,
    on: bool,
    chance: f32,
    trigger: bool,
  ) -> f32 {
    if trigger {
      let random = fastrand::f32();
      self.is_active = random <= chance
    }

    let (activity_fade_a, activity_fade_b) = self
      .crossfade
      .process(if on && self.is_active { 1. } else { 0. }, 20.);

    wet_signal * activity_fade_a + dry_signal * activity_fade_b
  }
}
