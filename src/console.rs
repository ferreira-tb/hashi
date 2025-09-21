use serde::Serialize;
use serde_wasm_bindgen::to_value;
use web_sys::console;

pub trait Console: Serialize {
  fn print(&self) {
    print(self);
  }

  fn print_err(&self) {
    print_err(self);
  }
}

impl<T> Console for T where T: Serialize {}

pub fn print<T>(value: &T)
where
  T: Serialize + ?Sized,
{
  if let Ok(value) = to_value(value) {
    console::log_1(&value);
  }
}

pub fn print_err<T>(value: &T)
where
  T: Serialize + ?Sized,
{
  if let Ok(value) = to_value(value) {
    console::error_1(&value);
  }
}
