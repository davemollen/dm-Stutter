use super::{ParamChangeEvent, UiData};
use crate::reverb_parameters::{BoolParam, Params, ReverbParameters};
use std::sync::Arc;
use vizia::{
  prelude::{ActionModifiers, Context, EmitContext, LensExt},
  state::Binding,
  views::{Checkbox, Label},
};

pub struct ParamCheckbox {}

impl ParamCheckbox {
  pub fn new<F, C>(cx: &mut Context, param: &BoolParam, params_to_param: F, on_change: C)
  where
    F: 'static + Fn(&Arc<ReverbParameters>) -> &BoolParam + Copy + Send + Sync,
    C: 'static + Fn(bool) -> ParamChangeEvent + Copy + Send + Sync,
  {
    let name = param.name;

    Binding::new(cx, UiData::params, move |cx, params| {
      Label::new(cx, name);
      Checkbox::new(
        cx,
        params.map(move |params| params_to_param(params).get_value()),
      )
      .on_press(move |cx| {
        let is_checked = params
          .map(move |params| params_to_param(params).get_value())
          .get(cx);
        cx.emit(on_change(!is_checked));
      });
    });
  }
}
