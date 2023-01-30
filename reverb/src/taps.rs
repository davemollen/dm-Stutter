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
      .map(|tap| -> f32 { tap.read(size, phase, depth) })
      .collect()
  }

  fn apply_matrix(&self, inputs: &Vec<f32>, decay: f32) -> Vec<f32> {
    let decay = decay * 0.5;
    let feedback_matrix = [
      [1.0, 1.0, 1.0, 1.0],
      [1.0, -1.0, 1.0, -1.0],
      [1.0, 1.0, -1.0, -1.0],
      [1.0, -1.0, -1.0, 1.0],
    ];
    feedback_matrix
      .iter()
      .map(|feedback_values| {
        inputs
          .iter()
          .zip(feedback_values.iter())
          .map(|(input, feedback)| input * feedback * decay)
          .sum()
      })
      .collect()

    // let a = inputs[0] - inputs[1];
    // let b = inputs[0] + inputs[1];
    // let c = inputs[2] - inputs[3];
    // let d = inputs[2] + inputs[3];
    // vec![(a-c) * decay, (a+c) * decay, (b-d) * decay, (b+d) * decay]
  }

  fn apply_diffuse_and_absorb(&mut self, inputs: Vec<f32>, diffuse: f32, absorb: f32) -> Vec<f32> {
    inputs
      .iter()
      .zip(self.taps.iter_mut())
      .map(|(input, tap)| {
        let absorb_output = tap.apply_absorb(*input, absorb);
        tap.apply_diffuse(absorb_output, diffuse)
      })
      .collect()
  }

  fn write_to_delay_taps(&mut self, dry_signal: f32, feedback_inputs: Vec<f32>) {
    self.taps.iter_mut().enumerate().for_each(|(i, tap)| {
      let dry_signal = if i < 2 { dry_signal } else { 0. };
      tap.write(dry_signal + feedback_inputs[i]);
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
    let diffused_and_absorbed_outputs =
      self.apply_diffuse_and_absorb(matrix_outputs, diffuse, absorb);
    self.write_to_delay_taps(input, diffused_and_absorbed_outputs);
    self.get_taps_output(read_outputs)
  }
}
