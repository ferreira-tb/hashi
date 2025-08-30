use futures::channel::oneshot;
use js_sys::Function;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use std::u32;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_name = "setTimeout")]
  pub fn set_timeout(handler: &Function, timeout: u32) -> JsValue;

  #[wasm_bindgen(js_name = "setInterval")]
  pub fn set_interval(handler: &Function, timeout: u32) -> JsValue;

  #[wasm_bindgen(js_name = "clearTimeout")]
  pub fn clear_timeout(handle: JsValue) -> JsValue;

  #[wasm_bindgen(js_name = "clearInterval")]
  pub fn clear_interval(handle: JsValue) -> JsValue;
}

#[must_use]
pub struct Timeout {
  id: Option<JsValue>,
  closure: Option<Closure<dyn FnMut()>>,
}

impl Timeout {
  pub fn new<F>(duration: Duration, callback: F) -> Timeout
  where
    F: FnOnce() + 'static,
  {
    let closure = Closure::once(callback);
    let closure_ref = closure
      .as_ref()
      .unchecked_ref::<js_sys::Function>();

    let duration = duration.as_millis();
    let timeout = u32::try_from(duration).unwrap_or(u32::MAX);
    let id = set_timeout(closure_ref, timeout);

    Timeout { id: Some(id), closure: Some(closure) }
  }

  pub fn forget(mut self) -> JsValue {
    let id = self.id.take().unwrap_throw();
    self.closure.take().unwrap_throw().forget();
    id
  }

  pub fn cancel(mut self) -> Closure<dyn FnMut()> {
    self.closure.take().unwrap_throw()
  }
}

impl Drop for Timeout {
  fn drop(&mut self) {
    if let Some(id) = self.id.take() {
      clear_timeout(id);
    }
  }
}

#[must_use]
pub struct TimeoutFuture {
  _inner: Timeout,
  rx: oneshot::Receiver<()>,
}

impl TimeoutFuture {
  pub fn new(duration: Duration) -> TimeoutFuture {
    let (tx, rx) = oneshot::channel();
    let inner = Timeout::new(duration, move || {
      tx.send(()).unwrap_throw();
    });

    TimeoutFuture { _inner: inner, rx }
  }
}

impl Future for TimeoutFuture {
  type Output = ();

  fn poll(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
    Future::poll(Pin::new(&mut self.rx), ctx).map(|it| it.unwrap_throw())
  }
}

pub async fn sleep(duration: Duration) {
  TimeoutFuture::new(duration).await;
}
