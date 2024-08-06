pub struct ManualTrigger {
  prev: bool,
}

impl ManualTrigger {
  pub fn new() -> Self {
    Self { prev: false }
  }

  pub fn process(&mut self, next: bool) -> bool {
    let output = next != self.prev;
    self.prev = next;
    output
  }
}
