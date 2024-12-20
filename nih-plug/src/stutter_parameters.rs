use crate::editor;
use nih_plug::{
  formatters::{s2v_f32_percentage, v2s_f32_percentage},
  params::{EnumParam, IntParam},
  prelude::{BoolParam, Enum, FloatParam, FloatRange, IntRange, Params},
};
use nih_plug_vizia::ViziaState;
use std::sync::Arc;
mod custom_formatters;
use custom_formatters::{s2v_f32_tempo_factor, v2s_f32_digits, v2s_f32_tempo_factor};

#[derive(Enum, PartialEq)]
pub enum Mix {
  #[name = "Dry or wet"]
  DryOrWet,
  #[name = "Dry and wet"]
  DryAndWet,
  #[name = "Wet only"]
  WetOnly,
}

#[derive(Params)]
pub struct StutterParameters {
  #[persist = "editor-state"]
  pub editor_state: Arc<ViziaState>,

  #[id = "on"]
  pub on: BoolParam,

  #[id = "trigger"]
  pub trigger: BoolParam,

  #[id = "auto"]
  pub auto: BoolParam,

  #[id = "sync"]
  pub sync: BoolParam,

  #[id = "mix"]
  pub mix: EnumParam<Mix>,

  #[id = "pulse"]
  pub pulse: FloatParam,

  #[id = "tempo_factor"]
  pub tempo_factor: IntParam,

  #[id = "chance"]
  pub chance: FloatParam,

  #[id = "duration"]
  pub duration: FloatParam,

  #[id = "half_notes"]
  pub half_notes: FloatParam,

  #[id = "seven_sixteenth_notes"]
  pub seven_sixteenth_notes: FloatParam,

  #[id = "six_sixteenth_notes"]
  pub six_sixteenth_notes: FloatParam,

  #[id = "half_triplet_notes"]
  pub half_triplet_notes: FloatParam,

  #[id = "five_sixteenth_notes"]
  pub five_sixteenth_notes: FloatParam,

  #[id = "quarter_notes"]
  pub quarter_notes: FloatParam,

  #[id = "three_sixteenth_notes"]
  pub three_sixteenth_notes: FloatParam,

  #[id = "quarter_triplet_notes"]
  pub quarter_triplet_notes: FloatParam,

  #[id = "eighth_notes"]
  pub eighth_notes: FloatParam,

  #[id = "eighth_triplet_notes"]
  pub eighth_triplet_notes: FloatParam,

  #[id = "sixteenth_notes"]
  pub sixteenth_notes: FloatParam,

  #[id = "sixteenth_triplet_notes"]
  pub sixteenth_triplet_notes: FloatParam,

  #[id = "thirty_second_notes"]
  pub thirty_second_notes: FloatParam,

  #[id = "thirty_second_triplet_notes"]
  pub thirty_second_triplet_notes: FloatParam,

  #[id = "sixty_fourth_notes"]
  pub sixty_fourth_notes: FloatParam,
}

impl Default for StutterParameters {
  fn default() -> Self {
    Self {
      editor_state: editor::default_state(),

      on: BoolParam::new("On", false),

      trigger: BoolParam::new("Trigger", false),

      auto: BoolParam::new("Auto", true),

      sync: BoolParam::new("Sync", true),

      mix: EnumParam::new("Mix", Mix::DryOrWet),

      pulse: FloatParam::new(
        "Pulse",
        500.,
        FloatRange::Skewed {
          min: 10.,
          max: 3000.,
          factor: 0.333333,
        },
      )
      .with_unit(" ms")
      .with_value_to_string(v2s_f32_digits(2)),

      tempo_factor: IntParam::new("Pulse", 2, IntRange::Linear { min: 0, max: 4 })
        .with_unit(" x")
        .with_value_to_string(v2s_f32_tempo_factor())
        .with_string_to_value(s2v_f32_tempo_factor()),

      chance: FloatParam::new("Chance", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      duration: FloatParam::new("Duration", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      half_notes: FloatParam::new("1/2", 0., FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(0))
        .with_string_to_value(s2v_f32_percentage()),

      seven_sixteenth_notes: FloatParam::new("7/16", 0., FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(0))
        .with_string_to_value(s2v_f32_percentage()),

      six_sixteenth_notes: FloatParam::new("3/8", 0.25, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(0))
        .with_string_to_value(s2v_f32_percentage()),

      half_triplet_notes: FloatParam::new(
        "1/2 tripl.",
        0.,
        FloatRange::Linear { min: 0., max: 1. },
      )
      .with_unit(" %")
      .with_value_to_string(v2s_f32_percentage(0))
      .with_string_to_value(s2v_f32_percentage()),

      five_sixteenth_notes: FloatParam::new("5/16", 0., FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(0))
        .with_string_to_value(s2v_f32_percentage()),

      quarter_notes: FloatParam::new("1/4", 0.25, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(0))
        .with_string_to_value(s2v_f32_percentage()),

      three_sixteenth_notes: FloatParam::new("3/16", 0., FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(0))
        .with_string_to_value(s2v_f32_percentage()),

      quarter_triplet_notes: FloatParam::new(
        "1/4 tripl.",
        0.,
        FloatRange::Linear { min: 0., max: 1. },
      )
      .with_unit(" %")
      .with_value_to_string(v2s_f32_percentage(0))
      .with_string_to_value(s2v_f32_percentage()),

      eighth_notes: FloatParam::new("1/8", 0.25, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(0))
        .with_string_to_value(s2v_f32_percentage()),

      eighth_triplet_notes: FloatParam::new(
        "1/8 tripl.",
        0.,
        FloatRange::Linear { min: 0., max: 1. },
      )
      .with_unit(" %")
      .with_value_to_string(v2s_f32_percentage(0))
      .with_string_to_value(s2v_f32_percentage()),

      sixteenth_notes: FloatParam::new("1/16", 0., FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(0))
        .with_string_to_value(s2v_f32_percentage()),

      sixteenth_triplet_notes: FloatParam::new(
        "1/16 tripl.",
        0.,
        FloatRange::Linear { min: 0., max: 1. },
      )
      .with_unit(" %")
      .with_value_to_string(v2s_f32_percentage(0))
      .with_string_to_value(s2v_f32_percentage()),

      thirty_second_notes: FloatParam::new("1/32", 0.25, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(0))
        .with_string_to_value(s2v_f32_percentage()),

      thirty_second_triplet_notes: FloatParam::new(
        "1/32 tripl.",
        0.,
        FloatRange::Linear { min: 0., max: 1. },
      )
      .with_unit(" %")
      .with_value_to_string(v2s_f32_percentage(0))
      .with_string_to_value(s2v_f32_percentage()),

      sixty_fourth_notes: FloatParam::new("1/64", 0., FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(0))
        .with_string_to_value(s2v_f32_percentage()),
    }
  }
}
