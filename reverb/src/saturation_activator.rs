use crate::{
  average::Average,
  one_pole_filter::{Mode, OnePoleFilter},
};

const SATURATION_THRESHOLD: f32 = 0.25;

pub struct SaturationActivator {
  average: Average,
  average_result: f32,
  smooth_saturation_gain: OnePoleFilter,
}

impl SaturationActivator {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      average: Average::new((sample_rate * 0.2) as usize),
      average_result: 0.,
      smooth_saturation_gain: OnePoleFilter::new(sample_rate),
    }
  }

  pub fn set_average(&mut self, left_input: f32, right_input: f32) {
    self.average_result = self.average.run((left_input + right_input) * 0.5);
  }

  pub fn get_saturation_gain(&mut self) -> f32 {
    let saturation_gain = if self.average_result > SATURATION_THRESHOLD {
      1.
    } else {
      0.
    };

    self
      .smooth_saturation_gain
      .run(saturation_gain, 1., Mode::Hertz)
  }
}
