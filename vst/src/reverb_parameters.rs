use reverb::{MAX_PREDELAY, MAX_SIZE, MIN_PREDELAY, MIN_SIZE};
use std::sync::Arc;
use vst::plugin::PluginParameters;
mod formatters;
use formatters::{s2v_f32_percentage, v2s_f32_digits, v2s_f32_percentage};
mod params;
pub use params::{BoolParam, FloatParam, FloatRange, Params};

pub struct ReverbParameters {
  pub reverse: BoolParam,
  pub predelay: FloatParam,
  pub size: FloatParam,
  pub speed: FloatParam,
  pub depth: FloatParam,
  pub absorb: FloatParam,
  pub decay: FloatParam,
  pub tilt: FloatParam,
  pub shimmer: FloatParam,
  pub mix: FloatParam,
}

impl Default for ReverbParameters {
  fn default() -> Self {
    Self {
      reverse: BoolParam::new("Reverse", false, 0),

      predelay: FloatParam::new(
        "Predelay",
        MIN_PREDELAY,
        1,
        FloatRange::Skewed {
          min: MIN_PREDELAY,
          max: MAX_PREDELAY,
          factor: 0.5,
        },
      )
      .with_unit(" ms")
      .with_value_to_string(v2s_f32_digits(2)),

      size: FloatParam::new(
        "Size",
        80.,
        2,
        FloatRange::Skewed {
          min: MIN_SIZE,
          max: MAX_SIZE,
          factor: 0.333333,
        },
      )
      .with_value_to_string(v2s_f32_digits(2)),

      speed: FloatParam::new(
        "Speed",
        2.,
        3,
        FloatRange::Skewed {
          min: 0.02,
          max: 150.,
          factor: 0.333333,
        },
      )
      .with_unit(" Hz")
      .with_value_to_string(v2s_f32_digits(2)),

      depth: FloatParam::new("Depth", 0.375, 4, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(Arc::new(move |value| format!("{:.2}", value * 200. - 100.)))
        .with_string_to_value(Arc::new(|string| {
          string
            .trim_end_matches(&[' ', '%'])
            .parse()
            .ok()
            .map(|x: f32| (x + 100.) / 200.0)
        })),

      absorb: FloatParam::new("Absorb", 0.5, 5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      decay: FloatParam::new("Decay", 0.9, 6, FloatRange::Linear { min: 0., max: 1.2 })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      tilt: FloatParam::new("Tilt", 0.5, 7, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(Arc::new(move |value| format!("{:.2}", value * 200. - 100.)))
        .with_string_to_value(Arc::new(|string| {
          string
            .trim_end_matches(&[' ', '%'])
            .parse()
            .ok()
            .map(|x: f32| (x + 100.) / 200.0)
        })),

      shimmer: FloatParam::new("Shimmer", 0., 8, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      mix: FloatParam::new("Mix", 0.5, 9, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),
    }
  }
}

impl PluginParameters for ReverbParameters {
  fn get_parameter(&self, index: i32) -> f32 {
    match index {
      0 => self.reverse.get_normalized_value(),
      1 => self.predelay.get_normalized_value(),
      2 => self.size.get_normalized_value(),
      3 => self.speed.get_normalized_value(),
      4 => self.depth.get_normalized_value(),
      5 => self.absorb.get_normalized_value(),
      6 => self.decay.get_normalized_value(),
      7 => self.tilt.get_normalized_value(),
      8 => self.shimmer.get_normalized_value(),
      9 => self.mix.get_normalized_value(),
      _ => 0.,
    }
  }

  fn get_parameter_text(&self, index: i32) -> String {
    match index {
      0 => self.reverse.get_display_value(true),
      1 => self.predelay.get_display_value(true),
      2 => self.size.get_display_value(true),
      3 => self.speed.get_display_value(true),
      4 => self.depth.get_display_value(true),
      5 => self.absorb.get_display_value(true),
      6 => self.decay.get_display_value(true),
      7 => self.tilt.get_display_value(true),
      8 => self.shimmer.get_display_value(true),
      9 => self.mix.get_display_value(true),
      _ => "".to_string(),
    }
  }

  fn get_parameter_name(&self, index: i32) -> String {
    match index {
      0 => self.reverse.name,
      1 => self.predelay.name,
      2 => self.size.name,
      3 => self.speed.name,
      4 => self.depth.name,
      5 => self.absorb.name,
      6 => self.decay.name,
      7 => self.tilt.name,
      8 => self.shimmer.name,
      9 => self.mix.name,
      _ => "",
    }
    .to_string()
  }

  fn set_parameter(&self, index: i32, val: f32) {
    match index {
      0 => self.reverse.set_normalized_value(val),
      1 => self.predelay.set_plain_value(val),
      2 => self.size.set_plain_value(val),
      3 => self.speed.set_plain_value(val),
      4 => self.depth.set_plain_value(val),
      5 => self.absorb.set_plain_value(val),
      6 => self.decay.set_plain_value(val),
      7 => self.tilt.set_plain_value(val),
      8 => self.shimmer.set_plain_value(val),
      9 => self.mix.set_plain_value(val),
      _ => (),
    }
  }
}
