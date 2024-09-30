use nih_plug::prelude::{Param, ParamPtr};
use nih_plug_vizia::vizia::{
  binding::Lens, layout::Units::Auto, modifiers::{StyleModifiers, TextModifiers}, prelude::{
    Context, EmitContext, LayoutModifiers, LensExt,
    Units::{Pixels, Stretch},
  }, style::FontWeightKeyword, view::Handle, views::{Button, Element, Label, VStack}
};
use std::any::Any;

pub struct ParamTrigger;

impl ParamTrigger {
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
    P: Param<Plain = bool>,
    F: 'static + Fn(&<L as Lens>::Target) -> &P + Copy + Send + Sync,
    M: Any + Send,
    C: 'static + Fn(ParamPtr, f32) -> M + Copy + Send + Sync,
  {
    VStack::new(cx, |cx| {
      Label::new(cx, name)
        .font_size(13.0)
        .font_weight(FontWeightKeyword::SemiBold)
        .text_wrap(true)
        .child_space(Stretch(1.0));

      Button::new(
        cx,
        move |cx| {
          let current_normalized_value = lens
            .map(move |params| params_to_param(params).modulated_normalized_value())
            .get(cx);

          cx.emit(on_change(param_ptr, 1. - current_normalized_value));
        },
        |cx| Element::new(cx),
      )
      .size(Pixels(29.0))
      .border_radius(Pixels(29.0));
    })
    .size(Auto)
    .child_left(Stretch(1.0))
    .child_right(Stretch(1.0))
    .row_between(Pixels(9.0))
  }
}
