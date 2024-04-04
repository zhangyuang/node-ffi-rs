use crate::define::{number_to_data_type, DataType};
use napi::bindgen_prelude::*;
use napi::JsUnknown;
use std::ffi::c_void;
use std::ffi::{c_char, CString};

pub fn get_js_function_call_value(
  env: Env,
  func_arg_type: JsUnknown,
  func_arg_val: *mut c_void,
) -> JsUnknown {
  return match func_arg_type.get_type().unwrap() {
    ValueType::Number => {
      let data_type: DataType = number_to_data_type(
        func_arg_type
          .coerce_to_number()
          .unwrap()
          .try_into()
          .unwrap(),
      );
      let data = match data_type {
        DataType::I32 => env
          .create_int32(func_arg_val as i32)
          .unwrap()
          .into_unknown(),
        DataType::String => unsafe {
          env
            .create_string(
              &CString::from_raw(func_arg_val as *mut c_char)
                .into_string()
                .unwrap(),
            )
            .unwrap()
            .into_unknown()
        },
        // DataType::Double => env
        //   .create_double(func_arg_val as f64)
        //   .unwrap()
        //   .into_unknown(),
        _ => panic!(""),
      };
      data
    }
    _ => panic!(""),
  };
}
