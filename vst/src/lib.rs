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
      inputs: 1,
      outputs: 2,
      parameters: 5,
      unique_id: 1358,
      f64_precision: true,
      category: Category::Effect,
      ..Default::default()
    }
  }

  fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
    let size = self.params.size.get();
    let diffuse = self.params.diffuse.get();
    let absorb = self.params.absorb.get();
    let decay = self.params.decay.get();
    let mix = self.params.mix.get();

    let (inputs, mut outputs) = buffer.split();
    let input = inputs.get(0);
    let zipped_outputs = outputs
      .get_mut(0)
      .iter_mut()
      .zip(outputs.get_mut(1).iter_mut());
    for (input, (output_left, output_right)) in input.iter().zip(zipped_outputs) {
      let (reverb_left, reverb_right) = self.reverb.run(*input, size, diffuse, absorb, decay, mix);
      *output_left = reverb_left;
      *output_right = reverb_right;
    }

    // for (input_buffer, output_buffer) in buffer.zip() {
    //   for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
    //     let reverb_output = self
    //       .reverb
    //       .run(*input_sample, size, diffuse, absorb, decay, mix);
    //     *output_sample = reverb_output.0;
    //   }
    // }
  }

  fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
    Arc::clone(&self.params) as Arc<dyn PluginParameters>
  }
}

plugin_main!(DmReverb);
