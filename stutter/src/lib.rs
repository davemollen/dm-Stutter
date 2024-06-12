mod activator;
mod crossfade;
mod delay;
mod delay_line;
mod duration_generator;
mod manual_trigger;
mod phasor;
mod time_fraction_generator;
mod shared {
  pub mod float_ext;
}
use {
  activator::Activator, crossfade::Crossfade, delay::Delay, duration_generator::DurationGenerator,
  manual_trigger::ManualTrigger, phasor::Phasor, time_fraction_generator::TimeFractionGenerator,
};

pub struct Stutter {
  time_fraction_generator: TimeFractionGenerator,
  duration_generator: DurationGenerator,
  manual_trigger: ManualTrigger,
  phasor: Phasor,
  delay_crossfade: Crossfade,
  delay: [Delay; 2],
  activator: Activator,
}

impl Stutter {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      manual_trigger: ManualTrigger::new(),
      time_fraction_generator: TimeFractionGenerator::new(),
      duration_generator: DurationGenerator::new(),
      phasor: Phasor::new(sample_rate),
      delay_crossfade: Crossfade::new(sample_rate),
      delay: [Delay::new(sample_rate), Delay::new(sample_rate)],
      activator: Activator::new(sample_rate),
    }
  }

  pub fn process(
    &mut self,
    input: f32,
    on: bool,
    auto_trigger: bool,
    manual_trigger: bool,
    pulse: f32,
    duration: f32,
    chance: f32,
  ) -> f32 {
    let manual_trigger = if !auto_trigger {
      self.manual_trigger.process(manual_trigger, on)
    } else {
      false
    };
    let trigger = self.phasor.get_trigger();
    let flip_flop = if self.phasor.get_flip_flop() { 1. } else { 0. };
    let any_trigger = trigger.0 || trigger.1;

    let time_fraction = self.time_fraction_generator.process(any_trigger);
    let delay_time = pulse / time_fraction;
    let duration =
      self
        .duration_generator
        .process(delay_time, time_fraction, duration, any_trigger);

    self.phasor.process(duration, auto_trigger, manual_trigger);

    let (delay_fade_a, delay_fade_b) = self
      .delay_crossfade
      .process(flip_flop, 20_f32.min(delay_time / 2.));
    let delay_out = self.delay[0].process(input, trigger.0, delay_time, delay_fade_a, delay_fade_b)
      + self.delay[1].process(input, trigger.1, delay_time, delay_fade_b, delay_fade_a);

    self
      .activator
      .process(input, delay_out, on, chance, any_trigger)
  }
}
