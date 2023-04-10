use crate::ReverbParameters;
use nih_plug::prelude::{GuiContext, Param};
use std::sync::Arc;
use vizia::{
  prelude::{
    Context, LayoutModifiers, StyleModifiers,
    Units::{Pixels, Stretch},
  },
  state::Model,
  style::Color,
  views::{HStack, VStack},
};
mod param_knob;
use param_knob::ParamKnob;
mod param_checkbox;
use param_checkbox::ParamCheckbox;
mod ui_data;
pub use ui_data::{ParamChangeEvent, UiData};

const STYLE: &str = include_str!("./ui/style.css");

pub fn plugin_gui(
  cx: &mut Context,
  params: Arc<ReverbParameters>,
  gui_context: Arc<dyn GuiContext>,
) {
  cx.add_theme(STYLE);

  UiData {
    params: params.clone(),
    gui_context: gui_context.clone(),
  }
  .build(cx);

  HStack::new(cx, |cx| {
    VStack::new(cx, |cx| {
      ParamCheckbox::new(
        cx,
        UiData::params,
        params.reverse.as_ptr(),
        |params| &params.reverse,
        |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
      );
    })
    .child_space(Stretch(0.5))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        UiData::params,
        params.predelay.as_ptr(),
        |params| &params.predelay,
        |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
      );
      ParamKnob::new(
        cx,
        UiData::params,
        params.size.as_ptr(),
        |params| &params.size,
        |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
      );
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        UiData::params,
        params.speed.as_ptr(),
        |params| &params.speed,
        |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
      );
      ParamKnob::new(
        cx,
        UiData::params,
        params.depth.as_ptr(),
        |params| &params.depth,
        |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
      );
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        UiData::params,
        params.absorb.as_ptr(),
        |params| &params.absorb,
        |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
      );
      ParamKnob::new(
        cx,
        UiData::params,
        params.decay.as_ptr(),
        |params| &params.decay,
        |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
      );
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        UiData::params,
        params.tilt.as_ptr(),
        |params| &params.tilt,
        |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
      );
      ParamKnob::new(
        cx,
        UiData::params,
        params.shimmer.as_ptr(),
        |params| &params.shimmer,
        |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
      );
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        UiData::params,
        params.mix.as_ptr(),
        |params| &params.mix,
        |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
      );
    })
    .child_space(Stretch(1.0))
    .child_top(Stretch(0.1))
    .row_between(Pixels(10.0));
  })
  .background_color(Color::rgb(80, 80, 80));
}
