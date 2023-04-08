use super::{ParamChangeEvent, UiData};
use crate::reverb_parameters::ReverbParameters;
use nih_plug::prelude::{Param, ParamPtr};
use std::sync::Arc;
use vizia::{
  prelude::{ActionModifiers, Context, EmitContext, LensExt},
  state::Binding,
  views::{Checkbox, Label},
};

pub struct ParamCheckbox {}

impl ParamCheckbox {
  pub fn new<P, F>(cx: &mut Context, param_ptr: ParamPtr, params_to_param: F)
  where
    P: Param<Plain = bool>,
    F: 'static + Fn(&Arc<ReverbParameters>) -> &P + Copy + Send + Sync,
  {
    Binding::new(cx, UiData::params, move |cx, params| {
      Label::new(cx, unsafe { param_ptr.name() });
      Checkbox::new(cx, {
        params.map(move |params| params_to_param(params).modulated_plain_value())
      })
      .on_press(move |cx| {
        let is_checked = params
          .map(move |params| params_to_param(params).modulated_plain_value())
          .get(cx);
        let val = if is_checked { 0. } else { 1. };

        cx.emit(ParamChangeEvent::SetParam(param_ptr, val));
      });
    });
  }
}
