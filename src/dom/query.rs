use super::document;
use super::node::from_node_list;
use crate::timer::sleep;
use js_sys::Date;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use web_sys::Element;

pub fn query(selector: &str) -> Option<Element> {
  document()
    .query_selector(selector)
    .unwrap_throw()
}

pub fn query_in(element: &Element, selector: &str) -> Option<Element> {
  element
    .query_selector(selector)
    .unwrap_throw()
}

pub fn query_all(selector: &str) -> Vec<Element> {
  let list = document()
    .query_selector_all(selector)
    .unwrap_throw();

  from_node_list(&list).unwrap_or_default()
}

pub fn query_all_in(element: &Element, selector: &str) -> Vec<Element> {
  let list = element
    .query_selector_all(selector)
    .unwrap_throw();

  from_node_list(&list).unwrap_or_default()
}

pub async fn wait_element(selector: &str, secs: u32) -> Option<Element> {
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
