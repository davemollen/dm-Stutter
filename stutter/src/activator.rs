use crate::{crossfade::Crossfade, shared::tuple_ext::TupleExt};

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
    dry_signal: (f32, f32),
    wet_signal: (f32, f32),
    on: bool,
    chance: f32,
    auto_trigger: bool,
    trigger: bool,
    manual_trigger: bool,
    mix: i32,
  ) -> (f32, f32) {
    if trigger {
      if auto_trigger && !manual_trigger {
        let random = fastrand::f32();
        self.is_active = random <= chance
      } else {
        self.is_active = true;
      }
    }

    let (activity_fade_a, activity_fade_b) = self
      .crossfade
      .process(if on && self.is_active { 1. } else { 0. }, 20.);

    match mix {
      0 => wet_signal
        .multiply_with(activity_fade_a)
        .add(dry_signal.multiply_with(activity_fade_b)),
      1 => wet_signal.multiply_with(activity_fade_a).add(dry_signal),
      _ => wet_signal.multiply_with(activity_fade_a),
    }
  }
}
