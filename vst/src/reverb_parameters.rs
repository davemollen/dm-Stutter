use vst::{plugin::PluginParameters, util::AtomicFloat};

pub struct ReverbParameters {
  pub size: AtomicFloat,
  pub diffuse: AtomicFloat,
  pub absorb: AtomicFloat,
  pub decay: AtomicFloat,
  pub mix: AtomicFloat,
}

impl Default for ReverbParameters {
  fn default() -> Self {
    Self {
      size: AtomicFloat::new(40.0),
      diffuse: AtomicFloat::new(0.5),
      absorb: AtomicFloat::new(0.5),
      decay: AtomicFloat::new(0.9),
      mix: AtomicFloat::new(0.5),
    }
  }
}

impl PluginParameters for ReverbParameters {
  fn get_parameter(&self, index: i32) -> f32 {
    match index {
      0 => ((self.size.get() - 1.) / 499.).powf(0.333333),
      1 => self.diffuse.get(),
      2 => self.absorb.get().powf(2.),
      3 => self.decay.get(),
      4 => self.mix.get(),
      _ => 0.0,
    }
  }

  fn get_parameter_text(&self, index: i32) -> String {
    match index {
      0 => format!("{:.2} m2", self.size.get()),
      1 => format!("{:.2}%", self.diffuse.get() * 100.0),
      2 => format!("{:.2}%", self.absorb.get() * 100.0),
      3 => format!("{:.2}%", self.decay.get() * 100.0),
      4 => format!("{:.2}%", self.mix.get() * 100.0),
      _ => "".to_string(),
    }
  }

  fn get_parameter_name(&self, index: i32) -> String {
    match index {
      0 => "Size",
      1 => "Diffuse",
      2 => "Absorb",
      3 => "Decay",
      4 => "Mix",
      _ => "",
    }
    .to_string()
  }

  fn set_parameter(&self, index: i32, val: f32) {
    match index {
      0 => self.size.set(val.powf(3.) * 499. + 1.),
      1 => self.diffuse.set(val),
      2 => self.absorb.set(val.powf(0.5)),
      3 => self.decay.set(val),
      4 => self.mix.set(val),
      _ => (),
    }
  }
}
