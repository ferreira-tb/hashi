use serde::Serialize;
use serde_wasm_bindgen::to_value;
use web_sys::console;

pub trait Console: Serialize {
  fn print(&self) {
    if let Ok(value) = to_value(self) {
      console::log_1(&value);
    }
  }

  fn print_err(&self) {
    if let Ok(value) = to_value(self) {
      console::error_1(&value);
    }
  }
}

impl<T> Console for T where T: Serialize {}
