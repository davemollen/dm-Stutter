use std::f32::consts::{FRAC_2_PI, FRAC_PI_2, PI};

pub trait FloatExtensions {
  fn clip(self, min: Self, max: Self) -> Self;
  fn fast_atan1(&self) -> Self;
  fn fast_atan2(&self) -> Self;
  fn fast_tanh1(&self) -> Self;
  fn fast_tanh2(&self) -> Self;
  fn fast_tanh3(&self) -> Self;
  fn fast_sin(&self) -> Self;
  fn fast_cos(&self) -> Self;
  fn sin_bhaskara(&self) -> Self;
  fn cos_bhaskara(&self) -> Self;
}

impl FloatExtensions for f32 {
  /// Clips the input value between specified min and max.  
  fn clip(self, min: Self, max: Self) -> Self {
    self.max(min).min(max)
  }

  /// This is an atan approximation
  fn fast_atan1(&self) -> Self {
    let a1 = 0.99997726;
    let a3 = -0.33262347;
    let a5 = 0.19354346;
    let a7 = -0.11643287;
    let a9 = 0.05265332;
    let a11 = -0.01172120;
    let squared_self = self * self;
    self
      * (a1
        + squared_self
          * (a3
            + squared_self * (a5 + squared_self * (a7 + squared_self * (a9 + squared_self * a11)))))
  }

  /// This is an atan approximation, not atan2. This variant only amplifies the first harmonic instead of multiple.
  /// https://www.dsprelated.com/showarticle/1052.php
  fn fast_atan2(&self) -> Self {
    let n1 = 0.97239411;
    let n2 = -0.19194795;
    (n1 + n2 * self * self) * self
  }

  /// This is a tanh approximation.
  fn fast_tanh1(&self) -> Self {
    let squared_self = self * self;
    let a = self * (135135. + squared_self * (17325. + squared_self * (378. + squared_self)));
    let b = 135135. + squared_self * (62370. + squared_self * (3150. + squared_self * 28.));
    a / b
  }

  /// This is a tanh approximation. It's cheaper than fast_tanh1, but looses accuracy for higher input values (< -1 and > 1).
  fn fast_tanh2(&self) -> Self {
    let x2 = self * self;
    let x3 = x2 * self;
    let x4 = x3 * self;
    (105. * self + 10. * x3) / (105. + 45. * x2 + x4)
  }

  /// This is a tanh approximation. For more accuracy (less aliasing) choose fast_tanh1 or fast_tanh2.
  fn fast_tanh3(&self) -> Self {
    let a = self.abs();
    let b = 1.26175667589988239 + a * (-0.54699348440059470 + a * (2.66559097474027817));
    (b * self) / (b * a + 1.)
  }

  /// This is a sine approximation. Use this to safe processing power.
  fn fast_sin(&self) -> Self {
    const FOUROVERPI: f32 = 1.2732395447351627;
    const FOUROVERPISQ: f32 = 0.40528473456935109;
    const Q: f32 = 0.77633023248007499;

    let mut p = 0.22308510060189463_f32.to_bits();
    let mut v = self.to_bits();

    let sign: u32 = v & 0x80000000;
    v &= 0x7FFFFFFF;

    let qpprox = FOUROVERPI * self - FOUROVERPISQ * self * Self::from_bits(v);

    p |= sign;

    qpprox * (Q + Self::from_bits(p) * qpprox)
  }

  /// This is a cosine approximation. Use this to safe processing power.
  fn fast_cos(&self) -> Self {
    const P: f32 = 0.54641335845679634;

    let v = self.to_bits() & 0x7FFFFFFF;

    let qpprox = 1.0_f32 - FRAC_2_PI * Self::from_bits(v);

    qpprox + P * qpprox * (1.0_f32 - qpprox * qpprox)
  }

  /// This is the Bhaskara sine approximation. It returns a sine from 0 to 180 degrees.
  /// This function expects values between 0. and 1.
  fn sin_bhaskara(&self) -> Self {
    let x = self * FRAC_PI_2;
    let pi_squared = 9.869604401089358;
    let a = x * (PI - x);
    (16. * a) / (5. * pi_squared - 4. * a)
  }

  /// This is the Bhaskara cosine approximation. It returns a sine from 0 to 180 degrees.
  /// This function expects values between 0. and 1.
  fn cos_bhaskara(&self) -> Self {
    let x = self * FRAC_PI_2;
    let x_squared = x * x;
    let pi_squared = 9.869604401089358;
    (pi_squared - 4. * x_squared) / (pi_squared + x_squared)
  }
}

