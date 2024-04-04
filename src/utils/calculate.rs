use crate::define::RsArgsValue;
use indexmap::IndexMap;
use libc::c_void;
use std::ffi::{c_char, c_double, c_int};

macro_rules! calculate_layout_for {
  ($variant:ident, $type:ty) => {
    fn $variant(size: usize, align: usize, offset: usize) -> (usize, usize, usize) {
      let type_align = std::mem::align_of::<$type>();
      let align = align.max(type_align);
      let padding = (type_align - (offset % type_align)) % type_align;
      let size = size + padding + std::mem::size_of::<$type>();
      let offset = offset + padding + std::mem::size_of::<$type>();
      (size, align, offset)
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
calculate_layout_for!(calculate_object, *const c_void);

pub fn calculate_layout(map: &IndexMap<String, RsArgsValue>) -> (usize, usize) {
  let (size, align, _) =
    map.iter().fold(
      (0, 0, 0),
      |(size, align, offset), (_, field_val)| match field_val {
        RsArgsValue::I32(_) => calculate_i32(size, align, offset),
        RsArgsValue::Double(_) => calculate_double(size, align, offset),
        RsArgsValue::String(_) => calculate_string(size, align, offset),
        RsArgsValue::Boolean(_) => calculate_boolean(size, align, offset),
        RsArgsValue::Void(_) => calculate_void(size, align, offset),
        RsArgsValue::Object(_) => calculate_object(size, align, offset),
        RsArgsValue::StringArray(_) => calculate_string_array(size, align, offset),
        RsArgsValue::DoubleArray(_) => calculate_double_array(size, align, offset),
        RsArgsValue::I32Array(_) => calculate_i32_array(size, align, offset),
        RsArgsValue::Function(_, _) => {
          panic!("{:?} calculate_layout error", field_val)
        }
      },
    );
  (size, align)
}
