use js_sys::Reflect;
use wasm_bindgen::prelude::*;

pub struct Object(js_sys::Object);

impl Object {
  pub fn new() -> Self {
    Self(js_sys::Object::new())
  }

  pub fn with(&self, key: &str, value: &JsValue) -> Result<Self, JsValue> {
    let object = Self::new();
    object.set(key, value)?;
    Ok(object)
  }

  pub fn get(&self, key: &str) -> Result<JsValue, JsValue> {
    Reflect::get(&self.0, &key.into())
  }

  pub fn get_dyn<T: JsCast>(&self, key: &str) -> Result<T, JsValue> {
    self.get(key).and_then(JsCast::dyn_into)
  }

  pub fn get_unchecked<T: JsCast>(&self, key: &str) -> Result<T, JsValue> {
    self.get(key).map(JsCast::unchecked_into)
  }

  pub fn set(&self, key: &str, value: &JsValue) -> Result<&Self, JsValue> {
    Reflect::set(&self.0, &key.into(), value)?;
    Ok(self)
  }

  pub fn into_inner(self) -> js_sys::Object {
    self.0
  }
}

impl Default for Object {
  fn default() -> Self {
    Self::new()
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

  fn try_from(value: JsValue) -> Result<Self, Self::Error> {
    Ok(Self(value.dyn_into()?))
  }
}
