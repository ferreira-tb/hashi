pub use crate::console::{Console, print, print_err};
pub use crate::dom::mouse::prelude::*;
pub use crate::dom::observer::create_observer;
pub use crate::dom::query::prelude::*;
pub use crate::dom::ready::{is_ready, wait_ready};
pub use crate::iter::{JsCastIter, JsUncheckedCastIter, cast_iter, unchecked_cast_iter};
pub use crate::location::{hostname, href, reload, set_href};
pub use crate::timer::{sleep, sleep_millis, sleep_mins, sleep_secs};
pub use crate::value::JsValueExt;
