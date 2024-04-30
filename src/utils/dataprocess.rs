use crate::datatype::array::*;

use crate::define::*;
use indexmap::IndexMap;
use napi::JsBuffer;
use napi::{Env, JsBoolean, JsExternal, JsNumber, JsObject, JsString, JsUnknown, ValueType};

pub fn get_params_value_rs_struct(
  env: &Env,
  params_type_object: &JsObject,
  params_value_object: &JsObject,
) -> IndexMap<String, RsArgsValue> {
  let mut index_map = IndexMap::new();
  JsObject::keys(&params_value_object)
    .unwrap()
    .into_iter()
    .for_each(|field| {
      let field_type: JsUnknown = params_type_object.get_named_property(&field).unwrap();
      match field_type.get_type().unwrap() {
        ValueType::Number => {
          let data_type: DataType =
            number_to_data_type(field_type.coerce_to_number().unwrap().try_into().unwrap());
          let val = match data_type {
            DataType::String => {
              let val: JsString = params_value_object.get_named_property(&field).unwrap();
              let val: String = val.into_utf8().unwrap().try_into().unwrap();
              RsArgsValue::String(val)
            }
            DataType::U8 => {
              let val: JsNumber = params_value_object.get_named_property(&field).unwrap();
              let val: u32 = val.try_into().unwrap();
              RsArgsValue::U8(val as u8)
            }
            DataType::I32 => {
              let val: JsNumber = params_value_object.get_named_property(&field).unwrap();
              let val: i32 = val.try_into().unwrap();
              RsArgsValue::I32(val)
            }
            DataType::I64 => {
              let val: JsNumber = params_value_object.get_named_property(&field).unwrap();
              let val: i64 = val.try_into().unwrap();
              RsArgsValue::I64(val)
            }
            DataType::U64 => {
              let val: JsNumber = params_value_object.get_named_property(&field).unwrap();
              let val: i64 = val.try_into().unwrap();
              RsArgsValue::U64(val as u64)
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
            DataType::U8Array => {
              let js_buffer: JsBuffer = params_value_object.get_named_property(&field).unwrap();
              RsArgsValue::U8Array(Some(js_buffer.into_value().unwrap()), None)
            }
            DataType::External => {
              let val: JsExternal = params_value_object.get_named_property(&field).unwrap();
              RsArgsValue::External(val)
            }
            DataType::Void => RsArgsValue::Void(()),
          };
          index_map.insert(field, val);
        }

        ValueType::Object => {
          let params_type = field_type.coerce_to_object().unwrap();
          let params_value: JsObject = params_value_object.get_named_property(&field).unwrap();
          let map = get_params_value_rs_struct(env, &params_type, &params_value);
          index_map.insert(field, RsArgsValue::Object(map));
        }
        _ => panic!(
          "receive {:?} but params type can only be number or object ",
          field_type.get_type().unwrap()
        ),
      };
    });
  index_map
}

pub fn type_object_to_rs_struct(params_type: &JsObject) -> IndexMap<String, RsArgsValue> {
  let mut index_map = IndexMap::new();
  JsObject::keys(params_type)
    .unwrap()
    .into_iter()
    .for_each(|field| {
      let field_type: JsUnknown = params_type.get_named_property(&field).unwrap();
      match field_type.get_type().unwrap() {
        ValueType::Number => {
          let number: JsNumber = field_type.try_into().unwrap();
          let val: i32 = number.try_into().unwrap();
          index_map.insert(field, RsArgsValue::I32(val));
        }

        ValueType::Object => {
          // maybe jsobject or jsarray
          let args_type = field_type.coerce_to_object().unwrap();
          let map = type_object_to_rs_struct(&args_type);
          index_map.insert(field, RsArgsValue::Object(map));
        }
        ValueType::String => {
          let str: JsString = field_type.try_into().unwrap();
          let str = js_string_to_string(str);
          index_map.insert(field, RsArgsValue::String(str));
        }
        _ => panic!(
          "receive {:?} but params type can only be number or object ",
          field_type.get_type().unwrap()
        ),
      };
    });
  index_map
}

// describe paramsType or retType, field can only be number or object
pub fn type_define_to_rs_args(type_define: JsUnknown) -> RsArgsValue {
  let ret_value_type = type_define.get_type().unwrap();
  let ret_value = match ret_value_type {
    ValueType::Number => {
      RsArgsValue::I32(js_number_to_i32(type_define.coerce_to_number().unwrap()))
    }
    ValueType::Object => RsArgsValue::Object(type_object_to_rs_struct(
      &type_define.coerce_to_object().unwrap(),
    )),
    _ => panic!(
      "ret_value_type can only be number or object but receive {}",
      ret_value_type
    ),
  };
  return ret_value;
}
