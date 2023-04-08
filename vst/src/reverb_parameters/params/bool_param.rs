use super::Params;
use std::sync::{
  atomic::{AtomicBool, Ordering},
  Arc,
};

pub struct BoolParam {
  pub name: &'static str,
  pub value: AtomicBool,
  pub default: bool,
  pub index: i32,
  pub value_to_string: Option<Arc<dyn Fn(bool) -> String + Send + Sync>>,
}

impl BoolParam {
  pub fn new(name: &'static str, default: bool, index: i32) -> Self {
    Self {
      name,
      value: AtomicBool::new(default),
      default,
      index,
      value_to_string: None,
    }
  }

  pub fn convert_bool_to_float(&self, value: bool) -> f32 {
    if value {
      1.
    } else {
      0.
    }
  }

  pub fn convert_float_to_bool(&self, value: f32) -> bool {
    value > 0.5
  }
}

impl Params for BoolParam {
  type Plain = bool;

  fn get_index(&self) -> i32 {
    self.index
  }

  fn get_value(&self) -> Self::Plain {
    self.value.load(Ordering::Relaxed)
  }

  fn get_normalized_value(&self) -> f32 {
    self.convert_bool_to_float(self.get_value())
  }

  fn set_plain_value(&self, value: Self::Plain) {
    self.value.store(value, Ordering::Relaxed);
  }

  fn set_normalized_value(&self, value: f32) {
    self
      .value
      .store(self.convert_float_to_bool(value), Ordering::Relaxed);
  }

  fn get_display_value(&self, _: bool) -> String {
    let value = self.value.load(Ordering::Relaxed);
    match (value, &self.value_to_string) {
      (true, None) => String::from("On"),
      (false, None) => String::from("Off"),
      (val, Some(f)) => f(val),
    }
  }

  fn get_default_normalized_value(&self) -> f32 {
    self.convert_bool_to_float(self.default)
  }

  fn with_value_to_string(
    mut self,
    callback: Arc<dyn Fn(Self::Plain) -> String + Send + Sync>,
  ) -> Self {
    self.value_to_string = Some(callback);
    self
  }
}
