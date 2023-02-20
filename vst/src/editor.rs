use crate::ui::plugin_gui;
use crate::ReverbParameters;
use std::sync::Arc;
use vizia::{prelude::WindowSize, Application, ParentWindow};
use vst::{editor::Editor, prelude::HostCallback};

pub struct ReverbEditor {
  pub params: Arc<ReverbParameters>,
  pub is_open: bool,
  pub host: Option<HostCallback>,
}

impl Editor for ReverbEditor {
  fn position(&self) -> (i32, i32) {
    (0, 0)
  }

  fn size(&self) -> (i32, i32) {
    (520, 260)
  }

  fn open(&mut self, parent: *mut ::std::ffi::c_void) -> bool {
    if self.is_open {
      return false;
    }

    self.is_open = true;

    let host = self.host;
    let params = self.params.clone();

    Application::new(move |cx| plugin_gui(cx, Arc::clone(&params), host))
      .title("Dm-Reverb")
      .inner_size(WindowSize {
        width: 520,
        height: 260,
      })
      .open_parented(&ParentWindow(parent));

    true
  }

  fn is_open(&mut self) -> bool {
    self.is_open
  }

  fn close(&mut self) {
    self.is_open = false;
  }
}
