use nih_plug::prelude::{Param, ParamPtr};
use std::any::Any;
use vizia::{
  prelude::{ActionModifiers, Context, EmitContext, LensExt},
  state::{Binding, Data, Lens},
  views::{Checkbox, Label},
};

pub struct ParamCheckbox {}

impl ParamCheckbox {
  pub fn new<L, P, F, M, C>(
    cx: &mut Context,
    lens: L,
    param_ptr: ParamPtr,
    params_to_param: F,
    on_change: C,
  ) where
    L: 'static + Lens + Copy + Send + Sync,
    <L as Lens>::Source: 'static,
    <L as Lens>::Target: Data,
    P: Param<Plain = bool>,
    F: 'static + Fn(&<L as Lens>::Target) -> &P + Copy + Send + Sync,
    M: Any + Send,
    C: 'static + Fn(ParamPtr, f32) -> M + Copy + Send + Sync,
  {
    Binding::new(cx, lens, move |cx, params| {
      Label::new(cx, unsafe { param_ptr.name() });
      Checkbox::new(cx, {
        params.map(move |params| params_to_param(params).modulated_plain_value())
      })
      .on_press(move |cx| {
        let is_checked = params
          .map(move |params| params_to_param(params).modulated_normalized_value())
          .get(cx);

        cx.emit(on_change(param_ptr, is_checked));
      });
    });
  }
}
