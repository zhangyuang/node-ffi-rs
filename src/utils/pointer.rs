use super::dataprocess::get_ffi_tag;
use super::object_utils::calculate_struct_size;
use crate::define::*;
use libffi_sys::{
  ffi_type, ffi_type_double, ffi_type_enum_STRUCT, ffi_type_float, ffi_type_pointer,
  ffi_type_sint16, ffi_type_sint32, ffi_type_sint64, ffi_type_uint64, ffi_type_uint8,
  ffi_type_void,
};
pub unsafe fn get_ffi_type(
  ret_type_rs: &RsArgsValue,
  ffi_type_cleanup: &mut FFITypeCleanup,
) -> *mut ffi_type {
  match ret_type_rs {
    RsArgsValue::I32(number) => {
      let ret_data_type = (*number).try_into().unwrap();
      match ret_data_type {
        BasicDataType::U8 => &mut ffi_type_uint8 as *mut ffi_type,
        BasicDataType::I32 => &mut ffi_type_sint32 as *mut ffi_type,
        BasicDataType::I16 => &mut ffi_type_sint16 as *mut ffi_type,
        BasicDataType::I64 | BasicDataType::BigInt => &mut ffi_type_sint64 as *mut ffi_type,
        BasicDataType::U64 => &mut ffi_type_uint64 as *mut ffi_type,
        BasicDataType::String | BasicDataType::WString => &mut ffi_type_pointer as *mut ffi_type,
        BasicDataType::Void => &mut ffi_type_void as *mut ffi_type,
        BasicDataType::Float => &mut ffi_type_float as *mut ffi_type,
        BasicDataType::Double => &mut ffi_type_double as *mut ffi_type,
        BasicDataType::Boolean => &mut ffi_type_uint8 as *mut ffi_type,
        BasicDataType::External => &mut ffi_type_pointer as *mut ffi_type,
      }
    }
    RsArgsValue::Object(struct_type) => {
      if get_ffi_tag(struct_type) == FFITypeTag::StackStruct {
        let mut elements: Vec<*mut ffi_type> = struct_type
          .iter()
          .filter(|(field_name, _)| field_name != &FFI_TAG_FIELD)
          .map(|(_, field_type)| get_ffi_type(field_type, ffi_type_cleanup))
          .collect();
        elements.push(std::ptr::null_mut());
        let (size, align) = calculate_struct_size(struct_type);
        let struct_type_box = ffi_type {
          size,
          alignment: align as u16,
          type_: ffi_type_enum_STRUCT as u16,
          elements: elements.as_mut_ptr(),
        };
        let elements_ptr = Box::into_raw(Box::new(elements));
        let struct_type_ptr = Box::into_raw(Box::new(struct_type_box));
        ffi_type_cleanup.elements_box = Some(elements_ptr);
        ffi_type_cleanup.struct_type_box = Some(struct_type_ptr);
        struct_type_ptr
      } else {
        &mut ffi_type_pointer as *mut ffi_type
      }
    }
    _ => &mut ffi_type_void as *mut ffi_type,
  }
}
