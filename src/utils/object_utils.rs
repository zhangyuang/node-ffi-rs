use super::dataprocess::get_ffi_tag;
use super::get_array_desc;

use crate::define::*;
use crate::{RefDataType, RsArgsValue, FFIARRARYDESC};
use indexmap::IndexMap;
use std::ffi::{c_char, c_double, c_float, c_int, c_longlong, c_short, c_uchar, c_void};
use widestring::WideChar;
pub fn get_size_align<T: Sized>() -> (usize, usize) {
  (std::mem::size_of::<T>(), std::mem::align_of::<T>())
}

#[macro_export]
macro_rules! calculate_layout_for {
  ($variant:ident, $type:ty) => {
    pub fn $variant(size: usize, align: usize, offset: usize) -> (usize, usize, usize) {
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
calculate_layout_for!(calculate_i16, c_short);
calculate_layout_for!(calculate_i32, c_int);
calculate_layout_for!(calculate_u32, u32);
calculate_layout_for!(calculate_i64, c_longlong);
calculate_layout_for!(calculate_float, c_float);
calculate_layout_for!(calculate_double, c_double);
calculate_layout_for!(calculate_boolean, bool);
calculate_layout_for!(calculate_void, ());
calculate_layout_for!(calculate_string, *const c_char);
calculate_layout_for!(calculate_w_string, *const WideChar);
calculate_layout_for!(calculate_pointer, *const c_void);

pub fn calculate_struct_size(struct_type: &IndexMap<String, RsArgsValue>) -> (usize, usize) {
  let (mut size, align, _) = struct_type.iter().fold(
    (0, 0, 0),
    |(size, align, offset), (field_name, field_type)| {
      if field_name == FFI_TAG_FIELD {
        return (size, align, offset);
      }
      if let RsArgsValue::I32(field_type_number) = field_type {
        return match (*field_type_number).try_into().unwrap() {
          BasicDataType::U8 => calculate_u8(size, align, offset),
          BasicDataType::I16 => calculate_i16(size, align, offset),
          BasicDataType::I32 => calculate_i32(size, align, offset),
          BasicDataType::U32 => calculate_u32(size, align, offset),
          BasicDataType::I64 | BasicDataType::U64 | BasicDataType::BigInt => {
            calculate_i64(size, align, offset)
          }
          BasicDataType::Float => calculate_float(size, align, offset),
          BasicDataType::Double => calculate_double(size, align, offset),
          BasicDataType::String => calculate_string(size, align, offset),
          BasicDataType::WString => calculate_w_string(size, align, offset),
          BasicDataType::Boolean => calculate_boolean(size, align, offset),
          BasicDataType::Void => calculate_void(size, align, offset),
          BasicDataType::External => calculate_pointer(size, align, offset),
        };
      } else if let RsArgsValue::Object(obj) = field_type {
        if let FFITypeTag::StackArray = get_ffi_tag(obj) {
          let array_desc = get_array_desc(obj);
          let FFIARRARYDESC {
            array_type,
            array_len,
            struct_item_type,
            ..
          } = array_desc;
          let (mut type_size, type_align) = match array_type {
            RefDataType::U8Array => get_size_align::<u8>(),
            RefDataType::I16Array => get_size_align::<i16>(),
            RefDataType::I32Array => get_size_align::<i32>(),
            RefDataType::FloatArray => get_size_align::<f32>(),
            RefDataType::StringArray => get_size_align::<*const c_char>(),
            RefDataType::DoubleArray => get_size_align::<f64>(),
            RefDataType::StructArray => calculate_struct_size(struct_item_type.as_ref().unwrap()),
          };
          type_size = type_size * array_len;
          let align = align.max(type_align);
          let padding = (type_align - (offset % type_align)) % type_align;
          let size = size + padding + type_size;
          let offset = offset + padding + type_size;
          return (size, align, offset);
        } else if let FFITypeTag::Array = get_ffi_tag(obj) {
          return calculate_pointer(size, align, offset);
        } else if get_ffi_tag(obj) == FFITypeTag::StackStruct {
          let (type_size, type_align) = calculate_struct_size(obj);
          let align = align.max(type_align);
          let padding = (type_align - (offset % type_align)) % type_align;
          let size = size + padding + type_size;
          let offset = offset + padding + type_size;
          return (size, align, offset);
        } else {
          return calculate_pointer(size, align, offset);
        }
      } else {
        panic!("unknown struct type {:?}", field_type)
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

pub unsafe fn create_static_array_from_pointer(
  ptr: *mut c_void,
  array_desc: &FFIARRARYDESC,
) -> RsArgsValue {
  let FFIARRARYDESC {
    array_type,
    array_len,
    ..
  } = array_desc;
  match array_type {
    RefDataType::U8Array => {
      let ptr = ptr as *mut u8;
      let arr = (0..*array_len).map(|n| *(ptr.offset(n as isize))).collect();
      RsArgsValue::U8Array(None, Some(arr))
    }
    RefDataType::I16Array => {
      let ptr = ptr as *mut i16;
      let arr = (0..*array_len).map(|n| *(ptr.offset(n as isize))).collect();
      RsArgsValue::I16Array(arr)
    }
    RefDataType::I32Array => {
      let ptr = ptr as *mut i32;
      let arr = (0..*array_len).map(|n| *(ptr.offset(n as isize))).collect();
      RsArgsValue::I32Array(arr)
    }
    RefDataType::DoubleArray => {
      let ptr = ptr as *mut f64;
      let arr = (0..*array_len).map(|n| *(ptr.offset(n as isize))).collect();
      RsArgsValue::DoubleArray(arr)
    }
    RefDataType::FloatArray => {
      let ptr = ptr as *mut f32;
      let arr = (0..*array_len).map(|n| *(ptr.offset(n as isize))).collect();
      RsArgsValue::FloatArray(arr)
    }
    RefDataType::StructArray => {
      panic!("struct array is not supported for static array");
    }
    RefDataType::StringArray => {
      panic!("string array is not supported for static array");
      // let ptr = ptr as *mut *const c_char;
      // let arr = (0..*array_len)
      //   .map(|n| {
      //     std::ffi::CStr::from_ptr(*(ptr.offset(n as isize)))
      //       .to_string_lossy()
      //       .to_string()
      //   })
      //   .collect();
      // RsArgsValue::StringArray(arr)
    }
  }
}
