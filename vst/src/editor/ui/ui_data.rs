use crate::reverb_parameters::{Params, ReverbParameters};
use std::sync::Arc;
use vizia::{
  prelude::{Event, EventContext, Lens},
  state::{Model, Wrapper},
};

pub enum ParamChangeEvent {
  SetParam(i32, f32),
}

#[derive(Lens)]
pub struct UiData {
  pub params: Arc<ReverbParameters>,
}

impl Model for UiData {
  fn event(&mut self, _: &mut EventContext, event: &mut Event) {
    event.map(|app_event, _| match app_event {
      ParamChangeEvent::SetParam(index, value) => {
        let param = self.params.get_param_by_index(*index);
        match param {
          Some(param) => param.set_plain_value(*value),
          None => (),
        }
      }
    });
  }
}
