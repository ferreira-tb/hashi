use crate::iter::JsCastIter;
use web_sys::Element;

pub async fn wait_element(selector: &str, secs: u32) -> Option<Element> {
  wait_element_ms(selector, secs.saturating_mul(1000)).await
}

pub async fn wait_element_ms(selector: &str, ms: u32) -> Option<Element> {
  internal::wait_element_ms(ms, || crate::query!(selector)).await
}

pub async fn wait_element_in(element: &Element, selector: &str, secs: u32) -> Option<Element> {
  wait_element_ms_in(element, selector, secs.saturating_mul(1000)).await
}

pub async fn wait_element_ms_in(element: &Element, selector: &str, ms: u32) -> Option<Element> {
  internal::wait_element_ms(ms, || crate::query_in!(element, selector)).await
}

pub async fn wait_elements(selector: &str, secs: u32) -> JsCastIter<Element> {
  wait_elements_ms(selector, secs.saturating_mul(1000)).await
}

pub async fn wait_elements_ms(selector: &str, ms: u32) -> JsCastIter<Element> {
  if wait_exists_ms(selector, ms).await {
    internal::query_all(selector)
  } else {
    JsCastIter::empty()
  }
}

pub async fn wait_elements_in(element: &Element, selector: &str, secs: u32) -> JsCastIter<Element> {
  wait_elements_ms_in(element, selector, secs.saturating_mul(1000)).await
}

pub async fn wait_elements_ms_in(
  element: &Element,
  selector: &str,
  ms: u32,
) -> JsCastIter<Element> {
  if wait_exists_ms_in(element, selector, ms).await {
    internal::query_all_in(element, selector)
  } else {
    JsCastIter::empty()
  }
}

pub async fn wait_exists(selector: &str, secs: u32) -> bool {
  wait_exists_ms(selector, secs.saturating_mul(1000)).await
}

pub async fn wait_exists_ms(selector: &str, ms: u32) -> bool {
  wait_element_ms(selector, ms).await.is_some()
}

pub async fn wait_exists_in(element: &Element, selector: &str, secs: u32) -> bool {
  wait_exists_ms_in(element, selector, secs.saturating_mul(1000)).await
}

pub async fn wait_exists_ms_in(element: &Element, selector: &str, ms: u32) -> bool {
  wait_element_ms_in(element, selector, ms)
    .await
    .is_some()
}

pub async fn wait_text(selector: &str, secs: u32) -> Option<String> {
  wait_text_ms(selector, secs.saturating_mul(1000)).await
}

pub async fn wait_text_ms(selector: &str, ms: u32) -> Option<String> {
  let element = wait_element_ms(selector, ms).await;
  internal::get_text(element.as_deref())
}

pub async fn wait_text_in(element: &Element, selector: &str, secs: u32) -> Option<String> {
  wait_text_ms_in(element, selector, secs.saturating_mul(1000)).await
}

pub async fn wait_text_ms_in(element: &Element, selector: &str, ms: u32) -> Option<String> {
  let element = &wait_element_ms_in(element, selector, ms).await;
  internal::get_text(element.as_deref())
}

#[macro_export]
macro_rules! query {
  ($sel:expr) => {{ $crate::dom::query::internal::query($sel) }};
  ($sel:expr, $time:literal) => {{ $crate::wait_element($sel, $time) }};
  ($sel:expr, $time:literal ms) => {{ $crate::wait_element_ms($sel, $time) }};
  ($sel:expr, $time:literal millis) => {{ $crate::wait_element_ms($sel, $time) }};
  ($sel:expr, $time:literal s) => {{ $crate::wait_element($sel, $time) }};
  ($sel:expr, $time:literal secs) => {{ $crate::wait_element($sel, $time) }};
}

