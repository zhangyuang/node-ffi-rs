#[macro_use]
extern crate napi_derive;

mod datatype;
mod define;
mod utils;

use define::*;
use libc::malloc;
use libc::{c_char, c_double, c_int, c_uchar};
use libffi_sys::{
  ffi_abi_FFI_DEFAULT_ABI, ffi_call, ffi_cif, ffi_prep_cif, ffi_type, ffi_type_double,
  ffi_type_pointer, ffi_type_sint32, ffi_type_sint64, ffi_type_uint64, ffi_type_uint8,
  ffi_type_void,
};
use libloading::{Library, Symbol};
use napi::{Env, JsExternal, JsUnknown};

use std::collections::HashMap;
use std::ffi::c_void;
use std::ffi::CStr;

use datatype::buffer::*;
use datatype::object_generate::*;
use datatype::pointer::*;
use utils::dataprocess::type_define_to_rs_args;

static mut LIBRARY_MAP: Option<HashMap<String, Library>> = None;

#[napi]
unsafe fn create_external(env: Env, params: CreateExternalParams) -> Vec<JsExternal> {
  let CreateExternalParams {
    params_type,
    params_value,
  } = params;
  let (_, arg_values) = get_arg_types_values(&env, params_type, params_value);
  let arg_values_c_void = get_value_pointer(&env, arg_values);
  arg_values_c_void
    .into_iter()
    .map(|p| env.create_external(*(p as *mut *mut c_void), None).unwrap())
    .collect()
}

#[napi]
unsafe fn restore_external(env: Env, params: ReStoreExternalParams) {
  let ReStoreExternalParams {
    ret_type,
    params_value,
  } = params;
}

#[napi]
fn open(params: OpenParams) {
  let OpenParams { library, path } = params;
  unsafe {
    if LIBRARY_MAP.is_none() {
      LIBRARY_MAP = Some(HashMap::new());
    }
    let map = LIBRARY_MAP.as_mut().unwrap();
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
unsafe fn load(env: Env, params: FFIParams) -> JsUnknown {
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

  let ret_type_rs = type_define_to_rs_args(ret_type);
  let r_type: *mut ffi_type = match ret_type_rs {
    RsArgsValue::I32(number) => {
      let ret_data_type = number_to_basic_data_type(number);
      match ret_data_type {
        BasicDataType::U8 => &mut ffi_type_uint8 as *mut ffi_type,
        BasicDataType::I32 => &mut ffi_type_sint32 as *mut ffi_type,
        BasicDataType::I64 => &mut ffi_type_sint64 as *mut ffi_type,
        BasicDataType::U64 => &mut ffi_type_uint64 as *mut ffi_type,
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
  let result = malloc(std::mem::size_of::<*mut c_void>());
  ffi_call(
    &mut cif,
    Some(*func),
    result,
    arg_values_c_void.as_mut_ptr(),
  );
  match ret_type_rs {
    RsArgsValue::I32(number) => {
      let ret_data_type = number_to_basic_data_type(number);
      match ret_data_type {
        BasicDataType::String => {
          let result_str = CStr::from_ptr(*(result as *mut *const c_char))
            .to_string_lossy()
            .to_string();
          rs_value_to_js_unknown(&env, RsArgsValue::String(result_str))
        }
        BasicDataType::U8 => rs_value_to_js_unknown(&env, RsArgsValue::U8(*(result as *mut u8))),
        BasicDataType::I32 => rs_value_to_js_unknown(&env, RsArgsValue::I32(*(result as *mut i32))),
        BasicDataType::I64 => rs_value_to_js_unknown(&env, RsArgsValue::I64(*(result as *mut i64))),
        BasicDataType::U64 => rs_value_to_js_unknown(&env, RsArgsValue::U64(*(result as *mut u64))),
        BasicDataType::Void => env.get_undefined().unwrap().into_unknown(),
        BasicDataType::Double => {
          rs_value_to_js_unknown(&env, RsArgsValue::Double(*(result as *mut f64)))
        }
        BasicDataType::Boolean => {
          rs_value_to_js_unknown(&env, RsArgsValue::Boolean(*(result as *mut bool)))
        }
        BasicDataType::External => {
          let js_external = env
            .create_external(*(result as *mut *mut c_void), None)
            .unwrap();
          rs_value_to_js_unknown(&env, RsArgsValue::External(js_external))
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
            let arr = create_array_from_pointer(*(result as *mut *mut c_uchar), array_len);
            if !result.is_null() {
              libc::free(result);
            }
            rs_value_to_js_unknown(&env, get_safe_buffer(&env, arr, false))
          }
          RefDataType::I32Array => {
            let arr = create_array_from_pointer(*(result as *mut *mut c_int), array_len);
            if !result.is_null() {
              libc::free(result);
            }
            rs_value_to_js_unknown(&env, RsArgsValue::I32Array(arr))
          }
          RefDataType::DoubleArray => {
            let arr = create_array_from_pointer(*(result as *mut *mut c_double), array_len);
            if !result.is_null() {
              libc::free(result);
            }
            rs_value_to_js_unknown(&env, RsArgsValue::DoubleArray(arr))
          }
          RefDataType::StringArray => {
            let arr = create_array_from_pointer(*(result as *mut *mut *mut c_char), array_len);
            if !result.is_null() {
              libc::free(result as *mut c_void);
            }
            rs_value_to_js_unknown(&env, RsArgsValue::StringArray(arr))
          }
        }
      } else {
        // raw object
        let rs_struct =
          create_rs_struct_from_pointer(&env, *(result as *mut *mut c_void), &obj, false);
        rs_value_to_js_unknown(&env, RsArgsValue::Object(rs_struct))
      }
    }
    _ => panic!("ret_type err {:?}", ret_type_rs),
  }
}
