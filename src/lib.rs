#![feature(iterator_try_collect)]

pub mod dom;
pub mod location;
pub mod timer;

use wasm_bindgen::prelude::*;

pub fn window() -> web_sys::Window {
  web_sys::window().unwrap_throw()
}
