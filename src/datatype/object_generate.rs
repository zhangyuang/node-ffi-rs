use super::array::*;
use super::buffer::*;
use super::pointer::*;
use crate::utils::dataprocess::get_array_desc;
use crate::utils::dataprocess::get_ffi_tag;
use crate::utils::object_utils::{create_static_array_from_pointer, get_size_align};

use crate::define::*;
use indexmap::IndexMap;
use libc::c_float;
use napi::{Env, JsObject, JsUnknown, Result};
use std::ffi::c_void;
use std::ffi::{c_char, c_double, c_int, c_longlong, c_uchar, c_ulonglong, CStr};

pub unsafe fn create_rs_struct_from_pointer(
  env: &Env,
  ptr: *mut c_void,
  ret_object: &IndexMap<String, RsArgsValue>,
  need_thread_safe: bool,
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
          let (size, align) = get_size_align::<c_uchar>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut c_uchar;
          rs_struct.insert(field, RsArgsValue::U8(*type_field_ptr));
          offset += size + padding;
          field_size = size
        }
        BasicDataType::I32 => {
          let (size, align) = get_size_align::<c_int>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut c_int;
          rs_struct.insert(field, RsArgsValue::I32(*type_field_ptr));
          offset += size + padding;
          field_size = size
        }
        BasicDataType::I64 => {
          let (size, align) = get_size_align::<c_longlong>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut c_longlong;
          rs_struct.insert(field, RsArgsValue::I64(*type_field_ptr));
          offset += size + padding;
          field_size = size
        }
        BasicDataType::U64 => {
          let (size, align) = get_size_align::<c_ulonglong>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut c_ulonglong;
          rs_struct.insert(field, RsArgsValue::U64(*type_field_ptr));
          offset += size + padding;
          field_size = size
        }
        BasicDataType::Float => {
          let (size, align) = get_size_align::<c_float>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut c_float;
          rs_struct.insert(field, RsArgsValue::Float(*type_field_ptr));
          offset += size + padding;
          field_size = size
        }
        BasicDataType::Double => {
          let (size, align) = get_size_align::<c_double>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut c_double;
          rs_struct.insert(field, RsArgsValue::Double(*type_field_ptr));
          offset += size + padding;
          field_size = size
        }
        BasicDataType::Boolean => {
          let (size, align) = get_size_align::<bool>();
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
          let (size, align) = get_size_align::<*const c_void>();
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
          let (size, align) = get_size_align::<*const c_void>();
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
      if let FFITag::Array = get_ffi_tag(obj) {
        let array_desc = get_array_desc(obj);
        // array
        let FFIARRARYDESC {
          array_type,
          array_len,
          dynamic_array,
          ..
        } = &array_desc;
        match array_type {
          RefDataType::StringArray => {
            let (size, align) = get_size_align::<*const c_void>();
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            let type_field_ptr = field_ptr as *mut *mut *mut c_char;
            let arr = create_array_from_pointer(*type_field_ptr, *array_len);
            rs_struct.insert(field, RsArgsValue::StringArray(arr));
            offset += size + padding;
            field_size = size
          }
          RefDataType::DoubleArray => {
            let (size, align) = if *dynamic_array {
              get_size_align::<*const c_void>()
            } else {
              let (size, align) = get_size_align::<c_double>();
              (size * array_len, align)
            };
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            if *dynamic_array {
              let type_field_ptr = field_ptr as *mut *mut c_double;
              let arr = create_array_from_pointer(*type_field_ptr, *array_len);
              rs_struct.insert(field, RsArgsValue::DoubleArray(arr));
            } else {
              let arr = create_static_array_from_pointer(field_ptr as *mut c_void, &array_desc);
              rs_struct.insert(field, arr);
            }
            offset += size + padding;
            field_size = size
          }
          RefDataType::FloatArray => {
            let (size, align) = if *dynamic_array {
              get_size_align::<*const c_void>()
            } else {
              let (size, align) = get_size_align::<c_double>();
              (size * array_len, align)
            };
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            if *dynamic_array {
              let type_field_ptr = field_ptr as *mut *mut c_float;
              let arr = create_array_from_pointer(*type_field_ptr, *array_len);
              rs_struct.insert(field, RsArgsValue::FloatArray(arr));
            } else {
              let arr = create_static_array_from_pointer(field_ptr as *mut c_void, &array_desc);
              rs_struct.insert(field, arr);
            }
            offset += size + padding;
            field_size = size
          }
          RefDataType::I32Array => {
            let (size, align) = if *dynamic_array {
              get_size_align::<*const c_void>()
            } else {
              let (size, align) = get_size_align::<c_int>();
              (size * array_len, align)
            };
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            if *dynamic_array {
              let type_field_ptr = field_ptr as *mut *mut c_int;
              let arr = create_array_from_pointer(*type_field_ptr, *array_len);
              rs_struct.insert(field, RsArgsValue::I32Array(arr));
            } else {
              let arr = create_static_array_from_pointer(field_ptr as *mut c_void, &array_desc);
              rs_struct.insert(field, arr);
            }
            offset += size + padding;
            field_size = size
          }
          RefDataType::U8Array => {
            let (size, align) = if *dynamic_array {
              get_size_align::<*const c_void>()
            } else {
              let (size, align) = get_size_align::<u8>();
              (size * array_len, align)
            };
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            if *dynamic_array {
              let type_field_ptr = field_ptr as *mut *mut c_uchar;
              let arr = create_array_from_pointer(*type_field_ptr, array_desc.array_len);
              rs_struct.insert(field, get_safe_buffer(env, arr, need_thread_safe));
            } else {
              let arr = create_static_array_from_pointer(field_ptr as *mut c_void, &array_desc);
              if let RsArgsValue::U8Array(_, arr) = arr {
                rs_struct.insert(field, get_safe_buffer(env, arr.unwrap(), need_thread_safe));
              }
            }
            offset += size + padding;
            field_size = size
          }
        };
      } else {
        // function | raw object
        let (size, align) = get_size_align::<*const c_void>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        let type_field_ptr = field_ptr as *mut *mut c_void;
        rs_struct.insert(
          field,
          RsArgsValue::Object(create_rs_struct_from_pointer(
            env,
            *type_field_ptr,
            obj,
            need_thread_safe,
          )),
        );
        offset += size + padding;
        field_size = size
      };
    };
    field_ptr = field_ptr.offset(field_size as isize) as *mut c_void;
  }
  rs_struct
}

