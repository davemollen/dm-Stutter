use crate::editor;
use nih_plug::{
  formatters::{s2v_f32_percentage, v2s_f32_percentage},
  prelude::{BoolParam, FloatParam, FloatRange, Params},
};
use nih_plug_vizia::ViziaState;
use std::sync::Arc;
mod custom_formatters;
use custom_formatters::v2s_f32_digits;

#[derive(Params)]
pub struct StutterParameters {
  #[persist = "editor-state"]
  pub editor_state: Arc<ViziaState>,

  #[id = "on"]
  pub on: BoolParam,

  #[id = "auto"]
  pub auto: BoolParam,

  #[id = "trigger"]
  pub trigger: BoolParam,

  #[id = "pulse"]
  pub pulse: FloatParam,

  #[id = "chance"]
  pub chance: FloatParam,

  #[id = "duration"]
  pub duration: FloatParam,
}

impl Default for StutterParameters {
  fn default() -> Self {
    Self {
      editor_state: editor::default_state(),

      on: BoolParam::new("On", false),

      auto: BoolParam::new("Auto", true),

      trigger: BoolParam::new("Trigger", false),

      pulse: FloatParam::new(
        "Pulse",
        500.,
        FloatRange::Skewed {
          min: 10.,
          max: 2000.,
          factor: 0.333333,
        },
      )
      .with_unit(" ms")
      .with_value_to_string(v2s_f32_digits(2)),

      chance: FloatParam::new("Chance", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      duration: FloatParam::new("Duration", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),
    }
  }
}
