use crate::{SINE, TABLE_SIZE};

#[derive(Default)]
pub struct WaveTable;

impl WaveTable {
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

  pub fn read_from_wavetable(&self, phase: f32) -> f32 {
    let floating_point_index = phase * TABLE_SIZE as f32;
    let truncated_index = floating_point_index.trunc();
    let mix = floating_point_index - truncated_index;
    let index = truncated_index as usize;
    self.linear_interp(SINE, index, mix)
  }
}

#[cfg(test)]
mod tests {
  use crate::sine_table::WaveTable;
  use std::f32::consts::PI;

  fn sine(phase: f32) -> f32 {
    (PI * 2. * phase).sin() * 0.5 + 0.5
  }

  fn assert_approximately_eq(left: f32, right: f32) {
    assert_eq!(
      (left / 1000.).floor() * 1000.,
      (right / 1000.).floor() * 1000.
    )
  }

  #[test]
  fn sine_lookup() {
    let wave_table = WaveTable::default();

    let mut phase: f32 = 1.;
    assert_approximately_eq(sine(phase), wave_table.read_from_wavetable(phase));
    phase = 0.5;
    assert_approximately_eq(sine(phase), wave_table.read_from_wavetable(phase));
  }
}
