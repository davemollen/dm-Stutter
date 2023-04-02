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
mod ui_data;
pub use ui_data::{ParamChangeEvent, UiData};

const STYLE: &str = include_str!("./ui/style.css");

pub fn plugin_gui(cx: &mut Context, params: Arc<ReverbParameters>, host: Option<HostCallback>) {
  cx.add_theme(STYLE);

  UiData {
    params: params.clone(),
  }
  .build(cx);

  HStack::new(cx, |cx| {
    VStack::new(cx, |cx| {
      ParamKnob::new(cx, &params.predelay, |params| &params.predelay, host);
      ParamKnob::new(cx, &params.size, |params| &params.size, host);
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(cx, &params.speed, |params| &params.speed, host);
      ParamKnob::new(cx, &params.depth, |params| &params.depth, host);
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(cx, &params.shimmer, |params| &params.shimmer, host);
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(cx, &params.absorb, |params| &params.absorb, host);
      ParamKnob::new(cx, &params.decay, |params| &params.decay, host);
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(cx, &params.tilt, |params| &params.tilt, host);
      ParamKnob::new(cx, &params.mix, |params| &params.mix, host);
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));
  })
  .background_color(Color::rgb(80, 80, 80));
}
