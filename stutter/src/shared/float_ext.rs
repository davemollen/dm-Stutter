pub trait FloatExt {
  fn mix(self, right: Self, factor: Self) -> Self;
  fn cubic_spline_curve(self) -> Self;
  fn mstosamps(self, sample_rate: Self) -> Self;
}

impl FloatExt for f32 {
  #[inline(always)]
  fn mix(self, right: f32, factor: f32) -> Self {
    self + (right - self) * factor
  }

  #[inline(always)]
  fn cubic_spline_curve(self) -> Self {
    self * self * (3.0 - 2.0 * self)
  }

  /// Convert milliseconds to samples based on the samplerate.
  #[inline(always)]
  fn mstosamps(self, sample_rate: Self) -> Self {
    self * 0.001 * sample_rate
  }
}

#[cfg(test)]
mod tests {
  use super::FloatExt;

  #[test]
  fn mix() {
    assert_eq!((1f32).mix(0., 0.), 1.);
    assert_eq!((1f32).mix(0., 0.5), 0.5);
    assert_eq!((1f32).mix(0., 1.), 0.);
  }
}
