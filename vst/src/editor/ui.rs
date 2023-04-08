use crate::reverb_parameters::ReverbParameters;
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
use vst::prelude::HostCallback;
mod param_knob;
pub use param_knob::ParamKnob;
mod param_checkbox;
pub use param_checkbox::ParamCheckbox;
mod ui_data;
pub use ui_data::{ParamChangeEvent, UiData};

const STYLE: &str = include_str!("./ui/style.css");

pub fn plugin_gui(cx: &mut Context, params: Arc<ReverbParameters>, host: Option<HostCallback>) {
  cx.add_theme(STYLE);

  UiData {
    params: params.clone(),
    host,
  }
  .build(cx);

  HStack::new(cx, |cx| {
    VStack::new(cx, |cx| {
      ParamCheckbox::new(
        cx,
        &params.reverse,
        |params| &params.reverse,
        |val| ParamChangeEvent::SetReverse(val),
      );
    })
    .child_space(Stretch(0.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        &params.predelay,
        |params| &params.predelay,
        |val| ParamChangeEvent::SetPredelay(val),
      );
      ParamKnob::new(
        cx,
        &params.size,
        |params| &params.size,
        |val| ParamChangeEvent::SetSize(val),
      );
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        &params.speed,
        |params| &params.speed,
        |val| ParamChangeEvent::SetSpeed(val),
      );
      ParamKnob::new(
        cx,
        &params.depth,
        |params| &params.depth,
        |val| ParamChangeEvent::SetDepth(val),
      );
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        &params.absorb,
        |params| &params.absorb,
        |val| ParamChangeEvent::SetAbsorb(val),
      );
      ParamKnob::new(
        cx,
        &params.decay,
        |params| &params.decay,
        |val| ParamChangeEvent::SetDecay(val),
      );
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        &params.tilt,
        |params| &params.tilt,
        |val| ParamChangeEvent::SetTilt(val),
      );
      ParamKnob::new(
        cx,
        &params.shimmer,
        |params| &params.shimmer,
        |val| ParamChangeEvent::SetShimmer(val),
      );
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        &params.mix,
        |params| &params.mix,
        |val| ParamChangeEvent::SetMix(val),
      );
    })
    .child_space(Stretch(1.0))
    .child_top(Stretch(0.1))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        &params.shimmer,
        |params| &params.shimmer,
        |val| ParamChangeEvent::SetSize(val),
      );
    })
    .child_top(Pixels(10.0))
    .row_between(Pixels(10.0));
  })
  .background_color(Color::rgb(80, 80, 80));
}
