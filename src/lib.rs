#![feature(iterator_try_collect)]

pub mod dom;
pub mod location;
pub mod object;
pub mod prelude;
pub mod timer;
pub mod value;

pub use prelude::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response};

pub fn window() -> web_sys::Window {
  web_sys::window().unwrap_throw()
}

pub async fn fetch(request: &Request) -> Result<Response, JsValue> {
  let promise = window().fetch_with_request(request);
  let response = JsFuture::from(promise).await?;
  Ok(response.dyn_into().unwrap_throw())
}

pub fn stop() {
  window().stop().unwrap_throw();
}
