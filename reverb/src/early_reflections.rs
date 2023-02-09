use crate::delay_line::DelayLine;

pub struct EarlyReflection {
  pub time_fraction: f32,
  pub gain: f32,
}

pub struct EarlyReflections {
  delay_line: DelayLine,
  early_reflections: Vec<EarlyReflection>,
}

impl EarlyReflections {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      delay_line: DelayLine::new((sample_rate * 1.5) as usize, sample_rate),
      early_reflections: vec![
        EarlyReflection {
          time_fraction: 0.,
          gain: 0.9,
        },
        EarlyReflection {
          time_fraction: 0.155,
          gain: 1.,
        },
        EarlyReflection {
          time_fraction: 0.3,
          gain: 0.8,
        },
        EarlyReflection {
          time_fraction: 0.41,
          gain: 0.6,
        },
      ],
    }
  }

  pub fn write(&mut self, input: f32) {
    self.delay_line.write(input);
  }

  pub fn read(&mut self, size: f32) -> f32 {
    let EarlyReflections {
      delay_line,
      early_reflections,
    } = self;
    early_reflections
      .iter()
      .map(|early_reflection| {
        delay_line.read(early_reflection.time_fraction * size + 5., "linear")
          * early_reflection.gain
      })
      .sum()
  }
}
