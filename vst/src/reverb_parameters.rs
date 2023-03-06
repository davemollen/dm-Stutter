use reverb::{MAX_SIZE, MIN_SIZE};
use std::sync::Arc;
use vst::{plugin::PluginParameters, util::AtomicFloat};
mod formatters;
use formatters::v2s_f32_digits;

pub enum FloatRange {
  Linear { min: f32, max: f32 },
  Skewed { min: f32, max: f32, factor: f32 },
}

impl FloatRange {
  pub fn normalize(&self, value: f32) -> f32 {
    match self {
      FloatRange::Linear { min, max } => (value.clamp(*min, *max) - min) / (max - min),
      FloatRange::Skewed { min, max, factor } => {
        ((value.clamp(*min, *max) - min) / (max - min)).powf(*factor)
      }
    }
  }

  pub fn unnormalize(&self, normalized: f32) -> f32 {
    match self {
      FloatRange::Linear { min, max } => (normalized * (max - min)) + min,
      FloatRange::Skewed { min, max, factor } => {
        (normalized.powf(factor.recip()) * (max - min)) + min
      }
    }
  }
}

pub struct FloatParam {
  pub name: &'static str,
  pub value: AtomicFloat,
  pub default: f32,
  pub index: i32,
  pub range: FloatRange,
  pub unit: &'static str,
  pub value_to_string: Option<Arc<dyn Fn(f32) -> String + Send + Sync>>,
}

impl FloatParam {
  pub fn new(name: &'static str, default: f32, index: i32, range: FloatRange) -> Self {
    Self {
      name,
      value: AtomicFloat::new(default),
      default,
      index,
      range,
      unit: "",
      value_to_string: None,
    }
  }

  pub fn get_value(&self) -> f32 {
    self.value.get()
  }

  pub fn get_normalized_value(&self) -> f32 {
    self.range.normalize(self.get_value())
  }

  pub fn set_plain_value(&self, value: f32) {
    let plain_value = self.range.unnormalize(value);
    self.value.set(plain_value);
  }

  pub fn get_display_value(&self, include_unit: bool) -> String {
    let value = self.get_value();
    match (&self.value_to_string, include_unit) {
      (Some(f), true) => format!("{}{}", f(value), self.unit),
      (Some(f), false) => f(value),
      (None, true) => format!("{}{}", value, self.unit),
      (None, false) => value.to_string(),
    }
  }

  pub fn get_default_normalized_value(&self) -> f32 {
    self.range.normalize(self.default)
  }

  pub fn with_unit(mut self, unit: &'static str) -> Self {
    self.unit = unit;
    self
  }

  pub fn with_value_to_string(
    mut self,
    callback: Arc<dyn Fn(f32) -> String + Send + Sync>,
  ) -> Self {
    self.value_to_string = Some(callback);
    self
  }
}

pub struct ReverbParameters {
  pub predelay: FloatParam,
  pub size: FloatParam,
  pub speed: FloatParam,
  pub depth: FloatParam,
  pub absorb: FloatParam,
  pub decay: FloatParam,
  pub tilt: FloatParam,
  pub mix: FloatParam,
}

impl Default for ReverbParameters {
  fn default() -> Self {
    Self {
      predelay: FloatParam::new(
        "Predelay",
        7.,
        0,
        FloatRange::Skewed {
          min: 7.,
          max: 500.,
          factor: 0.5,
        },
      )
      .with_unit("ms")
      .with_value_to_string(v2s_f32_digits(2)),
      size: FloatParam::new(
        "Size",
        40.,
        1,
        FloatRange::Skewed {
          min: MIN_SIZE,
          max: MAX_SIZE,
          factor: 0.333333,
        },
      )
      .with_unit("m2")
      .with_value_to_string(v2s_f32_digits(2)),
      speed: FloatParam::new(
        "Speed",
        2.,
        2,
        FloatRange::Skewed {
          min: 0.01,
          max: 50.,
          factor: 0.333333,
        },
      )
      .with_unit("Hz")
      .with_value_to_string(v2s_f32_digits(2)),
      depth: FloatParam::new("Depth", 0.25, 3, FloatRange::Linear { min: 0., max: 1. })
        .with_unit("%")
        .with_value_to_string(formatters::v2s_f32_percentage(2)),
      absorb: FloatParam::new("Absorb", 0.5, 4, FloatRange::Linear { min: 0., max: 1. })
        .with_unit("%")
        .with_value_to_string(formatters::v2s_f32_percentage(2)),
      decay: FloatParam::new("Decay", 0.9, 5, FloatRange::Linear { min: 0., max: 1.2 })
        .with_unit("%")
        .with_value_to_string(formatters::v2s_f32_percentage(2)),
      tilt: FloatParam::new("Tilt", 0.5, 6, FloatRange::Linear { min: 0., max: 1. })
        .with_unit("%")
        .with_value_to_string(Arc::new(move |value| format!("{:.2}", value * 200. - 100.))),
      mix: FloatParam::new("Mix", 0.5, 7, FloatRange::Linear { min: 0., max: 1. })
        .with_unit("%")
        .with_value_to_string(formatters::v2s_f32_percentage(2)),
    }
  }
}

impl PluginParameters for ReverbParameters {
  fn get_parameter(&self, index: i32) -> f32 {
    match index {
      0 => self.predelay.get_normalized_value(),
      1 => self.size.get_normalized_value(),
      2 => self.speed.get_normalized_value(),
      3 => self.depth.get_normalized_value(),
      4 => self.absorb.get_normalized_value(),
      5 => self.decay.get_normalized_value(),
      6 => self.tilt.get_normalized_value(),
      7 => self.mix.get_normalized_value(),
      _ => 0.0,
    }
  }

  fn get_parameter_text(&self, index: i32) -> String {
    match index {
      0 => self.predelay.get_display_value(true),
      1 => self.size.get_display_value(true),
      2 => self.speed.get_display_value(true),
      3 => self.depth.get_display_value(true),
      4 => self.absorb.get_display_value(true),
      5 => self.decay.get_display_value(true),
      6 => self.tilt.get_display_value(true),
      7 => self.mix.get_display_value(true),
      _ => "".to_string(),
    }
  }

  fn get_parameter_name(&self, index: i32) -> String {
    match index {
      0 => self.predelay.name,
      1 => self.size.name,
      2 => self.speed.name,
      3 => self.depth.name,
      4 => self.absorb.name,
      5 => self.decay.name,
      6 => self.tilt.name,
      7 => self.mix.name,
      _ => "",
    }
    .to_string()
  }

  fn set_parameter(&self, index: i32, val: f32) {
    match index {
      0 => self.predelay.set_plain_value(val),
      1 => self.size.set_plain_value(val),
      2 => self.speed.set_plain_value(val),
      3 => self.depth.set_plain_value(val),
      4 => self.absorb.set_plain_value(val),
      5 => self.decay.set_plain_value(val),
      6 => self.tilt.set_plain_value(val),
      7 => self.mix.set_plain_value(val),
      _ => (),
    }
  }
}
