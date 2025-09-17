use wasm_bindgen::prelude::*;
use web_sys::Location;

pub fn location() -> Location {
  super::window().location()
}

pub fn assign(url: &str) {
  location().assign(url).unwrap_throw();
}

pub fn hash() -> String {
  location().hash().unwrap_throw()
}

pub fn host() -> String {
  location().host().unwrap_throw()
}

pub fn hostname() -> String {
  location().hostname().unwrap_throw()
}

pub fn href() -> String {
  location().href().unwrap_throw()
}

pub fn origin() -> String {
  location().origin().unwrap_throw()
}

pub fn pathname() -> String {
  location().pathname().unwrap_throw()
}

pub fn port() -> String {
  location().port().unwrap_throw()
}

pub fn protocol() -> String {
  location().protocol().unwrap_throw()
}

pub fn reload() {
  location().reload().unwrap_throw();
}

pub fn replace(url: &str) {
  location().replace(url).unwrap_throw();
}

pub fn search() -> String {
  location().search().unwrap_throw()
}

pub fn set_hash(value: &str) {
  location().set_hash(value).unwrap_throw();
}

pub fn set_host(value: &str) {
  location().set_host(value).unwrap_throw();
}

pub fn set_hostname(value: &str) {
  location().set_hostname(value).unwrap_throw();
}

pub fn set_href(value: &str) {
  location().set_href(value).unwrap_throw();
}

pub fn set_pathname(value: &str) {
  location().set_pathname(value).unwrap_throw();
}

pub fn set_port(value: &str) {
  location().set_port(value).unwrap_throw();
}

pub fn set_protocol(value: &str) {
  location().set_protocol(value).unwrap_throw();
}

pub fn set_search(value: &str) {
  location().set_search(value).unwrap_throw();
}
