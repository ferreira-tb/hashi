use crate::JsResult;
use num_traits::ToPrimitive;
use serde::de::DeserializeOwned;
use serde_wasm_bindgen::from_value;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

pub trait JsValueExt {
  fn as_i8(&self) -> Option<i8>;
  fn as_i16(&self) -> Option<i16>;
  fn as_i32(&self) -> Option<i32>;
  fn as_i64(&self) -> Option<i64>;

  fn as_u8(&self) -> Option<u8>;
  fn as_u16(&self) -> Option<u16>;
  fn as_u32(&self) -> Option<u32>;
  fn as_u64(&self) -> Option<u64>;

  fn as_str_arc(&self) -> Option<Arc<str>>;

  fn into_serde<T>(self) -> JsResult<T>
  where
    T: DeserializeOwned;
}

impl JsValueExt for JsValue {
  fn as_i8(&self) -> Option<i8> {
    self.as_f64().and_then(|it| it.to_i8())
  }

  fn as_i16(&self) -> Option<i16> {
    self.as_f64().and_then(|it| it.to_i16())
  }

  fn as_i32(&self) -> Option<i32> {
    self.as_f64().and_then(|it| it.to_i32())
  }

  fn as_i64(&self) -> Option<i64> {
    self.as_f64().and_then(|it| it.to_i64())
  }

  fn as_u8(&self) -> Option<u8> {
    self.as_f64().and_then(|it| it.to_u8())
  }

  fn as_u16(&self) -> Option<u16> {
    self.as_f64().and_then(|it| it.to_u16())
  }

  fn as_u32(&self) -> Option<u32> {
    self.as_f64().and_then(|it| it.to_u32())
  }

  fn as_u64(&self) -> Option<u64> {
    self.as_f64().and_then(|it| it.to_u64())
  }

  fn as_str_arc(&self) -> Option<Arc<str>> {
    self.as_string().map(Arc::from)
  }

  fn into_serde<T>(self) -> JsResult<T>
  where
    T: DeserializeOwned,
  {
    from_value(self).map_err(Into::into)
  }
}
