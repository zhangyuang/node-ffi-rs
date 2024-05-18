use crate::define::*;
use crate::utils::dataprocess::{get_array_desc, get_ffi_tag, get_js_external_wrap_data};
use crate::utils::object_utils::get_size_align;
use crate::RefDataType;
use indexmap::IndexMap;
use libc::{c_float, c_ulonglong, c_void};
use napi::{Env, Result};
use std::alloc::{alloc, Layout};
use std::ffi::{c_char, c_double, c_int, c_longlong, c_uchar};

use super::string::string_to_c_string;

macro_rules! calculate_layout_for {
  ($variant:ident, $type:ty) => {
    fn $variant(size: usize, align: usize, offset: usize) -> (usize, usize, usize) {
      let (type_size, type_align) = get_size_align::<$type>();
      let align = align.max(type_align);
      let padding = (type_align - (offset % type_align)) % type_align;
      let size = size + padding + type_size;
      let offset = offset + padding + type_size;
      (size, align, offset)
    }
  };
}
calculate_layout_for!(calculate_u8, c_uchar);
calculate_layout_for!(calculate_i32, c_int);
calculate_layout_for!(calculate_i64, c_longlong);
calculate_layout_for!(calculate_float, c_float);
calculate_layout_for!(calculate_double, c_double);
calculate_layout_for!(calculate_boolean, bool);
calculate_layout_for!(calculate_void, ());
calculate_layout_for!(calculate_string, *const c_char);
calculate_layout_for!(calculate_pointer, *const c_void);

pub fn calculate_struct_size(map: &IndexMap<String, RsArgsValue>) -> (usize, usize) {
  let (mut size, align, _) =
    map.iter().fold(
      (0, 0, 0),
      |(size, align, offset), (_, field_val)| match field_val {
        RsArgsValue::U8(_) => calculate_u8(size, align, offset),
        RsArgsValue::I32(_) => calculate_i32(size, align, offset),
        RsArgsValue::I64(_) | RsArgsValue::U64(_) => calculate_i64(size, align, offset),
        RsArgsValue::Float(_) => calculate_float(size, align, offset),
        RsArgsValue::Double(_) => calculate_double(size, align, offset),
        RsArgsValue::String(_) => calculate_string(size, align, offset),
        RsArgsValue::Boolean(_) => calculate_boolean(size, align, offset),
        RsArgsValue::Void(_) => calculate_void(size, align, offset),
        RsArgsValue::Object(obj) => {
          if let FFITag::Array = get_ffi_tag(obj) {
            let array_desc = get_array_desc(obj);
            let FFIARRARYDESC {
              array_type,
              array_len,
              dynamic_array,
              ..
            } = array_desc;
            if !dynamic_array {
              let (mut type_size, type_align) = match array_type {
                RefDataType::U8Array => get_size_align::<u8>(),
                RefDataType::I32Array => get_size_align::<i32>(),
                RefDataType::DoubleArray => get_size_align::<f64>(),
                _ => panic!(
                  "write {:?} to static array in struct is unsupported",
                  array_type
                ),
              };
              type_size = type_size * array_len;
              let align = align.max(type_align);
              let padding = (type_align - (offset % type_align)) % type_align;
              let size = size + padding + type_size;
              let offset = offset + padding + type_size;
              (size, align, offset)
            } else {
              calculate_pointer(size, align, offset)
            }
          } else {
            calculate_pointer(size, align, offset)
          }
        }
        RsArgsValue::StringArray(_)
        | RsArgsValue::DoubleArray(_)
        | RsArgsValue::FloatArray(_)
        | RsArgsValue::I32Array(_)
        | RsArgsValue::U8Array(_, _)
        | RsArgsValue::External(_) => calculate_pointer(size, align, offset),
        RsArgsValue::Function(_, _) => {
          panic!("{:?} calculate_layout error", field_val)
        }
      },
    );
  let padding = if align > 0 && size % align != 0 {
    align - (size % align)
  } else {
    0
  };
  size += padding;
  (size, align)
}

