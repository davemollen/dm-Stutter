pub trait Clip {
  fn clip(self, min: Self, max: Self) -> Self;
}

macro_rules! impl_clip_for_int {
  ($($t:ty),*) => {
    $(
      impl Clip for $t {
        fn clip(self, min: Self, max: Self) -> Self {
          self.max(min).min(max)
        }
      }
    )*
  };
}
impl_clip_for_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

macro_rules! impl_clip_for_float {
  ($($t:ty),*) => {
    $(
      impl Clip for $t {
        fn clip(self, min: Self, max: Self) -> Self {
          self.max(min).min(max)
        }
      }
    )*
  };
}
impl_clip_for_float!(f32, f64);

#[cfg(test)]
mod tests {
  use crate::clip::Clip;

  #[test]
  fn clip() {
    assert_eq!((-1).clip(0, 10), 0);
    assert_eq!((11).clip(0, 10), 10);
    assert_eq!((1.2).clip(0., 1.), 1.);
    assert_eq!((-0.2).clip(0., 1.), 0.);
    assert_eq!((-2.2).clip(-1., 1.), -1.);
  }
}
