use crate::define::*;
use indexmap::IndexMap;
use std::ffi::{c_char, c_double, c_int, CStr};
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
pub fn get_rs_struct_size_align(data_type: &RsArgsValue) -> (usize, usize) {
  return match data_type {
    RsArgsValue::I32(_) => (std::mem::size_of::<i32>(), std::mem::align_of::<i32>()),
    RsArgsValue::Void(_) => (0, 0),
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
    RsArgsValue::Object(obj) => {
      let mut size = 0;
      let mut align = 0;
      for (_, val) in obj {
        size += get_rs_struct_size_align(val).0;
        align += get_rs_struct_size_align(val).1;
      }
      (size, align)
    }
    RsArgsValue::Function(_, _) => {
      panic!("{:?} Not available as a field type at this time", data_type)
    }
  };
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
calculate_layout_for!(calculate_void, ());
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
      RsArgsValue::Void(_) => calculate_void(size, align),
      RsArgsValue::Object(val) => {
        let (obj_size, obj_align) = calculate_layout(val);
        let align = align.max(obj_align);
        let size = size + obj_size;
        (size, align)
      }
      RsArgsValue::StringArray(_) => calculate_string_array(size, align),
      RsArgsValue::DoubleArray(_) => calculate_double_array(size, align),
      RsArgsValue::I32Array(_) => calculate_i32_array(size, align),
      RsArgsValue::Function(_, _) => {
        panic!("{:?} calculate_layout error", field_val)
      }
    });
  (size, align)
}