pub unsafe fn generate_c_struct(
  env: &Env,
  map: IndexMap<String, RsArgsValue>,
) -> Result<*mut c_void> {
  let (size, align) = calculate_struct_size(&map);
  let layout = if size > 0 {
    Layout::from_size_align(size, align).unwrap()
  } else {
    Layout::new::<i32>()
  };
  let ptr = alloc(layout) as *mut c_void;
  let mut field_ptr = ptr;
  let mut offset = 0;
  for (_, field_val) in map {
    let field_size = match field_val {
      RsArgsValue::U8(number) => {
        let (size, align) = get_size_align::<c_uchar>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut c_uchar).write(number);
        offset += size + padding;
        size
      }
      RsArgsValue::I32(number) => {
        let (size, align) = get_size_align::<c_int>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut c_int).write(number);
        offset += size + padding;
        size
      }
      RsArgsValue::I64(number) => {
        let (size, align) = get_size_align::<c_longlong>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut c_longlong).write(number);
        offset += size + padding;
        size
      }
      RsArgsValue::U64(number) => {
        let (size, align) = get_size_align::<c_ulonglong>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut c_ulonglong).write(number);
        offset += size + padding;
        size
      }
      RsArgsValue::Float(number) => {
        let (size, align) = get_size_align::<c_float>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut c_float).write(number);
        offset += size + padding;
        size
      }
      RsArgsValue::Double(double_number) => {
        let (size, align) = get_size_align::<c_double>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut c_double).write(double_number);
        offset += size + padding;
        size
      }
      RsArgsValue::Boolean(val) => {
        let (size, align) = get_size_align::<bool>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut bool).write(val);
        offset += size + padding;
        size
      }
      RsArgsValue::String(str) => {
        let (size, align) = get_size_align::<*mut c_void>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        let c_string = string_to_c_string(str);
        (field_ptr as *mut *const c_char).write(c_string.as_ptr());
        std::mem::forget(c_string);
        offset += size + padding;
        size
      }
      RsArgsValue::External(val) => {
        let (size, align) = get_size_align::<*mut c_void>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut *const c_void).write(get_js_external_wrap_data(&env, val)?);
        offset += size + padding;
        size
      }
      RsArgsValue::Void(_) => {
        let (size, align) = get_size_align::<()>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut ()).write(());
        offset += size + padding;
        size
      }
      RsArgsValue::Object(mut val) => {
        if let FFITag::Array = get_ffi_tag(&val) {
          let array_desc = get_array_desc(&mut val);
          let FFIARRARYDESC {
            array_type,
            array_len,
            array_value,
            dynamic_array,
            ..
          } = array_desc;
          let array_value = array_value.unwrap();
          let field_size = match array_type {
            RefDataType::U8Array => {
              if let RsArgsValue::U8Array(buffer, _) = array_value {
                let buffer = buffer.as_ref().unwrap();
                if !dynamic_array {
                  let (size, align) = get_size_align::<u8>();
                  let field_size = size * array_len;
                  let padding = (align - (offset % align)) % align;
                  field_ptr = field_ptr.offset(padding as isize);
                  std::ptr::copy(buffer.as_ptr(), field_ptr as *mut u8, array_len);
                  offset += field_size + padding;
                  field_size
                } else {
                  let (size, align) = get_size_align::<*mut c_void>();
                  let padding = (align - (offset % align)) % align;
                  field_ptr = field_ptr.offset(padding as isize);
                  (field_ptr as *mut *const c_uchar).write(buffer.as_ptr());
                  offset += size + padding;
                  size
                }
              } else {
                return Err(FFIError::Panic(format!("error array type {:?}", array_type)).into());
              }
            }
            RefDataType::I32Array => {
              if let RsArgsValue::I32Array(arr) = array_value.clone() {
                if !dynamic_array {
                  let (size, align) = get_size_align::<i32>();
                  let field_size = size * array_len;
                  let padding = (align - (offset % align)) % align;
                  field_ptr = field_ptr.offset(padding as isize);
                  std::ptr::copy(arr.as_ptr(), field_ptr as *mut i32, array_len);
                  offset += field_size + padding;
                  field_size
                } else {
                  let (size, align) = get_size_align::<*mut c_void>();
                  let padding = (align - (offset % align)) % align;
                  field_ptr = field_ptr.offset(padding as isize);
                  (field_ptr as *mut *const c_int).write(arr.as_ptr());
                  std::mem::forget(arr);
                  offset += size + padding;
                  size
                }
              } else {
                return Err(FFIError::Panic(format!("error array type {:?}", array_type)).into());
              }
            }
            RefDataType::DoubleArray => {
              if let RsArgsValue::DoubleArray(arr) = array_value.clone() {
                if !dynamic_array {
                  let (size, align) = get_size_align::<f64>();
                  let field_size = size * array_len;
                  let padding = (align - (offset % align)) % align;
                  field_ptr = field_ptr.offset(padding as isize);
                  std::ptr::copy(arr.as_ptr(), field_ptr as *mut f64, array_len);
                  offset += field_size + padding;
                  field_size
                } else {
                  let (size, align) = get_size_align::<*mut c_void>();
                  let padding = (align - (offset % align)) % align;
                  field_ptr = field_ptr.offset(padding as isize);
                  (field_ptr as *mut *const c_double).write(arr.as_ptr());
                  std::mem::forget(arr);
                  offset += size + padding;
                  size
                }
              } else {
                return Err(FFIError::Panic(format!("error array type {:?}", array_type)).into());
              }
            }
            RefDataType::FloatArray => {
              if let RsArgsValue::FloatArray(arr) = array_value.clone() {
                if !dynamic_array {
                  let (size, align) = get_size_align::<f32>();
                  let field_size = size * array_len;
                  let padding = (align - (offset % align)) % align;
                  field_ptr = field_ptr.offset(padding as isize);
                  std::ptr::copy(arr.as_ptr(), field_ptr as *mut f32, array_len);
                  offset += field_size + padding;
                  field_size
                } else {
                  let (size, align) = get_size_align::<*mut c_void>();
                  let padding = (align - (offset % align)) % align;
                  field_ptr = field_ptr.offset(padding as isize);
                  (field_ptr as *mut *const c_float).write(arr.as_ptr());
                  std::mem::forget(arr);
                  offset += size + padding;
                  size
                }
              } else {
                return Err(FFIError::Panic(format!("error array type {:?}", array_type)).into());
              }
            }
            RefDataType::StringArray => {
              if let RsArgsValue::StringArray(arr) = array_value.clone() {
                if !dynamic_array {
                  panic!(
                    "write {:?} to static array in struct is unsupported",
                    array_type
                  )
                } else {
                  let (size, align) = get_size_align::<*mut c_void>();
                  let padding = (align - (offset % align)) % align;
                  field_ptr = field_ptr.offset(padding as isize);
                  let c_char_vec: Vec<*const c_char> = arr
                    .into_iter()
                    .map(|str| {
                      let c_string = string_to_c_string(str);
                      let ptr = c_string.as_ptr();
                      std::mem::forget(c_string);
                      ptr
                    })
                    .collect();
                  (field_ptr as *mut *const *const c_char).write(c_char_vec.as_ptr());
                  std::mem::forget(c_char_vec);
                  offset += size + padding;
                  size
                }
              } else {
                return Err(FFIError::Panic(format!("error array type {:?}", array_type)).into());
              }
            }
          };
          field_size
        } else {
          // raw object or function
          let (size, align) = get_size_align::<*mut c_void>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let obj_ptr = generate_c_struct(env, val)?;
          (field_ptr as *mut *const c_void).write(obj_ptr);
          offset += size + padding;
          size
        }
      }
      RsArgsValue::Function(_, _) => panic!("write_data error {:?}", field_val),
      RsArgsValue::StringArray(_)
      | RsArgsValue::FloatArray(_)
      | RsArgsValue::I32Array(_)
      | RsArgsValue::DoubleArray(_)
      | RsArgsValue::U8Array(_, _) => {
        return Err(
          FFIError::Panic(format!(
          "In the latest ffi-rs version, please use ffi-rs.arrayConstrutor to describe array type"
        ))
          .into(),
        )
      }
    };
    field_ptr = field_ptr.offset(field_size as isize);
  }
  Ok(ptr)
}
