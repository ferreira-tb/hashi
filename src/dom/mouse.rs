use crate::dom::query::wait_element_ms;
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

pub async fn wait_click(selector: &str, secs: u32) -> Result<bool, JsValue> {
  wait_click_ms(selector, secs.saturating_mul(1000)).await
}

pub async fn wait_click_ms(selector: &str, ms: u32) -> Result<bool, JsValue> {
  match wait_element_ms(selector, ms).await {
    Some(element) => click_on(&element),
    None => Ok(false),
  }
}

#[macro_export]
macro_rules! click {
  ($sel:expr) => {{
    match $crate::query!($sel) {
      Some(element) => $crate::click_on(&element),
      None => Ok(false),
    }
  }};
  ($sel:expr, $time:literal) => {{ $crate::wait_click($sel, $time) }};
  ($sel:expr, $time:literal ms) => {{ $crate::wait_click_ms($sel, $time) }};
  ($sel:expr, $time:literal millis) => {{ $crate::wait_click_ms($sel, $time) }};
  ($sel:expr, $time:literal s) => {{ $crate::wait_click($sel, $time) }};
  ($sel:expr, $time:literal secs) => {{ $crate::wait_click($sel, $time) }};
}

pub mod prelude {
  pub use super::{click_on, wait_click, wait_click_ms};
  pub use crate::click;
}
