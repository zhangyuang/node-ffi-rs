use super::array::*;
use super::pointer::*;
use crate::define::*;
use indexmap::IndexMap;
use napi::{Env, JsBoolean, JsExternal, JsNumber, JsObject, JsString, JsUnknown, ValueType};
use std::ffi::c_void;
use std::ffi::{c_char, c_double, c_int, c_longlong, c_uchar, CStr};

pub unsafe fn create_rs_struct_from_pointer(
  env: &Env,
  ptr: *mut c_void,
  ret_object: &IndexMap<String, RsArgsValue>,
) -> IndexMap<String, RsArgsValue> {
  let mut rs_struct: IndexMap<String, RsArgsValue> = IndexMap::new();
  let mut field_ptr = ptr;
  let mut offset = 0;
  let mut field_size = 0;
  for (field, val) in ret_object {
    if let RsArgsValue::I32(number) = val {
      let field = field.clone();
      let data_type = number_to_basic_data_type(*number);
      match data_type {
        BasicDataType::U8 => {
          let (size, align) = (
            std::mem::size_of::<c_uchar>(),
            std::mem::align_of::<c_uchar>(),
          );
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut c_uchar;
          rs_struct.insert(field, RsArgsValue::U8(*type_field_ptr));
          offset += size + padding;
          field_size = size
        }
        BasicDataType::I32 => {
          let (size, align) = (std::mem::size_of::<c_int>(), std::mem::align_of::<c_int>());
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut c_int;
          rs_struct.insert(field, RsArgsValue::I32(*type_field_ptr));
          offset += size + padding;
          field_size = size
        }
        BasicDataType::I64 => {
          let (size, align) = (
            std::mem::size_of::<c_longlong>(),
            std::mem::align_of::<c_longlong>(),
          );
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut c_longlong;
          rs_struct.insert(field, RsArgsValue::I64(*type_field_ptr));
          offset += size + padding;
          field_size = size
        }
        BasicDataType::Double => {
          let (size, align) = (
            std::mem::size_of::<c_double>(),
            std::mem::align_of::<c_double>(),
          );
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut c_double;
          rs_struct.insert(field, RsArgsValue::Double(*type_field_ptr));
          offset += size + padding;
          field_size = size
        }
        BasicDataType::Boolean => {
          let (size, align) = (std::mem::size_of::<bool>(), std::mem::align_of::<bool>());
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut bool;
          rs_struct.insert(field, RsArgsValue::Boolean(*type_field_ptr));
          offset += size + padding;
          field_size = size
        }
        BasicDataType::Void => {
          let (size, align) = (std::mem::size_of::<()>(), std::mem::align_of::<()>());
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          rs_struct.insert(field, RsArgsValue::Void(()));
          offset += size + padding;
          field_size = size
        }
        BasicDataType::String => {
          let (size, align) = (
            std::mem::size_of::<*const char>(),
            std::mem::align_of::<*const char>(),
          );
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut *mut c_char;
          let js_string = CStr::from_ptr(*type_field_ptr)
            .to_string_lossy()
            .to_string();
          rs_struct.insert(field, RsArgsValue::String(js_string));
          offset += size + padding;
          field_size = size
        }
        BasicDataType::External => {
          let (size, align) = (
            std::mem::size_of::<*const c_void>(),
            std::mem::align_of::<*const c_void>(),
          );
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut *mut c_void;
          rs_struct.insert(
            field,
            RsArgsValue::External(env.create_external(*type_field_ptr, None).unwrap()),
          );
          offset += size + padding;
          field_size = size
        }
      };
    }
    if let RsArgsValue::Object(obj) = val {
      let field = field.clone();
      let array_desc = get_array_desc(obj);
      if array_desc.is_some() {
        // array
        let (array_len, array_type) = array_desc.unwrap();
        let size = match array_type {
          RefDataType::StringArray => {
            let (size, align) = (
              std::mem::size_of::<*const c_void>(),
              std::mem::align_of::<*const c_void>(),
            );
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            let type_field_ptr = field_ptr as *mut *mut *mut c_char;
            let arr = create_array_from_pointer(*type_field_ptr, array_len);
            rs_struct.insert(field, RsArgsValue::StringArray(arr));
            offset += size + padding;
            field_size = size
          }
          RefDataType::DoubleArray => {
            let (size, align) = (
              std::mem::size_of::<*const c_void>(),
              std::mem::align_of::<*const c_void>(),
            );
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            let type_field_ptr = field_ptr as *mut *mut c_double;
            let arr = create_array_from_pointer(*type_field_ptr, array_len);
            rs_struct.insert(field, RsArgsValue::DoubleArray(arr));
            offset += size + padding;
            field_size = size
          }
          RefDataType::I32Array => {
            let (size, align) = (
              std::mem::size_of::<*const c_void>(),
              std::mem::align_of::<*const c_void>(),
            );
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            let type_field_ptr = field_ptr as *mut *mut c_int;
            let arr = create_array_from_pointer(*type_field_ptr, array_len);
            rs_struct.insert(field, RsArgsValue::I32Array(arr));
            offset += size + padding;
            field_size = size
          }
          RefDataType::U8Array => {
            let (size, align) = (
              std::mem::size_of::<*const c_void>(),
              std::mem::align_of::<*const c_void>(),
            );
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            let type_field_ptr = field_ptr as *mut *mut c_uchar;
            let arr = create_array_from_pointer(*type_field_ptr, array_len);
            rs_struct.insert(field, RsArgsValue::U8Array(arr));
            offset += size + padding;
            field_size = size
          }
        };
        size
      } else {
        // function | raw object
        let (size, align) = (
          std::mem::size_of::<*const c_void>(),
          std::mem::align_of::<*const c_void>(),
        );
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        let type_field_ptr = field_ptr as *mut *mut c_void;
        rs_struct.insert(
          field,
          RsArgsValue::Object(create_rs_struct_from_pointer(env, *type_field_ptr, obj)),
        );
        offset += size + padding;
        field_size = size
      };
    };
    field_ptr = field_ptr.offset(field_size as isize) as *mut c_void;
  }
  rs_struct
}

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
              let js_array: JsObject = params_value_object.get_named_property(&field).unwrap();
              let arg_val: Vec<u32> = js_array_to_number_array(js_array);
              RsArgsValue::U8Array(arg_val.into_iter().map(|item| item as u8).collect())
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

