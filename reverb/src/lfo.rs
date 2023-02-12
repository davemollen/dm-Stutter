use crate::wave_table::WaveTable;

#[derive(Default)]
pub struct Lfo {
  wave_table: WaveTable,
}

impl Lfo {
  pub fn run(&mut self, master_phase: f32, phase_offset: f32) -> f32 {
    let phase = (master_phase + phase_offset) % 1.;
    self.wave_table.read_from_wavetable(phase)
  }
}
