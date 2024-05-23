use crate::{RefDataType, RsArgsValue, FFIARRARYDESC};
use std::ffi::{c_char, c_double, c_float, c_int, c_longlong, c_uchar, c_void};

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
calculate_layout_for!(calculate_i32, c_int);
calculate_layout_for!(calculate_i64, c_longlong);
calculate_layout_for!(calculate_float, c_float);
calculate_layout_for!(calculate_double, c_double);
calculate_layout_for!(calculate_boolean, bool);
calculate_layout_for!(calculate_void, ());
calculate_layout_for!(calculate_string, *const c_char);
calculate_layout_for!(calculate_pointer, *const c_void);

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
    _ => panic!(
      "{:?} type transform to static array is unsupported at now",
      array_type
    ),
  }
}
