#[path = "./editor/components/param_knob.rs"]
mod param_knob;
use param_knob::{ParamKnob, ParamKnobSize};
#[path = "./editor/components/param_checkbox.rs"]
mod param_checkbox;
use param_checkbox::ParamCheckbox;
mod ui_data;
use nih_plug::params::Param;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::{
  model::Model,
  modifiers::{LayoutModifiers, StyleModifiers, TextModifiers},
  prelude::Units::{Pixels, Stretch},
  style::FontWeightKeyword,
  views::{HStack, Label, VStack},
};
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;
use ui_data::{ParamChangeEvent, UiData};

use crate::stutter_parameters::StutterParameters;

const STYLE: &str = include_str!("./editor/style.css");

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
  ViziaState::new(|| (440, 200))
}

pub(crate) fn create(
  params: Arc<StutterParameters>,
  editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
  create_vizia_editor(
    editor_state,
    ViziaTheming::Custom,
    move |cx, gui_context| {
      let _ = cx.add_stylesheet(STYLE);

      UiData {
        params: params.clone(),
        gui_context: gui_context.clone(),
      }
      .build(cx);

      VStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
          ParamCheckbox::new(
            cx,
            params.on.name(),
            UiData::params,
            params.on.as_ptr(),
            |params| &params.on,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          )
          .width(Pixels(56.0))
          .height(Pixels(92.0));

          ParamCheckbox::new(
            cx,
            params.auto.name(),
            UiData::params,
            params.auto.as_ptr(),
            |params| &params.auto,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          )
          .width(Pixels(56.0))
          .height(Pixels(92.0));

          ParamCheckbox::new(
            cx,
            params.trigger.name(),
            UiData::params,
            params.trigger.as_ptr(),
            |params| &params.trigger,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          )
          .width(Pixels(56.0))
          .height(Pixels(92.0));

          ParamKnob::new(
            cx,
            params.pulse.name(),
            UiData::params,
            params.pulse.as_ptr(),
            |params| &params.pulse,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );

          ParamKnob::new(
            cx,
            params.chance.name(),
            UiData::params,
            params.chance.as_ptr(),
            |params| &params.chance,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );

          ParamKnob::new(
            cx,
            params.duration.name(),
            UiData::params,
            params.duration.as_ptr(),
            |params| &params.duration,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );
        })
        .space(Stretch(1.0))
        .child_space(Stretch(1.0));

        Label::new(cx, "dm-Stutter")
          .font_size(22.0)
          .font_weight(FontWeightKeyword::Bold)
          .border_radius(Pixels(16.0))
          .border_width(Pixels(1.))
          .border_color("#2c5494")
          .background_color("#3c6ab5")
          .child_space(Stretch(1.0))
          .child_top(Pixels(1.0))
          .child_bottom(Pixels(5.0))
          .width(Pixels(144.0))
          .left(Stretch(1.0));
      })
      .child_space(Pixels(16.0))
      .background_color("#505050");
    },
  )
}
