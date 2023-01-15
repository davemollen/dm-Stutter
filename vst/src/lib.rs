#[macro_use]
extern crate vst;
mod reverb_parameters;
use reverb::Reverb;
use reverb_parameters::ReverbParameters;
use std::sync::Arc;
use vst::{
  api::TimeInfo,
  buffer::AudioBuffer,
  host::Host,
  plugin::{HostCallback, Info, Plugin, PluginParameters},
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
  fn new(host: HostCallback) -> Self {
    fn get_sample_rate(info: TimeInfo) -> f64 {
      info.sample_rate
    }
    let sample_rate = host.get_time_info(0).map(get_sample_rate).unwrap();
    Self {
      params: Arc::new(ReverbParameters::default()),
      reverb: Reverb::new(sample_rate),
    }
  }

  fn set_sample_rate(&mut self, sample_rate: f32) {
    self.reverb = Reverb::new(f64::from(sample_rate));
  }

  fn get_info(&self) -> Info {
    Info {
      name: "dm-Reverb".to_string(),
      inputs: 1,
      outputs: 1,
      parameters: 5,
      unique_id: 1358,
      f64_precision: true,
      ..Default::default()
    }
  }

  fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
    let size = self.params.size.get();
    let diffuse = self.params.diffuse.get();
    let absorb = self.params.absorb.get();
    let decay = self.params.decay.get();
    let mix = self.params.mix.get();

    for (input_buffer, output_buffer) in buffer.zip() {
      for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
        let reverb_output = self
          .reverb
          .run(*input_sample, size, diffuse, absorb, decay, mix);
        *output_sample = reverb_output.0;
      }
    }
  }

  fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
    Arc::clone(&self.params) as Arc<dyn PluginParameters>
  }
}

plugin_main!(DmReverb);