pub fn create_js_object_from_rs_map(
  env: &Env,
  rs_struct: IndexMap<String, RsArgsValue>,
) -> Result<JsObject> {
  let mut js_object = env.create_object()?;
  for (field, value) in rs_struct {
    js_object
      .set_property(
        env.create_string(&field)?,
        rs_value_to_js_unknown(&env, value)?,
      )
      .unwrap();
  }
  Ok(js_object)
}
pub fn rs_value_to_js_unknown(env: &Env, data: RsArgsValue) -> Result<JsUnknown> {
  let res = match data {
    RsArgsValue::U8(number) => env.create_uint32(number as u32)?.into_unknown(),
    RsArgsValue::I32(number) => env.create_int32(number)?.into_unknown(),
    RsArgsValue::I64(number) => env.create_int64(number)?.into_unknown(),
    RsArgsValue::U64(number) => env.create_int64(number as i64)?.into_unknown(),
    RsArgsValue::Boolean(val) => env.get_boolean(val)?.into_unknown(),
    RsArgsValue::String(val) => env.create_string(&val)?.into_unknown(),
    RsArgsValue::Double(val) => env.create_double(val)?.into_unknown(),
    RsArgsValue::U8Array(buffer, arr) => {
      if buffer.is_some() {
        buffer.unwrap().into_unknown()
      } else {
        create_buffer_val(env, arr.unwrap()).into_unknown()
      }
    }
    RsArgsValue::I32Array(val) => val.to_js_array(env)?.into_unknown(),
    RsArgsValue::StringArray(val) => val.to_js_array(env)?.into_unknown(),
    RsArgsValue::DoubleArray(val) => val.to_js_array(env)?.into_unknown(),
    RsArgsValue::Object(obj) => create_js_object_from_rs_map(env, obj)?.into_unknown(),
    RsArgsValue::External(val) => val.into_unknown(),
    RsArgsValue::Void(_) => env.get_undefined()?.into_unknown(),
    RsArgsValue::Function(_, _) | RsArgsValue::Float(_) | RsArgsValue::FloatArray(_) => {
      return Err(FFIError::Panic(format!("{}", "JsNumber can only be double type")).into())
    }
  };
  Ok(res)
}
