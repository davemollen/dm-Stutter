use reverb::{MAX_SIZE, MIN_SIZE};
use std::sync::Arc;
use vst::plugin::PluginParameters;
mod formatters;
use formatters::v2s_f32_digits;
mod float_param;
pub use float_param::{FloatParam, FloatRange};
mod param_indexer;
pub use param_indexer::ParamIndexer;

pub struct ReverbParameters {
  pub predelay: FloatParam,
  pub size: FloatParam,
  pub speed: FloatParam,
  pub depth: FloatParam,
  pub shimmer: FloatParam,
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
          min: 0.02,
          max: 150.,
          factor: 0.333333,
        },
      )
      .with_unit("Hz")
      .with_value_to_string(v2s_f32_digits(2)),

      depth: FloatParam::new("Depth", 0.375, 3, FloatRange::Linear { min: 0., max: 1. })
        .with_unit("%")
        .with_value_to_string(Arc::new(move |value| format!("{:.2}", value * 200. - 100.))),

      shimmer: FloatParam::new("Shimmer", 0., 4, FloatRange::Linear { min: 0., max: 1. })
        .with_unit("%")
        .with_value_to_string(formatters::v2s_f32_percentage(2)),

      absorb: FloatParam::new("Absorb", 0.5, 5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit("%")
        .with_value_to_string(formatters::v2s_f32_percentage(2)),

      decay: FloatParam::new("Decay", 0.9, 6, FloatRange::Linear { min: 0., max: 1.2 })
        .with_unit("%")
        .with_value_to_string(formatters::v2s_f32_percentage(2)),

      tilt: FloatParam::new("Tilt", 0.5, 7, FloatRange::Linear { min: 0., max: 1. })
        .with_unit("%")
        .with_value_to_string(Arc::new(move |value| format!("{:.2}", value * 200. - 100.))),

      mix: FloatParam::new("Mix", 0.5, 8, FloatRange::Linear { min: 0., max: 1. })
        .with_unit("%")
        .with_value_to_string(formatters::v2s_f32_percentage(2)),
    }
  }
}

impl PluginParameters for ReverbParameters {
  fn get_parameter(&self, index: i32) -> f32 {
    let param = self.get_param_by_index(index);
    match param {
      Some(param) => param.get_normalized_value(),
      None => 0.,
    }
  }

  fn get_parameter_text(&self, index: i32) -> String {
    let param = self.get_param_by_index(index);
    match param {
      Some(param) => param.get_display_value(true),
      None => "".to_string(),
    }
  }

  fn get_parameter_name(&self, index: i32) -> String {
    let param = self.get_param_by_index(index);
    match param {
      Some(param) => param.name,
      None => "",
    }
    .to_string()
  }

  fn set_parameter(&self, index: i32, val: f32) {
    let param = self.get_param_by_index(index);
    match param {
      Some(param) => param.set_plain_value(val),
      None => (),
    }
  }
}
