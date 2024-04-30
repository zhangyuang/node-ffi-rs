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
  U8(Vec<u8>),
  I32(Vec<i32>),
  Double(Vec<f64>),
  String(Vec<String>),
}

pub fn rs_array_to_js_array(env: &Env, val: ArrayType) -> Result<JsObject> {
  match val {
    ArrayType::String(arr) => {
      let mut js_array = env.create_array_with_length(arr.len())?;
      arr
        .into_iter()
        .enumerate()
        .try_for_each(|(index, str)| js_array.set_element(index as u32, env.create_string(&str)?));
      Ok(js_array)
    }
    ArrayType::Double(arr) => {
      let mut js_array = env.create_array_with_length(arr.len())?;
      arr
        .into_iter()
        .enumerate()
        .try_for_each(|(index, item)| js_array.set_element(index as u32, env.create_double(item)?));
      Ok(js_array)
    }
    ArrayType::I32(arr) => {
      let mut js_array = env.create_array_with_length(arr.len())?;
      arr
        .into_iter()
        .enumerate()
        .try_for_each(|(index, item)| js_array.set_element(index as u32, env.create_int32(item)?));
      Ok(js_array)
    }
    ArrayType::U8(arr) => {
      let mut js_array = env.create_array_with_length(arr.len())?;
      arr.into_iter().enumerate().try_for_each(|(index, item)| {
        js_array.set_element(index as u32, env.create_uint32(item as u32)?)
      });
      Ok(js_array)
    }
  }
}
