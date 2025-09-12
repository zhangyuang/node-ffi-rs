#[macro_use]
extern crate napi_derive;

mod datatype;
mod define;
mod utils;
use datatype::pointer::{free_c_pointer_memory, free_rs_pointer_memory};
use define::*;
use dlopen::symbor::{Library, Symbol};
use libffi_sys::ffi_type;
use libffi_sys::{ffi_abi_FFI_DEFAULT_ABI, ffi_call, ffi_cif, ffi_prep_cif};
use napi::{Env, JsExternal, JsUnknown, Result};
use std::collections::HashMap;
use std::ffi::c_void;
use std::rc::Rc;
use utils::dataprocess::{
  get_arg_values, get_ffi_tag, get_js_external_wrap_data, get_js_unknown_from_pointer,
  get_value_pointer, type_define_to_rs_args,
};
use utils::pointer::get_ffi_type;

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
  let params_type_rs: Rc<Vec<RsArgsValue>> = Rc::new(
    params_type
      .into_iter()
      .map(|param| type_define_to_rs_args(&env, param).unwrap())
      .collect(),
  );
  let arg_values = get_arg_values(Rc::clone(&params_type_rs), params_value)?;
  let arg_values_c_void = get_value_pointer(&env, Rc::clone(&params_type_rs), arg_values)?;

  arg_values_c_void
    .into_iter()
    .map(|p| env.create_external(p, Some(std::mem::size_of::<*mut c_void>() as i64)))
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
    .zip(params_type_rs.iter())
    .for_each(|(js_external, ptr_desc)| {
      let ptr = get_js_external_wrap_data(&env, js_external).unwrap();
      match pointer_type {
        PointerType::CPointer => free_c_pointer_memory(ptr, ptr_desc),
        PointerType::RsPointer => free_rs_pointer_memory(ptr, ptr_desc),
      }
    });
}

#[napi]
unsafe fn is_null_pointer(env: Env, js_external: JsExternal) -> Result<bool> {
  let ptr = get_js_external_wrap_data(&env, js_external)?;
  Ok(ptr.is_null())
}
use serde::{Deserialize, Serialize};
#[napi(object)]
#[derive(Serialize, Deserialize)]
struct Metadata {
  pub version: String,
  pub author: String,
  pub created_at: u32,
  pub updated_at: u32,
  pub permissions: Vec<String>,
}
#[napi(object)]
#[derive(Serialize, Deserialize)]
struct Coordinate {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub label: String,
}
#[napi(object)]
#[derive(Serialize, Deserialize)]
struct bench_test_struct {
  pub id: i64,
  pub data: String,
  pub metadata: Metadata,
  pub coordinates: Vec<Coordinate>,
  pub is_active: bool,
  pub timestamp: u32,
  pub tags: Vec<String>,
}

#[napi]
unsafe fn create_bench_test_struct() -> Result<bench_test_struct> {
  Ok(bench_test_struct {
    id: 100,
    data: "test".to_string(),
    metadata: Metadata {
      version: "1.0.0".to_string(),
      author: "test_user".to_string(),
      created_at: 1640995200,
      updated_at: 1640995200,
      permissions: vec!["read".to_string(), "write".to_string()],
    },
    coordinates: vec![
      Coordinate {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        label: "origin".to_string(),
      },
      Coordinate {
        x: 4.0,
        y: 5.0,
        z: 6.0,
        label: "target".to_string(),
      },
    ],
    is_active: true,
    timestamp: 1640995200,
    tags: vec![
      "benchmark".to_string(),
      "test".to_string(),
      "performance".to_string(),
    ],
  })
}

