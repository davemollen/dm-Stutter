use crate::delay::Delay;

pub struct RepeatTrigger {
  next_write_index_trigger: f32,
  delay_length: f32,
  sample_rate: f32,
}

impl RepeatTrigger {
  pub fn new(sample_rate: f32, delay_length: usize) -> Self {
    Self {
      next_write_index_trigger: 0.,
      delay_length: delay_length.next_power_of_two() as f32,
      sample_rate,
    }
  }

  pub fn process(
    &mut self,
    delay: &[Delay; 2],
    trigger: bool,
    flip_flop: bool,
    delay_time: f32,
  ) -> bool {
    let active_delay_index = if flip_flop { 0 } else { 1 };
    let write_index = delay[active_delay_index].get_write_index();
    if trigger || write_index == self.next_write_index_trigger as usize {
      self.next_write_index_trigger = self.wrap(write_index as f32 + self.mstosamps(delay_time));
      return true;
    }

    false
  }

  fn wrap(&self, x: f32) -> f32 {
    if x >= self.delay_length {
      x - self.delay_length
    } else {
      x
    }
  }

  fn mstosamps(&self, time: f32) -> f32 {
    time * 0.001 * self.sample_rate
  }
}
