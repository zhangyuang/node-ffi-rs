use crate::utils::dataprocess::{get_array_desc, get_ffi_tag, get_func_desc};
use libc::{c_double, c_float, c_int, c_void, free};
use libffi::middle::Closure;
use std::ffi::{c_char, CStr, CString};

use crate::define::*;
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
impl_array_pointer!(*mut f32, f32);

impl ArrayPointer for *mut *mut c_char {
  type Output = String;
  unsafe fn get_and_advance(&mut self) -> Self::Output {
    let value = **self;
    *self = self.offset(1);
    CStr::from_ptr(value).to_string_lossy().to_string()
  }
}
pub fn create_array_from_pointer<P>(mut pointer: P, len: usize) -> Vec<P::Output>
where
  P: ArrayPointer,
{
  unsafe { (0..len).map(|_| pointer.get_and_advance()).collect() }
}

pub unsafe fn free_pointer_memory(ptr: *mut c_void, ptr_desc: RsArgsValue) {
  match ptr_desc {
    RsArgsValue::I32(number) => {
      let basic_data_type = number_to_basic_data_type(number);
      match basic_data_type {
        BasicDataType::String => {
          let _ = CString::from_raw(*(ptr as *mut *mut i8));
        }
        BasicDataType::U8 => free(ptr),
        BasicDataType::I32 => free(ptr),
        BasicDataType::I64 => free(ptr),
        BasicDataType::U64 => free(ptr),
        BasicDataType::Void => free(ptr),
        BasicDataType::Float => free(ptr),
        BasicDataType::Double => free(ptr),
        BasicDataType::Boolean => free(ptr),
        BasicDataType::External => {
          //
        }
      }
    }
    RsArgsValue::Object(obj) => {
      let ffi_tag = get_ffi_tag(&obj);
      if let FFITag::Array = ffi_tag {
        let array_desc = get_array_desc(&obj);
        // array
        let FFIARRARYDESC {
          array_type,
          array_len,
          ..
        } = array_desc;
        match array_type {
          RefDataType::U8Array => {
            let _ = Vec::from_raw_parts(*(ptr as *mut *mut c_char), array_len, array_len);
          }
          RefDataType::I32Array => {
            let _ = Vec::from_raw_parts(*(ptr as *mut *mut c_int), array_len, array_len);
          }
          RefDataType::DoubleArray => {
            let _ = Vec::from_raw_parts(*(ptr as *mut *mut c_double), array_len, array_len);
          }
          RefDataType::FloatArray => {
            let _ = Vec::from_raw_parts(*(ptr as *mut *mut c_float), array_len, array_len);
          }
          RefDataType::StringArray => {
            let v = Vec::from_raw_parts(*(ptr as *mut *mut *mut c_char), array_len, array_len);
            v.into_iter().for_each(|str_ptr| {
              let _ = CString::from_raw(str_ptr);
            });
          }
        }
      }
      if let FFITag::Function = ffi_tag {
        let func_desc = get_func_desc(&obj);
        if func_desc.need_free {
          let _ = Box::from_raw(*(ptr as *mut *mut Closure));
        }
      } else {
        // raw object
      }
    }
    _ => panic!("free memory error"),
  }
}
