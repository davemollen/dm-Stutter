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

pub fn map_tempo_factor(value: i32) -> f32 {
  match value {
    0 => 0.25,
    1 => 0.5,
    2 => 1.,
    3 => 2.,
    4 => 4.,
    _ => panic!("Unsupported value for tempo factor was found."),
  }
}

impl DmStutter {
  fn get_synced_pulse_time(&self, bpm: f32) -> f32 {
    60000. / bpm * map_tempo_factor(self.params.tempo_factor.value())
  }
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
  const NAME: &'static str = "Stutter";
  const VENDOR: &'static str = "DM";
  const URL: &'static str = "https://github.com/davemollen/dm-Stutter";
  const EMAIL: &'static str = "davemollen@gmail.com";
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
    main_input_channels: NonZeroU32::new(2),
    main_output_channels: NonZeroU32::new(2),
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
    context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
    let on = self.params.on.value();
    let trigger = self.params.trigger.value();
    let auto = self.params.auto.value();
    let mix = self.params.mix.value() as i32;
    let pulse = if self.params.sync.value() {
      let bpm = context.transport().tempo.unwrap_or(120.) as f32;
      self.get_synced_pulse_time(bpm)
    } else {
      self.params.pulse.value()
    };
    let duration = self.params.duration.value();
    let chance = self.params.chance.value();

    self.stutter.set_probability(
      self.params.half_notes.value(),
      self.params.seven_sixteenth_notes.value(),
      self.params.six_sixteenth_notes.value(),
      self.params.half_triplet_notes.value(),
      self.params.five_sixteenth_notes.value(),
      self.params.quarter_notes.value(),
      self.params.three_sixteenth_notes.value(),
      self.params.quarter_triplet_notes.value(),
      self.params.eighth_notes.value(),
      self.params.eighth_triplet_notes.value(),
      self.params.sixteenth_notes.value(),
      self.params.sixteenth_triplet_notes.value(),
      self.params.thirty_second_notes.value(),
      self.params.thirty_second_triplet_notes.value(),
      self.params.sixty_fourth_notes.value(),
    );

    buffer.iter_samples().for_each(|mut channel_samples| {
      let channel_iterator = &mut channel_samples.iter_mut();
      let left_channel = channel_iterator.next().unwrap();
      let right_channel = channel_iterator.next().unwrap();

      (*left_channel, *right_channel, _, _) = self.stutter.process(
        (*left_channel, *right_channel),
        on,
        trigger,
        auto,
        mix,
        pulse,
        duration,
        chance,
        false,
      );
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
