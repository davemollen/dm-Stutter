use std::sync::Arc;

pub fn v2s_f32_digits(digits: usize) -> Arc<dyn Fn(f32) -> String + Send + Sync> {
  Arc::new(move |value| format!("{:.digits$}", value))
}

pub fn v2s_f32_percentage(digits: usize) -> Arc<dyn Fn(f32) -> String + Send + Sync> {
  Arc::new(move |value| format!("{:.digits$}", value * 100.0))
}

pub fn s2v_f32_digits() -> Arc<dyn Fn(&str) -> Option<f32> + Send + Sync> {
  Arc::new(|string| string.parse().ok())
}

pub fn s2v_f32_percentage() -> Arc<dyn Fn(&str) -> Option<f32> + Send + Sync> {
  Arc::new(|string| {
    string
      .trim_end_matches(&[' ', '%'])
      .parse()
      .ok()
      .map(|x: f32| x / 100.0)
  })
}
