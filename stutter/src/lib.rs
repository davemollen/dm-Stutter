mod activator;
mod crossfade;
mod delay;
mod duration_generator;
mod manual_trigger;
mod phasor;
mod repeat_trigger;
mod stereo_delay_line;
mod time_fraction_generator;
mod toggle_trigger;
mod shared {
  pub mod float_ext;
  pub mod tuple_ext;
}
use {
  crate::repeat_trigger::RepeatTrigger, activator::Activator, crossfade::Crossfade, delay::Delay,
  duration_generator::DurationGenerator, manual_trigger::ManualTrigger, phasor::Phasor,
  shared::tuple_ext::TupleExt, time_fraction_generator::TimeFractionGenerator,
  toggle_trigger::ToggleTrigger,
};

pub struct Stutter {
  time_fraction_generator: TimeFractionGenerator,
  duration_generator: DurationGenerator,
  manual_trigger: ManualTrigger,
  toggle_trigger: ToggleTrigger,
  duration: f32,
  phasor: Phasor,
  repeat_trigger: RepeatTrigger,
  flip_flop: bool,
  delay_crossfade: Crossfade,
  delay: [Delay; 2],
  activator: Activator,
}

impl Stutter {
  pub fn new(sample_rate: f32) -> Self {
    let delay_length = (sample_rate * 12.) as usize;

    Self {
      time_fraction_generator: TimeFractionGenerator::new(),
      duration_generator: DurationGenerator::new(),
      manual_trigger: ManualTrigger::new(),
      toggle_trigger: ToggleTrigger::new(),
      duration: 0.,
      phasor: Phasor::new(sample_rate),
      repeat_trigger: RepeatTrigger::new(sample_rate, delay_length),
      flip_flop: false,
      delay_crossfade: Crossfade::new(sample_rate),
      delay: [
        Delay::new(sample_rate, delay_length),
        Delay::new(sample_rate, delay_length),
      ],
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
    manual_trigger: bool,
    auto_trigger: bool,
    mix: i32,
    pulse: f32,
    duration: f32,
    chance: f32,
    is_momentary_trigger: bool,
  ) -> (f32, f32, bool, bool) {
    let manual_trigger = self
      .manual_trigger
      .process(manual_trigger, is_momentary_trigger);
    let reset = self.toggle_trigger.process(on) || manual_trigger;
    if reset {
      self.phasor.reset();
    }
    let (trigger, (trigger_a, trigger_b)) = self.get_triggers(auto_trigger, reset);

    let time_fraction = self.time_fraction_generator.process(trigger);
    let delay_time = pulse * time_fraction;

    self.duration = self
      .duration_generator
      .process(delay_time, time_fraction, duration, trigger);

    let (delay_fade_a, delay_fade_b) = self
      .delay_crossfade
      .process(self.flip_flop, 20_f32.min(delay_time * 0.5));
    let delay_out = self.delay[0]
      .process(input, trigger_a, delay_time, delay_fade_a, delay_fade_b)
      .add(self.delay[1].process(input, trigger_b, delay_time, delay_fade_b, delay_fade_a));

    let repeat_trigger =
      self
        .repeat_trigger
        .process(&self.delay, trigger, self.flip_flop, delay_time);

    let stutter_output = self.activator.process(
      input,
      delay_out,
      on,
      chance,
      auto_trigger,
      trigger,
      manual_trigger,
      mix,
    );
    (
      stutter_output.0,
      stutter_output.1,
      trigger,
      trigger || repeat_trigger,
    )
  }

  fn get_triggers(&mut self, auto_trigger: bool, reset: bool) -> (bool, (bool, bool)) {
    let trigger = reset || (auto_trigger && self.phasor.process(self.duration));

    (
      trigger,
      match (trigger, self.flip_flop) {
        (true, false) => {
          self.flip_flop = true;
          (true, false)
        }
        (true, true) => {
          self.flip_flop = false;
          (false, true)
        }
        _ => (false, false),
      },
    )
  }
}
