use crate::ReverbParameters;
use nih_plug::prelude::{GuiContext, Param, ParamPtr};
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
mod param_knob;
use param_knob::ParamKnob;

const STYLE: &str = include_str!("./ui/style.css");

#[derive(Lens)]
pub struct UiData {
  params: Arc<ReverbParameters>,
  gui_context: Arc<dyn GuiContext>,
}

pub enum ParamChangeEvent {
  SetParam(ParamPtr, f32),
}

impl Model for UiData {
  fn event(&mut self, _: &mut EventContext, event: &mut Event) {
    event.map(|app_event, _| match app_event {
      ParamChangeEvent::SetParam(param_ptr, new_value) => {
        unsafe {
          self
            .gui_context
            .raw_set_parameter_normalized(*param_ptr, *new_value)
        };
      }
    });
  }
}

pub fn plugin_gui(
  cx: &mut Context,
  params: Arc<ReverbParameters>,
  gui_context: Arc<dyn GuiContext>,
) {
  cx.add_theme(STYLE);

  UiData {
    params: params.clone(),
    gui_context: gui_context.clone(),
  }
  .build(cx);

  HStack::new(cx, |cx| {
    VStack::new(cx, |cx| {
      ParamKnob::new(cx, params.predelay.as_ptr(), |params| &params.predelay);
      ParamKnob::new(cx, params.size.as_ptr(), |params| &params.size);
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(cx, params.speed.as_ptr(), |params| &params.speed);
      ParamKnob::new(cx, params.depth.as_ptr(), |params| &params.depth);
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(cx, params.absorb.as_ptr(), |params| &params.absorb);
      ParamKnob::new(cx, params.decay.as_ptr(), |params| &params.decay);
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      ParamKnob::new(cx, params.tilt.as_ptr(), |params| &params.tilt);
      ParamKnob::new(cx, params.mix.as_ptr(), |params| &params.mix);
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));
  })
  .background_color(Color::rgb(80, 80, 80));
}
