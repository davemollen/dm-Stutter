use reverb::{MAX_SIZE, MIN_SIZE};
use vst::{plugin::PluginParameters, util::AtomicFloat};

pub struct ReverbParameters {
  pub predelay: AtomicFloat,
  pub size: AtomicFloat,
  pub speed: AtomicFloat,
  pub depth: AtomicFloat,
  pub absorb: AtomicFloat,
  pub decay: AtomicFloat,
  pub tilt: AtomicFloat,
  pub mix: AtomicFloat,
}

impl Default for ReverbParameters {
  fn default() -> Self {
    Self {
      predelay: AtomicFloat::new(7.),
      size: AtomicFloat::new(40.),
      speed: AtomicFloat::new(2.),
      depth: AtomicFloat::new(0.25),
      absorb: AtomicFloat::new(0.5),
      decay: AtomicFloat::new(0.9),
      tilt: AtomicFloat::new(0.5),
      mix: AtomicFloat::new(0.5),
    }
  }
}

impl PluginParameters for ReverbParameters {
  fn get_parameter(&self, index: i32) -> f32 {
    match index {
      0 => ((self.predelay.get() - 7.) / 493.).powf(0.333333),
      1 => ((self.size.get() - MIN_SIZE) / (MAX_SIZE - MIN_SIZE)).powf(0.5),
      2 => ((self.speed.get() - 0.01) / 49.99).powf(0.333333),
      3 => self.depth.get(),
      4 => self.absorb.get(),
      5 => self.decay.get() / 1.2,
      6 => self.tilt.get(),
      7 => self.mix.get(),
      _ => 0.0,
    }
  }

  fn get_parameter_text(&self, index: i32) -> String {
    match index {
      0 => format!("{:.2}ms", self.predelay.get()),
      1 => format!("{:.2}m2", self.size.get()),
      2 => format!("{:.2}Hz", self.speed.get()),
      3 => format!("{:.2}%", self.depth.get() * 100.),
      4 => format!("{:.2}%", self.absorb.get() * 100.),
      5 => format!("{:.2}%", self.decay.get() * 100.),
      6 => format!("{:.2}%", self.tilt.get() * 200. - 100.),
      7 => format!("{:.2}%", self.mix.get() * 100.),
      _ => "".to_string(),
    }
  }

  fn get_parameter_name(&self, index: i32) -> String {
    match index {
      0 => "Predelay",
      1 => "Size",
      2 => "Speed",
      3 => "Depth",
      4 => "Absorb",
      5 => "Decay",
      6 => "Tilt",
      7 => "Mix",
      _ => "",
    }
    .to_string()
  }

  fn set_parameter(&self, index: i32, val: f32) {
    match index {
      0 => self.predelay.set(val.powf(3.) * 493. + 7.),
      1 => self
        .size
        .set(val.powf(2.) * (MAX_SIZE - MIN_SIZE) + MIN_SIZE),
      2 => self.speed.set(val.powf(3.) * 49.99 + 0.01),
      3 => self.depth.set(val),
      4 => self.absorb.set(val),
      5 => self.decay.set(val * 1.2),
      6 => self.tilt.set(val),
      7 => self.mix.set(val),
      _ => (),
    }
  }
}
