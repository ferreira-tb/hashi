use crate::timer::sleep;
use js_sys::Date;
use std::time::Duration;
use wasm_bindgen::prelude::*;

pub fn document() -> web_sys::Document {
  super::window().document().unwrap_throw()
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

pub fn query_all(selector: &str) -> Vec<web_sys::Element> {
  let list = document()
    .query_selector_all(selector)
    .unwrap_throw();

  js_sys::try_iter(&list)
    .unwrap_throw()
    .unwrap_throw()
    .map(|value| value.map(JsCast::unchecked_into))
    .try_collect()
    .unwrap_throw()
}

pub async fn wait_element(selector: &str, secs: u32) -> Option<web_sys::Element> {
  let interval = Duration::from_millis(100);
  let timeout = Date::now() + (f64::from(secs) * 1000.0);

  loop {
    if let Some(element) = query(selector) {
      return Some(element);
    } else if Date::now() > timeout {
      return None;
    }

    sleep(interval).await;
  }
}
