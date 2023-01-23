use crate::{lfo::{Lfo}, allpass_filter::AllpassFilter};
use super::{delay_line::DelayLine, one_pole_filter::OnePoleFilter};

pub struct TapInitializer {
  pub sample_rate: f32,
  pub time: f32,
  pub feedback_matrix: [f32; 4],
  pub diffuser_time: f32,
  pub lfo_phase_offset: f32,
}

pub struct Tap {
  pub time: f32,
  pub delay_line: DelayLine,
  all_pass_filter: AllpassFilter,
  one_pole_filter: OnePoleFilter,
  feedback_matrix: [f32; 4],
  diffuser_time: f32,
  pub lfo: Lfo,
  pub lfo_phase_offset: f32,
}

impl Tap {
  pub fn new(tap_initializer: TapInitializer) -> Self {
    let TapInitializer {
      sample_rate,
      time,
      feedback_matrix,
      diffuser_time,
      lfo_phase_offset
    } = tap_initializer;
    Self {
      time,
      delay_line: DelayLine::new((sample_rate * 1.5) as usize, sample_rate),
      all_pass_filter: AllpassFilter::new(sample_rate),
      feedback_matrix,
      diffuser_time,
      one_pole_filter: OnePoleFilter::new(sample_rate),
      lfo: Lfo::default(),
      lfo_phase_offset
    }
  }

  pub fn process_delay_tap(&mut self, read_outputs: &Vec<f32>, diffuse: f32, absorb: f32) -> f32 {
    let Tap {
      one_pole_filter,
      all_pass_filter,
      diffuser_time,
      ..
    } = self;

    read_outputs
      .iter()
      .zip(self.feedback_matrix.iter())
      .map(|(read_output, feedback_matrix_value)| {
        let one_filter_output =
          one_pole_filter.run(read_output * feedback_matrix_value, absorb, "linear");
        all_pass_filter.run(one_filter_output, *diffuser_time, diffuse)
      })
      .sum()
  }
}
