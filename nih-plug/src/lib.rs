use nih_plug::prelude::*;
use stutter::Stutter;
mod stutter_parameters;
use std::sync::Arc;
use stutter_parameters::StutterParameters;
mod editor;

struct DmStutter {
  params: Arc<StutterParameters>,
  stutter: Stutter,
}

impl Default for DmStutter {
  fn default() -> Self {
    let params = Arc::new(StutterParameters::default());
    Self {
      params: params.clone(),
      stutter: Stutter::new(44100.),
    }
  }
}

impl Plugin for DmStutter {
  const NAME: &'static str = "dm-Stutter";
  const VENDOR: &'static str = "DM";
  const URL: &'static str = "https://github.com/davemollen/dm-Stutter";
  const EMAIL: &'static str = "davemollen@gmail.com";
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
    main_input_channels: NonZeroU32::new(1),
    main_output_channels: NonZeroU32::new(1),
    ..AudioIOLayout::const_default()
  }];
  const MIDI_INPUT: MidiConfig = MidiConfig::None;
  const SAMPLE_ACCURATE_AUTOMATION: bool = true;

  // More advanced plugins can use this to run expensive background tasks. See the field's
  // documentation for more information. `()` means that the plugin does not have any background
  // tasks.
  type BackgroundTask = ();
  type SysExMessage = ();

  fn params(&self) -> Arc<dyn Params> {
    self.params.clone()
  }

  fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
    editor::create(self.params.clone(), self.params.editor_state.clone())
  }

  fn initialize(
    &mut self,
    _audio_io_layout: &AudioIOLayout,
    buffer_config: &BufferConfig,
    _context: &mut impl InitContext<Self>,
  ) -> bool {
    self.stutter = Stutter::new(buffer_config.sample_rate);
    true
  }

  fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    _context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
    let on = self.params.on.value();
    let auto = self.params.auto.value();
    let trigger = self.params.trigger.value();
    let pulse = self.params.pulse.value();
    let duration = self.params.duration.value();
    let chance = self.params.chance.value();

    buffer.iter_samples().for_each(|mut channel_samples| {
      let sample = channel_samples.iter_mut().next().unwrap();
      let stutter_output = self
        .stutter
        .process(*sample, on, auto, trigger, pulse, duration, chance);
      *sample = stutter_output;
    });
    ProcessStatus::Normal
  }

  // This can be used for cleaning up special resources like socket connections whenever the
  // plugin is deactivated. Most plugins won't need to do anything here.
  fn deactivate(&mut self) {}
}

impl ClapPlugin for DmStutter {
  const CLAP_ID: &'static str = "dm-Stutter";
  const CLAP_DESCRIPTION: Option<&'static str> = Some("A delay plugin");
  const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
  const CLAP_SUPPORT_URL: Option<&'static str> = None;
  const CLAP_FEATURES: &'static [ClapFeature] = &[
    ClapFeature::AudioEffect,
    ClapFeature::Stereo,
    ClapFeature::Delay,
  ];
}

impl Vst3Plugin for DmStutter {
  const VST3_CLASS_ID: [u8; 16] = *b"dm-Stutter......";
  const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
    Vst3SubCategory::Fx,
    Vst3SubCategory::Delay,
    Vst3SubCategory::Stereo,
  ];
}

nih_export_clap!(DmStutter);
nih_export_vst3!(DmStutter);
