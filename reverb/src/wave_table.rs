use crate::{clip::Clip, SINE, TABLE_SIZE, TANH};

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

  fn clip_index(&self, index: usize) -> usize {
    index.clip(0, TABLE_SIZE - 2)
  }

  fn linear_interp(
    &self,
    wavetable: [f32; TABLE_SIZE],
    index: usize,
    mix: f32,
    enable_clip_index: bool,
  ) -> f32 {
    if enable_clip_index {
      let clipped_index = self.clip_index(index);
      let x = wavetable[clipped_index];
      let y = wavetable[clipped_index + 1];
      x * (1. - mix) + y * mix
    } else {
      let x = wavetable[self.wrap(index)];
      let y = wavetable[self.wrap(index + 1)];
      x * (1. - mix) + y * mix
    }
  }

  pub fn read_from_wavetable(&self, phase: f32, waveform: &str) -> f32 {
    let floating_point_index = phase * TABLE_SIZE as f32;
    let truncated_index = floating_point_index.trunc();
    let mix = floating_point_index - truncated_index;
    let index = truncated_index as usize;
    match waveform {
      "SINE" => self.linear_interp(SINE, index, mix, false),
      "TANH" => self.linear_interp(TANH, index, mix, true),
      _ => self.linear_interp(SINE, index, mix, false),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::wave_table::WaveTable;
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
  fn tanh_lookup() {
    let wave_table = WaveTable::default();

    let mut input: f32 = 0.;
    assert_approximately_eq(
      input.tanh(),
      wave_table.read_from_wavetable(input * 0.25 + 0.5, "TANH"),
    );
    input = 0.5;
    assert_approximately_eq(
      input.tanh(),
      wave_table.read_from_wavetable(input * 0.25 + 0.5, "TANH"),
    );
    input = 2.;
    assert_approximately_eq(
      input.tanh(),
      wave_table.read_from_wavetable(input * 0.25 + 0.5, "TANH"),
    );
    input = 8.;
    assert!(wave_table.read_from_wavetable(input * 0.25 + 0.5, "TANH") < 1.);
    input = -1.;
    assert_approximately_eq(
      input.tanh(),
      wave_table.read_from_wavetable(input * 0.25 + 0.5, "TANH"),
    );
  }

  #[test]
  fn sine_lookup() {
    let wave_table = WaveTable::default();

    let mut phase: f32 = 1.;
    assert_approximately_eq(sine(phase), wave_table.read_from_wavetable(phase, "SINE"));
    phase = 0.5;
    assert_approximately_eq(sine(phase), wave_table.read_from_wavetable(phase, "SINE"));
  }
}
