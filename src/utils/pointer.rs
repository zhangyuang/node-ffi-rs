use super::dataprocess::get_ffi_tag;
use super::object_utils::calculate_struct_size;
use crate::define::*;
use libffi_sys::{
  ffi_type, ffi_type_double, ffi_type_enum_STRUCT, ffi_type_float, ffi_type_pointer,
  ffi_type_sint16, ffi_type_sint32, ffi_type_sint64, ffi_type_uint32, ffi_type_uint64, ffi_type_uint8,
  ffi_type_void,
};
pub unsafe fn get_ffi_type(ret_type_rs: &RsArgsValue) -> Box<ffi_type> {
  match ret_type_rs {
    RsArgsValue::I32(number) => {
      let ret_data_type = (*number).try_into().unwrap();
      match ret_data_type {
        BasicDataType::U8 => Box::new(ffi_type_uint8),
        BasicDataType::I32 => Box::new(ffi_type_sint32),
        BasicDataType::I16 => Box::new(ffi_type_sint16),
        BasicDataType::U32 => Box::new(ffi_type_uint32),
        BasicDataType::I64 | BasicDataType::BigInt => Box::new(ffi_type_sint64),
        BasicDataType::U64 => Box::new(ffi_type_uint64),
        BasicDataType::String | BasicDataType::WString => Box::new(ffi_type_pointer),
        BasicDataType::Void => Box::new(ffi_type_void),
        BasicDataType::Float => Box::new(ffi_type_float),
        BasicDataType::Double => Box::new(ffi_type_double),
        BasicDataType::Boolean => Box::new(ffi_type_uint8),
        BasicDataType::External => Box::new(ffi_type_pointer),
      }
    }
    RsArgsValue::Object(struct_type) => {
      if get_ffi_tag(struct_type) == FFITypeTag::StackStruct {
        let mut elements: Vec<*mut ffi_type> = struct_type
          .iter()
          .filter(|(field_name, _)| field_name != &FFI_TAG_FIELD)
          .map(|(_, field_type)| Box::into_raw(get_ffi_type(field_type)))
          .collect();
        elements.push(std::ptr::null_mut());
        let (size, align) = calculate_struct_size(struct_type);
        let struct_type_box = Box::new(ffi_type {
          size,
          alignment: align as u16,
          type_: ffi_type_enum_STRUCT as u16,
          elements: elements.as_mut_ptr(),
        });
        let _ = Box::into_raw(Box::new(elements));
        struct_type_box
      } else {
        Box::new(ffi_type_pointer)
      }
    }
    _ => Box::new(ffi_type_void),
  }
}
