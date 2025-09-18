mod query;
mod ready;

use wasm_bindgen::prelude::*;
use web_sys::Element;

pub use query::{query, query_all, query_all_in, query_in, wait_element};
pub use ready::{is_ready, wait_ready};

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
