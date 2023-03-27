use crate::{
  allpass_filter::AllpassFilter,
  dc_block::DcBlock,
  delay_line::{DelayLine, Interpolation},
  float_ext::FloatExt,
  grains::Grains,
  lfo::Lfo,
  one_pole_filter::{Mode, OnePoleFilter},
  pan::Pan,
  MAX_DEPTH, MAX_SIZE,
};

struct EarlyReflection {
  time_fraction: f32,
  gain: f32,
  pan: f32,
}

pub struct Tap {
  time_fraction: f32,
  delay_line: DelayLine,
  early_reflections: Vec<EarlyReflection>,
  all_pass_filter: AllpassFilter,
  one_pole_filter: OnePoleFilter,
  diffuser_time: f32,
  lfo: Lfo,
  lfo_phase_offset: f32,
  dc_block: DcBlock,
  grains: Grains,
}

impl Tap {
  /// early_reflections expects a vector of (time_fraction, gain, pan)
  pub fn new(
    sample_rate: f32,
    time_fraction: f32,
    early_reflections: Vec<(f32, f32, f32)>,
    diffuser_time: f32,
    lfo_phase_offset: f32,
  ) -> Self {
    Self {
      time_fraction,
      delay_line: DelayLine::new(
        (sample_rate * (MAX_SIZE * 0.001 * time_fraction + MAX_DEPTH)) as usize,
        sample_rate,
      ),
      early_reflections: early_reflections
        .iter()
        .map(|x| EarlyReflection {
          time_fraction: x.0,
          gain: x.1,
          pan: x.2,
        })
        .collect(),
      all_pass_filter: AllpassFilter::new(sample_rate),
      diffuser_time,
      one_pole_filter: OnePoleFilter::new(sample_rate),
      lfo: Lfo::default(),
      lfo_phase_offset,
      dc_block: DcBlock::new(sample_rate),
      grains: Grains::new(sample_rate),
    }
  }

  pub fn read(&mut self, size: f32, lfo_phase: f32, lfo_depth: f32) -> f32 {
    if lfo_depth == 0. {
      self
        .delay_line
        .read(self.time_fraction * size, Interpolation::Linear)
    } else if lfo_depth < 0. {
      self.vibrato_read(size, lfo_phase, lfo_depth)
    } else {
      self.grain_read(size, lfo_phase, lfo_depth)
    }
  }

  pub fn read_early_reflections(&mut self, size: f32) -> (f32, f32) {
    let Tap {
      early_reflections,
      delay_line,
      ..
    } = self;

    early_reflections
      .iter()
      .fold((0., 0.), |sum, early_reflection| {
        let interp = if early_reflection.time_fraction == 0. {
          Interpolation::Step
        } else {
          Interpolation::Linear
        };
        let early_reflection_out =
          delay_line.read(early_reflection.time_fraction * size, interp) * early_reflection.gain;
        let (left_out, right_out) = early_reflection_out.pan(early_reflection.pan);
        (sum.0 + left_out, sum.1 + right_out)
      })
  }

  pub fn write(&mut self, input: f32) {
    self.delay_line.write(input);
  }

  pub fn apply_absorb(&mut self, input: f32, absorb: f32) -> f32 {
    self.one_pole_filter.run(input, absorb, Mode::Linear)
  }

  pub fn apply_diffuse(&mut self, input: f32, diffuse: f32) -> f32 {
    self.all_pass_filter.run(input, self.diffuser_time, diffuse)
  }

  pub fn apply_saturation(&mut self, input: f32, decay: f32) -> f32 {
    let output = if decay < 1. {
      input
    } else {
      // TODO: check if atan doesn't explode
      let saturation_output = input.fast_atan1();
      let mix_factor = ((decay - 1.) * 100.).clamp(0., 1.);
      input * (1. - mix_factor) + saturation_output * mix_factor
    };
    self.dc_block.run(output * decay * 0.5)
  }

  fn vibrato_read(&mut self, size: f32, lfo_phase: f32, lfo_depth: f32) -> f32 {
    let lfo = self.lfo.run(lfo_phase, self.lfo_phase_offset) * lfo_depth.abs();
    self
      .delay_line
      .read(self.time_fraction * size + lfo, Interpolation::Linear)
  }

  fn grain_read(&mut self, size: f32, lfo_phase: f32, lfo_depth: f32) -> f32 {
    self.grains.run(
      &mut self.delay_line,
      lfo_phase,
      lfo_depth,
      self.time_fraction,
      size,
    )
  }
}
