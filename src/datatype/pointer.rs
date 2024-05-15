use std::ffi::{c_char, CStr, CString};

use libc::{c_void, free};

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

pub enum OneHeavyPointer {
  Single(*mut c_void),
  Array(Vec<*mut c_void>),
}
pub unsafe fn free_one_heavy_pointer(ptr: OneHeavyPointer) {
  match ptr {
    OneHeavyPointer::Single(ptr) => free(ptr),
    OneHeavyPointer::Array(ptr_arr) => ptr_arr.into_iter().for_each(|ptr| free(ptr)),
  }
}

pub unsafe fn free_pointer_memory(ptr: *mut c_void, ptr_desc: RsArgsValue) {
  match ptr_desc {
    RsArgsValue::I32(number) => {
      let basic_data_type = number_to_basic_data_type(number);
      match basic_data_type {
        BasicDataType::String => {
          CString::from_raw(ptr as *mut i8);
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
      if let FFITag::Array = get_ffi_tag(&obj) {
        let array_desc = get_array_desc(&obj);
        // array
        let FFIARRARYDESC {
          array_type,
          array_len,
          ..
        } = array_desc;
        match array_type {
          RefDataType::U8Array => {
            let arr = create_array_from_pointer(*(ptr as *mut *mut c_uchar), array_len);
            rs_value_to_js_unknown(env, get_safe_buffer(env, arr, false))
          }
          RefDataType::I32Array => {
            let arr = create_array_from_pointer(*(ptr as *mut *mut c_int), array_len);
            rs_value_to_js_unknown(env, RsArgsValue::I32Array(arr))
          }
          RefDataType::DoubleArray => {
            let arr = create_array_from_pointer(*(ptr as *mut *mut c_double), array_len);
            rs_value_to_js_unknown(env, RsArgsValue::DoubleArray(arr))
          }
          RefDataType::FloatArray => {
            let arr = create_array_from_pointer(*(ptr as *mut *mut c_float), array_len);
            rs_value_to_js_unknown(env, RsArgsValue::FloatArray(arr))
          }
          RefDataType::StringArray => {
            let arr = create_array_from_pointer(*(ptr as *mut *mut *mut c_char), array_len);
            rs_value_to_js_unknown(env, RsArgsValue::StringArray(arr))
          }
        }
      } else {
        // raw object
        let rs_struct = create_rs_struct_from_pointer(env, *(ptr as *mut *mut c_void), &obj, false);
        rs_value_to_js_unknown(env, RsArgsValue::Object(rs_struct))
      }
    }
    _ => Err(FFIError::Panic(format!("ret_type err {:?}", ret_type_rs)).into()),
  }
}
