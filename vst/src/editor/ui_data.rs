use crate::reverb_parameters::{Params, ReverbParameters};
use std::sync::Arc;
use vizia::{
  prelude::{Event, EventContext, Lens},
  model::Model,
  binding::Wrapper
};
use vst::{host::Host, prelude::HostCallback};

fn notify_host_parameter_changed(index: i32, value: f32, host: Option<HostCallback>) {
  match host {
    Some(host) => {
      host.begin_edit(index);
      host.automate(index, value);
      host.end_edit(index);
    }
    None => {}
  }
}

pub enum ParamChangeEvent {
  SetReverse(bool),
  SetPredelay(f32),
  SetSize(f32),
  SetSpeed(f32),
  SetDepth(f32),
  SetAbsorb(f32),
  SetDecay(f32),
  SetTilt(f32),
  SetShimmer(f32),
  SetMix(f32),
}

#[derive(Lens)]
pub struct UiData {
  pub params: Arc<ReverbParameters>,
  pub host: Option<HostCallback>,
}

impl Model for UiData {
  fn event(&mut self, _: &mut EventContext, event: &mut Event) {
    event.map(|app_event, _| match app_event {
      ParamChangeEvent::SetReverse(value) => {
        let param = &self.params.reverse;
        param.set_plain_value(*value);
        notify_host_parameter_changed(
          param.index,
          param.preview_normalized_value(*value),
          self.host,
        );
      }
      ParamChangeEvent::SetPredelay(value) => {
        let param = &self.params.predelay;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }
      ParamChangeEvent::SetSize(value) => {
        let param = &self.params.size;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }
      ParamChangeEvent::SetSpeed(value) => {
        let param = &self.params.speed;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }
      ParamChangeEvent::SetDepth(value) => {
        let param = &self.params.depth;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }
      ParamChangeEvent::SetAbsorb(value) => {
        let param = &self.params.absorb;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }
      ParamChangeEvent::SetDecay(value) => {
        let param = &self.params.decay;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }
      ParamChangeEvent::SetTilt(value) => {
        let param = &self.params.tilt;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }
      ParamChangeEvent::SetShimmer(value) => {
        let param = &self.params.shimmer;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }
      ParamChangeEvent::SetMix(value) => {
        let param = &self.params.mix;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }
    });
  }
}
