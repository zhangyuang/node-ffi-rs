#[macro_use]
extern crate napi_derive;

mod datatype;
mod define;

use define::*;
use libc::malloc;
use libc::{c_char, c_double, c_int, c_uchar};
use libffi_sys::{
  ffi_abi_FFI_DEFAULT_ABI, ffi_call, ffi_cif, ffi_prep_cif, ffi_type, ffi_type_double,
  ffi_type_pointer, ffi_type_sint32, ffi_type_sint64, ffi_type_uint8, ffi_type_void,
};
use libloading::{Library, Symbol};
use napi::{bindgen_prelude::*, Env, JsBuffer, JsBufferValue, JsExternal, JsUnknown};

use std::collections::HashMap;
use std::ffi::c_void;
use std::ffi::{CStr, CString};

use datatype::array::*;
use datatype::buffer::*;
use datatype::object_generate::*;
use datatype::pointer::*;

static mut LIBRARY_MAP: Option<HashMap<String, Library>> = None;

#[napi]
fn open(params: OpenParams) {
  let OpenParams { library, path } = params;
  unsafe {
    if LIBRARY_MAP.is_none() {
      LIBRARY_MAP = Some(HashMap::new());
    }
    let map = LIBRARY_MAP.as_mut().unwrap();
    println!("xx{:?}", path);
    if map.get(&library).is_none() {
      let lib = Library::new(path).unwrap();
      map.insert(library, lib);
    }
  }
}

#[napi]
fn close(library: String) {
  unsafe {
    if LIBRARY_MAP.is_none() {
      return;
    }
    let map = LIBRARY_MAP.as_mut().unwrap();
    map.remove(&library);
  }
}

