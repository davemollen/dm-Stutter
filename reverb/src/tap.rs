use super::{delay_line::DelayLine, diffuser::Diffuser, one_pole_filter::OnePoleFilter};

pub struct TapInitializer {
  pub sample_rate: f32,
  pub time: f32,
  pub feedback_matrix: [f32; 4],
  pub diffuser_time: f32,
}

pub struct Tap {
  pub time: f32,
  pub delay_line: DelayLine,
  diffuser: Diffuser,
  one_pole_filter: OnePoleFilter,
  feedback_matrix: [f32; 4],
}

impl Tap {
  pub fn new(tap_initializer: TapInitializer) -> Self {
    let TapInitializer {
      sample_rate,
      time,
      feedback_matrix,
      diffuser_time,
    } = tap_initializer;
    Self {
      time,
      delay_line: DelayLine::new((sample_rate * 1.5) as usize, sample_rate),
      diffuser: Diffuser::new(sample_rate, diffuser_time),
      feedback_matrix,
      one_pole_filter: OnePoleFilter::new(sample_rate),
    }
  }

  pub fn process_delay_tap(&mut self, read_outputs: &Vec<f32>, diffuse: f32, absorb: f32) -> f32 {
    let Tap {
      one_pole_filter,
      diffuser,
      ..
    } = self;

    read_outputs
      .iter()
      .zip(self.feedback_matrix.iter())
      .map(|(read_output, feedback_matrix_value)| {
        let one_filter_output =
          one_pole_filter.run(read_output * feedback_matrix_value, absorb, "linear");
        diffuser.run(one_filter_output, diffuse)
      })
      .sum()
  }
}
