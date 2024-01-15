use crate::reverb_parameters::ReverbParameters;
use std::sync::Arc;
use vizia::{
  prelude::{
    Context, LayoutModifiers, StyleModifiers,
    Units::{Pixels, Stretch}, LayoutType,
  },
  model::Model,
  views::{HStack, VStack, Label}, modifiers::TextModifiers, style::FontWeightKeyword, layout::Units::Auto,
};
use vst::prelude::HostCallback;
#[path="./components/param_knob.rs"]
mod param_knob;
use param_knob::{ParamKnob, ParamKnobSize};
#[path="./components/param_checkbox.rs"]
mod param_checkbox;
use param_checkbox::ParamCheckbox;
#[path="ui_data.rs"]
mod ui_data;
pub use ui_data::{ParamChangeEvent, UiData};

const STYLE: &str = include_str!("style.css");

pub fn plugin_gui(cx: &mut Context, params: Arc<ReverbParameters>, host: Option<HostCallback>) {
  let _ = cx.add_stylesheet(STYLE);

  UiData {
    params: params.clone(),
    host,
  }
  .build(cx);

  VStack::new(cx, |cx| {
    HStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        params.size.name,
        UiData::params,
        |params| &params.size,
        |val| ParamChangeEvent::SetSize(val),
        ParamKnobSize::Large
      ).top(Stretch(1.0)).bottom(Stretch(1.0));

      VStack::new(cx, |cx| {
        ParamKnob::new(
          cx,
          params.predelay.name,
          UiData::params,
          |params| &params.predelay,
          |val| ParamChangeEvent::SetPredelay(val),
          ParamKnobSize::Regular
        );
        ParamCheckbox::new(
          cx,
          params.reverse.name,
          UiData::params,
          |params| &params.reverse,
          |val| ParamChangeEvent::SetReverse(val),
        ).top(Pixels(4.)).left(Stretch(1.0)).right(Stretch(1.0)).height(Pixels(92.));
      }).child_space(Stretch(1.0)).left(Pixels(16.0));
      
      VStack::new(cx, |cx| {
        ParamKnob::new(
          cx,
          params.speed.name,
          UiData::params,
          |params| &params.speed,
          |val| ParamChangeEvent::SetSpeed(val),
          ParamKnobSize::Regular
        );
        ParamKnob::new(
          cx,
          params.depth.name,
          UiData::params,
          |params| &params.depth,
          |val| ParamChangeEvent::SetDepth(val),
          ParamKnobSize::Regular
        ).top(Pixels(4.));
      }).child_space(Stretch(1.0));
      
      VStack::new(cx, |cx| {
        ParamKnob::new(
          cx,
          params.absorb.name,
          UiData::params,
          |params| &params.absorb,
          |val| ParamChangeEvent::SetAbsorb(val),
          ParamKnobSize::Regular
        );
        ParamKnob::new(
          cx,
          params.decay.name,
          UiData::params,
          |params| &params.decay,
          |val| ParamChangeEvent::SetDecay(val),
          ParamKnobSize::Regular
        ).top(Pixels(4.));
      }).child_space(Stretch(1.0));

      VStack::new(cx, |cx| {
        ParamKnob::new(
          cx,
          params.tilt.name,
          UiData::params,
          |params| &params.tilt,
          |val| ParamChangeEvent::SetTilt(val),
          ParamKnobSize::Regular
        );
        ParamKnob::new(
          cx,
          params.shimmer.name,
          UiData::params,
          |params| &params.shimmer,
          |val| ParamChangeEvent::SetShimmer(val),
          ParamKnobSize::Regular
        ).top(Pixels(4.));
      }).child_space(Stretch(1.0));

      ParamKnob::new(
        cx,
        params.mix.name,
        UiData::params,
        |params| &params.mix,
        |val| ParamChangeEvent::SetMix(val),
        ParamKnobSize::Regular
      );
    });

    Label::new(cx, "dm-Reverb")
      .font_size(22.0)
      .font_weight(FontWeightKeyword::Bold)
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
