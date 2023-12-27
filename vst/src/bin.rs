mod reverb_parameters;
use reverb_parameters::ReverbParameters;
use std::sync::Arc;
use vizia::prelude::*;
mod editor;
use editor::{plugin_gui, WINDOW_SIZE};

fn main() {
  let params = Arc::new(ReverbParameters::default());

  Application::new(move |cx| plugin_gui(cx, Arc::clone(&params), None))
    .title("dm-Reverb")
    .inner_size((WINDOW_SIZE.width, WINDOW_SIZE.height))
    .run();
}
