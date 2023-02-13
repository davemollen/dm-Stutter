/*
  This is an atan approximation but instead of multiple harmonics it amplifies the first harmonic mostly.
  https://www.dsprelated.com/showarticle/1052.php
*/
pub trait AtanApproximation {
  fn atan_approximation(&self) -> f32;
}

impl AtanApproximation for f32 {
  fn atan_approximation(&self) -> f32 {
    let n1 = 0.97239411;
    let n2 = -0.19194795;
    (n1 + n2 * self * self) * self
  }
}
