pub struct ManualTrigger {
  prev: bool,
}

impl ManualTrigger {
  pub fn new() -> Self {
    Self { prev: false }
  }

  pub fn process(&mut self, next: bool, is_momentary_trigger: bool) -> bool {
    let output = if is_momentary_trigger {
      next && !self.prev
    } else {
      next != self.prev
    };
    self.prev = next;
    output
  }
}
