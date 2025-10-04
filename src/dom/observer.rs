use crate::JsResult;
use js_sys::Function;
use wasm_bindgen::prelude::*;
use web_sys::{MutationObserver, MutationRecord};

pub fn create_observer<F>(callback: F) -> JsResult<MutationObserver>
where
  F: Fn(Vec<MutationRecord>, MutationObserver) + 'static,
{
  type Callback = Closure<dyn Fn(Vec<MutationRecord>, MutationObserver)>;
  let closure = Callback::new(callback);
  let closure_ref = closure.as_ref().unchecked_ref::<Function>();
  let observer = MutationObserver::new(closure_ref)?;
  closure.forget();
  Ok(observer)
}
