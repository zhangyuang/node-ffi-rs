use crate::define::{number_to_data_type, DataType, RsArgsValue};
use crate::pointer::{create_array_from_pointer, create_object_from_pointer};
use indexmap::IndexMap;
use napi::bindgen_prelude::*;
use napi::{JsBoolean, JsNumber, JsObject, JsString, JsUnknown};
use std::ffi::c_void;
use std::ffi::{c_char, c_double, c_int, CStr};

pub unsafe fn get_js_function_call_value(
  env: &Env,
  func_arg_type: JsUnknown,
  func_arg_ptr: *mut c_void,
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
        DataType::I32 => {
          return env
            .create_int32(func_arg_ptr as i32)
            .unwrap()
            .into_unknown();
        }
        DataType::Boolean => env
          .get_boolean(if func_arg_ptr as i32 == 0 {
            false
          } else {
            true
          })
          .unwrap()
          .into_unknown(),
        DataType::String => {
          return env
            .create_string(&CStr::from_ptr(func_arg_ptr as *mut c_char).to_string_lossy())
            .unwrap()
            .into_unknown();
        }

        DataType::Double => {
          println!("{:?}", func_arg_ptr);
          return env.create_double(1.1).unwrap().into_unknown();
        }
        _ => panic!(
          "{:?} data_type as function args is unsupported at this time",
          data_type
        ),
      };
      data
    }
    ValueType::Object => {
      let args_type = func_arg_type.coerce_to_object().unwrap();
      let ffi_tag = args_type.has_named_property("ffiTypeTag").unwrap();
      if ffi_tag {
        let array_len: usize =
          js_nunmber_to_i32(args_type.get_named_property::<JsNumber>("length").unwrap()) as usize;
        let array_type: i32 =
          js_nunmber_to_i32(args_type.get_named_property::<JsNumber>("type").unwrap());
        let array_type = number_to_data_type(array_type);

        match array_type {
          DataType::StringArray => {
            let arr = create_array_from_pointer(func_arg_ptr as *mut *mut c_char, array_len);
            rs_array_to_js_array(env, ArrayType::String(arr)).into_unknown()
          }
          DataType::I32Array => {
            let arr = create_array_from_pointer(func_arg_ptr as *mut c_int, array_len);
            rs_array_to_js_array(env, ArrayType::I32(arr)).into_unknown()
          }
          DataType::DoubleArray => {
            let arr = create_array_from_pointer(func_arg_ptr as *mut c_double, array_len);
            rs_array_to_js_array(env, ArrayType::Double(arr)).into_unknown()
          }
          _ => panic!(
            "{:?} as function parameter is unsupported so far",
            array_type
          ),
        }
      } else {
        create_object_from_pointer(env, func_arg_ptr, args_type).into_unknown()
      }
    }
    _ => panic!("get_js_function_call_value err "),
  };
}
pub unsafe fn get_js_function_call_value_number(
  func_arg_type: i32,
  func_arg_ptr: *mut c_void,
) -> RsArgsValue {
  let data_type: DataType = number_to_data_type(func_arg_type);
  return match data_type {
    DataType::I32 => RsArgsValue::I32(func_arg_type as i32),
    DataType::Boolean => {
      return RsArgsValue::Boolean(if func_arg_ptr as i32 == 0 {
        false
      } else {
        true
      })
    }
    DataType::String => RsArgsValue::String(
      CStr::from_ptr(func_arg_ptr as *mut c_char)
        .to_string_lossy()
        .to_string(),
    ),

    _ => panic!(
      "{:?} data_type as function args is unsupported at this time",
      data_type
    ),
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

macro_rules! calculate_layout_for {
  ($variant:ident, $type:ty) => {
    fn $variant(size: usize, align: usize) -> (usize, usize) {
      let align = align.max(std::mem::align_of::<$type>());
      let size = size + std::mem::size_of::<$type>();
      (size, align)
    }
  };
}

calculate_layout_for!(calculate_i32, c_int);
calculate_layout_for!(calculate_double, c_double);
calculate_layout_for!(calculate_boolean, bool);
calculate_layout_for!(calculate_string, *const c_char);
calculate_layout_for!(calculate_string_array, *const *const c_char);
calculate_layout_for!(calculate_double_array, *const c_double);
calculate_layout_for!(calculate_i32_array, *const c_int);

pub fn calculate_layout(map: &IndexMap<String, RsArgsValue>) -> (usize, usize) {
  let (size, align) = map
    .iter()
    .fold((0, 0), |(size, align), (_, field_val)| match field_val {
      RsArgsValue::I32(_) => calculate_i32(size, align),
      RsArgsValue::Double(_) => calculate_double(size, align),
      RsArgsValue::String(_) => calculate_string(size, align),
      RsArgsValue::Boolean(_) => calculate_boolean(size, align),
      RsArgsValue::Object(val) => {
        let (obj_size, obj_align) = calculate_layout(val);
        let align = align.max(obj_align);
        let size = size + obj_size;
        (size, align)
      }
      RsArgsValue::StringArray(_) => calculate_string_array(size, align),
      RsArgsValue::DoubleArray(_) => calculate_double_array(size, align),
      RsArgsValue::I32Array(_) => calculate_i32_array(size, align),
      _ => panic!("calculate_layout"),
    });
  (size, align)
}

pub fn get_rs_value_size_align(val: &RsArgsValue) -> (usize, usize) {
  return match val {
    RsArgsValue::I32(_) => (std::mem::size_of::<i32>(), std::mem::align_of::<i32>()),
    RsArgsValue::Boolean(_) => (std::mem::size_of::<bool>(), std::mem::align_of::<bool>()),
    RsArgsValue::String(_) => (
      std::mem::size_of::<*const c_char>(),
      std::mem::align_of::<*const c_char>(),
    ),
    RsArgsValue::Double(_) => (
      std::mem::size_of::<c_double>(),
      std::mem::align_of::<c_double>(),
    ),
    RsArgsValue::StringArray(_) => (
      std::mem::size_of::<*const *const c_char>(),
      std::mem::align_of::<*const *const c_char>(),
    ),
    RsArgsValue::DoubleArray(_) => (
      std::mem::size_of::<*const c_double>(),
      std::mem::align_of::<*const c_double>(),
    ),
    RsArgsValue::I32Array(_) => (
      std::mem::size_of::<*const c_int>(),
      std::mem::align_of::<*const c_int>(),
    ),
    _ => {
      panic!("get_rs_value_size_align error")
    }
  };
}
pub fn get_data_type_size_align(data_type: DataType) -> (usize, usize) {
  return match data_type {
    DataType::I32 => (std::mem::size_of::<i32>(), std::mem::align_of::<i32>()),
    DataType::Boolean => (std::mem::size_of::<bool>(), std::mem::align_of::<bool>()),
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
    DataType::DoubleArray => (
      std::mem::size_of::<*const c_double>(),
      std::mem::align_of::<*const c_double>(),
    ),
    DataType::I32Array => (
      std::mem::size_of::<*const c_int>(),
      std::mem::align_of::<*const c_int>(),
    ),
    _ => {
      panic!("{:?} Not available as a field type at this time", data_type)
    }
  };
}

pub enum ArrayType {
  I32(Vec<i32>),
  Double(Vec<f64>),
  String(Vec<String>),
}

pub fn js_string_to_string(js_string: JsString) -> String {
  js_string.into_utf8().unwrap().try_into().unwrap()
}

pub fn js_nunmber_to_i32(js_number: JsNumber) -> i32 {
  js_number.try_into().unwrap()
}

pub fn js_unknown_to_data_type(val: JsUnknown) -> DataType {
  match val.get_type().unwrap() {
    ValueType::Number => {
      let val = val.coerce_to_number().unwrap();
      number_to_data_type(val.try_into().unwrap())
    }
    ValueType::Object => {
      let val = val.coerce_to_object().unwrap();
      let ffi_tag = val.has_named_property("ffiTypeTag").unwrap();
      if ffi_tag {
        number_to_data_type(js_nunmber_to_i32(
          val.get_named_property::<JsNumber>("type").unwrap(),
        ))
      } else {
        panic!("some error")
      }
    }
    _ => panic!("some error"),
  }
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

pub fn jsobject_to_rs_struct(
  params_type_object: JsObject,
  params_value_object: JsObject,
) -> IndexMap<String, RsArgsValue> {
  let mut index_map = IndexMap::new();
  JsObject::keys(&params_value_object)
    .unwrap()
    .into_iter()
    .for_each(|field| {
      let field_type: DataType = params_type_object.get_named_property(&field).unwrap();
      let val = match field_type {
        DataType::String => {
          let val: JsString = params_value_object.get_named_property(&field).unwrap();
          let val: String = val.into_utf8().unwrap().try_into().unwrap();
          RsArgsValue::String(val)
        }
        DataType::I32 => {
          let val: JsNumber = params_value_object.get_named_property(&field).unwrap();
          let val: i32 = val.try_into().unwrap();
          RsArgsValue::I32(val)
        }
        DataType::Boolean => {
          let val: JsBoolean = params_value_object.get_named_property(&field).unwrap();
          let val: bool = val.get_value().unwrap();
          RsArgsValue::Boolean(val)
        }
        DataType::Double => {
          let val: JsNumber = params_value_object.get_named_property(&field).unwrap();
          let val: f64 = val.try_into().unwrap();
          RsArgsValue::Double(val)
        }
        DataType::StringArray => {
          let js_array: JsObject = params_value_object.get_named_property(&field).unwrap();
          let arg_val = js_array_to_string_array(js_array);
          RsArgsValue::StringArray(arg_val)
        }
        DataType::DoubleArray => {
          let js_array: JsObject = params_value_object.get_named_property(&field).unwrap();
          let arg_val = js_array_to_number_array(js_array);
          RsArgsValue::DoubleArray(arg_val)
        }
        DataType::I32Array => {
          let js_array: JsObject = params_value_object.get_named_property(&field).unwrap();
          let arg_val = js_array_to_number_array(js_array);
          RsArgsValue::I32Array(arg_val)
        }
        // DataType::Object => {
        //   let val: JsObject = js_object.get_named_property(&field).unwrap();
        //   let index_map = jsobject_to_rs_struct(val);
        //   RsArgsValue::Object(index_map)
        // }
        _ => panic!("jsobject_to_rs_struct"),
      };
      index_map.insert(field, val);
    });
  index_map
}
