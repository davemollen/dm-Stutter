extern crate lv2;
extern crate stutter;
use lv2::prelude::*;
use stutter::Stutter;

#[derive(URIDCollection)]
struct URIDs {
  atom: AtomURIDCollection,
  unit: UnitURIDCollection,
  time: TimeURIDCollection,
}

#[derive(FeatureCollection)]
pub struct InitFeatures<'a> {
  map: LV2Map<'a>,
}

#[derive(PortCollection)]
struct Ports {
  control: InputPort<AtomPort>,
  on: InputPort<InPlaceControl>,
  trigger: InputPort<InPlaceControl>,
  auto: InputPort<InPlaceControl>,
  sync: InputPort<InPlaceControl>,
  mix: InputPort<InPlaceControl>,
  pulse: InputPort<InPlaceControl>,
  tempo_factor: InputPort<InPlaceControl>,
  duration: InputPort<InPlaceControl>,
  chance: InputPort<InPlaceControl>,
  half_notes: InputPort<InPlaceControl>,
  seven_sixteenth_notes: InputPort<InPlaceControl>,
  six_sixteenth_notes: InputPort<InPlaceControl>,
  half_triplet_notes: InputPort<InPlaceControl>,
  five_sixteenth_notes: InputPort<InPlaceControl>,
  quarter_notes: InputPort<InPlaceControl>,
  three_sixteenth_notes: InputPort<InPlaceControl>,
  quarter_triplet_notes: InputPort<InPlaceControl>,
  eighth_notes: InputPort<InPlaceControl>,
  eighth_triplet_notes: InputPort<InPlaceControl>,
  sixteenth_notes: InputPort<InPlaceControl>,
  sixteenth_triplet_notes: InputPort<InPlaceControl>,
  thirty_second_notes: InputPort<InPlaceControl>,
  thirty_second_triplet_notes: InputPort<InPlaceControl>,
  sixty_fourth_notes: InputPort<InPlaceControl>,
  input_left: InputPort<InPlaceAudio>,
  input_right: InputPort<InPlaceAudio>,
  output_left: OutputPort<InPlaceAudio>,
  output_right: OutputPort<InPlaceAudio>,
  trigger_cv_output: OutputPort<InPlaceCV>,
  repeat_trigger_cv_output: OutputPort<InPlaceCV>,
}

#[uri("https://github.com/davemollen/dm-Stutter")]
struct DmStutter {
  stutter: Stutter,
  urids: URIDs,
  bpm: f32,
}

impl DmStutter {
  fn get_synced_pulse_time(&self, ports: &mut Ports) -> f32 {
    60000. / self.bpm * Self::map_tempo_factor(ports.tempo_factor.get())
  }

  fn set_bpm(&mut self, ports: &mut Ports) {
    let control_sequence = match ports
      .control
      .read(self.urids.atom.sequence, self.urids.unit.beat)
    {
      Some(sequence_iter) => sequence_iter,
      None => return,
    };

    for (_, atom) in control_sequence {
      let (object_header, object_reader) = match atom.read(self.urids.atom.object, ()) {
        Some(x) => x,
        None => return,
      };

      if object_header.otype != self.urids.time.position_class {
        continue;
      }

      for (property_header, property) in object_reader {
        if property_header.key != self.urids.time.beats_per_minute {
          continue;
        }

        if let Some(bpm) = property.read(self.urids.atom.float, ()) {
          self.bpm = bpm;
        }
      }
    }
  }

  fn map_tempo_factor(tempo_factor: f32) -> f32 {
    match tempo_factor {
      0. => 0.25,
      1. => 0.5,
      2. => 1.,
      3. => 2.,
      4. => 4.,
      _ => panic!("Unsupported value for tempo factor was found."),
    }
  }
}

impl Plugin for DmStutter {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = InitFeatures<'static>;
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(plugin_info: &PluginInfo, features: &mut Self::InitFeatures) -> Option<Self> {
    Some(Self {
      bpm: 120.,
      stutter: Stutter::new(plugin_info.sample_rate() as f32),
      urids: features.map.populate_collection()?,
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut Self::AudioFeatures, _sample_count: u32) {
    let on = ports.on.get() == 1.;
    let trigger = ports.trigger.get() == 1.;
    let auto = ports.auto.get() == 1.;
    let mix = ports.mix.get() as i32 - 1;

    let pulse = if ports.sync.get() == 1. {
      self.set_bpm(ports);
      self.get_synced_pulse_time(ports)
    } else {
      ports.pulse.get()
    };
    let duration = ports.duration.get();
    let chance = ports.chance.get();

    self.stutter.set_probability(
      ports.half_notes.get(),
      ports.seven_sixteenth_notes.get(),
      ports.six_sixteenth_notes.get(),
      ports.half_triplet_notes.get(),
      ports.five_sixteenth_notes.get(),
      ports.quarter_notes.get(),
      ports.three_sixteenth_notes.get(),
      ports.quarter_triplet_notes.get(),
      ports.eighth_notes.get(),
      ports.eighth_triplet_notes.get(),
      ports.sixteenth_notes.get(),
      ports.sixteenth_triplet_notes.get(),
      ports.thirty_second_notes.get(),
      ports.thirty_second_triplet_notes.get(),
      ports.sixty_fourth_notes.get(),
    );

    let input_channels = ports.input_left.iter().zip(ports.input_right.iter());
    let output_channels = ports.output_left.iter().zip(ports.output_right.iter());
    let trigger_cv_output = ports.trigger_cv_output.iter();
    let repeat_trigger_cv_output = ports.repeat_trigger_cv_output.iter();

    for (
      (((input_left, input_right), (output_left, output_right)), trigger_cv_output),
      repeat_trigger_cv_output,
    ) in input_channels
      .zip(output_channels)
      .zip(trigger_cv_output)
      .zip(repeat_trigger_cv_output)
    {
      let stutter_output = self.stutter.process(
        (input_left.get(), input_right.get()),
        on,
        trigger,
        auto,
        mix,
        pulse,
        duration,
        chance,
        true,
      );
      output_left.set(stutter_output.0);
      output_right.set(stutter_output.1);
      trigger_cv_output.set(if stutter_output.2 { 10. } else { 0. });
      repeat_trigger_cv_output.set(if stutter_output.3 { 10. } else { 0. });
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmStutter);
