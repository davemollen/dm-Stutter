use super::{ParamChangeEvent, UiData};
use crate::reverb_parameters::{FloatParam, Params, ReverbParameters};
use std::sync::Arc;
use vizia::{
  prelude::{Context, EmitContext, LensExt},
  state::Binding,
  views::{Knob, Label},
};

pub struct ParamKnob {}

impl ParamKnob {
  pub fn new<F, C>(cx: &mut Context, param: &FloatParam, params_to_param: F, on_change: C)
  where
    F: 'static + Fn(&Arc<ReverbParameters>) -> &FloatParam + Copy,
    C: 'static + Fn(f32) -> ParamChangeEvent + Copy,
  {
    let index = param.index;
    Label::new(cx, param.name);

    Binding::new(cx, UiData::params, move |cx, params| {
      Knob::new(
        cx,
        params.map(move |params| params_to_param(params).get_default_normalized_value()),
        params.map(move |params| params_to_param(params).get_normalized_value()),
        false,
      )
      .on_changing(move |cx, val| {
        cx.emit(on_change(val));
      });

      Label::new(
        cx,
        params.map(move |params| params_to_param(params).get_display_value(true)),
      );
    })
  }
}