#[napi]
unsafe fn create_bench_test_struct_to_json_string() -> Result<String> {
  Ok(
    serde_json::to_string(&bench_test_struct {
      id: 100,
      data: "test".to_string(),
      metadata: Metadata {
        version: "1.0.0".to_string(),
        author: "test_user".to_string(),
        created_at: 1640995200,
        updated_at: 1640995200,
        permissions: vec!["read".to_string(), "write".to_string()],
      },
      coordinates: vec![
        Coordinate {
          x: 1.0,
          y: 2.0,
          z: 3.0,
          label: "origin".to_string(),
        },
        Coordinate {
          x: 4.0,
          y: 5.0,
          z: 6.0,
          label: "target".to_string(),
        },
      ],
      is_active: true,
      timestamp: 1640995200,
      tags: vec![
        "benchmark".to_string(),
        "test".to_string(),
        "performance".to_string(),
      ],
    })
    .unwrap(),
  )
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
      env.create_external(
        internal_ptr,
        Some(std::mem::size_of::<*mut c_void>() as i64),
      )
    })
    .collect()
}
#[napi]
unsafe fn wrap_pointer(env: Env, params: Vec<JsExternal>) -> Result<Vec<JsExternal>> {
  params
    .into_iter()
    .map(|js_external| {
      let ptr = get_js_external_wrap_data(&env, js_external)?;
      env.create_external(
        Box::into_raw(Box::new(ptr)),
        Some(std::mem::size_of::<*mut c_void>() as i64),
      )
    })
    .collect()
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
                                    FFIError::Panic(
                                        format!(
                                            "Please check whether the library has the same compilation and runtime environment.\n Error detail info: {:?}",
                                            e
                                        )
                                    ).into()
                                );
            } else {
              return Err(FFIError::Panic(e.to_string()).into());
            }
          }
          e => {
            return Err(FFIError::Panic(e.to_string()).into());
          }
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
    .map_err(|e| e.clone())?;
  Ok(**func)
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
  let params_type_rs: Rc<Vec<RsArgsValue>> = Rc::new(
    params_type
      .into_iter()
      .map(|param| type_define_to_rs_args(&env, param).unwrap())
      .collect(),
  );
  let mut arg_types: Vec<*mut ffi_type> = params_type_rs
    .iter()
    .map(|arg| Box::into_raw(get_ffi_type(arg)))
    .collect();
  let arg_values = get_arg_values(Rc::clone(&params_type_rs), params_value)?;
  let mut arg_values_c_void = get_value_pointer(&env, Rc::clone(&params_type_rs), arg_values)?;
  let ret_type_rs = type_define_to_rs_args(&env, ret_type)?;
  let mut r_type = get_ffi_type(&ret_type_rs);

  let mut cif = ffi_cif {
    abi: ffi_abi_FFI_DEFAULT_ABI,
    nargs: params_type_len as u32,
    arg_types: arg_types.as_mut_ptr(),
    rtype: &mut *r_type,
    bytes: 0,
    flags: 0,
    #[cfg(all(target_arch = "aarch64", target_os = "windows"))]
    is_variadic: 0,
    #[cfg(all(target_arch = "aarch64", target_vendor = "apple"))]
    aarch64_nfixedargs: params_type_len as u32,
    #[cfg(all(target_arch = "arm"))]
    vfp_used: 0,
    #[cfg(all(target_arch = "arm"))]
    vfp_reg_free: 0,
    #[cfg(all(target_arch = "arm"))]
    vfp_nargs: 0,
    #[cfg(all(target_arch = "arm"))]
    vfp_args: [0; 16],
  };

  ffi_prep_cif(
    &mut cif,
    ffi_abi_FFI_DEFAULT_ABI,
    params_type_len as u32,
    &mut *r_type,
    arg_types.as_mut_ptr(),
  );
  if run_in_new_thread == Some(true) {
    use napi::Task;
    impl Task for FFICALL {
      type Output = BarePointerWrap;
      type JsValue = JsUnknown;
      fn compute(&mut self) -> Result<BarePointerWrap> {
        let FFICALLPARAMS {
          mut cif,
          fn_pointer,
          ..
        } = self.data;

        let FFICALLPARAMS {
          arg_values_c_void, ..
        } = &mut self.data;
        unsafe {
          let result = libc::malloc(std::mem::size_of::<*mut c_void>());
          ffi_call(
            &mut cif,
            Some(fn_pointer),
            result,
            arg_values_c_void.as_mut_ptr(),
          );
          Ok(BarePointerWrap(result))
        }
      }

      fn resolve(&mut self, env: Env, output: Self::Output) -> Result<JsUnknown> {
        let FFICALLPARAMS {
          errno,
          free_result_memory,
          ..
        } = self.data;
        let FFICALLPARAMS {
          ret_type_rs,
          arg_types,
          arg_values_c_void,
          params_type_rs,
          ..
        } = &mut self.data;
        unsafe {
          let call_result = get_js_unknown_from_pointer(&env, &ret_type_rs, output.0);
          if free_result_memory {
            free_c_pointer_memory(output.0, &ret_type_rs);
          }
          arg_types.into_iter().for_each(|arg| {
            let _ = Box::from_raw(*arg);
          });
          arg_values_c_void
            .into_iter()
            .zip(params_type_rs.iter())
            .for_each(|(ptr, ptr_desc)| {
              free_rs_pointer_memory(*ptr, ptr_desc);
            });
          libc::free(output.0);
          if let Some(true) = errno {
            add_errno(&env, call_result?)
          } else {
            call_result
          }
        }
      }
    }
    let task = FFICALL::new(FFICALLPARAMS {
      cif,
      arg_values_c_void,
      ret_type_rs,
      fn_pointer: func,
      errno,
      arg_types,
      free_result_memory,
      params_type_rs,
    });
    let async_work_promise = env.spawn(task)?;
    Ok(async_work_promise.promise_object().into_unknown())
  } else {
    let result = &mut () as *mut _ as *mut c_void;
    ffi_call(&mut cif, Some(func), result, arg_values_c_void.as_mut_ptr());
    arg_types.into_iter().for_each(|arg| {
      let _ = Box::from_raw(arg);
    });
    let call_result = get_js_unknown_from_pointer(&env, &ret_type_rs, result);
    if free_result_memory {
      free_c_pointer_memory(result, &ret_type_rs);
    }
    arg_values_c_void
      .into_iter()
      .zip(params_type_rs.iter())
      .for_each(|(ptr, ptr_desc)| {
        free_rs_pointer_memory(ptr, ptr_desc);
      });
    if let Some(true) = errno {
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
