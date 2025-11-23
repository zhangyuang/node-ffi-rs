use super::string::{string_to_c_string, string_to_c_w_string};
use crate::define::*;
use crate::utils::{
  calculate_struct_size, get_array_desc, get_ffi_tag, get_js_external_wrap_data, get_size_align,
};
use crate::RefDataType;
use indexmap::IndexMap;
use napi::{Env, Result};
use std::alloc::{alloc, Layout};
use std::ffi::{
  c_char, c_double, c_float, c_int, c_longlong, c_short, c_uchar, c_ulonglong, c_void,
};
use widestring::WideChar;

pub fn get_array_value(obj: &mut IndexMap<String, RsArgsValue>) -> Option<RsArgsValue> {
  obj.shift_remove(ARRAY_VALUE_TAG)
}

pub unsafe fn generate_c_struct(
  env: &Env,
  struct_type: &IndexMap<String, RsArgsValue>,
  struct_val: IndexMap<String, RsArgsValue>,
  initial_ptr: Option<*mut c_void>,
) -> Result<*mut c_void> {
  let ptr = if initial_ptr.is_none() {
    let (size, align) = calculate_struct_size(&struct_type);
    let layout = if size > 0 {
      Layout::from_size_align(size, align).unwrap()
    } else {
      Layout::new::<i32>()
    };
    alloc(layout) as *mut c_void
  } else {
    initial_ptr.unwrap()
  };
  let mut field_ptr = ptr;
  let mut offset = 0;
  for (field, field_val) in struct_val {
    let field_size = match field_val {
      RsArgsValue::U8(number) => {
        let (size, align) = get_size_align::<c_uchar>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut c_uchar).write(number);
        offset += size + padding;
        size
      }
      RsArgsValue::I16(number) => {
        let (size, align) = get_size_align::<c_short>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut c_short).write(number);
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
      RsArgsValue::U32(number) => {
        let (size, align) = get_size_align::<u32>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut u32).write(number);
        offset += size + padding;
        size
      }
      RsArgsValue::I64(number) | RsArgsValue::BigInt(number) => {
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
      RsArgsValue::WString(str) => {
        let (size, align) = get_size_align::<*mut c_void>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        let c_string = string_to_c_w_string(str);
        (field_ptr as *mut *const WideChar).write(c_string.as_ptr());
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
      RsArgsValue::Object(mut obj_value) => {
        if let FFITypeTag::Array | FFITypeTag::StackArray = get_ffi_tag(&obj_value) {
          let array_desc = get_array_desc(&mut obj_value);
          let array_value = get_array_value(&mut obj_value).unwrap();
          let FFIARRARYDESC {
            array_type,
            array_len,
            struct_item_type,
            ..
          } = array_desc;
          let field_size = match array_type {
            RefDataType::U8Array => {
              if let RsArgsValue::U8Array(buffer, _) = array_value {
                let buffer = buffer.unwrap();
                if get_ffi_tag(&obj_value) == FFITypeTag::StackArray {
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
            RefDataType::I16Array => {
              if let RsArgsValue::I16Array(arr) = array_value {
                if get_ffi_tag(&obj_value) == FFITypeTag::StackArray {
                  let (size, align) = get_size_align::<i16>();
                  let field_size = size * array_len;
                  let padding = (align - (offset % align)) % align;
                  field_ptr = field_ptr.offset(padding as isize);
                  std::ptr::copy(arr.as_ptr(), field_ptr as *mut i16, array_len);
                  offset += field_size + padding;
                  field_size
                } else {
                  let (size, align) = get_size_align::<*mut c_void>();
                  let padding = (align - (offset % align)) % align;
                  field_ptr = field_ptr.offset(padding as isize);
                  (field_ptr as *mut *const i16).write(arr.as_ptr());
                  std::mem::forget(arr);
                  offset += size + padding;
                  size
                }
              } else {
                return Err(FFIError::Panic(format!("error array type {:?}", array_type)).into());
              }
            }
            RefDataType::I32Array => {
              if let RsArgsValue::I32Array(arr) = array_value {
                if get_ffi_tag(&obj_value) == FFITypeTag::StackArray {
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
              if let RsArgsValue::DoubleArray(arr) = array_value {
                if get_ffi_tag(&obj_value) == FFITypeTag::StackArray {
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
              if let RsArgsValue::FloatArray(arr) = array_value {
                if get_ffi_tag(&obj_value) == FFITypeTag::StackArray {
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
              if let RsArgsValue::StringArray(arr) = array_value {
                if get_ffi_tag(&obj_value) == FFITypeTag::StackArray {
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
            RefDataType::StructArray => {
              let is_stack_struct =
                get_ffi_tag(struct_item_type.as_ref().unwrap()) == FFITypeTag::StackStruct;
              if let RsArgsValue::StructArray(arr) = array_value {
                if is_stack_struct {
                  let (size, align) = calculate_struct_size(struct_item_type.as_ref().unwrap());
                  let field_size = size * array_len;
                  arr.into_iter().for_each(|struct_val| {
                    let padding = (align - (offset % align)) % align;
                    field_ptr = field_ptr.offset(padding as isize);
                    generate_c_struct(
                      env,
                      struct_item_type.as_ref().unwrap(),
                      struct_val,
                      Some(field_ptr),
                    )
                    .unwrap();
                    field_ptr = field_ptr.offset(size as isize);
                    offset += size;
                  });
                  field_size
                } else {
                  let (size, align) = get_size_align::<*mut c_void>();
                  arr.into_iter().for_each(|struct_val| {
                    let padding = (align - (offset % align)) % align;
                    field_ptr = field_ptr.offset(padding as isize);
                    generate_c_struct(
                      env,
                      struct_item_type.as_ref().unwrap(),
                      struct_val,
                      Some(field_ptr),
                    )
                    .unwrap();
                    field_ptr = field_ptr.offset(1);
                    offset += size;
                  });
                  array_len * std::mem::size_of::<*mut c_void>()
                }
              } else {
                return Err(FFIError::Panic(format!("error array type {:?}", array_type)).into());
              }
            }
          };
          field_size
        } else {
          let is_stack_struct = match struct_type.get(&field) {
            Some(RsArgsValue::Object(field_type)) => {
              get_ffi_tag(field_type) == FFITypeTag::StackStruct
            }
            _ => get_ffi_tag(struct_type) == FFITypeTag::StackStruct,
          };
          // struct
          if is_stack_struct {
            // stack struct
            let target_type = if let Some(RsArgsValue::Object(val_type)) = struct_type.get(&field) {
              val_type
            } else if get_ffi_tag(struct_type) == FFITypeTag::StackStruct {
              struct_type
            } else {
              return Err(FFIError::Panic(format!("unknown field type {:?}", struct_type)).into());
            };

            let (size, align) = calculate_struct_size(target_type);
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            generate_c_struct(env, target_type, obj_value, Some(field_ptr))?;
            offset += size + padding;
            size
          } else {
            let (size, align) = get_size_align::<*mut c_void>();
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            if let RsArgsValue::Object(val_type) = struct_type.get(&field).unwrap() {
              let start_ptr = generate_c_struct(env, val_type, obj_value, None)?;
              (field_ptr as *mut *const c_void).write(start_ptr);
            }
            offset += size + padding;
            size
          }
        }
      }
      RsArgsValue::Function(_, _) => panic!("write_data error {:?}", field_val),
      RsArgsValue::StringArray(_)
      | RsArgsValue::FloatArray(_)
      | RsArgsValue::I16Array(_)
      | RsArgsValue::I32Array(_)
      | RsArgsValue::DoubleArray(_)
      | RsArgsValue::StructArray(_)
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
