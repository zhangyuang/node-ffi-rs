use super::object_generate::create_rs_struct_from_pointer;
use super::pointer::*;
use crate::define::*;
use std::ffi::c_void;
use std::ffi::{c_char, c_double, c_int, c_uchar, CStr};

// change c function call value to RsArgsValue from bare pointer
pub unsafe fn get_js_function_call_value(
  func_arg_type: &RsArgsValue,
  func_val_ptr: *mut c_void,
) -> RsArgsValue {
  return match func_arg_type {
    RsArgsValue::I32(number) => {
      let data_type = number_to_basic_data_type(*number);
      let data = match data_type {
        BasicDataType::U8 => RsArgsValue::U8(func_val_ptr as u8),
        BasicDataType::I32 => RsArgsValue::I32(func_val_ptr as i32),
        BasicDataType::I64 => RsArgsValue::I64(func_val_ptr as i64),
        BasicDataType::Boolean => RsArgsValue::Boolean(if func_val_ptr as i32 == 0 {
          false
        } else {
          true
        }),
        BasicDataType::String => RsArgsValue::String(
          CStr::from_ptr(func_val_ptr as *mut c_char)
            .to_string_lossy()
            .to_string(),
        ),
        BasicDataType::Void => RsArgsValue::Void(()),
        // need to be improved
        BasicDataType::Double => {
          panic!("Double type cannot be used as function parameter type so far");
          RsArgsValue::Double(1.1)
        }
      };
      data
    }
    RsArgsValue::Object(obj) => {
      if obj.get(ARRAY_LENGTH_TAG).is_some() {
        let array_len = if let RsArgsValue::I32(number) = obj.get(ARRAY_LENGTH_TAG).unwrap() {
          *number as usize
        } else {
          0 as usize
        };
        let array_type = if let RsArgsValue::I32(number) = obj.get(ARRAY_TYPE_TAG).unwrap() {
          *number
        } else {
          -1
        };
        let array_type = number_to_ref_data_type(array_type);
        match array_type {
          RefDataType::StringArray => {
            let arr = create_array_from_pointer(func_val_ptr as *mut *mut c_char, array_len);
            return RsArgsValue::StringArray(arr);
          }
          RefDataType::I32Array => {
            let arr = create_array_from_pointer(func_val_ptr as *mut c_int, array_len);
            return RsArgsValue::I32Array(arr);
          }
          RefDataType::U8Array => {
            let arr = create_array_from_pointer(func_val_ptr as *mut c_uchar, array_len);
            return RsArgsValue::U8Array(arr);
          }
          RefDataType::DoubleArray => {
            let arr = create_array_from_pointer(func_val_ptr as *mut c_double, array_len);
            return RsArgsValue::DoubleArray(arr);
          }
        }
      } else {
        // function | raw object
        return RsArgsValue::Object(create_rs_struct_from_pointer(func_val_ptr, obj));
      }
    }

    _ => panic!("get_js_function_call_value{:?}", func_arg_type),
  };
}
