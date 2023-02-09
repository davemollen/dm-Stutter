use nih_plug::{
  formatters,
  prelude::{FloatParam, FloatRange, Params},
};

#[derive(Params)]
pub struct ReverbParameters {
  #[id = "predelay"]
  pub predelay: FloatParam,

  #[id = "size"]
  pub size: FloatParam,

  #[id = "speed"]
  pub speed: FloatParam,

  #[id = "depth"]
  pub depth: FloatParam,

  #[id = "absorb"]
  pub absorb: FloatParam,

  #[id = "decay"]
  pub decay: FloatParam,

  #[id = "mix"]
  pub mix: FloatParam,
}

impl Default for ReverbParameters {
  fn default() -> Self {
    Self {
      predelay: FloatParam::new(
        "Predelay",
        7.,
        FloatRange::Skewed {
          min: 7.,
          max: 500.,
          factor: 0.5,
        },
      )
      .with_unit("ms"),
      size: FloatParam::new(
        "Size",
        40.,
        FloatRange::Skewed {
          min: 1.,
          max: 500.,
          factor: 0.333333,
        },
      )
      .with_unit("m2"),
      speed: FloatParam::new(
        "Speed",
        2.,
        FloatRange::Skewed {
          min: 0.01,
          max: 50.,
          factor: 0.333333,
        },
      )
      .with_unit("Hz"),
      depth: FloatParam::new("Depth", 0.25, FloatRange::Linear { min: 0., max: 1. })
        .with_unit("%")
        .with_value_to_string(formatters::v2s_f32_percentage(2))
        .with_string_to_value(formatters::s2v_f32_percentage()),
      absorb: FloatParam::new("Absorb", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit("%")
        .with_value_to_string(formatters::v2s_f32_percentage(2))
        .with_string_to_value(formatters::s2v_f32_percentage()),
      decay: FloatParam::new("Decay", 0.9, FloatRange::Linear { min: 0., max: 1.2 })
        .with_unit("%")
        .with_value_to_string(formatters::v2s_f32_percentage(2))
        .with_string_to_value(formatters::s2v_f32_percentage()),
      mix: FloatParam::new("Mix", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit("%")
        .with_value_to_string(formatters::v2s_f32_percentage(2))
        .with_string_to_value(formatters::s2v_f32_percentage()),
    }
  }
}
