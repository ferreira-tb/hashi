use crate::timer::sleep;
use std::time::{Duration, Instant};
use wasm_bindgen::prelude::*;

pub fn window() -> web_sys::Window {
  web_sys::window().unwrap_throw()
}

pub fn document() -> web_sys::Document {
  window().document().unwrap_throw()
}

pub fn head() -> web_sys::HtmlHeadElement {
  document().head().unwrap_throw()
}

pub fn body() -> web_sys::HtmlElement {
  document().body().unwrap_throw()
}

pub fn document_element() -> web_sys::Element {
  document().document_element().unwrap_throw()
}

pub fn query(selector: &str) -> Option<web_sys::Element> {
  document()
    .query_selector(selector)
    .unwrap_throw()
}

#[bon::builder]
pub async fn wait_element(
  #[builder(start_fn)] selector: &str,
  #[builder(default =  Duration::from_millis(100))] interval: Duration,
  #[builder(default =  Duration::from_secs(5))] timeout: Duration,
) -> Option<web_sys::Element> {
  let start = Instant::now();
  loop {
    if let Some(element) = query(selector) {
      return Some(element);
    } else if start.elapsed() >= timeout {
      return None;
    }

    sleep(interval).await;
  }
}
