use std::sync::Arc;

pub fn v2s_f32_digits(digits: usize) -> Arc<dyn Fn(f32) -> String + Send + Sync> {
  Arc::new(move |value| format!("{:.digits$}", value))
}

pub fn s2v_f32_digits() -> Arc<dyn Fn(&str) -> Option<f32> + Send + Sync> {
  Arc::new(move |string| string.parse().ok())
}