#[cfg(test)]
mod tests {
  use crate::float_extensions::FloatExtensions;
  use std::f32::consts::{FRAC_1_SQRT_2, PI};

  fn assert_approximately_eq(left: f32, right: f32) {
    assert_eq!((left * 100.).floor() / 100., (right * 100.).floor() / 100.)
  }

  #[test]
  fn clip() {
    assert_eq!((1.2).clip(0., 1.), 1.);
    assert_eq!((-0.2).clip(0., 1.), 0.);
    assert_eq!((-2.2).clip(-1., 1.), -1.);
  }

  #[test]
  fn fast_atan1() {
    assert_approximately_eq((0.5).fast_atan1(), (0.5f32).atan());
    assert_approximately_eq((-0.5).fast_atan1(), (-0.5f32).atan());
    assert_approximately_eq((1.).fast_atan1(), (1f32).atan());
    assert_approximately_eq((-1.).fast_atan1(), (-1f32).atan());
  }

  #[test]
  fn fast_atan2() {
    assert_approximately_eq((0.5).fast_atan2(), (0.5f32).atan());
    assert_approximately_eq((-0.5).fast_atan2(), (-0.5f32).atan());
    assert_approximately_eq((1.).fast_atan2(), (1f32).atan());
    assert_approximately_eq((-1.).fast_atan2(), (-1f32).atan());
  }

  #[test]
  fn fast_tanh1() {
    assert_approximately_eq((0.5).fast_tanh1(), (0.5f32).tanh());
    assert_approximately_eq((-0.5).fast_tanh1(), (-0.5f32).tanh());
    assert_approximately_eq((1.).fast_tanh1(), (1f32).tanh());
    assert_approximately_eq((-1.).fast_tanh1(), (-1f32).tanh());
    assert_approximately_eq((1.5).fast_tanh1(), (1.5f32).tanh());
    assert_approximately_eq((-1.5).fast_tanh1(), (-1.5f32).tanh());
  }

  #[test]
  fn fast_tanh2() {
    assert_approximately_eq((0.5).fast_tanh2(), (0.5f32).tanh());
    assert_approximately_eq((-0.5).fast_tanh2(), (-0.5f32).tanh());
    assert_approximately_eq((1.).fast_tanh2(), (1f32).tanh());
    assert_approximately_eq((-1.).fast_tanh2(), (-1f32).tanh());
    assert_approximately_eq((1.5).fast_tanh2(), (1.5f32).tanh());
    assert_approximately_eq((-1.5).fast_tanh2(), (-1.5f32).tanh());
  }

  #[test]
  fn fast_tanh3() {
    assert_approximately_eq((0.5).fast_tanh2(), (0.5f32).tanh());
    assert_approximately_eq((-0.5).fast_tanh2(), (-0.5f32).tanh());
    assert_approximately_eq((1.).fast_tanh2(), (1f32).tanh());
    assert_approximately_eq((-1.).fast_tanh2(), (-1f32).tanh());
    assert_approximately_eq((1.5).fast_tanh2(), (1.5f32).tanh());
    assert_approximately_eq((-1.5).fast_tanh2(), (-1.5f32).tanh());
  }

  #[test]
  fn fast_sin() {
    assert_approximately_eq((0.).fast_sin(), (0f32).sin());
    assert_approximately_eq((PI).fast_sin(), (PI).sin());
    // assert_approximately_eq((PI * 2.).fast_sin(), (PI * 2.).sin());
  }

  #[test]
  fn fast_cos() {
    assert_approximately_eq((0.).fast_cos(), (0f32).cos());
    assert_approximately_eq((PI).fast_cos(), (PI).cos());
    // assert_approximately_eq((PI * 2.).fast_cos(), (PI * 2.).cos());
    assert_approximately_eq((PI * 0.25).fast_cos(), FRAC_1_SQRT_2);
  }

  #[test]
  fn bhaskara() {
    assert_approximately_eq((0.).sin_bhaskara(), 0.);
    assert_approximately_eq((0.5).sin_bhaskara(), FRAC_1_SQRT_2);
    assert_approximately_eq((1.).sin_bhaskara(), 1.);
    assert_approximately_eq((0.).cos_bhaskara(), 1.);
    assert_approximately_eq((0.5).cos_bhaskara(), FRAC_1_SQRT_2);
    assert_approximately_eq((0.5).cos_bhaskara(), (0.5).sin_bhaskara());
    assert_approximately_eq((0.).cos_bhaskara(), (1.).sin_bhaskara());
  }
}
