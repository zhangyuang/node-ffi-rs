use napi::{JsString, Result};
use std::ffi::CString;
use std::ffi::{c_char, CStr};
use widestring::{WideCString, WideChar};

pub fn js_string_to_string(js_string: JsString) -> Result<String> {
  let s: String = js_string.into_utf16()?.try_into()?;
  Ok(s)
}

pub unsafe fn string_to_c_string(s: String) -> CString {
  let bytes = s.into_bytes();
  let c_string = CString::from_vec_unchecked(bytes);
  c_string
}

pub fn string_to_c_w_string(s: String) -> WideCString {
  WideCString::from_str(&s).unwrap()
}

pub unsafe fn create_c_string_from_ptr(pointer: *mut c_char) -> String {
  CStr::from_ptr(pointer).to_string_lossy().to_string()
}

pub unsafe fn create_c_w_string_from_ptr(pointer: *mut WideChar) -> String {
  WideCString::from_ptr_str(pointer)
    .to_string_lossy()
    .to_string()
}
