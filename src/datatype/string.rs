use napi::{JsString, Result};
use std::ffi::CString;

pub fn js_string_to_string(js_string: JsString) -> Result<String> {
  let s: String = js_string.into_utf16()?.try_into()?;
  Ok(s)
}

pub unsafe fn string_to_c_string(s: String) -> CString {
  let bytes = s.into_bytes();
  let c_string = CString::from_vec_unchecked(bytes);
  c_string
}
