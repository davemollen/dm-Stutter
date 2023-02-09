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

  // TODO: add early reflections
  fn read_from_delay_taps(&mut self, size: f32, speed: f32, depth: f32) -> Vec<f32> {
    let phase = self.lfo_phasor.run(speed);
    self
      .taps
      .iter_mut()
      .map(|tap| -> f32 { tap.read(size, phase, depth) })
      .collect()
  }

  fn apply_feedback_matrix<'a>(
    &self,
    inputs: &'a Vec<f32>,
    decay: f32,
  ) -> impl Iterator<Item = f32> + 'a {
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
    feedback_matrix_outputs: impl Iterator<Item = f32> + 'a,
    diffuse: f32,
    absorb: f32,
    decay: f32,
  ) {
    self
      .taps
      .iter_mut()
      .zip([input, input, 0., 0.].iter())
      .zip(feedback_matrix_outputs)
      .for_each(|((tap, dry_signal), feedback_matrix_output)| {
        let saturation_output = tap.apply_saturation(feedback_matrix_output, decay);
        let absorb_output = tap.apply_absorb(dry_signal + saturation_output, absorb);
        let diffuse_output = tap.apply_diffuse(absorb_output, diffuse);
        tap.write(diffuse_output);
      });
  }

  fn get_output(&mut self, inputs: Vec<f32>) -> (f32, f32) {
    let left_out = (inputs[0] + inputs[2]) * 0.7071;
    let right_out = (inputs[1] + inputs[3]) * 0.7071;
    (left_out, right_out)
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
    let feedback_matrix_outputs = self.apply_feedback_matrix(&read_outputs, decay);
    self.process_and_write_taps(input, feedback_matrix_outputs, diffuse, absorb, decay);
    self.get_output(read_outputs)
  }
}
