extern crate lv2;
extern crate reverb;
use lv2::prelude::*;
use reverb::Reverb;

#[derive(PortCollection)]
struct Ports {
  reverse: InputPort<Control>,
  predelay: InputPort<Control>,
  size: InputPort<Control>,
  speed: InputPort<Control>,
  depth: InputPort<Control>,
  absorb: InputPort<Control>,
  decay: InputPort<Control>,
  tilt: InputPort<Control>,
  shimmer: InputPort<Control>,
  mix: InputPort<Control>,
  input_left: InputPort<Audio>,
  input_right: InputPort<Audio>,
  output_left: OutputPort<Audio>,
  output_right: OutputPort<Audio>,
}

#[uri("https://github.com/davemollen/dm-Reverb")]
struct DmReverb {
  reverb: Reverb,
}

impl Plugin for DmReverb {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    Some(Self {
      reverb: Reverb::new(_plugin_info.sample_rate() as f32),
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    let reverse = *ports.reverse;
    let predelay = *ports.predelay;
    let size = *ports.size;
    let speed = *ports.speed;
    let depth = *ports.depth;
    let absorb = *ports.absorb;
    let decay = *ports.decay;
    let tilt = *ports.tilt;
    let shimmer = *ports.shimmer;
    let mix = *ports.mix;

    let input_channels = ports
      .input_left
      .iter_mut()
      .zip(ports.output_right.iter_mut());
    let output_channels = ports
      .output_left
      .iter_mut()
      .zip(ports.output_right.iter_mut());

    for (input, output) in input_channels.zip(output_channels) {
      *output = self.reverb.run(
        *input, reverse, predelay, size, speed, depth, absorb, decay, tilt, shimmer, mix,
      );
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmReverb);
