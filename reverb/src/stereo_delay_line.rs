use std::f32::consts::PI;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Interpolation {
  Step,
  Linear,
  Cosine,
  Cubic,
  Spline,
}

#[derive(Clone)]
pub struct StereoDelayLine {
  buffer: Vec<(f32, f32)>,
  write_pointer: usize,
  sample_rate: f32,
}

impl StereoDelayLine {
  pub fn new(length: usize, sample_rate: f32) -> Self {
    Self {
      buffer: vec![(0.0, 0.0); length + 1],
      write_pointer: 0,
      sample_rate,
    }
  }

  fn mstosamps(&self, time: f32) -> f32 {
    time * 0.001 * self.sample_rate
  }

  fn wrap(&self, index: usize) -> usize {
    let buffer_len = self.buffer.len();
    if index >= buffer_len {
      index - buffer_len
    } else {
      index
    }
  }

  fn step_interp(&self, index: usize) -> (f32, f32) {
    self.buffer[self.wrap(index)]
  }

  fn mix(&self, x: f32, y: f32, mix: f32) -> f32 {
    x * (1. - mix) + y * mix
  }

  fn linear_interp(&self, index: usize, mix: f32) -> (f32, f32) {
    let x = self.buffer[self.wrap(index)];
    let y = self.buffer[self.wrap(index + 1)];
    (self.mix(x.0, y.0, mix), self.mix(x.1, y.1, mix))
  }

  fn cosine_interp(&self, index: usize, mix: f32) -> (f32, f32) {
    let cosine_mix = (1. - (mix * PI).cos()) / 2.;
    let x = self.buffer[self.wrap(index)];
    let y = self.buffer[self.wrap(index + 1)];
    (self.mix(x.0, y.0, mix), self.mix(x.1, y.1, mix))
  }

  fn cubix_mix(&self, w: f32, x: f32, y: f32, z: f32, mix: f32) -> f32 {
    let a1 = 1. + mix;
    let aa = mix * a1;
    let b = 1. - mix;
    let b1 = 2. - mix;
    let bb = b * b1;
    let fw = -0.1666667 * bb * mix;
    let fx = 0.5 * bb * a1;
    let fy = 0.5 * aa * b1;
    let fz = -0.1666667 * aa * b;
    w * fw + x * fx + y * fy + z * fz
  }

  fn cubic_interp(&self, index: usize, mix: f32) -> (f32, f32) {
    let w = self.buffer[self.wrap(index)];
    let x = self.buffer[self.wrap(index + 1)];
    let y = self.buffer[self.wrap(index + 2)];
    let z = self.buffer[self.wrap(index + 3)];

    (
      self.cubix_mix(w.0, x.0, y.0, z.0, mix),
      self.cubix_mix(w.1, x.1, y.1, z.1, mix),
    )
  }

  fn spline_mix(&self, w: f32, x: f32, y: f32, z: f32, mix: f32) -> f32 {
    let c0 = x;
    let c1 = (0.5) * (y - w);
    let c2 = w - (2.5) * x + y + y - (0.5) * z;
    let c3 = (0.5) * (z - w) + (1.5) * (x - y);
    ((c3 * mix + c2) * mix + c1) * mix + c0
  }

  fn spline_interp(&self, index: usize, mix: f32) -> (f32, f32) {
    let w = self.buffer[self.wrap(index)];
    let x = self.buffer[self.wrap(index + 1)];
    let y = self.buffer[self.wrap(index + 2)];
    let z = self.buffer[self.wrap(index + 3)];

    (
      self.spline_mix(w.0, x.0, y.0, z.0, mix),
      self.spline_mix(w.1, x.1, y.1, z.1, mix),
    )
  }

  pub fn read(&mut self, time: f32, interp: Interpolation) -> (f32, f32) {
    let read_pointer = (self.write_pointer - 1 + self.buffer.len()) as f32 - self.mstosamps(time);
    let rounded_read_pointer = read_pointer.trunc();
    let mix = read_pointer - rounded_read_pointer;
    let index = rounded_read_pointer as usize;

    match interp {
      Interpolation::Step => self.step_interp(index),
      Interpolation::Linear => self.linear_interp(index, mix),
      Interpolation::Cosine => self.cosine_interp(index, mix),
      Interpolation::Cubic => self.cubic_interp(index - 1, mix),
      Interpolation::Spline => self.spline_interp(index - 1, mix),
    }
  }

  pub fn write(&mut self, input: (f32, f32)) {
    self.buffer[self.write_pointer].0 = input.0;
    self.buffer[self.write_pointer].1 = input.1;
    self.write_pointer = self.wrap(self.write_pointer + 1);
  }
}
