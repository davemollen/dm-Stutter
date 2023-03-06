use crate::reverb_parameters::ReverbParameters;
use std::sync::Arc;
use vizia::{
  prelude::{Event, EventContext, Lens},
  state::{Model, Wrapper},
};

pub enum ParamChangeEvent {
  SetPredelay(f32),
  SetSize(f32),
  SetSpeed(f32),
  SetDepth(f32),
  SetAbsorb(f32),
  SetDecay(f32),
  SetTilt(f32),
  SetMix(f32),
}

#[derive(Lens)]
pub struct UiData {
  pub params: Arc<ReverbParameters>,
}

impl Model for UiData {
  fn event(&mut self, _: &mut EventContext, event: &mut Event) {
    event.map(|app_event, _| match app_event {
      ParamChangeEvent::SetPredelay(value) => {
        self.params.predelay.set_plain_value(*value);
      }
      ParamChangeEvent::SetSize(value) => {
        self.params.size.set_plain_value(*value);
      }
      ParamChangeEvent::SetSpeed(value) => {
        self.params.speed.set_plain_value(*value);
      }
      ParamChangeEvent::SetDepth(value) => {
        self.params.depth.set_plain_value(*value);
      }
      ParamChangeEvent::SetAbsorb(value) => {
        self.params.absorb.set_plain_value(*value);
      }
      ParamChangeEvent::SetDecay(value) => {
        self.params.decay.set_plain_value(*value);
      }
      ParamChangeEvent::SetTilt(value) => {
        self.params.tilt.set_plain_value(*value);
      }
      ParamChangeEvent::SetMix(value) => {
        self.params.mix.set_plain_value(*value);
      }
    });
  }
}
