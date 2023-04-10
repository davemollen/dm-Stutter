use std::sync::Arc;

use crate::reverb_parameters::ReverbParameters;

use super::{ParamChangeEvent, UiData};
use nih_plug::prelude::{Param, ParamPtr};
use vizia::{
  prelude::{ActionModifiers, Context, EmitContext, LensExt, StyleModifiers},
  state::Binding,
  views::{Knob, Label, TextEvent, Textbox},
};

pub struct ParamKnob {}

impl ParamKnob {
  pub fn new<P, F>(cx: &mut Context, param_ptr: ParamPtr, params_to_param: F)
  where
    P: Param,
    F: 'static + Fn(&Arc<ReverbParameters>) -> &P + Copy + Send + Sync,
  {
    Label::new(cx, unsafe { param_ptr.name() });

    Binding::new(cx, UiData::params, move |cx, params| {
      Knob::new(
        cx,
        params.map(move |params| params_to_param(params).default_normalized_value()),
        params.map(move |params| {
          params_to_param(params)
            .preview_normalized(params_to_param(params).modulated_plain_value())
        }),
        false,
      )
      .on_changing(move |cx, val| {
        cx.emit(ParamChangeEvent::SetParam(param_ptr, val));
      });

      Textbox::new(
        cx,
        params.map(move |params| {
          params_to_param(params)
            .normalized_value_to_string(params_to_param(params).modulated_normalized_value(), true)
        }),
      )
      .on_mouse_down(|cx, _| {
        cx.emit(TextEvent::StartEdit);
        cx.emit(TextEvent::ResetText("".to_string()));
      })
      .on_submit(move |cx, text, success| {
        if success {
          let normalized_value =
            params.map(move |params| params_to_param(params).string_to_normalized_value(&text));
          match normalized_value.get(cx) {
            Some(val) => cx.emit(ParamChangeEvent::SetParam(param_ptr, val)),
            _ => (),
          };
        }
      })
      .class("align_center");
    })
  }
}
