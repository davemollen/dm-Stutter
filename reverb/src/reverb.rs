use super::{
  mix::Mix,
  one_pole_filter::OnePoleFilter,
  tap::{Tap, TapInitializer},
  phasor::Phasor
};

pub struct Reverb {
  taps: Vec<Tap>,
  smooth_predelay: OnePoleFilter,
  smooth_size: OnePoleFilter,
  smooth_depth: OnePoleFilter,
  smooth_absorb: OnePoleFilter,
  lfo_phasor: Phasor
}

impl Reverb {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      taps: vec![
        Tap::new(TapInitializer {
          sample_rate,
          time: 0.68,
          feedback_matrix: [1.0, 1.0, 1.0, 1.0],
          diffuser_time: 5.,
          lfo_phase_offset: 0.,
        }),
        Tap::new(TapInitializer {
          sample_rate,
          time: 0.77,
          feedback_matrix: [1.0, -1.0, 1.0, -1.0],
          diffuser_time: 7.,
          lfo_phase_offset: 0.25,
        }),
        Tap::new(TapInitializer {
          sample_rate,
          time: 0.90,
          feedback_matrix: [1.0, 1.0, -1.0, -1.0],
          diffuser_time: 11.,
          lfo_phase_offset: 0.5,
        }),
        Tap::new(TapInitializer {
          sample_rate,
          time: 0.99,
          feedback_matrix: [1.0, -1.0, -1.0, 1.0],
          diffuser_time: 13.,
          lfo_phase_offset: 0.75,
        }),
      ],
      lfo_phasor: Phasor::new(sample_rate),
      smooth_predelay: OnePoleFilter::new(sample_rate),
      smooth_size: OnePoleFilter::new(sample_rate),
      smooth_depth: OnePoleFilter::new(sample_rate),
      smooth_absorb: OnePoleFilter::new(sample_rate),
    }
  }

  fn read_from_delay_taps(&mut self, size: f32, speed: f32, depth: f32) -> Vec<f32> {
    let master_phase = self.lfo_phasor.run(speed);
    self.taps
      .iter_mut()
      .map(|tap,| -> f32 { 
        let lfo = tap.lfo.run(master_phase, tap.lfo_phase_offset) * depth;
        tap.delay_line.read(tap.time * size + lfo, "linear") })
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
    self.taps.iter_mut().enumerate().for_each(|(i, tap)| {
      let dry_signal = if i < 2 { input } else { 0. };
      let processed_outputs: f32 = tap.process_delay_tap(read_outputs, diffuse, absorb);
      tap
        .delay_line
        .write(dry_signal + processed_outputs * decay * 0.5)
    })
  }

  fn get_reverb_output(&mut self, read_outputs: Vec<f32>) -> (f32, f32) {
    (read_outputs[0] + read_outputs[2], read_outputs[1] + read_outputs[3])
  }

  pub fn run(
    &mut self,
    input: f32,
    size: f32,
    speed: f32,
    depth: f32,
    predelay: f32,
    absorb: f32,
    decay: f32,
    mix: f32,
  ) -> (f32, f32) {
    let predelay = self.smooth_predelay.run(predelay, 12., "hertz");
    let size = self.smooth_size.run(size, 12., "hertz");
    let depth = self.smooth_depth.run(depth.powf(4.) * 4., 12., "hertz");
    let absorb = self.smooth_absorb.run(absorb, 12., "hertz");
    let decay = decay.powf(0.3333333);
    let diffuse = (absorb * 3.).min(1.) * 0.8;
    let absorb = ((absorb - 0.3333333).max(0.) * 1.5).powf(0.3333333);
    
    let read_outputs = self.read_from_delay_taps(size, speed, depth);
    self.write_to_delay_taps(input, &read_outputs, diffuse, absorb, decay);
    let reverb = self.get_reverb_output(read_outputs);
    Mix::run(input, reverb, mix)
  }
}
