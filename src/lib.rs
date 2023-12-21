#[macro_use]
extern crate napi_derive;

#[macro_use]
mod ffi_macro;

mod define;
mod utils;
use define::*;
use indexmap::IndexMap;
use libc::malloc;
use libc::{c_char, c_double, c_int};
use libffi_sys::{
  ffi_abi_FFI_DEFAULT_ABI, ffi_call, ffi_cif, ffi_prep_cif, ffi_type, ffi_type_double,
  ffi_type_pointer, ffi_type_sint32, ffi_type_uint8, ffi_type_void,
};
use libloading::{Library, Symbol};
use napi::bindgen_prelude::*;
use napi::{Env, JsFunction, JsNumber, JsObject, JsUnknown};

use std::alloc::{alloc, Layout};
use std::collections::HashMap;
use std::ffi::c_void;
use std::ffi::{CStr, CString};

use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use utils::calculate::*;
use utils::pointer::*;
use utils::struct_utils::*;
use utils::transform::*;

static mut LIBRARY_MAP: Option<HashMap<String, Library>> = None;
static mut FUNC_DESC: Option<HashMap<usize, IndexMap<String, RsArgsValue>>> = None;
static mut TS_FN: Option<
  HashMap<usize, ThreadsafeFunction<Vec<RsArgsValue>, ErrorStrategy::Fatal>>,
