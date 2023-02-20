mod reverb_parameters;
mod ui;
use reverb_parameters::ReverbParameters;
use std::sync::Arc;
use ui::plugin_gui;
use vizia::prelude::*;

fn main() {
  let params = Arc::new(ReverbParameters::default());

  Application::new(move |cx| plugin_gui(cx, Arc::clone(&params), None))
    .title("DM Reverb")
    .inner_size((520, 260))
    .run();
}
