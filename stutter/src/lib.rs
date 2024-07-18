mod activator;
mod crossfade;
mod delay;
mod duration_generator;
mod manual_trigger;
mod phasor;
mod stereo_delay_line;
mod time_fraction_generator;
mod shared {
  pub mod float_ext;
  pub mod tuple_ext;
}
use {
  activator::Activator, crossfade::Crossfade, delay::Delay, duration_generator::DurationGenerator,
  manual_trigger::ManualTrigger, phasor::Phasor, shared::tuple_ext::TupleExt,
  time_fraction_generator::TimeFractionGenerator,
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

  pub fn set_probability(
    &mut self,
    half_notes: f32,
    seven_sixteenth_notes: f32,
    six_sixteenth_notes: f32,
    half_triplet_notes: f32,
    five_sixteenth_notes: f32,
    quarter_notes: f32,
    three_sixteenth_notes: f32,
    quarter_triplet_notes: f32,
    eighth_notes: f32,
    eighth_triplet_notes: f32,
    sixteenth_notes: f32,
    sixteenth_triplet_notes: f32,
    thirty_second_notes: f32,
    thirty_second_triplet_notes: f32,
    sixty_fourth_notes: f32,
  ) {
    self.time_fraction_generator.set_probability([
      (half_notes, 2.),
      (seven_sixteenth_notes, 1.75),
      (six_sixteenth_notes, 1.5),
      (half_triplet_notes, 1.33333333),
      (five_sixteenth_notes, 1.25),
      (quarter_notes, 1.),
      (three_sixteenth_notes, 0.75),
      (quarter_triplet_notes, 1.5_f32.recip()),
      (eighth_notes, 2_f32.recip()),
      (eighth_triplet_notes, 3_f32.recip()),
      (sixteenth_notes, 4_f32.recip()),
      (sixteenth_triplet_notes, 6_f32.recip()),
      (thirty_second_notes, 8_f32.recip()),
      (thirty_second_triplet_notes, 12_f32.recip()),
      (sixty_fourth_notes, 16_f32.recip()),
    ]);
  }

  pub fn process(
    &mut self,
    input: (f32, f32),
    on: bool,
    auto_trigger: bool,
    manual_trigger: bool,
    pulse: f32,
    duration: f32,
    chance: f32,
  ) -> (f32, f32) {
    let manual_trigger = self.manual_trigger.process(manual_trigger, on);
    let trigger = self.phasor.get_trigger();
    let flip_flop = if self.phasor.get_flip_flop() { 1. } else { 0. };
    let any_trigger = trigger.0 || trigger.1;

    let time_fraction = self.time_fraction_generator.process(any_trigger);
    let fraction = match time_fraction {
      Some(f) => f,
      None => 1.,
    };
    let delay_time = pulse * fraction;
    let duration = self
      .duration_generator
      .process(delay_time, fraction, duration, any_trigger);

    self.phasor.process(duration, auto_trigger, manual_trigger);

    let (delay_fade_a, delay_fade_b) = self
      .delay_crossfade
      .process(flip_flop, 20_f32.min(delay_time / 2.));
    let delay_out = self.delay[0]
      .process(input, trigger.0, delay_time, delay_fade_a, delay_fade_b)
      .add(self.delay[1].process(input, trigger.1, delay_time, delay_fade_b, delay_fade_a));

    self.activator.process(
      input,
      delay_out,
      on,
      time_fraction,
      chance,
      auto_trigger,
      any_trigger,
    )
  }
}
