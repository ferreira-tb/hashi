use js_sys::{IntoIter, try_iter};
use std::marker::PhantomData;
use wasm_bindgen::prelude::*;

pub struct JsCastIter<T: JsCast> {
  #[expect(unused)]
  value: JsValue,
  iter: Option<IntoIter>,
  phantom: PhantomData<T>,
}

impl<T: JsCast> JsCastIter<T> {
  pub fn new(value: JsValue) -> Self {
    let iter = try_iter(&value).unwrap_or_default();
    Self { value, iter, phantom: PhantomData }
  }
}

impl<T: JsCast> Iterator for JsCastIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(iter) = &mut self.iter {
      for value in iter.by_ref() {
        if let Ok(value) = value.and_then(JsCast::dyn_into) {
          return Some(value);
        }
      }
    }

    None
  }
}

pub fn cast_iter<T: JsCast>(value: JsValue) -> JsCastIter<T> {
  JsCastIter::new(value)
}
