use super::delay_line::DelayLine;

#[derive(Clone)]
pub struct AllpassFilter {
  delay_line: DelayLine,
}

impl AllpassFilter {
  pub fn new(sample_rate: f64) -> Self {
    Self {
      delay_line: DelayLine::new((sample_rate * 0.015) as usize, sample_rate),
    }
  }

  pub fn run(&mut self, input: f32, time: f32, gain: f32) -> f32 {
    let read_output = self.delay_line.read(time, "linear");
    let feedback = read_output * gain * 0.5;
    let allpass_input = input + feedback;
    let feedforward = allpass_input * gain * -0.5;
    let allpass_output = read_output + feedforward;
    self.delay_line.write(input + feedback);
    allpass_output
  }
}
