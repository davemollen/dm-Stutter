use crate::reverb_parameters::ReverbParameters;
use std::sync::Arc;
use vizia::{
  prelude::{
    Context, Event, EventContext, LayoutModifiers, Lens, StyleModifiers,
    Units::{Pixels, Stretch},
  },
  state::{Model, Wrapper},
  style::Color,
  views::{HStack, VStack},
};
use vst::prelude::HostCallback;
mod param_knob;
use param_knob::ParamKnob;

const STYLE: &str = include_str!("./ui/style.css");

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
  params: Arc<ReverbParameters>,
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

pub fn plugin_gui(cx: &mut Context, params: Arc<ReverbParameters>, host: Option<HostCallback>) {
  cx.add_theme(STYLE);

  UiData {
    params: params.clone(),
  }
  .build(cx);

  HStack::new(cx, |cx| {
    VStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        &params.predelay,
        |params| &params.predelay,
        |val| {
          ParamChangeEvent::SetPredelay(val);
        },
        host,
      );

      ParamKnob::new(
        cx,
        &params.size,
        |params| &params.size,
        |val| {
          ParamChangeEvent::SetSize(val);
        },
        host,
      );
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        &params.speed,
        |params| &params.speed,
        |val| {
          ParamChangeEvent::SetSpeed(val);
        },
        host,
      );

      ParamKnob::new(
        cx,
        &params.depth,
        |params| &params.depth,
        |val| {
          ParamChangeEvent::SetDepth(val);
        },
        host,
      );
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        &params.absorb,
        |params| &params.absorb,
        |val| {
          ParamChangeEvent::SetAbsorb(val);
        },
        host,
      );

      ParamKnob::new(
        cx,
        &params.decay,
        |params| &params.decay,
        |val| {
          ParamChangeEvent::SetDecay(val);
        },
        host,
      );
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        &params.tilt,
        |params| &params.tilt,
        |val| {
          ParamChangeEvent::SetTilt(val);
        },
        host,
      );

      ParamKnob::new(
        cx,
        &params.mix,
        |params| &params.mix,
        |val| {
          ParamChangeEvent::SetMix(val);
        },
        host,
      );
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));
  })
  .background_color(Color::rgb(80, 80, 80));
}
