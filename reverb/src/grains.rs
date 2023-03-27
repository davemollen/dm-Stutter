use crate::{
  delay_line::{DelayLine, Interpolation},
  delta::Delta,
  phasor::Phasor,
  MAX_DEPTH,
};
use rand::random;
use std::f32::consts::{FRAC_PI_2, PI};

const SHIMMER_THRESHOLD_FACTOR: f32 = 0.6;
const SHIMMER_THRESHOLD: f32 = MAX_DEPTH * SHIMMER_THRESHOLD_FACTOR;

#[derive(Clone, Copy)]
struct Grain {
  start_position: f32,
  delta: Delta,
}

struct Shimmer {
  freq: f32,
  phasor: Phasor,
  delay_line: DelayLine,
}

pub struct Grains {
  grains: [Grain; 2],
  shimmer: Shimmer,
}

impl Grains {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      grains: [Grain {
        start_position: 0.,
        delta: Delta::new(),
      }; 2],
      shimmer: Shimmer {
        freq: 5.,
        phasor: Phasor::new(sample_rate),
        delay_line: DelayLine::new(sample_rate as usize, sample_rate),
      },
    }
  }

  pub fn run(
    &mut self,
    delay_line: &mut DelayLine,
    lfo_phase: f32,
    lfo_depth: f32,
    time_fraction: f32,
    size: f32,
  ) -> f32 {
    let grains_out = self
      .grains
      .iter_mut()
      .enumerate()
      .map(|(index, grain)| {
        let phase = (lfo_phase + index as f32 * 0.5) % 1.;
        let trigger = grain.delta.run(phase) < 0.;
        if trigger {
          grain.start_position = random::<f32>() * lfo_depth;
        };
        let window = (phase * PI).sin();
        let time = size * time_fraction + grain.start_position;

        delay_line.read(time, Interpolation::Linear) * window * window
      })
      .sum();

    self.shimmer.delay_line.write(grains_out);

    if lfo_depth > SHIMMER_THRESHOLD {
      let shimmer_out = self.apply_shimmer();
      self.mix(grains_out, shimmer_out, lfo_depth)
    } else {
      grains_out
    }
  }

  fn get_mix_factor(&self, lfo_depth: f32) -> f32 {
    lfo_depth / MAX_DEPTH - SHIMMER_THRESHOLD_FACTOR * (1. - SHIMMER_THRESHOLD_FACTOR).recip()
  }

  fn mix(&self, grains_out: f32, shimmer_out: f32, lfo_depth: f32) -> f32 {
    let factor = self.get_mix_factor(lfo_depth);
    grains_out * (factor * FRAC_PI_2).sin() + shimmer_out * (factor * FRAC_PI_2).cos()
  }

  fn apply_shimmer(&mut self) -> f32 {
    let phasor = self.shimmer.phasor.run(-self.shimmer.freq);
    let window_size = 1000. / self.shimmer.freq;

    (0..2)
      .map(|index| {
        let phase = (phasor + index as f32 * 0.5) % 1.;
        let window = (phase * PI).sin();
        self
          .shimmer
          .delay_line
          .read(phase * window_size, Interpolation::Linear)
          * window
          * window
      })
      .sum()
  }
}

// #[cfg(test)]
// mod tests {
//   use super::Grains;

//   fn assert_approximately_eq(left: f32, right: f32) {
//     assert_eq!(
//       (left * 1000.).floor() / 1000.,
//       (right * 1000.).floor() / 1000.
//     )
//   }

//   #[test]
//   fn mix() {
//     let grains = Grains::new(44100.);

//     assert_eq!(grains.get_mix_factor(3.), 0.);
//     assert_eq!(grains.get_mix_factor(4.), 0.5);
//     assert_eq!(grains.get_mix_factor(5.), 1.);
//   }
// }
