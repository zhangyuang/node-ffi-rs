use crate::{RefDataType, RsArgsValue, FFIARRARYDESC};
use libc::{c_char, c_void};

pub fn get_size_align<T: Sized>() -> (usize, usize) {
  (std::mem::size_of::<T>(), std::mem::align_of::<T>())
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
