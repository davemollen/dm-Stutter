use crate::{
  delay_line::{DelayLine, Interpolation},
  float_ext::FloatExt,
  mix::Mix,
  one_pole_filter::{Mode, OnePoleFilter},
  reverse::Reverse,
  shimmer::Shimmer,
  taps::Taps,
  tilt_filter::TiltFilter,
  MAX_DEPTH, MAX_PREDELAY, MIN_PREDELAY,
};

const TWELVE_DB: f32 = 3.981072;
const TWENTY_FOUR_DB: f32 = 15.848932;

pub struct Reverb {
  predelay_tap: DelayLine,
  reverse: Reverse,
  shimmer: Shimmer,
  taps: Taps,
  tilt_filter: TiltFilter,
  smooth_reverse: OnePoleFilter,
  smooth_predelay: OnePoleFilter,
  smooth_size: OnePoleFilter,
  smooth_depth: OnePoleFilter,
  smooth_absorb: OnePoleFilter,
  smooth_tilt: OnePoleFilter,
  smooth_shimmer: OnePoleFilter,
  smooth_mix: OnePoleFilter,
}

impl Reverb {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      predelay_tap: DelayLine::new(
        (sample_rate * (MIN_PREDELAY + MAX_PREDELAY) / 1000.) as usize,
        sample_rate,
      ),
      reverse: Reverse::new(sample_rate),
      shimmer: Shimmer::new(sample_rate),
      taps: Taps::new(sample_rate),
      tilt_filter: TiltFilter::new(sample_rate),
      smooth_reverse: OnePoleFilter::new(sample_rate),
      smooth_predelay: OnePoleFilter::new(sample_rate),
      smooth_size: OnePoleFilter::new(sample_rate),
      smooth_depth: OnePoleFilter::new(sample_rate),
      smooth_absorb: OnePoleFilter::new(sample_rate),
      smooth_tilt: OnePoleFilter::new(sample_rate),
      smooth_shimmer: OnePoleFilter::new(sample_rate),
      smooth_mix: OnePoleFilter::new(sample_rate),
    }
  }

  fn map_reverb_parameters(
    &mut self,
    reverse: bool,
    predelay: f32,
    size: f32,
    speed: f32,
    depth: f32,
    absorb: f32,
    decay: f32,
    tilt: f32,
    shimmer: f32,
    mix: f32,
  ) -> (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32) {
    let reverse = self
      .smooth_reverse
      .run(if reverse { 1. } else { 0. }, 12., Mode::Hertz);
    let predelay = self.smooth_predelay.run(predelay, 12., Mode::Hertz);
    let size = self.smooth_size.run(size, 12., Mode::Hertz);
    let depth = self.smooth_depth.run(
      depth.powf(2.) * depth.signum() * MAX_DEPTH,
      12.,
      Mode::Hertz,
    );
    let absorb = self.smooth_absorb.run(absorb, 12., Mode::Hertz);
    let tilt = self.smooth_tilt.run(tilt, 12., Mode::Hertz);
    let shimmer = self.smooth_shimmer.run(shimmer, 12., Mode::Hertz);
    let mix = self.smooth_mix.run(mix, 12., Mode::Hertz);
    let diffuse = (absorb * 3.).min(1.) * 0.8;
    let absorb = (absorb - 0.3333333).max(0.) * 1.5;

    (
      reverse, predelay, size, speed, depth, absorb, diffuse, decay, tilt, shimmer, mix,
    )
  }

  fn get_predelay_output(&mut self, input: (f32, f32), time: f32, reverse: f32) -> f32 {
    let predelay_output = if reverse == 0. {
      self.predelay_tap.read(time, Interpolation::Linear)
    } else if reverse == 1. {
      self.reverse.run(&mut self.predelay_tap, time)
    } else {
      self
        .predelay_tap
        .read(time, Interpolation::Linear)
        .mix(self.reverse.run(&mut self.predelay_tap, time), reverse)
    };
    self.predelay_tap.write((input.0 + input.1) * 0.5);
    predelay_output
  }

  fn get_shimmer_output(&mut self, input: f32, shimmer: f32) -> f32 {
    self.shimmer.run(input, shimmer)
  }

  fn get_delay_taps_output(
    &mut self,
    input: f32,
    size: f32,
    speed: f32,
    depth: f32,
    diffuse: f32,
    absorb: f32,
    decay: f32,
  ) -> (f32, f32) {
    self
      .taps
      .run(input, size, speed, depth, diffuse, absorb, decay)
  }

  fn apply_tilt_filter(&mut self, input: (f32, f32), tilt: f32) -> (f32, f32) {
    self
      .tilt_filter
      .run(input, 520., 4000., TWELVE_DB, TWENTY_FOUR_DB, tilt)
  }

  pub fn run(
    &mut self,
    input: (f32, f32),
    reverse: bool,
    predelay: f32,
    size: f32,
    speed: f32,
    depth: f32,
    absorb: f32,
    decay: f32,
    tilt: f32,
    shimmer: f32,
    mix: f32,
  ) -> (f32, f32) {
    let (reverse, predelay, size, speed, depth, absorb, diffuse, decay, tilt, shimmer, mix) = self
      .map_reverb_parameters(
        reverse, predelay, size, speed, depth, absorb, decay, tilt, shimmer, mix,
      );

    let predelay_output = self.get_predelay_output(input, predelay, reverse);
    let shimmer_output = self.get_shimmer_output(predelay_output, shimmer);
    let taps_output =
      self.get_delay_taps_output(shimmer_output, size, speed, depth, diffuse, absorb, decay);
    let tilt_filter_output = self.apply_tilt_filter(taps_output, tilt);
    Mix::run(input, tilt_filter_output, mix)
  }
}
