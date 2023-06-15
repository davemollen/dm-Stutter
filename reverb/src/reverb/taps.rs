mod early_reflections;
mod saturation_activator;
mod tap;
use crate::shared::phasor::Phasor;
use {early_reflections::EarlyReflections, saturation_activator::SaturationActivator, tap::Tap};

pub struct Taps {
  early_reflections: EarlyReflections,
  taps: [Tap; 4],
  lfo_phasor: Phasor,
  saturation_activator: SaturationActivator,
}

impl Taps {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      early_reflections: EarlyReflections::new(),
      taps: [
        Tap::new(sample_rate, 0.34306569343065696, 5.75, 0.),
        Tap::new(sample_rate, 0.48905109489051096, 9.416666666666668, 0.25),
        Tap::new(sample_rate, 0.7372262773722628, 13.083333333333332, 0.5),
        Tap::new(sample_rate, 1., 14.916666666666666, 0.75),
      ],
      lfo_phasor: Phasor::new(sample_rate),
      saturation_activator: SaturationActivator::new(sample_rate),
    }
  }

  fn read_from_delay_network(&mut self, size: f32, speed: f32, depth: f32) -> Vec<f32> {
    let phase = self.lfo_phasor.run(speed);
    self
      .taps
      .iter_mut()
      .map(|tap| tap.delay_network_read(size, phase, depth))
      .collect()
  }

  fn apply_feedback_matrix(inputs: &Vec<f32>) -> impl Iterator<Item = f32> + '_ {
    [
      [1.0, -1.0, -1.0, 1.0],
      [1.0, 1.0, -1.0, -1.0],
      [1.0, -1.0, 1.0, -1.0],
      [1.0, 1.0, 1.0, 1.0],
    ]
    .iter()
    .map(move |feedback_values| -> f32 {
      feedback_values
        .iter()
        .zip(inputs)
        .map(|(feedback, input)| input * feedback)
        .sum()
    })
  }

  fn process_and_write_taps(
    &mut self,
    input: f32,
    feedback_matrix_outputs: impl Iterator<Item = f32>,
    diffuse: f32,
    absorb: f32,
    decay: f32,
  ) {
    let saturation_gain = self.saturation_activator.get_saturation_gain();

    self
      .taps
      .iter_mut()
      .zip([input, input, 0., 0.])
      .zip(feedback_matrix_outputs)
      .for_each(|((tap, dry_signal), feedback_matrix_output)| {
        let saturation_output =
          tap.apply_saturation(feedback_matrix_output, decay, saturation_gain);
        let absorb_output = tap.apply_absorb(dry_signal + saturation_output, absorb);
        let diffuse_output = tap.apply_diffuse(absorb_output, diffuse);
        tap.write(diffuse_output);
      });
  }

  fn mix_delay_network_and_reflections(
    &mut self,
    inputs: Vec<f32>,
    early_reflections_output: Vec<f32>,
  ) -> (f32, f32) {
    let left_delay_network_out = inputs[0] + inputs[2];
    let right_delay_network_out = inputs[1] + inputs[3];
    self
      .saturation_activator
      .set_average(left_delay_network_out, right_delay_network_out);

    let left_out = (left_delay_network_out + early_reflections_output[0]) * 0.5;
    let right_out = (right_delay_network_out + early_reflections_output[1]) * 0.5;
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
    let early_reflections_outputs = self.early_reflections.run(size, &mut self.taps);

    let delay_network_outputs = self.read_from_delay_network(size, speed, depth);
    let feedback_matrix_outputs = Self::apply_feedback_matrix(&delay_network_outputs);
    self.process_and_write_taps(input, feedback_matrix_outputs, diffuse, absorb, decay);
    self.mix_delay_network_and_reflections(delay_network_outputs, early_reflections_outputs)
  }
}
