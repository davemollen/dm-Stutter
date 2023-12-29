#[path="./editor/components/param_checkbox.rs"]
mod param_checkbox;
use param_checkbox::ParamCheckbox;
#[path="./editor/components/param_knob.rs"]
mod param_knob;
use param_knob::ParamKnob;
mod ui_data;
use nih_plug::params::Param;
pub use ui_data::{UiData, ParamChangeEvent};
use nih_plug::prelude::Editor;
use nih_plug_vizia::{ViziaState, ViziaTheming, create_vizia_editor};
use nih_plug_vizia::vizia::{
  prelude::{Units::{Stretch, Pixels}, Weight},
  views::{HStack, VStack, Label}, 
  state::Model,
  modifiers::{LayoutModifiers, TextModifiers, StyleModifiers}
};
use std::sync::Arc;
use crate::reverb_parameters::ReverbParameters;

const STYLE: &str = include_str!("./editor/style.css");

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (480, 296))
}

pub(crate) fn create(
    params: Arc<ReverbParameters>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
  create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, gui_context| { 
    cx.add_theme(STYLE);

    UiData {
      params: params.clone(),
      gui_context: gui_context.clone(),
    }
    .build(cx);

    VStack::new(cx, |cx| {
      HStack::new(cx, |cx| {
        ParamCheckbox::new(
          cx,
          params.reverse.name(),
          UiData::params,
          params.reverse.as_ptr(),
          |params| &params.reverse,
          |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
        )
        .child_space(Stretch(1.0))
        .row_between(Pixels(8.0));
  
        VStack::new(cx, |cx| {
          ParamKnob::new(
            cx,
            params.predelay.name(),
            UiData::params,
            params.predelay.as_ptr(),
            |params| &params.predelay,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          );
          ParamKnob::new(
            cx,
            params.size.name(),
            UiData::params,
            params.size.as_ptr(),
            |params| &params.size,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          );
        })
        .child_space(Stretch(1.0))
        .row_between(Pixels(8.0));
  
        VStack::new(cx, |cx| {
          ParamKnob::new(
            cx,
            params.speed.name(),
            UiData::params,
            params.speed.as_ptr(),
            |params| &params.speed,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          );
          ParamKnob::new(
            cx,
            params.depth.name(),
            UiData::params,
            params.depth.as_ptr(),
            |params| &params.depth,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          );
        })
        .child_space(Stretch(1.0))
        .row_between(Pixels(8.0));
  
        VStack::new(cx, |cx| {
          ParamKnob::new(
            cx,
            params.absorb.name(),
            UiData::params,
            params.absorb.as_ptr(),
            |params| &params.absorb,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          );
          ParamKnob::new(
            cx,
            params.decay.name(),
            UiData::params,
            params.decay.as_ptr(),
            |params| &params.decay,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          );
        })
        .child_space(Stretch(1.0))
        .row_between(Pixels(8.0));
  
        VStack::new(cx, |cx| {
          ParamKnob::new(
            cx,
            params.tilt.name(),
            UiData::params,
            params.tilt.as_ptr(),
            |params| &params.tilt,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          );
          ParamKnob::new(
            cx,
            params.shimmer.name(),
            UiData::params,
            params.shimmer.as_ptr(),
            |params| &params.shimmer,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          );
        })
        .child_space(Stretch(1.0))
        .row_between(Pixels(8.0));
  
        
        ParamKnob::new(
          cx,
          params.mix.name(),
          UiData::params,
          params.mix.as_ptr(),
          |params| &params.mix,
          |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
        )
        .child_space(Stretch(1.0))
        .top(Pixels(-128.0));
      }).top(Pixels(32.0));
  
      Label::new(cx, "dm-Reverb")
          .font_size(22.0)
          .font_weight(Weight::BOLD)
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
  })
}
