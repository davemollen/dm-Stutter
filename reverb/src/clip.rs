pub struct Clip;

impl Clip {
  pub fn run(input: f32, min: f32, max: f32) -> f32 {
    if input < min {
      min
    } else if input > max {
      max
    } else {
      input
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn clip() {
    assert_eq!(Clip::run(1.2, 0., 1.), 1.);
    assert_eq!(Clip::run(-0.2, 0., 1.), 0.)
  }
}
