use napi::bindgen_prelude::*;
use napi::{JsNumber, JsObject, JsString};

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
