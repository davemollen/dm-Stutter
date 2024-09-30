use criterion::{criterion_group, criterion_main, Criterion};
use stutter::Stutter;

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn generate_signal_stream(length: usize) -> Vec<(f32, f32)> {
  (0..length).map(|_| (generate_signal(), generate_signal()).collect()
}

fn stutter_bench(c: &mut Criterion) {
  let mut stutter = Stutter::new(44100.);
  let signal_stream = generate_signal_stream(44100);

  c.bench_function("stutter", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        stutter.process(*signal, true, true, false, 200., 0.5, 1.);
      }
    })
  });
}

criterion_group!(benches, stutter_bench);
criterion_main!(benches);