// describe paramsType or retType, field can only be number or object
pub fn type_define_to_rs_struct(params_type: &JsObject) -> IndexMap<String, RsArgsValue> {
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
          let map = type_define_to_rs_struct(&args_type);
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

pub fn get_array_desc(obj: &IndexMap<String, RsArgsValue>) -> Option<(usize, RefDataType)> {
  if obj.get(ARRAY_LENGTH_TAG).is_none() {
    return None;
  }
  let (mut array_len, mut array_type) = (0, 0);
  if let RsArgsValue::I32(number) = obj.get(ARRAY_LENGTH_TAG).unwrap() {
    array_len = *number as usize
  }
  if let RsArgsValue::I32(number) = obj.get(ARRAY_TYPE_TAG).unwrap() {
    array_type = *number
  }
  let array_type = number_to_ref_data_type(array_type);
  Some((array_len, array_type))
}
pub fn create_js_object_from_rs_map(
  env: &Env,
  rs_struct: IndexMap<String, RsArgsValue>,
) -> JsObject {
  let mut js_object = env.create_object().unwrap();
  for (field, value) in rs_struct {
    js_object
      .set_property(
        env.create_string(&field).unwrap(),
        rs_value_to_js_unknown(&env, value),
      )
      .unwrap();
  }
  js_object
}
pub fn rs_value_to_js_unknown(env: &Env, data: RsArgsValue) -> JsUnknown {
  return match data {
    RsArgsValue::U8(number) => env.create_uint32(number as u32).unwrap().into_unknown(),
    RsArgsValue::I32(number) => env.create_int32(number).unwrap().into_unknown(),
    RsArgsValue::I64(number) => env.create_int64(number).unwrap().into_unknown(),
    RsArgsValue::Boolean(val) => env.get_boolean(val).unwrap().into_unknown(),
    RsArgsValue::String(val) => env.create_string(&val).unwrap().into_unknown(),
    RsArgsValue::Double(val) => env.create_double(val).unwrap().into_unknown(),
    RsArgsValue::U8Array(val) => rs_array_to_js_array(env, ArrayType::U8(val)).into_unknown(),
    RsArgsValue::I32Array(val) => rs_array_to_js_array(env, ArrayType::I32(val)).into_unknown(),
    RsArgsValue::StringArray(val) => {
      rs_array_to_js_array(env, ArrayType::String(val)).into_unknown()
    }
    RsArgsValue::DoubleArray(val) => {
      rs_array_to_js_array(env, ArrayType::Double(val)).into_unknown()
    }
    RsArgsValue::Object(obj) => create_js_object_from_rs_map(env, obj).into_unknown(),
    RsArgsValue::External(val) => val.into_unknown(),
    RsArgsValue::Void(_) => panic!("void cannot be as a call param type"),
    RsArgsValue::Function(_, _) => panic!("function need to be improved"),
  };
}