> = None;

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
unsafe fn load(
  env: Env,
  params: FFIParams,
) -> Either9<String, i32, (), f64, Vec<i32>, Vec<String>, Vec<f64>, bool, JsObject> {
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
  let (mut arg_types, arg_values): (Vec<*mut ffi_type>, Vec<RsArgsValue>) = params_type
    .into_iter()
    .zip(params_value.into_iter())
    .map(|(param, value)| {
      let value_type = param.get_type().unwrap();
      match value_type {
        ValueType::Number => {
          let param_data_type =
            number_to_data_type(param.coerce_to_number().unwrap().try_into().unwrap());
          match param_data_type {
            DataType::I32 => {
              let arg_type = &mut ffi_type_sint32 as *mut ffi_type;
              let arg_val: i32 = value.coerce_to_number().unwrap().try_into().unwrap();
              (arg_type, RsArgsValue::I32(arg_val))
            }
            DataType::Double => {
              let arg_type = &mut ffi_type_double as *mut ffi_type;
              let arg_val: f64 = value.coerce_to_number().unwrap().try_into().unwrap();
              (arg_type, RsArgsValue::Double(arg_val))
            }
            DataType::String => {
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
            DataType::I32Array => {
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
            DataType::DoubleArray => {
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
            DataType::StringArray => {
              let arg_type = &mut ffi_type_pointer as *mut ffi_type;
              let js_object = value.coerce_to_object().unwrap();
              let arg_val = js_array_to_string_array(js_object);
              (arg_type, RsArgsValue::StringArray(arg_val))
            }
            DataType::Boolean => {
              let arg_type = &mut ffi_type_uint8 as *mut ffi_type;
              let arg_val: bool = value.coerce_to_bool().unwrap().get_value().unwrap();
              (arg_type, RsArgsValue::Boolean(arg_val))
            }
            DataType::Void => {
              let arg_type = &mut ffi_type_void as *mut ffi_type;
              (arg_type, RsArgsValue::Void(()))
            }
          }
        }
        ValueType::Object => {
          let params_type_object: JsObject = param.coerce_to_object().unwrap();
          let arg_type = &mut ffi_type_pointer as *mut ffi_type;
          let params_value_object = value.coerce_to_object().unwrap();
          let index_map = get_params_value_rs_struct(&params_type_object, &params_value_object);
          (arg_type, RsArgsValue::Object(index_map))
        }
        ValueType::Function => {
          let params_type_function: JsFunction = param.try_into().unwrap();
          let params_val_function: JsFunction = value.try_into().unwrap();
          let arg_type = &mut ffi_type_pointer as *mut ffi_type;
          (
            arg_type,
            RsArgsValue::Function(params_type_function, params_val_function),
          )
        }
        _ => panic!("unsupported params type {:?}", value_type),
      }
    })
    .unzip();
  let mut arg_values_c_void: Vec<*mut c_void> = arg_values
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
      RsArgsValue::Boolean(val) => {
        let c_bool = Box::new(val);
        Box::into_raw(c_bool) as *mut c_void
      }
      RsArgsValue::Void(_) => Box::into_raw(Box::new(())) as *mut c_void,
      RsArgsValue::Function(func_desc, js_function) => {
        use libffi::high::*;
        let func_desc_obj = func_desc
          .call_without_args(None)
          .unwrap()
          .coerce_to_object()
          .unwrap();
        let func_args_type: JsObject = func_desc_obj
          .get_property(env.create_string("paramsType").unwrap())
          .unwrap();
        let args_len = func_args_type.get_array_length().unwrap();
        let func_args_type_rs = type_define_to_rs_struct(&func_args_type);
        let func_args_type_rs_ptr = Box::into_raw(Box::new(func_args_type_rs));
        if args_len > 10 {
          panic!("The number of function parameters needs to be less than or equal to 10")
        }

        let tsfn: ThreadsafeFunction<Vec<RsArgsValue>, ErrorStrategy::Fatal> = (&js_function)
          .create_threadsafe_function(0, |ctx| {
            let value: Vec<RsArgsValue> = ctx.value;
            let js_call_params: Vec<JsUnknown> = value
              .into_iter()
              .map(|rs_args| return rs_value_to_js_unknown(&ctx.env, rs_args))
              .collect();

            Ok(js_call_params)
          })
          .unwrap();
        let tsfn_ptr = Box::into_raw(Box::new(tsfn));
        return match_args_len!(args_len, tsfn_ptr, func_args_type_rs_ptr,
            1 => Closure1, a
            ,2 => Closure2, a,b
            ,3 => Closure3, a,b,c
            ,4 => Closure4, a,b,c,d
            ,5 => Closure5, a,b,c,d,e
            ,6 => Closure6, a,b,c,d,e,f
            ,7 => Closure7, a,b,c,d,e,f,g
            ,8 => Closure8, a,b,c,d,e,f,g,h
            ,9 => Closure9, a,b,c,d,e,f,g,h,i
            ,10 => Closure10, a,b,c,d,e,f,g,h,i,j
        );
      }
      RsArgsValue::Object(val) => {
        let (size, _) = calculate_layout(&val);
        let layout = if size > 0 {
          let (_, first_field) = val.get_index(0).unwrap();
          let (_, align) = get_rs_value_size_align(first_field);
          Layout::from_size_align(size, align).unwrap()
        } else {
          Layout::new::<i32>()
        };

        let ptr = alloc(layout) as *mut c_void;
        let field_ptr = ptr;
        unsafe fn write_data(map: IndexMap<String, RsArgsValue>, mut field_ptr: *mut c_void) {
          let mut offset = 0;
          for (_, field_val) in map {
            match field_val {
              RsArgsValue::I32(number) => {
                let align = std::mem::align_of::<c_int>();
                let padding = (align - (offset % align)) % align;
                field_ptr = field_ptr.offset(padding as isize);
                (field_ptr as *mut c_int).write(number);
                offset = std::mem::size_of::<c_int>();
              }
              RsArgsValue::Double(double_number) => {
                let align = std::mem::align_of::<c_double>();
                let padding = (align - (offset % align)) % align;
                field_ptr = field_ptr.offset(padding as isize);
                (field_ptr as *mut c_double).write(double_number);
                offset = std::mem::size_of::<c_double>();
              }
              RsArgsValue::Boolean(val) => {
                let align = std::mem::align_of::<bool>();
                let padding = (align - (offset % align)) % align;
                field_ptr = field_ptr.offset(padding as isize);
                (field_ptr as *mut bool).write(val);
                offset = std::mem::size_of::<bool>();
              }
              RsArgsValue::String(str) => {
                let align = std::mem::align_of::<*const c_char>();
                let padding = (align - (offset % align)) % align;
                field_ptr = field_ptr.offset(padding as isize);
                let c_string = CString::new(str).unwrap();
                (field_ptr as *mut *const c_char).write(c_string.as_ptr());
                std::mem::forget(c_string);
                offset = std::mem::size_of::<*const c_char>();
              }
              RsArgsValue::StringArray(str_arr) => {
                let align = std::mem::align_of::<*const *const c_char>();
                let padding = (align - (offset % align)) % align;
                field_ptr = field_ptr.offset(padding as isize);
                let c_char_vec: Vec<*const c_char> = str_arr
                  .into_iter()
                  .map(|str| {
                    let c_string = CString::new(str).unwrap();
                    let ptr = c_string.as_ptr();
                    std::mem::forget(c_string);
                    return ptr;
                  })
                  .collect();
                (field_ptr as *mut *const *const c_char).write(c_char_vec.as_ptr());
                std::mem::forget(c_char_vec);
                offset = std::mem::size_of::<*const *const c_char>();
              }
              RsArgsValue::DoubleArray(arr) => {
                let align = std::mem::align_of::<*const c_double>();
                let padding = (align - (offset % align)) % align;
                field_ptr = field_ptr.offset(padding as isize);
                (field_ptr as *mut *const c_double).write(arr.as_ptr());
                std::mem::forget(arr);
                offset = std::mem::size_of::<*const c_double>();
              }
              RsArgsValue::I32Array(arr) => {
                let align = std::mem::align_of::<*const c_int>();
                let padding = (align - (offset % align)) % align;
                field_ptr = field_ptr.offset(padding as isize);
                (field_ptr as *mut *const c_int).write(arr.as_ptr());
                std::mem::forget(arr);
                offset = std::mem::size_of::<*const c_int>();
              }
              RsArgsValue::Object(val) => {
                let (mut size, align) = calculate_layout(&val);
                let padding = (align - (offset % align)) % align;
                field_ptr = field_ptr.offset(padding as isize);
                write_data(val, field_ptr);
                offset = size;
              }
              _ => panic!("write_data error {:?}", field_val),
            }
            field_ptr = field_ptr.offset(offset as isize);
          }
        }
        write_data(val, field_ptr);
        return Box::into_raw(Box::new(ptr)) as *mut c_void;
      }
    })
    .collect();
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
        BasicDataType::I32 => &mut ffi_type_sint32 as *mut ffi_type,
        BasicDataType::String => &mut ffi_type_pointer as *mut ffi_type,
        BasicDataType::Void => &mut ffi_type_void as *mut ffi_type,
        BasicDataType::Double => &mut ffi_type_double as *mut ffi_type,
        BasicDataType::Boolean => &mut ffi_type_uint8 as *mut ffi_type,
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

          Either9::A(result_str)
        }
        BasicDataType::I32 => {
          let mut result: i32 = 0;
          ffi_call(
            &mut cif,
            Some(*func),
            &mut result as *mut i32 as *mut c_void,
            arg_values_c_void.as_mut_ptr(),
          );
          Either9::B(result)
        }
        BasicDataType::Void => {
          let mut result = ();
          ffi_call(
            &mut cif,
            Some(*func),
            &mut result as *mut () as *mut c_void,
            arg_values_c_void.as_mut_ptr(),
          );
          Either9::C(())
        }
        BasicDataType::Double => {
          let mut result: f64 = 0.0;
          ffi_call(
            &mut cif,
            Some(*func),
            &mut result as *mut f64 as *mut c_void,
            arg_values_c_void.as_mut_ptr(),
          );
          Either9::D(result)
        }
        BasicDataType::Boolean => {
          let mut result: bool = false;
          ffi_call(
            &mut cif,
            Some(*func),
            &mut result as *mut bool as *mut c_void,
            arg_values_c_void.as_mut_ptr(),
          );

          Either9::H(result)
        }
      }
    }
    RsArgsValue::Object(obj) => {
      let array_desc = get_array_desc(&obj);
      if array_desc.is_some() {
        // array
        let (array_len, array_type) = array_desc.unwrap();
        match array_type {
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
            Either9::E(arr)
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
            Either9::G(arr)
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
            Either9::F(arr)
          }
        }
      } else {
        // raw object
        let ret_fields_size = obj.keys().into_iter().fold(0, |pre, current| {
          let size = pre;
          let val = obj.get(current).unwrap();
          let (field_size, _) = get_rs_struct_size_align(val);
          size + field_size
        });

        let mut result: *mut c_void = malloc(ret_fields_size);
        ffi_call(
          &mut cif,
          Some(*func),
          &mut result as *mut _ as *mut c_void,
          arg_values_c_void.as_mut_ptr(),
        );
        let rs_struct = create_rs_struct_from_pointer(result, &obj);
        let mut js_object = env.create_object().unwrap();
        for (field, value) in rs_struct {
          js_object
            .set_property(
              env.create_string(&field).unwrap(),
              rs_value_to_js_unknown(&env, value),
            )
            .unwrap();
        }
        Either9::I(js_object)
      }
    }
    _ => panic!("ret_type err {:?}", ret_value),
  }
}
