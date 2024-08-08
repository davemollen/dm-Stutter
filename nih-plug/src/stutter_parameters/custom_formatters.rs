use std::sync::Arc;

use crate::map_tempo_factor;

pub fn v2s_f32_digits(digits: usize) -> Arc<dyn Fn(f32) -> String + Send + Sync> {
  Arc::new(move |value| format!("{:.digits$}", value))
}

pub fn v2s_f32_tempo_factor() -> Arc<dyn Fn(i32) -> String + Send + Sync> {
  Arc::new(move |value| map_tempo_factor(value).to_string())
}

pub fn s2v_f32_tempo_factor() -> Arc<dyn Fn(&str) -> Option<i32> + Send + Sync> {
  Arc::new(|string| {
    string
      .trim_end_matches(&[' ', 'x'])
      .parse()
      .ok()
      .map(|x: f32| match x {
        0.25 => 0,
        0.5 => 1,
        1. => 2,
        2. => 3,
        4. => 4,
        _ => panic!("Unsupported value for tempo factor was found."),
      })
  })
}
