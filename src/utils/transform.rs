use super::pointer::*;
use super::struct_utils::create_rs_struct_from_pointer;
use crate::define::*;
use napi::bindgen_prelude::*;
use napi::{JsNumber, JsObject, JsString};
use std::ffi::c_void;
use std::ffi::{c_char, c_double, c_int, CStr};

// change c function call value to RsArgsValue from bare pointer
pub unsafe fn get_js_function_call_value(
  func_arg_type: &RsArgsValue,
  func_val_ptr: *mut c_void,
) -> RsArgsValue {
  return match func_arg_type {
    RsArgsValue::I32(number) => {
      let data_type = number_to_basic_data_type(*number);
      let data = match data_type {
        BasicDataType::I32 => RsArgsValue::I32(func_val_ptr as i32),
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
        BasicDataType::Double => RsArgsValue::Double(1.1),
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

pub fn js_array_to_string_array(js_array: JsObject) -> Vec<String> {
  (0..js_array.get_array_length().unwrap())
    .enumerate()
    .map(|(index, _)| {
      let js_element: JsString = js_array.get_element(index as u32).unwrap();
      return js_element.into_utf8().unwrap().try_into().unwrap();
    })
    .collect::<Vec<String>>()
}

pub fn js_array_to_number_array<T>(js_array: JsObject) -> Vec<T>
where
  T: TryFrom<JsNumber>,
  <T as TryFrom<JsNumber>>::Error: std::fmt::Debug,
{
  (0..js_array.get_array_length().unwrap())
    .enumerate()
    .map(|(index, _)| {
      let js_element: JsNumber = js_array.get_element(index as u32).unwrap();
      return js_element.try_into().unwrap();
    })
    .collect::<Vec<T>>()
}

pub enum ArrayType {
  I32(Vec<i32>),
  Double(Vec<f64>),
  String(Vec<String>),
}

pub fn js_string_to_string(js_string: JsString) -> String {
  js_string.into_utf8().unwrap().try_into().unwrap()
}

pub fn js_number_to_i32(js_number: JsNumber) -> i32 {
  js_number.try_into().unwrap()
}

pub fn rs_array_to_js_array(env: &Env, val: ArrayType) -> JsObject {
  match val {
    ArrayType::String(arr) => {
      let mut js_array = env.create_array_with_length(arr.len()).unwrap();
      arr.into_iter().enumerate().for_each(|(index, str)| {
        js_array
          .set_element(index as u32, env.create_string(&str).unwrap())
          .unwrap();
      });
      js_array
    }
    ArrayType::Double(arr) => {
      let mut js_array = env.create_array_with_length(arr.len()).unwrap();
      arr.into_iter().enumerate().for_each(|(index, item)| {
        js_array
          .set_element(index as u32, env.create_double(item).unwrap())
          .unwrap();
      });
      js_array
    }
    ArrayType::I32(arr) => {
      let mut js_array = env.create_array_with_length(arr.len()).unwrap();
      arr.into_iter().enumerate().for_each(|(index, item)| {
        js_array
          .set_element(index as u32, env.create_int32(item).unwrap())
          .unwrap();
      });
      js_array
    }
  }
}
