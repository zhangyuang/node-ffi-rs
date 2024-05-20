#[macro_use]
extern crate napi_derive;

mod datatype;
mod define;
mod utils;
use define::*;
use dlopen::symbor::{Library, Symbol};
use libc::{free, malloc};
use libffi_sys::{
  ffi_abi_FFI_DEFAULT_ABI, ffi_call, ffi_cif, ffi_prep_cif, ffi_type, ffi_type_double,
  ffi_type_float, ffi_type_pointer, ffi_type_sint32, ffi_type_sint64, ffi_type_uint64,
  ffi_type_uint8, ffi_type_void,
};
use napi::{Env, JsExternal, JsUnknown, Result};

use datatype::pointer::{free_c_pointer_memory, free_rs_pointer_memory};
use std::collections::HashMap;
use std::ffi::c_void;
use utils::dataprocess::{
  get_arg_types_values, get_js_external_wrap_data, get_js_unknown_from_pointer, get_value_pointer,
  type_define_to_rs_args,
};

static mut LIBRARY_MAP: Option<
  HashMap<
    String,
    (
      Library,
      HashMap<String, Result<Symbol<unsafe extern "C" fn()>>>,
    ),
  >,
> = None;

#[napi]
unsafe fn create_pointer(env: Env, params: CreatePointerParams) -> Result<Vec<JsExternal>> {
  let CreatePointerParams {
    params_type,
    params_value,
  } = params;
  let params_type_rs: Vec<RsArgsValue> = params_type
    .into_iter()
    .map(|param| type_define_to_rs_args(&env, param).unwrap())
    .collect();
  let (_, arg_values) = get_arg_types_values(&env, params_type_rs, params_value)?;
  let arg_values_c_void = get_value_pointer(&env, arg_values)?;

  arg_values_c_void
    .into_iter()
    .map(|p| env.create_external(p, None))
    .collect()
}

