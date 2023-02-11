use crate::{
  allpass_filter::AllpassFilter, clip::Clip, dc_block::DcBlock, delay_line::DelayLine, lfo::Lfo,
  one_pole_filter::OnePoleFilter, wave_table::WaveTable,
};

pub struct Tap {
  time_fraction: f32,
  delay_line: DelayLine,
  all_pass_filter: AllpassFilter,
  one_pole_filter: OnePoleFilter,
  diffuser_time: f32,
  lfo: Lfo,
  lfo_phase_offset: f32,
  wave_table: WaveTable,
  dc_block: DcBlock,
}

impl Tap {
  pub fn new(
    sample_rate: f32,
    time_fraction: f32,
    diffuser_time: f32,
    lfo_phase_offset: f32,
  ) -> Self {
    Self {
      time_fraction,
      // TODO: calculate max length
      delay_line: DelayLine::new((sample_rate * 1.5) as usize, sample_rate),
      all_pass_filter: AllpassFilter::new(sample_rate),
      diffuser_time,
      one_pole_filter: OnePoleFilter::new(sample_rate),
      lfo: Lfo::default(),
      lfo_phase_offset,
      wave_table: WaveTable::default(),
      dc_block: DcBlock::new(sample_rate),
    }
  }

  pub fn read(&mut self, size: f32, lfo_phase: f32, lfo_depth: f32) -> f32 {
    let lfo = self.lfo.run(lfo_phase, self.lfo_phase_offset) * lfo_depth;
    self
      .delay_line
      .read(self.time_fraction * size + lfo, "linear")
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

  pub fn apply_saturation(&mut self, input: f32, decay: f32) -> f32 {
    let output = if decay < 0.99 {
      input
    } else {
      let interp = ((decay - 0.99) * 100.).clip(0., 1.);
      let saturation_output = self.dc_block.run(
        self
          .wave_table
          .read_from_wavetable(input * 0.25 + 0.5, "TANH"),
      );
      input * (1. - interp) + saturation_output * interp
    };
    output * decay * 0.5
  }
}
