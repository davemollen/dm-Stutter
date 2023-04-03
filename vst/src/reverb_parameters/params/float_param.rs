use std::sync::Arc;
use vst::util::AtomicFloat;
mod float_range;
use super::Params;
pub use float_range::FloatRange;

pub struct FloatParam {
  pub name: &'static str,
  pub value: AtomicFloat,
  pub default: f32,
  pub index: i32,
  pub range: FloatRange,
  pub unit: &'static str,
  pub value_to_string: Option<Arc<dyn Fn(f32) -> String + Send + Sync>>,
}

impl FloatParam {
  pub fn new(name: &'static str, default: f32, index: i32, range: FloatRange) -> Self {
    Self {
      name,
      value: AtomicFloat::new(default),
      default,
      index,
      range,
      unit: "",
      value_to_string: None,
    }
  }

  pub fn with_unit(mut self, unit: &'static str) -> Self {
    self.unit = unit;
    self
  }
}

impl Params for FloatParam {
  type Plain = f32;

  fn get_name(&self) -> &str {
    self.name
  }

  fn get_value(&self) -> f32 {
    self.value.get()
  }

  fn get_normalized_value(&self) -> f32 {
    self.range.normalize(self.get_value())
  }

  fn set_plain_value(&self, value: f32) {
    let plain_value = self.range.unnormalize(value);
    self.value.set(plain_value);
  }

  fn get_display_value(&self, include_unit: bool) -> String {
    let value = self.get_value();
    match (&self.value_to_string, include_unit) {
      (Some(f), true) => format!("{}{}", f(value), self.unit),
      (Some(f), false) => f(value),
      (None, true) => format!("{}{}", value, self.unit),
      (None, false) => value.to_string(),
    }
  }

  fn get_default_normalized_value(&self) -> f32 {
    self.range.normalize(self.default)
  }

  fn with_value_to_string(
    mut self,
    callback: Arc<dyn Fn(Self::Plain) -> String + Send + Sync>,
  ) -> Self {
    self.value_to_string = Some(callback);
    self
  }
}
