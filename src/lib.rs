#[macro_use]
extern crate napi_derive;

mod datatype;
mod define;
mod utils;

use define::*;
use dlopen::symbor::{Library, Symbol};
use libc::malloc;
use libffi_sys::{
  ffi_abi_FFI_DEFAULT_ABI, ffi_call, ffi_cif, ffi_prep_cif, ffi_type, ffi_type_double,
  ffi_type_pointer, ffi_type_sint32, ffi_type_sint64, ffi_type_uint64, ffi_type_uint8,
  ffi_type_void,
};
use napi::{Env, JsExternal, JsUnknown, Result};

use std::collections::HashMap;
use std::ffi::c_void;
use utils::dataprocess::{
  get_arg_types_values, get_js_external_wrap_data, get_js_unknown_from_pointer, get_value_pointer,
  type_define_to_rs_args,
};

static mut LIBRARY_MAP: Option<
  HashMap<String, (Library, HashMap<String, Symbol<unsafe extern "C" fn()>>)>,
> = None;

#[napi]
unsafe fn create_pointer(env: Env, params: createPointerParams) -> Result<Vec<JsExternal>> {
  let createPointerParams {
    params_type,
    params_value,
  } = params;
  let (_, arg_values) = get_arg_types_values(&env, params_type, params_value)?;
  let arg_values_c_void = get_value_pointer(&env, arg_values)?;

  arg_values_c_void
    .into_iter()
    .map(|p| env.create_external(p, None))
    .collect()
}

#[napi]
unsafe fn restore_pointer(env: Env, params: storePointerParams) -> Result<Vec<JsUnknown>> {
  let storePointerParams {
    ret_type,
    params_value,
  } = params;

  ret_type
    .into_iter()
    .zip(params_value.into_iter())
    .map(|(ret_type_item, js_external)| {
      let ptr = get_js_external_wrap_data(&env, js_external)?;
      let ret_type_rs = type_define_to_rs_args(ret_type_item)?;
      get_js_unknown_from_pointer(&env, ret_type_rs, ptr)
    })
    .collect()
}

#[napi]
unsafe fn unwrap_pointer(env: Env, params: Vec<JsExternal>) -> Result<Vec<JsExternal>> {
  params
    .into_iter()
    .map(|js_external| {
      let ptr = get_js_external_wrap_data(&env, js_external)?;
      let internal_ptr = *(ptr as *mut *mut c_void);
      env.create_external(internal_ptr, None)
    })
    .collect()
}

#[napi]
unsafe fn wrap_pointer(env: Env, params: Vec<JsExternal>) -> Result<Vec<JsExternal>> {
  params
    .into_iter()
    .map(|js_external| {
      let ptr = get_js_external_wrap_data(&env, js_external)?;
      env.create_external(Box::into_raw(Box::new(ptr)), None)
    })
    .collect()
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
      let lib = if path == "" {
        Library::open_self().unwrap()
      } else {
        Library::open(path).unwrap()
      };
      map.insert(library, (lib, HashMap::new()));
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
unsafe fn load(env: Env, params: FFIParams) -> napi::Result<JsUnknown> {
  let FFIParams {
    library,
    func_name,
    ret_type,
    params_type,
    params_value,
    callback,
  } = params;

  let lib = LIBRARY_MAP.as_mut().unwrap();
  let (lib, func_map) = lib
    .get_mut(&library)
    .ok_or(FFIError::LibraryNotFound(format!(
      "Before calling load, you need to open the file {:?} with the open method",
      library
    )))?;
  let func_name_str = func_name.as_str();
  let func = if func_map.get(func_name_str).is_some() {
    *(func_map.get(func_name_str).unwrap())
  } else {
    let func = lib.symbol(func_name_str).map_err(|_| {
      FFIError::FunctionNotFound(format!(
        "Cannot find {:?} function in share library",
        func_name_str
      ))
    })?;
    func_map.insert(func_name, func);
    func
  };
  let params_type_len = params_type.len();

  let (mut arg_types, arg_values) = get_arg_types_values(&env, params_type, params_value)?;
  let mut arg_values_c_void = get_value_pointer(&env, arg_values)?;

  let ret_type_rs = type_define_to_rs_args(ret_type)?;
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
  if callback.is_some() {
    use datatype::function::get_js_function_call_value_from_ptr;
    use datatype::object_generate::rs_value_to_js_unknown;
    use napi::threadsafe_function::{
      ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode,
    };
    use napi::{Error, Ref, Task};
    let callback_thread_safe_fn: ThreadsafeFunction<Vec<RsArgsValue>, ErrorStrategy::Fatal> =
      callback.unwrap().create_threadsafe_function(0, |ctx| {
        let value: Vec<RsArgsValue> = ctx.value;
        let js_call_params: Vec<JsUnknown> = value
          .into_iter()
          .map(|rs_args| rs_value_to_js_unknown(&ctx.env, rs_args))
          .collect::<Result<Vec<JsUnknown>, _>>()?;
        Ok(js_call_params)
      })?;
    struct CountBufferLength {
      data: Ref<RsArgsValue>,
    }

    impl CountBufferLength {
      pub fn new(data: Ref<RsArgsValue>) -> Self {
        Self { data }
      }
    }

    impl Task for CountBufferLength {
      type Output = RsArgsValue;
      type JsValue = JsUnknown;

      unsafe fn compute(&mut self) -> Result<Self::Output> {
        ffi_call(
          &mut cif,
          Some(*func),
          result,
          arg_values_c_void.as_mut_ptr(),
        );
      }

      fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
        // env.create_uint32(output as _)
      }

      fn reject(&mut self, env: Env, err: Error) -> Result<Self::JsValue> {
        Err(err)
      }

      fn finally(&mut self, env: Env) -> Result<()> {
        self.data.unref(env)?;
        Ok(())
      }
    }
    Ok(env.create_int32(0).unwrap().into_unknown())
  } else {
    ffi_call(
      &mut cif,
      Some(*func),
      result,
      arg_values_c_void.as_mut_ptr(),
    );
    get_js_unknown_from_pointer(&env, ret_type_rs, result)
  }
}
