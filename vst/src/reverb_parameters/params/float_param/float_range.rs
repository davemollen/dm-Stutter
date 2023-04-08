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

  pub fn unnormalize(&self, value: f32) -> f32 {
    match self {
      FloatRange::Linear { min, max } => (value * (max - min)) + min,
      FloatRange::Skewed { min, max, factor } => (value.powf(factor.recip()) * (max - min)) + min,
    }
  }
}
