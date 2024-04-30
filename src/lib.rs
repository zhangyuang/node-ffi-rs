#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;

use napi::{JsNumber, JsObject, JsString, JsUnknown};

use libc::malloc;
use libc::{c_char, c_double, c_int};
use libffi_sys::{
  ffi_abi_FFI_DEFAULT_ABI, ffi_call, ffi_cif, ffi_prep_cif, ffi_type, ffi_type_double,
  ffi_type_pointer, ffi_type_sint32, ffi_type_void,
};
use libloading::{Library, Symbol};
use std::ffi::c_void;
use std::ffi::{CStr, CString};

#[napi]
pub enum RetType {
  String,
  I32,
  Void,
  Double,
  I32Array,
  StringArray,
  DoubleArray,
}
pub enum RsArgsValue {
  String(String),
  I32(i32),
  Double(f64),
  I32Array(Vec<i32>),
  StringArray(Vec<String>),
  DoubleArray(Vec<f64>),
}

#[napi]
pub enum ParamsType {
  String,
  I32,
  Double,
  I32Array,
  StringArray,
  DoubleArray,
}

#[napi(object)]
struct FFIParams {
  pub library: String,
  pub func_name: String,
  pub ret_type: RetType,
  pub ret_type_len: Option<u32>,
  pub params_type: Vec<ParamsType>,
  pub params_value: Vec<JsUnknown>,
}

