use nih_plug::prelude::*;
use reverb::Reverb;
mod reverb_parameters;
use reverb_parameters::ReverbParameters;
use std::sync::Arc;

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
  const NAME: &'static str = "dm-Reverb";
  const VENDOR: &'static str = "DM";
  // You can use `env!("CARGO_PKG_HOMEPAGE")` to reference the homepage field from the
  // `Cargo.toml` file here
  const URL: &'static str = "https://youtu.be/dQw4w9WgXcQ";
  const EMAIL: &'static str = "davemollen@gmail.com";
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");
  const DEFAULT_INPUT_CHANNELS: u32 = 2;
  const DEFAULT_OUTPUT_CHANNELS: u32 = 2;
  const DEFAULT_AUX_INPUTS: Option<AuxiliaryIOConfig> = None;
  const DEFAULT_AUX_OUTPUTS: Option<AuxiliaryIOConfig> = None;
  const MIDI_INPUT: MidiConfig = MidiConfig::None;
  const SAMPLE_ACCURATE_AUTOMATION: bool = true;

  // More advanced plugins can use this to run expensive background tasks. See the field's
  // documentation for more information. `()` means that the plugin does not have any background
  // tasks.
  type BackgroundTask = ();

  fn params(&self) -> Arc<dyn Params> {
    self.params.clone()
  }

  fn accepts_bus_config(&self, config: &BusConfig) -> bool {
    // This works with any symmetrical IO layout
    config.num_input_channels == config.num_output_channels && config.num_input_channels > 0
  }

  fn initialize(
    &mut self,
    _: &BusConfig,
    buffer_config: &BufferConfig,
    _context: &mut impl InitContext<Self>,
  ) -> bool {
    self.reverb = Reverb::new(buffer_config.sample_rate);
    true
  }

  fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    _context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
    let predelay = self.params.predelay.value();
    let size = self.params.size.value();
    let speed = self.params.speed.value();
    let depth = self.params.depth.value();
    let absorb = self.params.absorb.value();
    let decay = self.params.decay.value();
    let tilt = self.params.tilt.value();
    let mix = self.params.mix.value();

    for mut channel_samples in buffer.iter_samples() {
      let left_channel = channel_samples.get_mut(0).unwrap();
      let input_left = *left_channel;
      let right_channel = channel_samples.get_mut(1).unwrap();
      let input_right = *right_channel;

      let (reverb_left, reverb_right) = self.reverb.run(
        (input_left, input_right),
        size,
        speed,
        depth,
        predelay,
        absorb,
        decay,
        tilt,
        mix,
      );

      let left_channel_out = channel_samples.get_mut(0).unwrap();
      *left_channel_out = reverb_left;
      let right_channel_out = channel_samples.get_mut(1).unwrap();
      *right_channel_out = reverb_right;
    }
    ProcessStatus::Normal
  }

  // This can be used for cleaning up special resources like socket connections whenever the
  // plugin is deactivated. Most plugins won't need to do anything here.
  fn deactivate(&mut self) {}
}

impl ClapPlugin for DmReverb {
  const CLAP_ID: &'static str = "dm-Reverb";
  const CLAP_DESCRIPTION: Option<&'static str> = Some("A reverb plugin");
  const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
  const CLAP_SUPPORT_URL: Option<&'static str> = None;
  const CLAP_FEATURES: &'static [ClapFeature] = &[
    ClapFeature::AudioEffect,
    ClapFeature::Stereo,
    ClapFeature::Mono,
    ClapFeature::Utility,
  ];
}

impl Vst3Plugin for DmReverb {
  const VST3_CLASS_ID: [u8; 16] = *b"DaveM-ReverbPlug";
  const VST3_CATEGORIES: &'static str = "Fx|Reverb";
}

nih_export_clap!(DmReverb);
nih_export_vst3!(DmReverb);
