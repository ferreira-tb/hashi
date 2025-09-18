use super::document;
use crate::timer::sleep;
use js_sys::Date;
use std::time::Duration;

pub fn is_ready() -> bool {
  // https://developer.mozilla.org/en-US/docs/Web/API/Document/readyState
  document()
    .ready_state()
    .eq_ignore_ascii_case("complete")
}

pub async fn wait_ready(secs: u32) {
  let interval = Duration::from_millis(50);
  let timeout = Date::now() + (f64::from(secs) * 1000.0);

  loop {
    if !is_ready() && Date::now() < timeout {
      sleep(interval).await;
    } else {
      break;
    }
  }
}
