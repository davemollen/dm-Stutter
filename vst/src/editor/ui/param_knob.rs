use super::{ParamChangeEvent, UiData};
use crate::reverb_parameters::{FloatParam, ReverbParameters};
use std::sync::Arc;
use vizia::{
  prelude::{Context, EmitContext, LensExt},
  state::Binding,
  views::{Knob, Label},
};
use vst::{host::Host, prelude::HostCallback};

fn notify_host_parameter_changed(val: f32, index: i32, host: Option<HostCallback>) {
  match host {
    Some(host) => {
      host.begin_edit(index);
      host.automate(index, val);
      host.end_edit(index);
    }
    None => {}
  }
}

pub struct ParamKnob {}

impl ParamKnob {
  pub fn new<F>(
    cx: &mut Context,
    param: &FloatParam,
    params_to_param: F,
    host: Option<HostCallback>,
  ) where
    F: 'static + Fn(&Arc<ReverbParameters>) -> &FloatParam + Copy,
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
        cx.emit(ParamChangeEvent::SetParam(index, val));
        notify_host_parameter_changed(val, index, host);
      });

      Label::new(
        cx,
        params.map(move |params| params_to_param(params).get_display_value(true)),
      );
    })
  }
}
