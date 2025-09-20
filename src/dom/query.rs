use super::document;
use crate::iter::JsCastIter;
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

pub fn exists(selector: &str) -> bool {
  query(selector).is_some()
}

pub fn exists_in(element: &Element, selector: &str) -> bool {
  query_in(element, selector).is_some()
}

pub async fn wait_element(selector: &str, secs: u32) -> Option<Element> {
  wait_element_ms(selector, secs.saturating_mul(1000)).await
}

pub async fn wait_element_ms(selector: &str, ms: u32) -> Option<Element> {
  __wait_element_ms(ms, || query(selector)).await
}

pub async fn wait_element_in(element: &Element, selector: &str, secs: u32) -> Option<Element> {
  wait_element_ms_in(element, selector, secs.saturating_mul(1000)).await
}

pub async fn wait_element_ms_in(element: &Element, selector: &str, ms: u32) -> Option<Element> {
  __wait_element_ms(ms, || query_in(element, selector)).await
}

async fn __wait_element_ms<F>(ms: u32, f: F) -> Option<Element>
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

#[macro_export]
macro_rules! q {
  ($sel:expr) => {{ $crate::query($sel) }};
  ($sel:expr, $time:literal) => {{ $crate::wait_element($sel, $time) }};
  ($sel:expr, $time:literal ms) => {{ $crate::wait_element_ms($sel, $time) }};
  ($sel:expr, $time:literal millis) => {{ $crate::wait_element_ms($sel, $time) }};
  ($sel:expr, $time:literal s) => {{ $crate::wait_element($sel, $time) }};
  ($sel:expr, $time:literal secs) => {{ $crate::wait_element($sel, $time) }};
}

#[macro_export]
macro_rules! qi {
  ($element:expr, $sel:expr) => {{ $crate::query_in($element, $sel) }};
  ($element:expr, $sel:expr, $time:literal) => {{ $crate::wait_element_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal ms) => {{ $crate::wait_element_ms_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal millis) => {{ $crate::wait_element_ms_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal s) => {{ $crate::wait_element_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal secs) => {{ $crate::wait_element_in($element, $sel, $time) }};
}

#[macro_export]
macro_rules! e {
  ($sel:expr) => {{ $crate::exists($sel) }};
  ($sel:expr, $time:literal) => {{ $crate::wait_exists($sel, $time) }};
  ($sel:expr, $time:literal ms) => {{ $crate::wait_exists_ms($sel, $time) }};
  ($sel:expr, $time:literal millis) => {{ $crate::wait_exists_ms($sel, $time) }};
  ($sel:expr, $time:literal s) => {{ $crate::wait_exists($sel, $time) }};
  ($sel:expr, $time:literal secs) => {{ $crate::wait_exists($sel, $time) }};
}

#[macro_export]
macro_rules! ei {
  ($element:expr, $sel:expr) => {{ $crate::exists_in($element, $sel) }};
  ($element:expr, $sel:expr, $time:literal) => {{ $crate::wait_exists_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal ms) => {{ $crate::wait_exists_ms_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal millis) => {{ $crate::wait_exists_ms_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal s) => {{ $crate::wait_exists_in($element, $sel, $time) }};
  ($element:expr, $sel:expr, $time:literal secs) => {{ $crate::wait_exists_in($element, $sel, $time) }};
}
