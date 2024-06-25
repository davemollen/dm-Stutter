#[path = "./editor/components/param_knob.rs"]
mod param_knob;
use param_knob::{ParamKnob, ParamKnobSize};
#[path = "./editor/components/param_checkbox.rs"]
mod param_checkbox;
use param_checkbox::ParamCheckbox;
#[path = "./editor/components/param_slider.rs"]
mod param_slider;
use param_slider::ParamSlider;
mod ui_data;
use nih_plug::params::Param;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::{
  binding::LensExt,
  model::Model,
  modifiers::{LayoutModifiers, StyleModifiers, TextModifiers},
  prelude::Units::{Pixels, Stretch},
  style::FontWeightKeyword,
  views::{Element, HStack, Label, VStack},
};
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;
use ui_data::{ParamChangeEvent, UiData};

use crate::stutter_parameters::StutterParameters;

const STYLE: &str = include_str!("./editor/style.css");

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
  ViziaState::new(|| (596, 424))
}

pub(crate) fn create(
  params: Arc<StutterParameters>,
  editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
  create_vizia_editor(
    editor_state,
    ViziaTheming::Custom,
    move |cx, gui_context| {
      let _ = cx.add_stylesheet(STYLE);

      UiData {
        params: params.clone(),
        gui_context: gui_context.clone(),
      }
      .build(cx);

      HStack::new(cx, |cx| {
        VStack::new(cx, |cx| {
          HStack::new(cx, |cx| {
            ParamCheckbox::new(
              cx,
              params.on.name(),
              UiData::params,
              params.on.as_ptr(),
              |params| &params.on,
              |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            )
            .width(Pixels(56.0))
            .height(Pixels(92.0));

            ParamCheckbox::new(
              cx,
              params.auto.name(),
              UiData::params,
              params.auto.as_ptr(),
              |params| &params.auto,
              |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            )
            .width(Pixels(56.0))
            .height(Pixels(92.0));

            ParamCheckbox::new(
              cx,
              params.trigger.name(),
              UiData::params,
              params.trigger.as_ptr(),
              |params| &params.trigger,
              |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            )
            .disabled(UiData::params.map(|params| params.auto.value()))
            .width(Pixels(56.0))
            .height(Pixels(92.0));
          })
          .child_space(Stretch(1.0));

          HStack::new(cx, |cx| {
            ParamKnob::new(
              cx,
              params.pulse.name(),
              UiData::params,
              params.pulse.as_ptr(),
              |params| &params.pulse,
              |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              ParamKnobSize::Regular,
            );

            ParamKnob::new(
              cx,
              params.chance.name(),
              UiData::params,
              params.chance.as_ptr(),
              |params| &params.chance,
              |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              ParamKnobSize::Regular,
            )
            .disabled(UiData::params.map(|params| !params.auto.value()));

            ParamKnob::new(
              cx,
              params.duration.name(),
              UiData::params,
              params.duration.as_ptr(),
              |params| &params.duration,
              |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              ParamKnobSize::Regular,
            )
            .disabled(UiData::params.map(|params| !params.auto.value()));
          })
          .child_space(Stretch(1.0));

          HStack::new(cx, |cx| {
            Label::new(cx, "Stutter")
              .font_size(22.0)
              .font_weight(FontWeightKeyword::Bold)
              .border_radius(Pixels(16.0))
              .border_width(Pixels(1.))
              .border_color("#2c5494")
              .background_color("#3c6ab5")
              .child_space(Stretch(1.0))
              .child_top(Pixels(1.0))
              .child_bottom(Pixels(5.0))
              .width(Pixels(112.0))
              .top(Pixels(16.0));
          })
          .child_space(Stretch(1.0));
        })
        .col_between(Pixels(40.))
        .top(Stretch(1.0));

        HStack::new(cx, |cx| {
          Element::new(cx)
            .width(Pixels(3.0))
            .top(Pixels(32.0))
            .height(Stretch(1.0))
            .right(Pixels(8.0))
            .background_color("#363636");

          VStack::new(cx, |cx| {
            Label::new(cx, "Note probability")
              .font_size(16.0)
              .font_weight(FontWeightKeyword::SemiBold)
              .left(Stretch(1.0))
              .bottom(Pixels(8.0));

            HStack::new(cx, |cx| {
              ParamSlider::new(
                cx,
                params.half_notes.name(),
                UiData::params,
                params.half_notes.as_ptr(),
                |params| &params.half_notes,
                |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              );
              ParamSlider::new(
                cx,
                params.seven_sixteenth_notes.name(),
                UiData::params,
                params.seven_sixteenth_notes.as_ptr(),
                |params| &params.seven_sixteenth_notes,
                |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              );
              ParamSlider::new(
                cx,
                params.six_sixteenth_notes.name(),
                UiData::params,
                params.six_sixteenth_notes.as_ptr(),
                |params| &params.six_sixteenth_notes,
                |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              );
              ParamSlider::new(
                cx,
                params.half_triplet_notes.name(),
                UiData::params,
                params.half_triplet_notes.as_ptr(),
                |params| &params.half_triplet_notes,
                |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              );
              ParamSlider::new(
                cx,
                params.five_sixteenth_notes.name(),
                UiData::params,
                params.five_sixteenth_notes.as_ptr(),
                |params| &params.five_sixteenth_notes,
                |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              );
            });

            HStack::new(cx, |cx| {
              ParamSlider::new(
                cx,
                params.quarter_notes.name(),
                UiData::params,
                params.quarter_notes.as_ptr(),
                |params| &params.quarter_notes,
                |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              );
              ParamSlider::new(
                cx,
                params.three_sixteenth_notes.name(),
                UiData::params,
                params.three_sixteenth_notes.as_ptr(),
                |params| &params.three_sixteenth_notes,
                |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              );
              ParamSlider::new(
                cx,
                params.quarter_triplet_notes.name(),
                UiData::params,
                params.quarter_triplet_notes.as_ptr(),
                |params| &params.quarter_triplet_notes,
                |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              );
              ParamSlider::new(
                cx,
                params.eighth_notes.name(),
                UiData::params,
                params.eighth_notes.as_ptr(),
                |params| &params.eighth_notes,
                |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              );
              ParamSlider::new(
                cx,
                params.eighth_triplet_notes.name(),
                UiData::params,
                params.eighth_triplet_notes.as_ptr(),
                |params| &params.eighth_triplet_notes,
                |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              );
            });

            HStack::new(cx, |cx| {
              ParamSlider::new(
                cx,
                params.sixteenth_notes.name(),
                UiData::params,
                params.sixteenth_notes.as_ptr(),
                |params| &params.sixteenth_notes,
                |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              );
              ParamSlider::new(
                cx,
                params.sixteenth_triplet_notes.name(),
                UiData::params,
                params.sixteenth_triplet_notes.as_ptr(),
                |params| &params.sixteenth_triplet_notes,
                |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              );
              ParamSlider::new(
                cx,
                params.thirty_second_notes.name(),
                UiData::params,
                params.thirty_second_notes.as_ptr(),
                |params| &params.thirty_second_notes,
                |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              );
              ParamSlider::new(
                cx,
                params.thirty_second_triplet_notes.name(),
                UiData::params,
                params.thirty_second_triplet_notes.as_ptr(),
                |params| &params.thirty_second_triplet_notes,
                |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              );
              ParamSlider::new(
                cx,
                params.sixty_fourth_notes.name(),
                UiData::params,
                params.sixty_fourth_notes.as_ptr(),
                |params| &params.sixty_fourth_notes,
                |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              );
            });
          });
        });
      })
      .child_space(Pixels(16.0))
      .background_color("#505050");
    },
  )
}
