use super::pointer::*;
use super::transform::*;
use crate::define::*;
use indexmap::IndexMap;
use napi::{Env, JsBoolean, JsNumber, JsObject, JsString, JsUnknown, ValueType};
use std::ffi::c_void;
use std::ffi::{c_char, c_double, c_int, CStr};

pub unsafe fn create_rs_struct_from_pointer(
  ptr: *mut c_void,
  ret_object: &IndexMap<String, RsArgsValue>,
) -> IndexMap<String, RsArgsValue> {
  let mut rs_struct: IndexMap<String, RsArgsValue> = IndexMap::new();
  let mut field_ptr = ptr;
  let mut offset = 0;
  for (field, val) in ret_object {
    if let RsArgsValue::I32(number) = val {
      let data_type = number_to_basic_data_type(*number);
      match data_type {
        BasicDataType::I32 => {
          let align = std::mem::align_of::<c_int>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut c_int;
          rs_struct.insert(field.clone(), RsArgsValue::I32(*type_field_ptr));
          offset = std::mem::size_of::<c_int>();
        }
        BasicDataType::Double => {
          let align = std::mem::align_of::<c_double>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut c_double;
          rs_struct.insert(field.clone(), RsArgsValue::Double(*type_field_ptr));
          offset = std::mem::size_of::<c_double>();
        }
        BasicDataType::Boolean => {
          let align = std::mem::align_of::<bool>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut bool;
          rs_struct.insert(field.clone(), RsArgsValue::Boolean(*type_field_ptr));
          offset = std::mem::size_of::<bool>();
        }
        BasicDataType::Void => {
          let align = std::mem::align_of::<()>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          rs_struct.insert(field.clone(), RsArgsValue::Void(()));
          offset = std::mem::size_of::<bool>();
        }
        BasicDataType::String => {
          let align = std::mem::align_of::<*const c_char>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut *mut c_char;
          let js_string = CStr::from_ptr(*type_field_ptr)
            .to_string_lossy()
            .to_string();
          rs_struct.insert(field.clone(), RsArgsValue::String(js_string));
          offset = std::mem::size_of::<*const c_char>();
        }
      }
    }
    if let RsArgsValue::Object(obj) = val {
      if obj.get(ARRAY_LENGTH_TAG).is_some() {
        // array
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
            let align = std::mem::align_of::<*const *const c_char>();
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            let type_field_ptr = field_ptr as *mut *mut *mut c_char;
            let arr = create_array_from_pointer(*type_field_ptr, array_len);
            rs_struct.insert(field.clone(), RsArgsValue::StringArray(arr));
            offset = std::mem::size_of::<*const *const c_char>();
          }
          RefDataType::DoubleArray => {
            let align = std::mem::align_of::<*const c_double>();
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            let type_field_ptr = field_ptr as *mut *mut c_double;
            let arr = create_array_from_pointer(*type_field_ptr, array_len);
            rs_struct.insert(field.clone(), RsArgsValue::DoubleArray(arr));
            offset = std::mem::size_of::<*const c_double>();
          }
          RefDataType::I32Array => {
            let align = std::mem::align_of::<*const c_int>();
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            let type_field_ptr = field_ptr as *mut *mut c_int;
            let arr = create_array_from_pointer(*type_field_ptr, array_len);
            rs_struct.insert(field.clone(), RsArgsValue::I32Array(arr));
            offset = std::mem::size_of::<*const c_int>();
          }
        }
      } else {
        // function | raw object
        rs_struct.insert(
          field.clone(),
          RsArgsValue::Object(create_rs_struct_from_pointer(field_ptr, obj)),
        );
      };
    }
    field_ptr = field_ptr.offset(offset as isize) as *mut c_void;
  }
  rs_struct
}

pub fn get_params_value_rs_struct(
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
            DataType::Void => RsArgsValue::Void(()),
          };
          index_map.insert(field, val);
        }

        ValueType::Object => {
          let args_type = field_type.coerce_to_object().unwrap();
          let map = get_params_value_rs_struct(&args_type, &args_type);
          index_map.insert(field, RsArgsValue::Object(map));
        }
        _ => panic!("jsobject_to_rs_struct err "),
      };
    });
  index_map
}

pub fn type_define_to_rs_struct(params_type_object: &JsObject) -> IndexMap<String, RsArgsValue> {
  let mut index_map = IndexMap::new();
  JsObject::keys(params_type_object)
    .unwrap()
    .into_iter()
    .for_each(|field| {
      let field_type: JsUnknown = params_type_object.get_named_property(&field).unwrap();
      match field_type.get_type().unwrap() {
        ValueType::Number => {
          let number: JsNumber = field_type.try_into().unwrap();
          let val: i32 = number.try_into().unwrap();
          index_map.insert(field, RsArgsValue::I32(val));
        }

        ValueType::Object => {
          let args_type = field_type.coerce_to_object().unwrap();
          let map = type_define_to_rs_struct(&args_type);
          index_map.insert(field, RsArgsValue::Object(map));
        }
        ValueType::String => {
          let str: JsString = field_type.try_into().unwrap();
          let str = js_string_to_string(str);
          index_map.insert(field, RsArgsValue::String(str));
        }
        _ => panic!("get_params_type_rs_struct err "),
      };
    });
  index_map
}

pub unsafe fn rs_value_to_js_unknown(env: &Env, data: RsArgsValue) -> JsUnknown {
  return match data {
    RsArgsValue::I32(number) => env.create_int32(number as i32).unwrap().into_unknown(),
    RsArgsValue::Boolean(val) => env.get_boolean(val).unwrap().into_unknown(),
    RsArgsValue::String(val) => env.create_string(&val).unwrap().into_unknown(),
    RsArgsValue::Double(val) => env.create_double(val).unwrap().into_unknown(),
    RsArgsValue::I32Array(val) => rs_array_to_js_array(env, ArrayType::I32(val)).into_unknown(),
    RsArgsValue::StringArray(val) => {
      rs_array_to_js_array(env, ArrayType::String(val)).into_unknown()
    }
    RsArgsValue::DoubleArray(val) => {
      rs_array_to_js_array(env, ArrayType::Double(val)).into_unknown()
    }
    RsArgsValue::Object(obj) => {
      let mut js_object = env.create_object().unwrap();
      for (field, value) in obj {
        js_object
          .set_property(
            env.create_string(&field).unwrap(),
            rs_value_to_js_unknown(env, value),
          )
          .unwrap();
      }
      js_object.into_unknown()
    }
    RsArgsValue::Void(_) => panic!("void cannot be as a call param type"),
    RsArgsValue::Function(_, _) => panic!("function need to be improved"),
  };
}
