use crate::{
  allpass_filter::AllpassFilter, delay_line::DelayLine, lfo::Lfo, one_pole_filter::OnePoleFilter,
};

pub struct Tap {
  time: f32,
  delay_line: DelayLine,
  all_pass_filter: AllpassFilter,
  one_pole_filter: OnePoleFilter,
  diffuser_time: f32,
  lfo: Lfo,
  lfo_phase_offset: f32,
}

impl Tap {
  pub fn new(sample_rate: f32, time: f32, diffuser_time: f32, lfo_phase_offset: f32) -> Self {
    Self {
      time,
      delay_line: DelayLine::new((sample_rate * 1.5) as usize, sample_rate),
      all_pass_filter: AllpassFilter::new(sample_rate),
      diffuser_time,
      one_pole_filter: OnePoleFilter::new(sample_rate),
      lfo: Lfo::default(),
      lfo_phase_offset,
    }
  }

  pub fn read(&mut self, size: f32, lfo_phase: f32, lfo_depth: f32) -> f32 {
    let lfo = self.lfo.run(lfo_phase, self.lfo_phase_offset) * lfo_depth;
    self.delay_line.read(self.time * size + lfo, "linear")
  }

  pub fn write(&mut self, input: f32) {
    self.delay_line.write(input);
  }

  pub fn apply_absorb(&mut self, input: f32, absorb: f32) -> f32 {
    self.one_pole_filter.run(input, absorb, "linear")
  }

  pub fn apply_diffuse(&mut self, input: f32, diffuse: f32) -> f32 {
    self.all_pass_filter.run(input, self.diffuser_time, diffuse)
  }
}