#[napi]
fn load(params: FFIParams) -> Either7<String, i32, (), f64, Vec<i32>, Vec<String>, Vec<f64>> {
  let FFIParams {
    library,
    func_name,
    ret_type,
    params_type,
    params_value,
    ret_type_len,
  } = params;
  unsafe {
    let lib = Library::new(library).unwrap();
    let func: Symbol<unsafe extern "C" fn()> = lib.get(func_name.as_str().as_bytes()).unwrap();

    let (mut arg_types, arg_values): (Vec<*mut ffi_type>, Vec<RsArgsValue>) = params_type
      .iter()
      .zip(params_value.into_iter())
      .map(|(param, value)| match param {
        ParamsType::I32 => {
          let arg_type = &mut ffi_type_sint32 as *mut ffi_type;
          let arg_val: i32 = value.coerce_to_number().unwrap().try_into().unwrap();
          (arg_type, RsArgsValue::I32(arg_val))
        }
        ParamsType::Double => {
          let arg_type = &mut ffi_type_double as *mut ffi_type;
          let arg_val: f64 = value.coerce_to_number().unwrap().try_into().unwrap();
          (arg_type, RsArgsValue::Double(arg_val))
        }
        ParamsType::String => {
          let arg_type = &mut ffi_type_pointer as *mut ffi_type;
          let arg_val: String = value
            .coerce_to_string()
            .unwrap()
            .into_utf8()
            .unwrap()
            .try_into()
            .unwrap();
          (arg_type, RsArgsValue::String(arg_val))
        }
        ParamsType::I32Array => {
          let arg_type = &mut ffi_type_pointer as *mut ffi_type;
          let js_object = value.coerce_to_object().unwrap();
          let arg_val = vec![0; js_object.get_array_length().unwrap() as usize]
            .iter()
            .enumerate()
            .map(|(index, _)| {
              let js_element: JsNumber = js_object.get_element(index as u32).unwrap();
              return js_element.get_int32().unwrap();
            })
            .collect::<Vec<i32>>();

          (arg_type, RsArgsValue::I32Array(arg_val))
        }
        ParamsType::DoubleArray => {
          let arg_type = &mut ffi_type_pointer as *mut ffi_type;
          let js_object = value.coerce_to_object().unwrap();
          let arg_val = vec![0; js_object.get_array_length().unwrap() as usize]
            .iter()
            .enumerate()
            .map(|(index, _)| {
              let js_element: JsNumber = js_object.get_element(index as u32).unwrap();
              return js_element.get_double().unwrap();
            })
            .collect::<Vec<f64>>();

          (arg_type, RsArgsValue::DoubleArray(arg_val))
        }
        ParamsType::StringArray => {
          let arg_type = &mut ffi_type_pointer as *mut ffi_type;
          let js_object = value.coerce_to_object().unwrap();
          let arg_val = vec![0; js_object.get_array_length().unwrap() as usize]
            .iter()
            .enumerate()
            .map(|(index, _)| {
              let js_element: JsString = js_object.get_element(index as u32).unwrap();
              return js_element.into_utf8().unwrap().try_into().unwrap();
            })
            .collect::<Vec<String>>();
          (arg_type, RsArgsValue::StringArray(arg_val))
        }
      })
      .unzip();
    let mut arg_values: Vec<*mut c_void> = arg_values
      .into_iter()
      .map(|val| match val {
        RsArgsValue::I32(val) => {
          let c_num = Box::new(val);
          Box::into_raw(c_num) as *mut c_void
        }
        RsArgsValue::String(val) => {
          let c_string = Box::new(CString::new(val).unwrap());
          Box::into_raw(c_string) as *mut c_void
        }
        RsArgsValue::Double(val) => {
          let c_double = Box::new(val);
          Box::into_raw(c_double) as *mut c_void
        }
        RsArgsValue::I32Array(val) => {
          let ptr = val.as_ptr();
          let boxed_ptr = Box::new(ptr);
          let raw_ptr = Box::into_raw(boxed_ptr);
          std::mem::forget(val);
          return raw_ptr as *mut c_void;
        }
        RsArgsValue::DoubleArray(val) => {
          let ptr = val.as_ptr();
          let boxed_ptr = Box::new(ptr);
          let raw_ptr = Box::into_raw(boxed_ptr);
          std::mem::forget(val);
          return raw_ptr as *mut c_void;
        }
        RsArgsValue::StringArray(val) => {
          let c_char_vec: Vec<*const c_char> = val
            .into_iter()
            .map(|str| {
              let c_string = CString::new(str).unwrap();
              let ptr = c_string.as_ptr();
              std::mem::forget(c_string);
              return ptr;
            })
            .collect();

          let ptr = c_char_vec.as_ptr();
          std::mem::forget(c_char_vec);
          Box::into_raw(Box::new(ptr)) as *mut c_void
        }
      })
      .collect();

    let r_type: *mut ffi_type = match ret_type {
      RetType::I32 => &mut ffi_type_sint32 as *mut ffi_type,
      RetType::String => &mut ffi_type_pointer as *mut ffi_type,
      RetType::Void => &mut ffi_type_void as *mut ffi_type,
      RetType::Double => &mut ffi_type_double as *mut ffi_type,
      RetType::I32Array => &mut ffi_type_pointer as *mut ffi_type,
      RetType::StringArray => &mut ffi_type_pointer as *mut ffi_type,
      RetType::DoubleArray => &mut ffi_type_pointer as *mut ffi_type,
    };

    let mut cif = ffi_cif {
      abi: ffi_abi_FFI_DEFAULT_ABI,
      nargs: params_type.len() as u32,
      arg_types: arg_types.as_mut_ptr(),
      rtype: r_type,
      bytes: 0,
      flags: 0,
      #[cfg(all(target_arch = "aarch64", target_vendor = "apple"))]
      aarch64_nfixedargs: params_type.len() as u32,
    };

    ffi_prep_cif(
      &mut cif as *mut ffi_cif,
      ffi_abi_FFI_DEFAULT_ABI,
      params_type.len() as u32,
      r_type,
      arg_types.as_mut_ptr(),
    );

    match ret_type {
      RetType::String => {
        let mut result: *mut c_char = malloc(std::mem::size_of::<*mut c_char>()) as *mut c_char;
        ffi_call(
          &mut cif,
          Some(*func),
          &mut result as *mut *mut c_char as *mut c_void,
          arg_values.as_mut_ptr(),
        );

        let result_str = CString::from_raw(result)
          .into_string()
          .expect(format!("{} retType is not string", func_name).as_str());

        Either7::A(result_str)
      }
      RetType::I32 => {
        let mut result: i32 = 0;
        ffi_call(
          &mut cif,
          Some(*func),
          &mut result as *mut i32 as *mut c_void,
          arg_values.as_mut_ptr(),
        );
        Either7::B(result)
      }
      RetType::Void => {
        let mut result = ();
        ffi_call(
          &mut cif,
          Some(*func),
          &mut result as *mut () as *mut c_void,
          arg_values.as_mut_ptr(),
        );
        Either7::C(())
      }
      RetType::Double => {
        let mut result: f64 = 0.0;
        ffi_call(
          &mut cif,
          Some(*func),
          &mut result as *mut f64 as *mut c_void,
          arg_values.as_mut_ptr(),
        );
        Either7::D(result)
      }
      RetType::I32Array => {
        let mut result: *mut c_int = malloc(std::mem::size_of::<*mut c_int>()) as *mut c_int;
        let arg_values = arg_values.as_mut_ptr();
        ffi_call(
          &mut cif,
          Some(*func),
          &mut result as *mut _ as *mut c_void,
          arg_values,
        );

        let result_slice = std::slice::from_raw_parts(result, ret_type_len.unwrap() as usize);
        let result_vec = result_slice.to_vec();
        if !result.is_null() {
          libc::free(result as *mut c_void);
        }
        Either7::E(result_vec)
      }
      RetType::StringArray => {
        let mut result: *mut *mut c_char =
          malloc(std::mem::size_of::<*mut *mut c_char>()) as *mut *mut c_char;
        let ptr = arg_values.as_mut_ptr();

        ffi_call(
          &mut cif,
          Some(*func),
          &mut result as *mut _ as *mut c_void,
          ptr,
        );
        let output_vec = vec![0; ret_type_len.unwrap() as usize]
          .iter()
          .enumerate()
          .map(|(index, _)| {
            let c_str = CStr::from_ptr(*result.offset(index as isize));
            let str_slice = c_str.to_str().unwrap();
            str_slice.to_string()
          })
          .collect();
        if !result.is_null() {
          libc::free(result as *mut c_void);
        }
        Either7::F(output_vec)
      }
      RetType::DoubleArray => {
        let mut result: *mut c_double =
          malloc(std::mem::size_of::<*mut c_double>()) as *mut c_double;
        let arg_values = arg_values.as_mut_ptr();
        ffi_call(
          &mut cif,
          Some(*func),
          &mut result as *mut _ as *mut c_void,
          arg_values,
        );

        let result_slice = std::slice::from_raw_parts(result, ret_type_len.unwrap() as usize);
        let result_vec = result_slice.to_vec();
        if !result.is_null() {
          libc::free(result as *mut c_void);
        }
        Either7::G(result_vec)
      }
    }
  }
}
