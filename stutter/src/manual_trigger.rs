pub struct ManualTrigger {
  prev: bool,
  prev_on: bool,
}

impl ManualTrigger {
  pub fn new() -> Self {
    Self {
      prev: false,
      prev_on: false,
    }
  }

  pub fn process(&mut self, next: bool, next_on: bool) -> bool {
    let output = next != self.prev || !self.prev_on && next_on;
    self.prev = next;
    self.prev_on = next_on;
    output
  }
}
