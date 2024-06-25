use std::any::Any;

use nih_plug::{params::Param, prelude::ParamPtr};
use nih_plug_vizia::vizia::{
  binding::{Lens, LensExt},
  context::{Context, EmitContext},
  layout::Units::{Auto, Pixels, Stretch},
  modifiers::{LayoutModifiers, StyleModifiers, TextModifiers},
  style::FontWeightKeyword,
  view::Handle,
  views::{Label, Slider, VStack},
};

pub struct ParamSlider;

impl ParamSlider {
  pub fn new<'a, L, P, F, M, C>(
    cx: &'a mut Context,
    name: &'a str,
    lens: L,
    param_ptr: ParamPtr,
    params_to_param: F,
    on_change: C,
  ) -> Handle<'a, VStack>
  where
    L: 'static + Lens + Copy + Send + Sync,
    <L as Lens>::Source: 'static,
    P: Param,
    F: 'static + Fn(&<L as Lens>::Target) -> &P + Copy + Send + Sync,
    M: Any + Send,
    C: 'static + Fn(ParamPtr, f32) -> M + Copy + Send + Sync,
  {
    VStack::new(cx, |cx| {
      Label::new(cx, name)
        .width(Pixels(56.))
        .font_size(13.0)
        .font_weight(FontWeightKeyword::SemiBold)
        .text_wrap(false)
        .child_space(Stretch(1.0));

      Slider::new(
        cx,
        lens.map(move |p| {
          params_to_param(p).preview_normalized(params_to_param(p).modulated_plain_value())
        }),
      )
      .on_changing(move |cx, val| cx.emit(on_change(param_ptr, val)))
      .height(Pixels(80.))
      .class("vertical");
    })
    .size(Auto)
    .child_space(Stretch(1.0))
    .row_between(Pixels(8.0))
  }
}
