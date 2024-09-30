use stutter::Stutter;

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn main() {
  let mut stutter = Stutter::new(44100.);

  loop {
    let input = (generate_signal(), generate_signal());
    stutter.process(input, true, true, false, 200., 0.5, 1.);
  }
}
