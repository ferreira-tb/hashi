use super::document;
use crate::timer::sleep;
use js_sys::Date;
use std::time::Duration;

pub fn is_ready() -> bool {
  document()
    .ready_state()
    .eq_ignore_ascii_case("complete")
}

pub async fn wait_ready(secs: u32) {
  let mut ready = is_ready();
  let interval = Duration::from_millis(50);
  let timeout = Date::now() + (f64::from(secs) * 1000.0);

  loop {
    if ready || Date::now() > timeout {
      break;
    }

    sleep(interval).await;
    ready = is_ready();
  }
}