#[napi]
unsafe fn restore_pointer(env: Env, params: StorePointerParams) -> Result<Vec<JsUnknown>> {
  let StorePointerParams {
    ret_type,
    params_value,
  } = params;

  ret_type
    .into_iter()
    .zip(params_value.into_iter())
    .map(|(ret_type_item, js_external)| {
      let ptr = get_js_external_wrap_data(&env, js_external)?;
      let ret_type_rs = type_define_to_rs_args(&env, ret_type_item)?;
      get_js_unknown_from_pointer(&env, &ret_type_rs, ptr)
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
unsafe fn free_pointer(env: Env, params: FreePointerParams) {
  let FreePointerParams {
    params_type,
    params_value,
    pointer_type,
  } = params;
  let params_type_rs: Vec<RsArgsValue> = params_type
    .into_iter()
    .map(|param| type_define_to_rs_args(&env, param).unwrap())
    .collect();
  params_value
    .into_iter()
    .zip(params_type_rs.into_iter())
    .for_each(|(js_external, ptr_desc)| {
      let ptr = get_js_external_wrap_data(&env, js_external).unwrap();
      match pointer_type {
        PointerType::CPointer => free_c_pointer_memory(ptr, ptr_desc, true),
        PointerType::RsPointer => free_rs_pointer_memory(ptr, ptr_desc, true),
      }
    });
}

#[napi]
unsafe fn open(params: OpenParams) -> Result<()> {
  let OpenParams { library, path } = params;
  if LIBRARY_MAP.is_none() {
    LIBRARY_MAP = Some(HashMap::new());
  }
  let map = LIBRARY_MAP.as_mut().unwrap();
  if map.get(&library).is_none() {
    let lib = if path == "" {
      Library::open_self().unwrap()
    } else {
      match Library::open(&path) {
        Ok(lib) => lib,
        Err(e) => match e {
          dlopen::Error::OpeningLibraryError(e) => {
            let err_msg = e.to_string();
            if err_msg.contains("is not a valid Win32 application")
              || err_msg.contains("not a mach-o file")
              || err_msg.contains("invalid ELF header")
            {
              return Err(
                      FFIError::Panic(format!(
                        "Please check whether the library has the same compilation and runtime environment.\n Error detail info: {:?}",
                        e
                      ))
                      .into(),
                    );
            } else {
              return Err(FFIError::Panic(e.to_string()).into());
            }
          }
          e => return Err(FFIError::Panic(e.to_string()).into()),
        },
      }
    };
    map.insert(library, (lib, HashMap::new()));
  }
  Ok(())
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

unsafe fn get_symbol<'a>(
  library: &String,
  func_name: &String,
) -> napi::Result<unsafe extern "C" fn()> {
  let library_map = LIBRARY_MAP.as_mut().unwrap();
  let (lib, func_map) = library_map
    .get_mut(library)
    .ok_or(FFIError::LibraryNotFound(format!(
      "Before calling load, you need to open the file {:?} with the open method",
      library
    )))?;
  let func = func_map
    .entry(func_name.clone())
    .or_insert_with(|| {
      lib
        .symbol::<unsafe extern "C" fn()>(&func_name)
        .map_err(|_| {
          FFIError::FunctionNotFound(format!(
            "Cannot find {:?} function in shared library",
            &func_name
          ))
          .into()
        })
    })
    .as_ref()
    .unwrap();
  let func = **func;
  Ok(func)
}
#[napi]
unsafe fn load(env: Env, params: FFIParams) -> napi::Result<JsUnknown> {
  let FFIParams {
    library,
    func_name,
    ret_type,
    params_type,
    params_value,
    errno,
    run_in_new_thread,
    free_result_memory,
  } = params;
  let func = get_symbol(&library, &func_name)?;
  let params_type_len = params_type.len();
  let params_type_rs: Vec<RsArgsValue> = params_type
    .into_iter()
    .map(|param| type_define_to_rs_args(&env, param).unwrap())
    .collect();
  let (mut arg_types, arg_values) =
    get_arg_types_values(&env, params_type_rs.clone(), params_value)?;
  let mut arg_values_c_void = get_value_pointer(&env, arg_values)?;
  let ret_type_rs = type_define_to_rs_args(&env, ret_type)?;
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
        BasicDataType::Float => &mut ffi_type_float as *mut ffi_type,
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
    #[cfg(all(target_arch = "aarch64", target_os = "windows"))]
    is_variadic: 0,
    #[cfg(all(target_arch = "aarch64", target_vendor = "apple"))]
    aarch64_nfixedargs: params_type_len as u32,
  };
  ffi_prep_cif(
    &mut cif,
    ffi_abi_FFI_DEFAULT_ABI,
    params_type_len as u32,
    r_type,
    arg_types.as_mut_ptr(),
  );
  if run_in_new_thread.is_some() && run_in_new_thread.unwrap() {
    let r_type_p = Box::into_raw(Box::new(r_type));
    let arg_types_p = Box::into_raw(Box::new(arg_types));
    use napi::Task;
    impl Task for FFICALL {
      type Output = BarePointerWrap;
      type JsValue = JsUnknown;
      fn compute(&mut self) -> Result<BarePointerWrap> {
        let FFICALLPARAMS {
          cif,
          fn_pointer,
          arg_values_c_void,
          ..
        } = &mut self.data;
        unsafe {
          let result = malloc(std::mem::size_of::<*mut c_void>());
          ffi_call(
            *cif,
            Some(*fn_pointer),
            result,
            arg_values_c_void.as_mut_ptr(),
          );
          Ok(BarePointerWrap(result))
        }
      }

      fn resolve(&mut self, env: Env, output: Self::Output) -> Result<JsUnknown> {
        let FFICALLPARAMS {
          ret_type_rs,
          errno,
          arg_values_c_void,
          free_result_memory,
          params_type_rs,
          cif,
          r_type_p,
          arg_types_p,
          ..
        } = &mut self.data;
        unsafe {
          let call_result = get_js_unknown_from_pointer(&env, &ret_type_rs, output.0);
          if *free_result_memory {
            free_c_pointer_memory(output.0, ret_type_rs.clone(), false);
          }
          arg_values_c_void
            .into_iter()
            .zip(params_type_rs.into_iter())
            .for_each(|(ptr, ptr_desc)| {
              free_rs_pointer_memory(*ptr, ptr_desc.clone(), false);
            });
          let _ = Box::from_raw(*cif);
          let _ = Box::from_raw(*r_type_p);
          let _ = Box::from_raw(*arg_types_p);
          free(output.0);
          if errno.is_some() && errno.unwrap() {
            add_errno(&env, call_result?)
          } else {
            call_result
          }
        }
      }
    }
    let task = FFICALL::new(FFICALLPARAMS {
      cif: Box::into_raw(Box::new(cif)),
      arg_values_c_void,
      ret_type_rs,
      fn_pointer: func,
      errno,
      free_result_memory,
      params_type_rs,
      r_type_p,
      arg_types_p,
    });
    let async_work_promise = env.spawn(task)?;
    Ok(async_work_promise.promise_object().into_unknown())
  } else {
    let result = &mut () as *mut _ as *mut c_void;
    ffi_call(&mut cif, Some(func), result, arg_values_c_void.as_mut_ptr());
    let call_result = get_js_unknown_from_pointer(&env, &ret_type_rs, result);
    if free_result_memory {
      free_c_pointer_memory(result, ret_type_rs, false);
    }
    arg_values_c_void
      .into_iter()
      .zip(params_type_rs.into_iter())
      .for_each(|(ptr, ptr_desc)| {
        free_rs_pointer_memory(ptr, ptr_desc, false);
      });
    if errno.is_some() && errno.unwrap() {
      add_errno(&env, call_result?)
    } else {
      call_result
    }
  }
}

fn add_errno(env: &Env, call_result: JsUnknown) -> Result<JsUnknown> {
  use std::io::Error;
  let last_error = Error::last_os_error();
  let error_code = last_error.raw_os_error().unwrap_or(0);
  let error_message = last_error.to_string();
  let mut obj = env.create_object()?;
  obj.set_named_property("errnoCode", env.create_int32(error_code)?)?;
  obj.set_named_property("errnoMessage", env.create_string(&error_message)?)?;
  obj.set_named_property("value", call_result)?;
  Ok(obj.into_unknown())
}
