#![feature(iterator_try_collect)]

pub mod console;
pub mod dom;
pub mod iter;
pub mod location;
pub mod prelude;
pub mod timer;
pub mod value;

pub use prelude::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response};

pub type JsResult<T = JsValue, E = JsValue> = Result<T, E>;

pub fn window() -> web_sys::Window {
  web_sys::window().unwrap_throw()
}

pub async fn fetch(request: &Request) -> JsResult<Response> {
  let promise = window().fetch_with_request(request);
  let response = JsFuture::from(promise).await?;
  response.dyn_into()
}
