use std::{env, f32::consts::PI, fs, path::Path};

const TABLE_SIZE: usize = 1024;

fn generate_wave_vector(function: fn(f32) -> f32) -> Vec<f32> {
  (0..TABLE_SIZE)
    .map(|i| {
      let phase = i as f32 / TABLE_SIZE as f32;
      function(phase)
    })
    .collect()
}

fn sine(phase: f32) -> f32 {
  (PI * 2. * phase).sin() * 0.5 + 0.5
}

fn tanh(phase: f32) -> f32 {
  (phase * 4. - 2.).tanh()
}

fn write_wave_to_string(name: String, wave: Vec<f32>) -> String {
  let mut contents = format!("pub static {name}: [f32; {TABLE_SIZE}] = [\r\n");
  for value in wave {
    contents.push_str("\u{20}\u{20}\u{20}\u{20}");
    contents.push_str(&format!("{}f32", value));
    contents.push_str(",\r\n");
  }
  contents.push_str("];\r\n");
  contents
}

fn main() {
  let sine: Vec<f32> = generate_wave_vector(sine);
  let tanh: Vec<f32> = generate_wave_vector(tanh);
  let sine_table = write_wave_to_string(String::from("SINE"), sine);
  let tanh_table = write_wave_to_string(String::from("TANH"), tanh);
  let wave_table_contents =
    format!("pub const TABLE_SIZE: usize = 1024;\r\n {sine_table} {tanh_table}");

  let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("wave_table.rs");
  fs::write(&dest_path, wave_table_contents).unwrap();
}
