use std::f32::consts::FRAC_PI_2;

pub trait Pan {
  fn pan(self, pan: f32) -> (f32, f32);
}

impl Pan for f32 {
  fn pan(self, pan: f32) -> (f32, f32) {
    let radians = pan * 0.0025 + FRAC_PI_2;
    (self * radians.sin(), self * radians.cos())
  }
}
