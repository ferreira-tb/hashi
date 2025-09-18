use crate::dom::query::{query, wait_element};
use crate::window;
use wasm_bindgen::prelude::*;
use web_sys::{Element, MouseEvent, MouseEventInit};

pub fn click_on(element: &Element) -> Result<bool, JsValue> {
  let init = MouseEventInit::new();
  init.set_bubbles(true);
  init.set_cancelable(true);
  init.set_view(Some(&window()));

  MouseEvent::new_with_mouse_event_init_dict("click", &init)
    .and_then(|event| element.dispatch_event(&event))
}

pub fn click(selector: &str) -> Result<bool, JsValue> {
  match query(selector) {
    Some(element) => click_on(&element),
    None => Ok(false),
  }
}

pub async fn wait_click(selector: &str, secs: u32) -> Result<bool, JsValue> {
  match wait_element(selector, secs).await {
    Some(element) => click_on(&element),
    None => Ok(false),
  }
}
