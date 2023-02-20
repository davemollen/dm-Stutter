use std::sync::Arc;

use crate::reverb_parameters::ReverbParameters;

use super::{ParamChangeEvent, UiData};
use nih_plug::prelude::{Param, ParamPtr};
use vizia::{
  prelude::{Context, EmitContext, LensExt},
  state::Binding,
  views::{Knob, Label},
};

pub struct ParamKnob {}

impl ParamKnob {
  pub fn new<P, F>(cx: &mut Context, param_ptr: ParamPtr, params_to_param: F)
  where
    P: Param,
    F: 'static + Fn(&Arc<ReverbParameters>) -> &P + Copy,
  {
    Label::new(cx, unsafe { param_ptr.name() });
    Binding::new(cx, UiData::params, move |cx, params| {
      Knob::new(
        cx,
        0.,
        params.map(move |params| {
          params_to_param(params)
            .preview_normalized(params_to_param(params).modulated_plain_value())
        }),
        false,
      )
      .on_changing(move |cx, val| {
        cx.emit(ParamChangeEvent::SetParam(param_ptr, val));
      });

      Label::new(
        cx,
        params.map(move |params| unsafe {
          param_ptr.normalized_value_to_string(
            params_to_param(params)
              .preview_normalized(params_to_param(params).modulated_plain_value()),
            true,
          )
        }),
      );
    })
  }
}
