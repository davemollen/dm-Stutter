use crate::reverb_parameters::{Params, ReverbParameters};
use std::sync::Arc;
use vizia::{
  prelude::{Event, EventContext, Lens},
  state::{Model, Wrapper},
};
use vst::{host::Host, prelude::HostCallback};

fn notify_host_parameter_changed(param: impl Params, host: Option<HostCallback>) {
  match host {
    Some(host) => {
      let index = param.get_index();

      host.begin_edit(index);
      host.automate(index, param.get_normalized_value());
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
  host: Option<HostCallback>,
}

impl Model for UiData {
  fn event(&mut self, _: &mut EventContext, event: &mut Event) {
    event.map(|app_event, _| match app_event {
      ParamChangeEvent::SetReverse(value) => {
        self.params.reverse.set_plain_value(*value);
        notify_host_parameter_changed(self.params.reverse, self.host);
      }
      ParamChangeEvent::SetPredelay(value) => {
        self.params.predelay.set_plain_value(*value);
        notify_host_parameter_changed(self.params.predelay, self.host);
      }
      ParamChangeEvent::SetSize(value) => {
        self.params.size.set_plain_value(*value);
        notify_host_parameter_changed(self.params.size, self.host);
      }
      ParamChangeEvent::SetSpeed(value) => {
        self.params.speed.set_plain_value(*value);
        notify_host_parameter_changed(self.params.speed, self.host);
      }
      ParamChangeEvent::SetDepth(value) => {
        self.params.depth.set_plain_value(*value);
        notify_host_parameter_changed(self.params.depth, self.host);
      }
      ParamChangeEvent::SetAbsorb(value) => {
        self.params.absorb.set_plain_value(*value);
        notify_host_parameter_changed(self.params.absorb, self.host);
      }
      ParamChangeEvent::SetDecay(value) => {
        self.params.decay.set_plain_value(*value);
        notify_host_parameter_changed(self.params.decay, self.host);
      }
      ParamChangeEvent::SetTilt(value) => {
        self.params.tilt.set_plain_value(*value);
        notify_host_parameter_changed(self.params.tilt, self.host);
      }
      ParamChangeEvent::SetShimmer(value) => {
        self.params.shimmer.set_plain_value(*value);
        notify_host_parameter_changed(self.params.shimmer, self.host);
      }
      ParamChangeEvent::SetMix(value) => {
        self.params.mix.set_plain_value(*value);
        notify_host_parameter_changed(self.params.mix, self.host);
      }
    });
  }
}
