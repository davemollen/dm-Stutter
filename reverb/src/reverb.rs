use super::{
  mix::Mix,
  one_pole_filter::OnePoleFilter,
  tap::{Tap, TapInitializer},
};

pub struct Reverb {
  taps: Vec<Tap>,
  smooth_size: OnePoleFilter,
}

impl Reverb {
  pub fn new(sample_rate: f64) -> Self {
    Self {
      taps: vec![
        Tap::new(TapInitializer {
          sample_rate,
          time: 0.625597,
          feedback_matrix: [1.0, 1.0, 1.0, 1.0],
          diffuser_times: [5.020833, 1.854167, 7.229167, 14.604167],
        }),
        Tap::new(TapInitializer {
          sample_rate,
          time: 0.719094,
          feedback_matrix: [1.0, -1.0, 1.0, -1.0],
          diffuser_times: [4.145833, 3.145833, 7.979167, 13.145833],
        }),
        Tap::new(TapInitializer {
          sample_rate,
          time: 0.842925,
          feedback_matrix: [1.0, 1.0, -1.0, -1.0],
          diffuser_times: [5.229167, 2.645833, 10.395833, 12.770833],
        }),
        Tap::new(TapInitializer {
          sample_rate,
          time: 1.0,
          feedback_matrix: [1.0, -1.0, -1.0, 1.0],
          diffuser_times: [4.395833, 3.770833, 5.854167, 14.020833],
        }),
      ],
      smooth_size: OnePoleFilter::new(sample_rate),
    }
  }

  fn read_from_delay_taps(&mut self, size: f32) -> Vec<f32> {
    self
      .taps
      .iter_mut()
      .map(|tap| -> f32 { tap.delay_line.read(tap.time * size, "linear") })
      .collect()
  }

  fn write_to_delay_taps(
    &mut self,
    input: f32,
    read_outputs: &Vec<f32>,
    diffuse: f32,
    absorb: f32,
    decay: f32,
  ) {
    let Reverb { taps, .. } = self;
    taps.iter_mut().enumerate().for_each(|(i, tap)| {
      let dry_signal = if i < 2 { input } else { 0. };
      let processed_outputs: f32 = tap.process_delay_tap(read_outputs, diffuse, absorb);
      tap
        .delay_line
        .write(dry_signal + processed_outputs * decay * 0.5)
    })
  }

  fn get_reverb_output(&mut self) -> (f32, f32) {
    (
      self.taps[0].delay_line.read(0., "step"),
      self.taps[1].delay_line.read(0., "step"),
    )
  }

  pub fn run(
    &mut self,
    input: f32,
    size: f32,
    diffuse: f32,
    absorb: f32,
    decay: f32,
    mix: f32,
  ) -> (f32, f32) {
    let size = self.smooth_size.run(size, 3., "hertz");
    let read_outputs = self.read_from_delay_taps(size);
    self.write_to_delay_taps(input, &read_outputs, diffuse * 0.8, absorb, decay);
    let reverb = self.get_reverb_output();
    Mix::run(input, reverb, mix)
  }
}
