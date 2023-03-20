use crate::{
  delay_line::{DelayLine, Interpolation},
  delta::Delta,
};
use rand::random;
use std::f32::consts::PI;

#[derive(Clone, Copy)]
struct Grain {
  start_position: f32,
  delta: Delta,
}

pub struct Grains {
  grains: [Grain; 2],
}

impl Grains {
  pub fn new() -> Self {
    Self {
      grains: [Grain {
        start_position: 0.,
        delta: Delta::new(),
      }; 2],
    }
  }

  pub fn run(
    &mut self,
    delay_line: &mut DelayLine,
    lfo_phase: f32,
    lfo_depth: f32,
    time_fraction: f32,
    size: f32,
  ) -> f32 {
    self
      .grains
      .iter_mut()
      .enumerate()
      .map(|(index, grain)| {
        let phase = (lfo_phase + index as f32 * 0.5) % 1.;
        let trigger = grain.delta.run(phase) < 0.;
        if trigger {
          grain.start_position = random::<f32>() * lfo_depth;
        };
        let window = ((phase - 0.5) * PI).cos();
        let time = size * time_fraction + grain.start_position;

        delay_line.read(time, Interpolation::Linear) * window * window
      })
      .sum()
  }
}
