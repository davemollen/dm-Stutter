extern crate lv2;
extern crate stutter;
use lv2::prelude::*;
use stutter::Stutter;

#[derive(PortCollection)]
struct Ports {
  on: InputPort<Control>,
  auto: InputPort<Control>,
  trigger: InputPort<Control>,
  pulse: InputPort<Control>,
  duration: InputPort<Control>,
  chance: InputPort<Control>,
  half_notes: InputPort<Control>,
  seven_sixteenth_notes: InputPort<Control>,
  six_sixteenth_notes: InputPort<Control>,
  half_triplet_notes: InputPort<Control>,
  five_sixteenth_notes: InputPort<Control>,
  quarter_notes: InputPort<Control>,
  three_sixteenth_notes: InputPort<Control>,
  quarter_triplet_notes: InputPort<Control>,
  eighth_notes: InputPort<Control>,
  eighth_triplet_notes: InputPort<Control>,
  sixteenth_notes: InputPort<Control>,
  sixteenth_triplet_notes: InputPort<Control>,
  thirty_second_notes: InputPort<Control>,
  thirty_second_triplet_notes: InputPort<Control>,
  sixty_fourth_notes: InputPort<Control>,
  input: InputPort<Audio>,
  output: OutputPort<Audio>,
}

#[uri("https://github.com/davemollen/dm-Stutter")]
struct DmStutter {
  stutter: Stutter,
}

impl Plugin for DmStutter {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    Some(Self {
      stutter: Stutter::new(_plugin_info.sample_rate() as f32),
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    let on = *ports.on == 1.;
    let auto = *ports.auto == 1.;
    let trigger = *ports.trigger == 1.;
    let pulse = *ports.pulse;
    let duration = *ports.duration;
    let chance = *ports.chance;

    self.stutter.set_probability(
      *ports.half_notes,
      *ports.seven_sixteenth_notes,
      *ports.six_sixteenth_notes,
      *ports.half_triplet_notes,
      *ports.five_sixteenth_notes,
      *ports.quarter_notes,
      *ports.three_sixteenth_notes,
      *ports.quarter_triplet_notes,
      *ports.eighth_notes,
      *ports.eighth_triplet_notes,
      *ports.sixteenth_notes,
      *ports.sixteenth_triplet_notes,
      *ports.thirty_second_notes,
      *ports.thirty_second_triplet_notes,
      *ports.sixty_fourth_notes,
    );

    for (input, output) in ports.input.iter().zip(ports.output.iter_mut()) {
      *output = self
        .stutter
        .process(*input, on, auto, trigger, pulse, duration, chance);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmStutter);
