pub struct ToggleTrigger {
  prev_on: bool,
}

impl ToggleTrigger {
  pub fn new() -> Self {
    Self { prev_on: false }
  }

  pub fn process(&mut self, next_on: bool) -> bool {
    let output = !self.prev_on && next_on;
    self.prev_on = next_on;
    output
  }
}
