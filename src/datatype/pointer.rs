use libc::c_void;
use napi::{bindgen_prelude::*, Env, JsExternal};
use std::ffi::{c_char, CString};

pub trait ArrayPointer {
  type Output;
  unsafe fn get_and_advance(&mut self) -> Self::Output;
}

macro_rules! impl_array_pointer {
  ($type:ty, $output:ty) => {
    impl ArrayPointer for $type {
      type Output = $output;
      unsafe fn get_and_advance(&mut self) -> Self::Output {
        let value = **self;
        *self = self.offset(1);
        value
      }
    }
  };
}
impl_array_pointer!(*mut u8, u8);
impl_array_pointer!(*mut i32, i32);
impl_array_pointer!(*mut f64, f64);

impl ArrayPointer for *mut *mut c_char {
  type Output = String;
  unsafe fn get_and_advance(&mut self) -> Self::Output {
    let value = **self;
    *self = self.offset(1);
    CString::from_raw(value).into_string().unwrap()
  }
}
pub fn create_array_from_pointer<P>(mut pointer: P, len: usize) -> Vec<P::Output>
where
  P: ArrayPointer,
{
  unsafe { (0..len).map(|_| pointer.get_and_advance()).collect() }
}

pub unsafe fn get_js_external_wrap_Data(env: &Env, js_external: JsExternal) -> *mut c_void {
  let js_external_raw = JsExternal::to_napi_value(env.raw(), js_external).unwrap();
  let external: External<*mut c_void> =
    External::from_napi_value(env.raw(), js_external_raw).unwrap();
  *external
}
