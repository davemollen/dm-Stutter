mod reverb_parameters;
use reverb_parameters::ReverbParameters;
use std::sync::Arc;
use vizia::prelude::*;
mod editor;
use editor::plugin_gui;

fn main() {
  let params = Arc::new(ReverbParameters::default());

  Application::new(move |cx| plugin_gui(cx, Arc::clone(&params), None))
    .title("DM Reverb")
    .inner_size((520, 260))
    .run();
}
