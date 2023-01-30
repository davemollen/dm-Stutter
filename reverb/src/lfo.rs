use crate::{SINE, TABLE_SIZE};

#[derive(Default)]
pub struct Lfo {}

impl Lfo {
  fn wrap(&self, index: usize) -> usize {
    if index >= TABLE_SIZE {
      index - TABLE_SIZE
    } else {
      index
    }
  }

  fn linear_interp(&self, wavetable: [f32; TABLE_SIZE], index: usize, mix: f32) -> f32 {
    let x = wavetable[self.wrap(index)];
    let y = wavetable[self.wrap(index + 1)];
    x * (1. - mix) + y * mix
  }

  fn read_from_wavetable(&self, floating_point_index: f32) -> f32 {
    let truncated_index = floating_point_index.trunc();
    let mix = floating_point_index - truncated_index;
    let index = truncated_index as usize;
    self.linear_interp(SINE, index, mix)
  }

  pub fn run(&mut self, master_phase: f32, phase_offset: f32) -> f32 {
    let phase = (master_phase + phase_offset) % 1.;
    self.read_from_wavetable(phase * TABLE_SIZE as f32)
  }
}
