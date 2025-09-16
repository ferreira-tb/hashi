use crate::window;
use futures::channel::oneshot;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use wasm_bindgen::prelude::*;

#[must_use]
pub struct Timeout {
  id: Option<i32>,
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
    let timeout = i32::try_from(duration).unwrap_throw();
    let id = window()
      .set_timeout_with_callback_and_timeout_and_arguments_0(closure_ref, timeout)
      .unwrap_throw();

    Timeout { id: Some(id), closure: Some(closure) }
  }

  pub fn forget(mut self) -> i32 {
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
    if let Some(id) = self.id {
      window().clear_timeout_with_handle(id);
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
