#[macro_use]
extern crate vst;
mod reverb_parameters;
use reverb::Reverb;
use reverb_parameters::ReverbParameters;
use std::sync::Arc;
use vst::{
  buffer::AudioBuffer,
  plugin::{Category, Info, Plugin, PluginParameters},
};

struct DmReverb {
  params: Arc<ReverbParameters>,
  reverb: Reverb,
}

impl Default for DmReverb {
  fn default() -> Self {
    Self {
      params: Arc::new(ReverbParameters::default()),
      reverb: Reverb::new(44100.),
    }
  }
}

impl Plugin for DmReverb {
  fn set_sample_rate(&mut self, sample_rate: f32) {
    self.reverb = Reverb::new(sample_rate);
  }

  fn get_info(&self) -> Info {
    Info {
      name: "dm-Reverb".to_string(),
      vendor: "DM".to_string(),
      version: 1,
      inputs: 2,
      outputs: 2,
      parameters: 8,
      unique_id: 1358,
      f64_precision: true,
      category: Category::Effect,
      ..Default::default()
    }
  }

  fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
    let predelay = self.params.predelay.get();
    let size = self.params.size.get();
    let speed = self.params.speed.get();
    let depth = self.params.depth.get();
    let absorb = self.params.absorb.get();
    let decay = self.params.decay.get();
    let tilt = self.params.tilt.get();
    let mix = self.params.mix.get();

    let (input_channels, mut output_channels) = buffer.split();
    let zipped_input_channels = input_channels.get(0).iter().zip(input_channels.get(1));
    let zipped_output_channels = output_channels
      .get_mut(0)
      .iter_mut()
      .zip(output_channels.get_mut(1));
    for ((input_left, input_right), (output_left, output_right)) in
      zipped_input_channels.zip(zipped_output_channels)
    {
      let (reverb_left, reverb_right) = self.reverb.run(
        (*input_left, *input_right),
        size,
        speed,
        depth,
        predelay,
        absorb,
        decay,
        tilt,
        mix,
      );
      *output_left = reverb_left;
      *output_right = reverb_right;
    }
  }

  fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
    Arc::clone(&self.params) as Arc<dyn PluginParameters>
  }
}

plugin_main!(DmReverb);
