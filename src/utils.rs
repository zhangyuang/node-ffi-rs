use crate::define::{number_to_data_type, DataType, RsArgsValue};
use indexmap::IndexMap;
use napi::bindgen_prelude::*;
use napi::{JsObject, JsString, JsUnknown};
use std::ffi::c_void;
use std::ffi::{c_char, c_double, c_int, CString};

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
        _ => panic!("get_js_function_call_value err"),
      };
      data
    }
    _ => panic!("get_js_function_call_value err "),
  };
}

pub fn js_array_to_string_array(js_array: JsObject) -> Vec<String> {
  vec![0; js_array.get_array_length().unwrap() as usize]
    .iter()
    .enumerate()
    .map(|(index, _)| {
      let js_element: JsString = js_array.get_element(index as u32).unwrap();
      return js_element.into_utf8().unwrap().try_into().unwrap();
    })
    .collect::<Vec<String>>()
}

pub fn align_ptr(ptr: *mut c_void, align: usize) -> *mut c_void {
  let align_minus_one = align - 1;
  let ptr_int = ptr as usize;
  let aligned = (ptr_int + align_minus_one) & !align_minus_one;
  aligned as *mut c_void
}

pub fn calculate_layout(map: &IndexMap<String, RsArgsValue>) -> (usize, usize) {
  let (size, align) = map
    .iter()
    .fold((0, 0), |(size, align), (_, field_val)| match field_val {
      RsArgsValue::I32(_) => {
        let align = align.max(std::mem::align_of::<c_int>());
        let size = size + std::mem::size_of::<c_int>();
        (size, align)
      }
      RsArgsValue::Double(_) => {
        let align = align.max(std::mem::align_of::<c_double>());
        let size = size + std::mem::size_of::<c_double>();
        (size, align)
      }
      RsArgsValue::String(_) => {
        let align = align.max(std::mem::align_of::<*const c_char>());
        let size = size + std::mem::size_of::<*const c_char>();
        (size, align)
      }
      RsArgsValue::Object(val) => {
        let (obj_size, obj_align) = calculate_layout(val);
        let align = align.max(obj_align);
        let size = size + obj_size;
        (size, align)
      }
      RsArgsValue::StringArray(_) => {
        let align = align.max(std::mem::align_of::<*const *const c_char>());
        let size = size + std::mem::size_of::<*const *const c_char>();
        (size, align)
      }
      _ => panic!("calculate_layout"),
    });
  (size, align)
}

pub fn get_data_type_size_align(data_type: DataType) -> (usize, usize) {
  return match data_type {
    DataType::I32 => (std::mem::size_of::<c_int>(), std::mem::align_of::<c_int>()),
    DataType::String => (
      std::mem::size_of::<*const c_char>(),
      std::mem::align_of::<*const c_char>(),
    ),
    DataType::Double => (
      std::mem::size_of::<c_double>(),
      std::mem::align_of::<c_double>(),
    ),
    DataType::StringArray => (
      std::mem::size_of::<*const *const c_char>(),
      std::mem::align_of::<*const *const c_char>(),
    ),
    _ => {
      panic!("{:?} Not available as a field type at this time", data_type)
    }
  };
}
