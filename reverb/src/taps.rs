use crate::{phasor::Phasor, tap::Tap};

pub struct Taps {
  taps: Vec<Tap>,
  lfo_phasor: Phasor,
}

impl Taps {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      taps: vec![
        Tap::new(sample_rate, 0.68, 5., 0.),
        Tap::new(sample_rate, 0.77, 7., 0.25),
        Tap::new(sample_rate, 0.90, 11., 0.5),
        Tap::new(sample_rate, 0.99, 13., 0.75),
      ],
      lfo_phasor: Phasor::new(sample_rate),
    }
  }

  fn read_from_delay_taps(&mut self, size: f32, speed: f32, depth: f32) -> Vec<f32> {
    let phase = self.lfo_phasor.run(speed);
    self
      .taps
      .iter_mut()
      .map(move |tap| -> f32 { tap.read(size, phase, depth) })
      .collect()
  }

  fn apply_matrix<'a>(&self, inputs: &'a Vec<f32>, decay: f32) -> impl Iterator<Item = f32> + 'a {
    let decay = decay * 0.5;
    [
      [1.0, 1.0, 1.0, 1.0],
      [1.0, -1.0, 1.0, -1.0],
      [1.0, 1.0, -1.0, -1.0],
      [1.0, -1.0, -1.0, 1.0],
    ]
    .iter()
    .map(move |feedback_values| -> f32 {
      feedback_values
        .iter()
        .zip(inputs.iter())
        .map(|(feedback, input)| input * feedback * decay)
        .sum()
    })
  }

  fn process_and_write_taps<'a>(
    &'a mut self,
    input: f32,
    matrix_outputs: impl Iterator<Item = f32> + 'a,
    diffuse: f32,
    absorb: f32,
  ) {
    self
      .taps
      .iter_mut()
      .zip(matrix_outputs)
      .enumerate()
      .for_each(move |(i, (tap, matrix_output))| {
        let dry_signal = if i < 2 { input } else { 0. };
        let absorb_output = tap.apply_absorb(matrix_output + dry_signal, absorb);
        let diffuse_output = tap.apply_diffuse(absorb_output, diffuse);
        tap.write(diffuse_output);
      })
  }

  fn get_taps_output(&self, inputs: Vec<f32>) -> (f32, f32) {
    (inputs[0] + inputs[2], inputs[1] + inputs[3])
  }

  pub fn run(
    &mut self,
    input: f32,
    size: f32,
    speed: f32,
    depth: f32,
    diffuse: f32,
    absorb: f32,
    decay: f32,
  ) -> (f32, f32) {
    let read_outputs = self.read_from_delay_taps(size, speed, depth);
    let matrix_outputs = self.apply_matrix(&read_outputs, decay);
    self.process_and_write_taps(input, matrix_outputs, diffuse, absorb);
    self.get_taps_output(read_outputs)
  }
}
