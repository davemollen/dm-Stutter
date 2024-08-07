use std::sync::Arc;

pub fn v2s_f32_digits(digits: usize) -> Arc<dyn Fn(f32) -> String + Send + Sync> {
  Arc::new(move |value| format!("{:.digits$}", value))
}

pub fn v2s_f32_tempo_multiplier(digits: usize) -> Arc<dyn Fn(f32) -> String + Send + Sync> {
  Arc::new(move |value| format!("{:.digits$}", 2_f32.powf(value * 8. - 4.)))
}

pub fn s2v_f32_tempo_multiplier() -> Arc<dyn Fn(&str) -> Option<f32> + Send + Sync> {
  Arc::new(|string| {
    string
      .trim_end_matches(&[' ', 'x'])
      .parse()
      .ok()
      .map(|x: f32| (x.ln() / 2_f32.ln() + 4.) / 8.)
  })
}
