use wasm_bindgen::prelude::*;
use web_sys::NodeList;

pub fn from_node_list<T: JsCast>(list: &NodeList) -> Result<Vec<T>, JsValue> {
  let mut values = Vec::new();
  if let Some(iter) = js_sys::try_iter(list)? {
    for value in iter {
      values.push(JsCast::dyn_into(value?)?);
    }
  }

  Ok(values)
}