#[napi]
unsafe fn load(env: Env, params: FFIParams) -> Either<JsUnknown, ()> {
  let FFIParams {
    library,
    func_name,
    ret_type,
    params_type,
    params_value,
  } = params;

  let lib = LIBRARY_MAP.as_ref().unwrap();
  let lib = lib.get(&library).unwrap();
  let func: Symbol<unsafe extern "C" fn()> = lib.get(func_name.as_str().as_bytes()).unwrap();
  let params_type_len = params_type.len();
  let (mut arg_types, arg_values) = get_arg_types_values(&env, params_type, params_value);
  let mut arg_values_c_void = get_value_pointer(&env, arg_values);
  let ret_value_type = ret_type.get_type().unwrap();
  let ret_value = match ret_value_type {
    ValueType::Number => RsArgsValue::I32(js_number_to_i32(ret_type.coerce_to_number().unwrap())),
    ValueType::Object => RsArgsValue::Object(type_define_to_rs_struct(
      &ret_type.coerce_to_object().unwrap(),
    )),
    _ => panic!(
      "ret_value_type can only be number or object but receive {}",
      ret_value_type
    ),
  };
  let r_type: *mut ffi_type = match ret_value {
    RsArgsValue::I32(number) => {
      let ret_data_type = number_to_basic_data_type(number);
      match ret_data_type {
        BasicDataType::U8 => &mut ffi_type_uint8 as *mut ffi_type,
        BasicDataType::I32 => &mut ffi_type_sint32 as *mut ffi_type,
        BasicDataType::I64 => &mut ffi_type_sint64 as *mut ffi_type,
        BasicDataType::String => &mut ffi_type_pointer as *mut ffi_type,
        BasicDataType::Void => &mut ffi_type_void as *mut ffi_type,
        BasicDataType::Double => &mut ffi_type_double as *mut ffi_type,
        BasicDataType::Boolean => &mut ffi_type_uint8 as *mut ffi_type,
        BasicDataType::External => &mut ffi_type_pointer as *mut ffi_type,
      }
    }
    RsArgsValue::Object(_) => &mut ffi_type_pointer as *mut ffi_type,
    _ => &mut ffi_type_void as *mut ffi_type,
  };

  let mut cif = ffi_cif {
    abi: ffi_abi_FFI_DEFAULT_ABI,
    nargs: params_type_len as u32,
    arg_types: arg_types.as_mut_ptr(),
    rtype: r_type,
    bytes: 0,
    flags: 0,
    #[cfg(all(target_arch = "aarch64", target_vendor = "apple"))]
    aarch64_nfixedargs: params_type_len as u32,
  };

  ffi_prep_cif(
    &mut cif as *mut ffi_cif,
    ffi_abi_FFI_DEFAULT_ABI,
    params_type_len as u32,
    r_type,
    arg_types.as_mut_ptr(),
  );

  match ret_value {
    RsArgsValue::I32(number) => {
      let ret_data_type = number_to_basic_data_type(number);
      match ret_data_type {
        BasicDataType::String => {
          let mut result: *mut c_char = malloc(std::mem::size_of::<*mut c_char>()) as *mut c_char;
          ffi_call(
            &mut cif,
            Some(*func),
            &mut result as *mut *mut c_char as *mut c_void,
            arg_values_c_void.as_mut_ptr(),
          );
          let result_str = CStr::from_ptr(result).to_string_lossy().to_string();
          Either::A(rs_value_to_js_unknown(
            &env,
            RsArgsValue::String(result_str),
          ))
        }
        BasicDataType::U8 => {
          let mut result: u8 = 0;
          ffi_call(
            &mut cif,
            Some(*func),
            &mut result as *mut u8 as *mut c_void,
            arg_values_c_void.as_mut_ptr(),
          );
          Either::A(rs_value_to_js_unknown(&env, RsArgsValue::U8(result)))
        }
        BasicDataType::I32 => {
          let mut result: i32 = 0;
          ffi_call(
            &mut cif,
            Some(*func),
            &mut result as *mut i32 as *mut c_void,
            arg_values_c_void.as_mut_ptr(),
          );

          Either::A(rs_value_to_js_unknown(&env, RsArgsValue::I32(result)))
        }
        BasicDataType::I64 => {
          let mut result: i64 = 0;
          ffi_call(
            &mut cif,
            Some(*func),
            &mut result as *mut i64 as *mut c_void,
            arg_values_c_void.as_mut_ptr(),
          );
          Either::A(rs_value_to_js_unknown(&env, RsArgsValue::I64(result)))
        }
        BasicDataType::Void => {
          let mut result = ();
          ffi_call(
            &mut cif,
            Some(*func),
            &mut result as *mut () as *mut c_void,
            arg_values_c_void.as_mut_ptr(),
          );
          Either::B(())
        }
        BasicDataType::Double => {
          let mut result: f64 = 0.0;
          ffi_call(
            &mut cif,
            Some(*func),
            &mut result as *mut f64 as *mut c_void,
            arg_values_c_void.as_mut_ptr(),
          );
          Either::A(rs_value_to_js_unknown(&env, RsArgsValue::Double(result)))
        }
        BasicDataType::Boolean => {
          let mut result: bool = false;
          ffi_call(
            &mut cif,
            Some(*func),
            &mut result as *mut bool as *mut c_void,
            arg_values_c_void.as_mut_ptr(),
          );

          Either::A(rs_value_to_js_unknown(&env, RsArgsValue::Boolean(result)))
        }
        BasicDataType::External => {
          let mut result: *mut c_void = malloc(std::mem::size_of::<*mut c_void>()) as *mut c_void;
          ffi_call(
            &mut cif,
            Some(*func),
            &mut result as *mut _ as *mut c_void,
            arg_values_c_void.as_mut_ptr(),
          );

          let js_external = env.create_external(result, None).unwrap();
          Either::A(rs_value_to_js_unknown(
            &env,
            RsArgsValue::External(js_external),
          ))
        }
      }
    }
    RsArgsValue::Object(obj) => {
      let array_desc = get_array_desc(&obj);
      if array_desc.is_some() {
        // array
        let (array_len, array_type) = array_desc.unwrap();
        match array_type {
          RefDataType::U8Array => {
            let mut result: *mut c_uchar =
              malloc(std::mem::size_of::<*mut c_uchar>()) as *mut c_uchar;
            ffi_call(
              &mut cif,
              Some(*func),
              &mut result as *mut _ as *mut c_void,
              arg_values_c_void.as_mut_ptr(),
            );

            let arr = create_array_from_pointer(result, array_len);

            if !result.is_null() {
              libc::free(result as *mut c_void);
            }
            Either::A(rs_value_to_js_unknown(
              &env,
              get_safe_buffer(&env, arr, false),
            ))
          }
          RefDataType::I32Array => {
            let mut result: *mut c_int = malloc(std::mem::size_of::<*mut c_int>()) as *mut c_int;
            ffi_call(
              &mut cif,
              Some(*func),
              &mut result as *mut _ as *mut c_void,
              arg_values_c_void.as_mut_ptr(),
            );
            let arr = create_array_from_pointer(result, array_len);
            if !result.is_null() {
              libc::free(result as *mut c_void);
            }
            Either::A(rs_value_to_js_unknown(&env, RsArgsValue::I32Array(arr)))
          }
          RefDataType::DoubleArray => {
            let mut result: *mut c_double =
              malloc(std::mem::size_of::<*mut c_double>()) as *mut c_double;
            ffi_call(
              &mut cif,
              Some(*func),
              &mut result as *mut _ as *mut c_void,
              arg_values_c_void.as_mut_ptr(),
            );
            let arr = create_array_from_pointer(result, array_len);
            if !result.is_null() {
              libc::free(result as *mut c_void);
            }
            Either::A(rs_value_to_js_unknown(&env, RsArgsValue::DoubleArray(arr)))
          }
          RefDataType::StringArray => {
            let mut result: *mut *mut c_char =
              malloc(std::mem::size_of::<*mut *mut c_char>()) as *mut *mut c_char;

            ffi_call(
              &mut cif,
              Some(*func),
              &mut result as *mut _ as *mut c_void,
              arg_values_c_void.as_mut_ptr(),
            );
            let arr = create_array_from_pointer(result, array_len);
            if !result.is_null() {
              libc::free(result as *mut c_void);
            }
            Either::A(rs_value_to_js_unknown(&env, RsArgsValue::StringArray(arr)))
          }
        }
      } else {
        // raw object
        let mut result: *mut c_void = malloc(std::mem::size_of::<*mut *mut c_void>());
        ffi_call(
          &mut cif,
          Some(*func),
          &mut result as *mut _ as *mut c_void,
          arg_values_c_void.as_mut_ptr(),
        );
        let rs_struct = create_rs_struct_from_pointer(&env, result, &obj, false);
        Either::A(rs_value_to_js_unknown(&env, RsArgsValue::Object(rs_struct)))
      }
    }
    _ => panic!("ret_type err {:?}", ret_value),
  }
}
