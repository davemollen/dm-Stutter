use crate::ReverbParameters;
use reverb::{MAX_SIZE, MIN_SIZE};
use std::sync::Arc;
use vizia::{
  prelude::{
    Context, EmitContext, Event, EventContext, LayoutModifiers, Lens, LensExt, StyleModifiers,
    Units::{Pixels, Stretch},
  },
  state::{Binding, Model, Wrapper},
  style::Color,
  views::{HStack, Knob, Label, VStack},
};
use vst::{host::Host, prelude::HostCallback};

const STYLE: &str = include_str!("./ui/style.css");

#[derive(Lens)]
pub struct UiData {
  params: Arc<ReverbParameters>,
}

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

impl Model for UiData {
  fn event(&mut self, _: &mut EventContext, event: &mut Event) {
    event.map(|app_event, _| match app_event {
      ParamChangeEvent::SetPredelay(value) => {
        let predelay = value.powf(3.) * 493. + 7.;
        self.params.predelay.set(predelay);
      }
      ParamChangeEvent::SetSize(value) => {
        let size = value.powf(2.) * (MAX_SIZE - MIN_SIZE) + MIN_SIZE;
        self.params.size.set(size);
      }
      ParamChangeEvent::SetSpeed(value) => {
        let speed = value.powf(3.) * 49.99 + 0.01;
        self.params.speed.set(speed);
      }
      ParamChangeEvent::SetDepth(value) => {
        self.params.depth.set(*value);
      }
      ParamChangeEvent::SetAbsorb(value) => {
        self.params.absorb.set(*value);
      }
      ParamChangeEvent::SetDecay(value) => {
        let decay = value * 1.2;
        self.params.decay.set(decay);
      }
      ParamChangeEvent::SetTilt(value) => {
        self.params.tilt.set(*value);
      }
      ParamChangeEvent::SetMix(value) => {
        self.params.mix.set(*value);
      }
    });
  }
}

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

pub fn plugin_gui(cx: &mut Context, params: Arc<ReverbParameters>, host: Option<HostCallback>) {
  cx.add_theme(STYLE);

  UiData {
    params: params.clone(),
  }
  .build(cx);

  HStack::new(cx, |cx| {
    VStack::new(cx, |cx| {
      Label::new(cx, "Predelay");
      Binding::new(cx, UiData::params, move |cx, params| {
        Knob::new(
          cx,
          0.,
          params.map(move |params| ((params.predelay.get() - 7.) / 493.).powf(0.333333)),
          false,
        )
        .on_changing(move |cx, val| {
          cx.emit(ParamChangeEvent::SetPredelay(val));
          notify_host_parameter_changed(val, 0, host);
        });
        Label::new(cx, params.map(move |params| params.predelay.get()));
      });

      Label::new(cx, "Size");
      Binding::new(cx, UiData::params, move |cx, params| {
        Knob::new(
          cx,
          ((40. - MIN_SIZE) / (MAX_SIZE - MIN_SIZE)).powf(0.5),
          params
            .map(move |params| ((params.size.get() - MIN_SIZE) / (MAX_SIZE - MIN_SIZE)).powf(0.5)),
          false,
        )
        .on_changing(move |cx, val| {
          cx.emit(ParamChangeEvent::SetSize(val));
          notify_host_parameter_changed(val, 1, host);
        });
        Label::new(cx, params.map(move |params| params.size.get()));
      });
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      Label::new(cx, "Speed");
      Binding::new(cx, UiData::params, move |cx, params| {
        Knob::new(
          cx,
          1.,
          params.map(move |params| ((params.speed.get() - 0.01) / 49.99).powf(0.333333)),
          false,
        )
        .on_changing(move |cx, val| {
          cx.emit(ParamChangeEvent::SetSpeed(val));
          notify_host_parameter_changed(val, 2, host);
        });
        Label::new(cx, params.map(move |params| params.speed.get()));
      });

      Label::new(cx, "Depth");
      Binding::new(cx, UiData::params, move |cx, params| {
        Knob::new(cx, 0., params.map(move |params| params.depth.get()), false).on_changing(
          move |cx, val| {
            cx.emit(ParamChangeEvent::SetDepth(val));
            notify_host_parameter_changed(val, 3, host);
          },
        );
        Label::new(cx, params.map(move |params| params.depth.get() * 100.));
      });
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      Label::new(cx, "Absorb");
      Binding::new(cx, UiData::params, move |cx, params| {
        Knob::new(cx, 1., params.map(move |params| params.absorb.get()), false).on_changing(
          move |cx, val| {
            cx.emit(ParamChangeEvent::SetAbsorb(val));
            notify_host_parameter_changed(val, 4, host);
          },
        );
        Label::new(cx, params.map(move |params| params.absorb.get() * 100.));
      });

      Label::new(cx, "Decay");
      Binding::new(cx, UiData::params, move |cx, params| {
        Knob::new(
          cx,
          0.,
          params.map(move |params| params.decay.get() / 1.2),
          false,
        )
        .on_changing(move |cx, val| {
          cx.emit(ParamChangeEvent::SetDecay(val));
          notify_host_parameter_changed(val, 5, host);
        });
        Label::new(cx, params.map(move |params| params.decay.get() * 100.));
      });
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));

    VStack::new(cx, |cx| {
      Label::new(cx, "Tilt");
      Binding::new(cx, UiData::params, move |cx, params| {
        Knob::new(cx, 1., params.map(move |params| params.tilt.get()), false).on_changing(
          move |cx, val| {
            cx.emit(ParamChangeEvent::SetTilt(val));
            notify_host_parameter_changed(val, 6, host);
          },
        );
        Label::new(
          cx,
          params.map(move |params| params.tilt.get() * 200. - 100.),
        );
      });

      Label::new(cx, "Mix");
      Binding::new(cx, UiData::params, move |cx, params| {
        Knob::new(cx, 0., params.map(move |params| params.mix.get()), false).on_changing(
          move |cx, val| {
            cx.emit(ParamChangeEvent::SetMix(val));
            notify_host_parameter_changed(val, 7, host);
          },
        );
        Label::new(cx, params.map(move |params| params.mix.get() * 100.));
      });
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(10.0));
  })
  .background_color(Color::rgb(80, 80, 80));
}
