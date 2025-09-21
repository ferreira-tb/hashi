use crate::JsResult;
use js_sys::Reflect;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[derive(Clone, Default)]
pub struct Object(js_sys::Object);

impl Object {
  pub fn new() -> Self {
    Self(js_sys::Object::new())
  }

  pub fn with(&self, key: &str, value: &JsValue) -> JsResult<Self> {
    let object = Self::new();
    object.set(key, value)?;
    Ok(object)
  }

  pub fn get(&self, key: &str) -> JsResult {
    Reflect::get(&self.0, &key.into())
  }

  pub fn get_dyn<T: JsCast>(&self, key: &str) -> JsResult<T> {
    self.get(key).and_then(JsCast::dyn_into)
  }

  pub fn get_unchecked<T: JsCast>(&self, key: &str) -> JsResult<T> {
    self.get(key).map(JsCast::unchecked_into)
  }

  pub fn get_serde<T: DeserializeOwned>(&self, key: &str) -> JsResult<T> {
    from_value(self.get(key)?).map_err(Into::into)
  }

  pub fn set(&self, key: &str, value: &JsValue) -> JsResult<&Self> {
    Reflect::set(&self.0, &key.into(), value)?;
    Ok(self)
  }

  pub fn set_serde<T: Serialize>(&self, key: &str, value: &T) -> JsResult<&Self> {
    self.set(key, &to_value(value)?)
  }

  pub fn into_inner(self) -> js_sys::Object {
    self.0
  }
}

impl AsRef<JsValue> for Object {
  fn as_ref(&self) -> &JsValue {
    self.0.as_ref()
  }
}

impl From<Object> for js_sys::Object {
  fn from(value: Object) -> Self {
    value.0
  }
}

impl From<js_sys::Object> for Object {
  fn from(value: js_sys::Object) -> Self {
    Self(value)
  }
}

impl From<Object> for JsValue {
  fn from(value: Object) -> Self {
    value.0.into()
  }
}

impl TryFrom<JsValue> for Object {
  type Error = JsValue;

  fn try_from(value: JsValue) -> JsResult<Self> {
    Ok(Self(value.dyn_into()?))
  }
}
