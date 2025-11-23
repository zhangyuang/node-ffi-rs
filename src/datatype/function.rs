use super::buffer::*;
use super::pointer::*;
use super::restore_struct::create_rs_struct_from_pointer;
use super::string::{create_c_string_from_ptr, create_c_w_string_from_ptr};
use crate::define::*;
use crate::utils::{get_array_desc, get_ffi_tag};
use libc::c_float;
use napi::Env;
use std::ffi::c_void;
use std::ffi::{c_char, c_double, c_int, c_uchar};
use widestring::WideChar;

pub unsafe fn get_rs_value_from_pointer(
  env: &Env,
  type_desc: &RsArgsValue,
  pointer: *mut c_void,
  need_thread_safe: bool,
) -> RsArgsValue {
  match type_desc {
    RsArgsValue::I32(number) => {
      let data = match (*number).try_into().unwrap() {
        BasicDataType::U8 => RsArgsValue::U8(*(pointer as *mut u8)),
        BasicDataType::I16 => RsArgsValue::I16(*(pointer as *mut i16)),
        BasicDataType::I32 => RsArgsValue::I32(*(pointer as *mut i32)),
        BasicDataType::U32 => RsArgsValue::U32(*(pointer as *mut u32)),
        BasicDataType::I64 => RsArgsValue::I64(*(pointer as *mut i64)),
        BasicDataType::BigInt => RsArgsValue::BigInt(*(pointer as *mut i64)),
        BasicDataType::U64 => RsArgsValue::U64(*(pointer as *mut u64)),
        BasicDataType::Float => RsArgsValue::Float(*(pointer as *mut f32)),
        BasicDataType::Double => RsArgsValue::Double(*(pointer as *mut f64)),
        BasicDataType::Boolean => RsArgsValue::Boolean(!*(pointer as *mut i32) == 0),
        BasicDataType::String => {
          RsArgsValue::String(create_c_string_from_ptr(*(pointer as *mut *mut c_char)))
        }
        BasicDataType::WString => {
          RsArgsValue::WString(create_c_w_string_from_ptr(*(pointer as *mut *mut WideChar)))
        }
        BasicDataType::External => RsArgsValue::External(
          env
            .create_external(
              *(pointer as *mut *mut c_void),
              Some(std::mem::size_of::<*mut c_void>() as i64),
            )
            .unwrap(),
        ),
        BasicDataType::Void => RsArgsValue::Void(()),
      };
      data
    }
    RsArgsValue::Object(obj) => {
      if let FFITypeTag::Array = get_ffi_tag(obj) {
        let array_desc = get_array_desc(obj);
        let FFIARRARYDESC {
          array_type,
          array_len,
          ..
        } = array_desc;
        match array_type {
          RefDataType::StringArray => {
            let arr = create_array_from_pointer(*(pointer as *mut *mut *mut c_char), array_len);
            RsArgsValue::StringArray(arr)
          }
          RefDataType::I16Array => {
            let arr = create_array_from_pointer(*(pointer as *mut *mut i16), array_len);
            RsArgsValue::I16Array(arr)
          }
          RefDataType::I32Array => {
            let arr = create_array_from_pointer(*(pointer as *mut *mut c_int), array_len);
            RsArgsValue::I32Array(arr)
          }
          RefDataType::U8Array => {
            let arr = create_array_from_pointer(*(pointer as *mut *mut c_uchar), array_len);
            get_safe_buffer(env, arr, need_thread_safe)
          }
          RefDataType::DoubleArray => {
            let arr = create_array_from_pointer(*(pointer as *mut *mut c_double), array_len);
            RsArgsValue::DoubleArray(arr)
          }
          RefDataType::FloatArray => {
            let arr = create_array_from_pointer(*(pointer as *mut *mut c_float), array_len);
            RsArgsValue::FloatArray(arr)
          }
          RefDataType::StructArray => {
            panic!("struct array not supported");
          }
        }
      } else {
        // function | raw object
        let is_stack_struct = get_ffi_tag(obj) == FFITypeTag::StackStruct;

        RsArgsValue::Object(create_rs_struct_from_pointer(
          env,
          if is_stack_struct {
            pointer
          } else {
            *(pointer as *mut *mut c_void)
          },
          obj,
          true,
        ))
      }
    }

    _ => panic!("get_js_function_call_value{:?}", type_desc),
  }
}