#[macro_export]
macro_rules! query_in {
  ($element:expr, $sel:expr) => {{ $crate::dom::query::internal::query_in($element, $sel) }};
  ($element:expr, $sel:expr, $time:literal) => {{ $crate::wait_element_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal ms) => {{ $crate::wait_element_ms_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal millis) => {{ $crate::wait_element_ms_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal s) => {{ $crate::wait_element_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal secs) => {{ $crate::wait_element_in($element, $sel, $time) }};
}

#[macro_export]
macro_rules! query_all {
  ($sel:expr) => {{ $crate::dom::query::internal::query_all($sel) }};
  ($sel:expr, $time:literal) => {{ $crate::wait_elements($sel, $time) }};
  ($sel:expr, $time:literal ms) => {{ $crate::wait_elements_ms($sel, $time) }};
  ($sel:expr, $time:literal millis) => {{ $crate::wait_elements_ms($sel, $time) }};
  ($sel:expr, $time:literal s) => {{ $crate::wait_elements($sel, $time) }};
  ($sel:expr, $time:literal secs) => {{ $crate::wait_elements($sel, $time) }};
}

#[macro_export]
macro_rules! query_all_in {
  ($element:expr, $sel:expr) => {{ $crate::dom::query::internal::query_all_in($element, $sel) }};
  ($element:expr, $sel:expr, $time:literal) => {{ $crate::wait_elements_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal ms) => {{ $crate::wait_elements_ms_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal millis) => {{ $crate::wait_elements_ms_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal s) => {{ $crate::wait_elements_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal secs) => {{ $crate::wait_elements_in($element, $sel, $time) }};
}

#[macro_export]
macro_rules! exists {
  ($sel:expr) => {{ $crate::query!($sel).is_some() }};
  ($sel:expr, $time:literal) => {{ $crate::wait_exists($sel, $time) }};
  ($sel:expr, $time:literal ms) => {{ $crate::wait_exists_ms($sel, $time) }};
  ($sel:expr, $time:literal millis) => {{ $crate::wait_exists_ms($sel, $time) }};
  ($sel:expr, $time:literal s) => {{ $crate::wait_exists($sel, $time) }};
  ($sel:expr, $time:literal secs) => {{ $crate::wait_exists($sel, $time) }};
}

#[macro_export]
macro_rules! exists_in {
  ($element:expr, $sel:expr) => {{ $crate::query_in!($element, $sel).is_some() }};
  ($element:expr, $sel:expr, $time:literal) => {{ $crate::wait_exists_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal ms) => {{ $crate::wait_exists_ms_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal millis) => {{ $crate::wait_exists_ms_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal s) => {{ $crate::wait_exists_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal secs) => {{ $crate::wait_exists_in($element, $sel, $time) }};
}

#[macro_export]
macro_rules! text {
  ($sel:expr) => {{
    use $crate::dom::query::internal::get_text;
    get_text($crate::query!($sel).as_deref())
  }};
  ($sel:expr, $time:literal) => {{ $crate::wait_text($sel, $time) }};
  ($sel:expr, $time:literal ms) => {{ $crate::wait_text_ms($sel, $time) }};
  ($sel:expr, $time:literal millis) => {{ $crate::wait_text_ms($sel, $time) }};
  ($sel:expr, $time:literal s) => {{ $crate::wait_text($sel, $time) }};
  ($sel:expr, $time:literal secs) => {{ $crate::wait_text($sel, $time) }};
}

#[macro_export]
macro_rules! text_in {
  ($element:expr, $sel:expr) => {{
    use $crate::dom::query::internal::get_text;
    get_text($crate::query_in!($element, $sel).as_deref())
  }};
  ($element:expr, $sel:expr, $time:literal) => {{ $crate::wait_text_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal ms) => {{ $crate::wait_text_ms_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal millis) => {{ $crate::wait_text_ms_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal s) => {{ $crate::wait_text_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal secs) => {{ $crate::wait_text_in($element, $sel, $time) }};
}

pub mod prelude {
  pub use super::{
    wait_element,
    wait_element_in,
    wait_element_ms,
    wait_element_ms_in,
    wait_elements,
    wait_elements_in,
    wait_elements_ms,
    wait_elements_ms_in,
    wait_exists,
    wait_exists_in,
    wait_exists_ms,
    wait_exists_ms_in,
    wait_text,
    wait_text_in,
    wait_text_ms,
    wait_text_ms_in,
  };
  pub use crate::{exists, exists_in, query, query_all, query_all_in, query_in, text, text_in};
}

pub mod internal {
  use crate::dom::document;
  use crate::iter::JsCastIter;
  use crate::timer::sleep;
  use js_sys::Date;
  use std::time::Duration;
  use wasm_bindgen::prelude::*;
  use web_sys::{Element, Node};

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

  pub fn query_all(selector: &str) -> JsCastIter<Element> {
    let list = document()
      .query_selector_all(selector)
      .unwrap_throw();

    JsCastIter::new(&list)
  }

  pub fn query_all_in(element: &Element, selector: &str) -> JsCastIter<Element> {
    let list = element
      .query_selector_all(selector)
      .unwrap_throw();

    JsCastIter::new(&list)
  }

  pub(super) async fn wait_element_ms<F>(ms: u32, f: F) -> Option<Element>
  where
    F: Fn() -> Option<Element>,
  {
    let interval = Duration::from_millis(100);
    let timeout = Date::now() + f64::from(ms);

    loop {
      if let Some(element) = f() {
        return Some(element);
      } else if Date::now() > timeout {
        return None;
      }

      sleep(interval).await;
    }
  }

  pub fn get_text(node: Option<&Node>) -> Option<String> {
    node
      .and_then(Node::text_content)
      .filter(|text| !text.is_empty())
  }
}
