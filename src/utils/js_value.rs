use napi::bindgen_prelude::FromNapiValue;
use napi::{JsUnknown, Result};

pub unsafe fn create_js_value_unchecked<T: FromNapiValue>(js_known: JsUnknown) -> Result<T> {
  Ok(T::from_unknown(js_known)?)
}
