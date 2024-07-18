pub trait TupleExt {
  fn add(self, other: Self) -> Self;
  fn multiply_with(self, factor: f32) -> Self;
}

impl TupleExt for (f32, f32) {
  fn add(self, other: Self) -> Self {
    (self.0 + other.0, self.1 + other.1)
  }

  fn multiply_with(self, factor: f32) -> Self {
    (self.0 * factor, self.1 * factor)
  }
}
