pub mod mouse;
pub mod observer;
pub mod query;
pub mod ready;

use wasm_bindgen::prelude::*;
use web_sys::Element;

pub fn document() -> web_sys::Document {
  super::window().document().unwrap_throw()
}

pub fn head() -> web_sys::HtmlHeadElement {
  document().head().unwrap_throw()
}

pub fn body() -> web_sys::HtmlElement {
  document().body().unwrap_throw()
}

pub fn document_element() -> Element {
  document().document_element().unwrap_throw()
}
