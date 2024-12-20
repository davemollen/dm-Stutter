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
pub struct Features<'a> {
  map: LV2Map<'a>,
}

#[derive(PortCollection)]
struct Ports {
  control: InputPort<AtomPort>,
  on: InputPort<Control>,
  trigger: InputPort<Control>,
  auto: InputPort<Control>,
  sync: InputPort<Control>,
  mix: InputPort<Control>,
  pulse: InputPort<Control>,
  tempo_factor: InputPort<Control>,
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
  input_left: InputPort<Audio>,
  input_right: InputPort<Audio>,
  output_left: OutputPort<Audio>,
  output_right: OutputPort<Audio>,
}

#[uri("https://github.com/davemollen/dm-Stutter")]
struct DmStutter {
  stutter: Stutter,
  urids: URIDs,
  bpm: f32,
}

impl DmStutter {
  fn get_synced_pulse_time(&self, ports: &mut Ports) -> f32 {
    60000. / self.bpm * Self::map_tempo_factor(*ports.tempo_factor)
  }

  fn set_bpm(&mut self, ports: &mut Ports) {
    let sequence_header_reader = match ports.control.read(self.urids.atom.sequence) {
      Ok(sequence_header_reader) => sequence_header_reader,
      Err(_) => return,
    };
    let sequence_iter = match sequence_header_reader.with_unit(self.urids.unit.beat) {
      Ok(sequence_iter) => sequence_iter,
      Err(_) => return,
    };

    for (_, atom) in sequence_iter {
      let (object_header, object_reader) = match atom
        .read(self.urids.atom.object)
        .or_else(|_| atom.read(self.urids.atom.blank))
      {
        Ok(pair) => pair,
        Err(_) => continue,
      };

      if object_header.otype != self.urids.time.position_class {
        continue;
      }

      for (property_header, property) in object_reader {
        if property_header.key != self.urids.time.beats_per_minute {
          continue;
        }

        if let Ok(bpm) = property.read(self.urids.atom.float) {
          self.bpm = *bpm;
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
  type InitFeatures = Features<'static>;
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(_plugin_info: &PluginInfo, features: &mut Features<'static>) -> Option<Self> {
    Some(Self {
      bpm: 120.,
      stutter: Stutter::new(_plugin_info.sample_rate() as f32),
      urids: features.map.populate_collection()?,
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    let on = *ports.on == 1. && self.stutter.has_note_length_values();
    let trigger = *ports.trigger == 1.;
    let auto = *ports.auto == 1.;
    let mix = *ports.mix as i32 - 1;

    let pulse = if *ports.sync == 1. {
      self.set_bpm(ports);
      self.get_synced_pulse_time(ports)
    } else {
      *ports.pulse
    };
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

    let input_channels = ports.input_left.iter().zip(ports.input_right.iter());
    let output_channels = ports
      .output_left
      .iter_mut()
      .zip(ports.output_right.iter_mut());

    for ((input_left, input_right), (output_left, output_right)) in
      input_channels.zip(output_channels)
    {
      (*output_left, *output_right) = self.stutter.process(
        (*input_left, *input_right),
        on,
        trigger,
        auto,
        mix,
        pulse,
        duration,
        chance,
        true,
      );
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmStutter);
