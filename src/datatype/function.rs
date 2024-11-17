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
      let data = match number.to_basic_data_type() {
        BasicDataType::U8 => RsArgsValue::U8(*(pointer as *mut u8)),
        BasicDataType::I32 => RsArgsValue::I32(*(pointer as *mut i32)),
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
        BasicDataType::External => {
          RsArgsValue::External(env.create_external(pointer, None).unwrap())
        }
        BasicDataType::Void => RsArgsValue::Void(()),
      };
      data
    }
    RsArgsValue::Object(obj) => {
      if let FFITag::Array = get_ffi_tag(obj) {
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
        }
      } else {
        // function | raw object
        RsArgsValue::Object(create_rs_struct_from_pointer(
          env,
          *(pointer as *mut *mut c_void),
          obj,
          true,
        ))
      }
    }

    _ => panic!("get_js_function_call_value{:?}", type_desc),
  }
}

// has been deprecated
// pub unsafe fn get_js_function_call_value(
//   env: &Env,
//   func_arg_type: &RsArgsValue,
//   func_val_ptr: *mut c_void,
//   need_thread_safe: bool,
// ) -> RsArgsValue {
//   match func_arg_type {
//     RsArgsValue::I32(number) => {
//       let data_type = number_to_basic_data_type(*number);
//       let data = match data_type {
//         BasicDataType::U8 => RsArgsValue::U8(func_val_ptr as u8),
//         BasicDataType::I32 => {
//           return RsArgsValue::I32(func_val_ptr as i32);
//         }
//         BasicDataType::I64 => RsArgsValue::I64(func_val_ptr as i64),
//         BasicDataType::U64 => RsArgsValue::U64(func_val_ptr as u64),
//         BasicDataType::Boolean => RsArgsValue::Boolean(if func_val_ptr as i32 == 0 {
//           false
//         } else {
//           true
//         }),
//         BasicDataType::String => RsArgsValue::String(
//           CStr::from_ptr(func_val_ptr as *mut c_char)
//             .to_string_lossy()
//             .to_string(),
//         ),
//         BasicDataType::External => {
//           RsArgsValue::External(env.create_external(func_val_ptr, None).unwrap())
//         }
//         BasicDataType::Void => RsArgsValue::Void(()),
//         BasicDataType::Double => {
//           panic!("Double type cannot be used as function parameter type so far");
//         }
//       };
//       data
//     }
//     RsArgsValue::Object(obj) => {
//       if let FFITag::Array = get_ffi_tag(obj) {
//         let array_desc = get_array_desc(obj);
//         let FFIARRARYDESC {
//           array_type,
//           array_len,
//           ..
//         } = array_desc;
//         match array_type {
//           RefDataType::StringArray => {
//             let arr = create_array_from_pointer(func_val_ptr as *mut *mut c_char, array_len);
//             RsArgsValue::StringArray(arr)
//           }
//           RefDataType::I32Array => {
//             let arr = create_array_from_pointer(func_val_ptr as *mut c_int, array_len);
//             RsArgsValue::I32Array(arr)
//           }
//           RefDataType::U8Array => {
//             let arr = create_array_from_pointer(func_val_ptr as *mut c_uchar, array_len);
//             get_safe_buffer(env, arr, need_thread_safe)
//           }
//           RefDataType::DoubleArray => {
//             let arr = create_array_from_pointer(func_val_ptr as *mut c_double, array_len);
//             RsArgsValue::DoubleArray(arr)
//           }
//         }
//       } else {
//         // function | raw object
//         RsArgsValue::Object(create_rs_struct_from_pointer(env, func_val_ptr, obj, true))
//       }
//     }

//     _ => panic!("get_js_function_call_value{:?}", func_arg_type),
//   }
// }
