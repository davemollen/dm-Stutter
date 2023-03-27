use crate::{
  delay_line::{DelayLine, Interpolation},
  delta::Delta,
  phasor::Phasor,
  MAX_DEPTH,
};
use rand::random;
use std::f32::consts::PI;

const FADE_THRESHOLD_FACTOR: f32 = 0.05;
const SHIMMER_THRESHOLD_FACTOR: f32 = 0.6;
const FADE_THRESHOLD: f32 = MAX_DEPTH * FADE_THRESHOLD_FACTOR;
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
    size: f32,
    time_fraction: f32,
    lfo_phase: f32,
    lfo_depth: f32,
  ) -> f32 {
    let apply_grains_out = self.apply_grains(delay_line, size, time_fraction, lfo_phase, lfo_depth);
    let grains_out = if lfo_depth < FADE_THRESHOLD {
      self.mix(
        delay_line.read(size * time_fraction, Interpolation::Linear),
        apply_grains_out,
        lfo_depth,
        FADE_THRESHOLD_FACTOR,
        false,
      )
    } else {
      apply_grains_out
    };

    self.shimmer.delay_line.write(grains_out);

    if lfo_depth > SHIMMER_THRESHOLD {
      let shimmer_out = self.apply_shimmer();
      self.mix(
        grains_out,
        shimmer_out,
        lfo_depth,
        SHIMMER_THRESHOLD_FACTOR,
        true,
      )
    } else {
      grains_out
    }
  }

  fn get_mix_factor(&self, lfo_depth: f32, threshold: f32, should_be_above_threshold: bool) -> f32 {
    if should_be_above_threshold {
      (lfo_depth / MAX_DEPTH - threshold) * (1. - threshold).recip()
    } else {
      lfo_depth / MAX_DEPTH * threshold.recip()
    }
  }

  fn mix(
    &self,
    a: f32,
    b: f32,
    lfo_depth: f32,
    threshold: f32,
    should_be_above_threshold: bool,
  ) -> f32 {
    let factor = self.get_mix_factor(lfo_depth, threshold, should_be_above_threshold);
    a * (1. - factor) + b * factor
  }

  fn apply_grains(
    &mut self,
    delay_line: &mut DelayLine,
    size: f32,
    time_fraction: f32,
    lfo_phase: f32,
    lfo_depth: f32,
  ) -> f32 {
    self
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
      .sum()
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

#[cfg(test)]
mod tests {
  use crate::grains::{Grains, FADE_THRESHOLD_FACTOR, SHIMMER_THRESHOLD_FACTOR};

  #[test]
  fn mix_factor() {
    let grains = Grains::new(44100.);

    assert_eq!(
      grains.get_mix_factor(2.4, SHIMMER_THRESHOLD_FACTOR, true),
      0.
    );
    assert_eq!(
      grains.get_mix_factor(3.2, SHIMMER_THRESHOLD_FACTOR, true),
      0.5
    );
    assert_eq!(
      grains.get_mix_factor(4., SHIMMER_THRESHOLD_FACTOR, true),
      1.
    );

    assert_eq!(grains.get_mix_factor(0., FADE_THRESHOLD_FACTOR, false), 0.);
    assert_eq!(
      grains.get_mix_factor(0.1, FADE_THRESHOLD_FACTOR, false),
      0.5
    );
    assert_eq!(grains.get_mix_factor(0.2, FADE_THRESHOLD_FACTOR, false), 1.);
  }
}
