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
      ParamKnob::new(cx, params.reverse.as_ptr(), |params| &params.reverse);
    })
    .child_space(Stretch(0.5))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(cx, params.predelay.as_ptr(), |params| &params.predelay);
      ParamKnob::new(cx, params.size.as_ptr(), |params| &params.size);
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(cx, params.speed.as_ptr(), |params| &params.speed);
      ParamKnob::new(cx, params.depth.as_ptr(), |params| &params.depth);
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(cx, params.absorb.as_ptr(), |params| &params.absorb);
      ParamKnob::new(cx, params.decay.as_ptr(), |params| &params.decay);
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(cx, params.tilt.as_ptr(), |params| &params.tilt);
      ParamKnob::new(cx, params.shimmer.as_ptr(), |params| &params.shimmer);
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(cx, params.mix.as_ptr(), |params| &params.mix);
    })
    .child_space(Stretch(1.0))
    .child_top(Stretch(0.1))
    .row_between(Pixels(10.0));
  })
  .background_color(Color::rgb(80, 80, 80));
}
