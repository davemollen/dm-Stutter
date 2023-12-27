use crate::reverb_parameters::ReverbParameters;
use std::sync::Arc;
use vizia::{
  prelude::{
    Context, LayoutModifiers, StyleModifiers,
    Units::{Pixels, Stretch}, Weight,
  },
  state::Model,
  views::{HStack, VStack, Label}, modifiers::TextModifiers,
};
use vst::prelude::HostCallback;
#[path="./components/param_knob.rs"]
mod param_knob;
use param_knob::ParamKnob;
#[path="./components/param_checkbox.rs"]
mod param_checkbox;
use param_checkbox::ParamCheckbox;
#[path="ui_data.rs"]
mod ui_data;
pub use ui_data::{ParamChangeEvent, UiData};

const STYLE: &str = include_str!("style.css");

pub fn plugin_gui(cx: &mut Context, params: Arc<ReverbParameters>, host: Option<HostCallback>) {
  cx.add_theme(STYLE);

  UiData {
    params: params.clone(),
    host,
  }
  .build(cx);

  VStack::new(cx, |cx| {
    HStack::new(cx, |cx| {
      ParamCheckbox::new(
        cx,
        params.reverse.name,
        UiData::params,
        |params| &params.reverse,
        |val| ParamChangeEvent::SetReverse(val),
      )
      .child_space(Stretch(1.0))
      .row_between(Pixels(8.0));

      VStack::new(cx, |cx| {
        ParamKnob::new(
          cx,
          params.predelay.name,
          UiData::params,
          |params| &params.predelay,
          |val| ParamChangeEvent::SetPredelay(val),
        );
        ParamKnob::new(
          cx,
          params.size.name,
          UiData::params,
          |params| &params.size,
          |val| ParamChangeEvent::SetSize(val),
        );
      })
      .child_space(Stretch(1.0))
      .row_between(Pixels(8.0));

      VStack::new(cx, |cx| {
        ParamKnob::new(
          cx,
          params.speed.name,
          UiData::params,
          |params| &params.speed,
          |val| ParamChangeEvent::SetSpeed(val),
        );
        ParamKnob::new(
          cx,
          params.depth.name,
          UiData::params,
          |params| &params.depth,
          |val| ParamChangeEvent::SetDepth(val),
        );
      })
      .child_space(Stretch(1.0))
      .row_between(Pixels(8.0));

      VStack::new(cx, |cx| {
        ParamKnob::new(
          cx,
          params.absorb.name,
          UiData::params,
          |params| &params.absorb,
          |val| ParamChangeEvent::SetAbsorb(val),
        );
        ParamKnob::new(
          cx,
          params.decay.name,
          UiData::params,
          |params| &params.decay,
          |val| ParamChangeEvent::SetDecay(val),
        );
      })
      .child_space(Stretch(1.0))
      .row_between(Pixels(8.0));

      VStack::new(cx, |cx| {
        ParamKnob::new(
          cx,
          params.tilt.name,
          UiData::params,
          |params| &params.tilt,
          |val| ParamChangeEvent::SetTilt(val),
        );
        ParamKnob::new(
          cx,
          params.shimmer.name,
          UiData::params,
          |params| &params.shimmer,
          |val| ParamChangeEvent::SetShimmer(val),
        );
      })
      .child_space(Stretch(1.0))
      .row_between(Pixels(8.0));

      
      ParamKnob::new(
        cx,
        params.mix.name,
        UiData::params,
        |params| &params.mix,
        |val| ParamChangeEvent::SetMix(val),
      )
      .child_space(Stretch(1.0))
      .top(Pixels(-128.0));
    }).top(Pixels(32.0));

    Label::new(cx, "dm-Reverb")
        .font_size(22.0)
        .font_weight(Weight::BOLD)
        .border_radius(Pixels(16.0))
        .border_width(Pixels(1.))
        .border_color("#2c5494")
        .background_color("#3c6ab5")
        .child_space(Stretch(1.0))
        .child_top(Pixels(1.0))
        .child_bottom(Pixels(5.0))
        .width(Pixels(144.0))
        .left(Stretch(1.0));
  })
  .child_space(Pixels(16.0))
  .background_color("#505050");
}
