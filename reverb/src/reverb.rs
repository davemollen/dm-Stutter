use crate::{
  delay_line::DelayLine, mix::Mix, one_pole_filter::OnePoleFilter, taps::Taps,
  tilt_filter::TiltFilter,
};

pub struct Reverb {
  predelay_tap: DelayLine,
  taps: Taps,
  tilt_filter: TiltFilter,
  smooth_predelay: OnePoleFilter,
  smooth_size: OnePoleFilter,
  smooth_depth: OnePoleFilter,
  smooth_absorb: OnePoleFilter,
  smooth_tilt: OnePoleFilter,
}

impl Reverb {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      predelay_tap: DelayLine::new((sample_rate * 0.5) as usize, sample_rate),
      taps: Taps::new(sample_rate),
      tilt_filter: TiltFilter::new(sample_rate),
      smooth_predelay: OnePoleFilter::new(sample_rate),
      smooth_size: OnePoleFilter::new(sample_rate),
      smooth_depth: OnePoleFilter::new(sample_rate),
      smooth_absorb: OnePoleFilter::new(sample_rate),
      smooth_tilt: OnePoleFilter::new(sample_rate),
    }
  }

  pub fn run(
    &mut self,
    input: (f32, f32),
    size: f32,
    speed: f32,
    depth: f32,
    predelay: f32,
    absorb: f32,
    decay: f32,
    tilt: f32,
    mix: f32,
  ) -> (f32, f32) {
    let predelay = self.smooth_predelay.run(predelay, 12., "hertz");
    let size = self.smooth_size.run(size, 12., "hertz");
    let depth = self.smooth_depth.run(depth.powf(4.) * 4., 12., "hertz");
    let absorb = self.smooth_absorb.run(absorb, 12., "hertz");
    let tilt = self.smooth_tilt.run(tilt, 12., "hertz");
    let decay = decay.powf(0.3333333);
    let diffuse = (absorb * 3.).min(1.) * 0.8;
    let absorb = ((absorb - 0.3333333).max(0.) * 1.5).powf(0.3333333);

    let predelay_output = self.predelay_tap.read(predelay, "linear");
    self.predelay_tap.write((input.0 + input.1) * 0.7071);
    let taps_output = self
      .taps
      .run(predelay_output, size, speed, depth, diffuse, absorb, decay);
    let reverb_output = self
      .tilt_filter
      .run(taps_output, 520., 6000., 3.981072, 15.848932, tilt);
    Mix::run(input, reverb_output, mix)
  }
}
