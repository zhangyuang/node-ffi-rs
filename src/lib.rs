#[macro_use]
extern crate napi_derive;
mod define;
mod utils;
use define::{number_to_data_type, DataType, FFIParams, OpenParams, RsArgsValue};
use napi::bindgen_prelude::*;
use utils::{
  align_ptr, calculate_layout, create_array_from_pointer, get_data_type_size_align,
  get_js_function_call_value, js_array_to_number_array, js_array_to_string_array,
  js_nunmber_to_i32, js_string_to_string, js_unknown_to_data_type, rs_array_to_js_array,
  ArrayPointerType, ArrayType,
};

use indexmap::IndexMap;
use libc::malloc;
use libc::{c_char, c_double, c_int};
use libffi_sys::{
  ffi_abi_FFI_DEFAULT_ABI, ffi_call, ffi_cif, ffi_prep_cif, ffi_type, ffi_type_double,
  ffi_type_pointer, ffi_type_sint32, ffi_type_uint8, ffi_type_void,
};
use libloading::{Library, Symbol};
use napi::{Env, JsFunction, JsNumber, JsObject, JsString, JsUnknown};
use std::alloc::{alloc, Layout};
use std::collections::HashMap;
use std::ffi::c_void;
use std::ffi::CString;

enum FFIJsValue {
  I32(i32),
  JsObject(JsObject),
  Unknown,
}

static mut LibraryMap: Option<HashMap<String, Library>> = None;

#[napi]
fn open(params: OpenParams) {
  let OpenParams { library, path } = params;
  unsafe {
    if LibraryMap.is_none() {
      LibraryMap = Some(HashMap::new());
    }
    let map = LibraryMap.as_mut().unwrap();
    if map.get(&library).is_none() {
      let lib = Library::new(path).unwrap();
      map.insert(library, lib);
    }
  }
}

#[napi]
fn close(library: String) {
  unsafe {
    if LibraryMap.is_none() {
      return;
    }
    let map = LibraryMap.as_mut().unwrap();
    map.remove(&library);
  }
}

#[napi]
fn load(
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
  unsafe {
    let lib = LibraryMap.as_ref().unwrap();
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
                let arg_val: bool = value.coerce_to_bool().unwrap().try_into().unwrap();
                (arg_type, RsArgsValue::Boolean(arg_val))
              }
              DataType::Void => {
                let arg_type = &mut ffi_type_void as *mut ffi_type;
                (arg_type, RsArgsValue::Void(()))
              }
              _ => panic!(""),
            }
          }
          ValueType::Object => {
            let params_type_object: JsObject = param.coerce_to_object().unwrap();
            let arg_type = &mut ffi_type_pointer as *mut ffi_type;
            let params_value_object = value.coerce_to_object().unwrap();
            fn jsobject_to_rs_struct(
              params_type_object: JsObject,
              params_value_object: JsObject,
            ) -> IndexMap<String, RsArgsValue> {
              let mut index_map = IndexMap::new();
              JsObject::keys(&params_value_object)
                .unwrap()
                .into_iter()
                .for_each(|field| {
                  let field_type: DataType = params_type_object.get_named_property(&field).unwrap();
                  let val = match field_type {
                    DataType::String => {
                      let val: JsString = params_value_object.get_named_property(&field).unwrap();
                      let val: String = val.into_utf8().unwrap().try_into().unwrap();
                      RsArgsValue::String(val)
                    }
                    DataType::I32 => {
                      let val: JsNumber = params_value_object.get_named_property(&field).unwrap();
                      let val: i32 = val.try_into().unwrap();
                      RsArgsValue::I32(val)
                    }
                    DataType::Double => {
                      let val: JsNumber = params_value_object.get_named_property(&field).unwrap();
                      let val: f64 = val.try_into().unwrap();
                      RsArgsValue::Double(val)
                    }
                    DataType::StringArray => {
                      let js_array: JsObject =
                        params_value_object.get_named_property(&field).unwrap();
                      let arg_val = js_array_to_string_array(js_array);
                      RsArgsValue::StringArray(arg_val)
                    }
                    DataType::DoubleArray => {
                      let js_array: JsObject =
                        params_value_object.get_named_property(&field).unwrap();
                      let arg_val = js_array_to_number_array(js_array);
                      RsArgsValue::DoubleArray(arg_val)
                    }
                    DataType::I32Array => {
                      let js_array: JsObject =
                        params_value_object.get_named_property(&field).unwrap();
                      let arg_val = js_array_to_number_array(js_array);
                      RsArgsValue::I32Array(arg_val)
                    }
                    // DataType::Object => {
                    //   let val: JsObject = js_object.get_named_property(&field).unwrap();
                    //   let index_map = jsobject_to_rs_struct(val);
                    //   RsArgsValue::Object(index_map)
                    // }
                    _ => panic!("jsobject_to_rs_struct"),
                  };
                  index_map.insert(field, val);
                });
              index_map
            }
            let index_map = jsobject_to_rs_struct(params_type_object, params_value_object);
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
          _ => panic!("unknow params type"),
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
          let func_desc_obj = func_desc
            .call_without_args(None)
            .unwrap()
            .coerce_to_object()
            .unwrap();
          let func_args: JsObject = func_desc_obj.get_named_property("paramsType").unwrap();
          match func_args.get_array_length().unwrap() {
            0 => {
              use libffi::high::Closure0;
              let lambda = || {
                js_function.call_without_args(None).unwrap();
              };
              let closure = Box::into_raw(Box::new(Closure0::new(&lambda)));
              return std::mem::transmute((*closure).code_ptr());
            }
            1 => {
              use libffi::high::Closure1;
              let lambda = |a: *mut c_void| {
                let value = func_args.get_element::<JsUnknown>(0).unwrap();
                let foo = get_js_function_call_value(env, value, a);
              };
              let closure = Box::into_raw(Box::new(Closure1::new(&lambda)));
              return std::mem::transmute((*closure).code_ptr());
            }
            _ => panic!("func_args get array error"),
          };
        }
        RsArgsValue::Object(val) => {
          let (size, align) = calculate_layout(&val);
          let layout = if size > 0 && align > 0 {
            Layout::from_size_align(size, align).unwrap()
          } else {
            Layout::new::<i32>()
          };

          let ptr = alloc(layout) as *mut c_void;
          let field_ptr = ptr;
          unsafe fn write_data(
            map: IndexMap<String, RsArgsValue>,
            mut field_ptr: *mut c_void,
            align: usize,
          ) {
            for (_, field_val) in map {
              match field_val {
                RsArgsValue::I32(number) => {
                  (field_ptr as *mut c_int).write(number);
                  field_ptr =
                    field_ptr.offset(std::mem::size_of::<c_int>() as isize) as *mut c_void;
                  field_ptr = align_ptr(field_ptr, align);
                }
                RsArgsValue::Double(double_number) => {
                  (field_ptr as *mut c_double).write(double_number);
                  field_ptr =
                    field_ptr.offset(std::mem::size_of::<c_double>() as isize) as *mut c_void;
                  field_ptr = align_ptr(field_ptr, align);
                }
                RsArgsValue::String(str) => {
                  let c_string = CString::new(str).unwrap();
                  (field_ptr as *mut *const c_char).write(c_string.as_ptr());
                  std::mem::forget(c_string);
                  field_ptr =
                    field_ptr.offset(std::mem::size_of::<*const c_char>() as isize) as *mut c_void;
                  field_ptr = align_ptr(field_ptr, align);
                }
                RsArgsValue::StringArray(str_arr) => {
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
                  field_ptr = field_ptr.offset(std::mem::size_of::<*const *const c_char>() as isize)
                    as *mut c_void;
                  field_ptr = align_ptr(field_ptr, align);
                }
                RsArgsValue::DoubleArray(arr) => {
                  (field_ptr as *mut *const c_double).write(arr.as_ptr());
                  std::mem::forget(arr);
                  field_ptr = field_ptr.offset(std::mem::size_of::<*const c_double>() as isize)
                    as *mut c_void;
                  field_ptr = align_ptr(field_ptr, align);
                }
                RsArgsValue::I32Array(arr) => {
                  (field_ptr as *mut *const c_int).write(arr.as_ptr());
                  std::mem::forget(arr);
                  field_ptr =
                    field_ptr.offset(std::mem::size_of::<*const c_int>() as isize) as *mut c_void;
                  field_ptr = align_ptr(field_ptr, align);
                }
                RsArgsValue::Object(val) => {
                  let (size, _) = calculate_layout(&val);
                  write_data(val, field_ptr, align);
                  field_ptr = field_ptr.offset(size as isize) as *mut c_void;
                  field_ptr = align_ptr(field_ptr, align);
                }
                _ => panic!("write_data"),
              }
            }
          }
          write_data(val, field_ptr, align);
          return Box::into_raw(Box::new(ptr)) as *mut c_void;
        }
      })
      .collect();

    let ret_value_type = ret_type.get_type().unwrap();
    let ret_value: FFIJsValue = match ret_value_type {
      ValueType::Number => {
        FFIJsValue::I32(ret_type.coerce_to_number().unwrap().try_into().unwrap())
      }
      ValueType::Object => FFIJsValue::JsObject(ret_type.coerce_to_object().unwrap()),
      _ => FFIJsValue::Unknown,
    };
    let r_type: *mut ffi_type = match ret_value {
      FFIJsValue::I32(number) => {
        let ret_data_type = number_to_data_type(number);
        match ret_data_type {
          DataType::I32 => &mut ffi_type_sint32 as *mut ffi_type,
          DataType::String => &mut ffi_type_pointer as *mut ffi_type,
          DataType::Void => &mut ffi_type_void as *mut ffi_type,
          DataType::Double => &mut ffi_type_double as *mut ffi_type,
          DataType::I32Array => &mut ffi_type_pointer as *mut ffi_type,
          DataType::StringArray => &mut ffi_type_pointer as *mut ffi_type,
          DataType::DoubleArray => &mut ffi_type_pointer as *mut ffi_type,
          DataType::Boolean => &mut ffi_type_uint8 as *mut ffi_type,
        }
      }
      FFIJsValue::JsObject(_) => &mut ffi_type_pointer as *mut ffi_type,
      FFIJsValue::Unknown => &mut ffi_type_void as *mut ffi_type,
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
      FFIJsValue::I32(number) => {
        let ret_data_type = number_to_data_type(number);
        match ret_data_type {
          DataType::String => {
            let mut result: *mut c_char = malloc(std::mem::size_of::<*mut c_char>()) as *mut c_char;
            ffi_call(
              &mut cif,
              Some(*func),
              &mut result as *mut *mut c_char as *mut c_void,
              arg_values_c_void.as_mut_ptr(),
            );

            let result_str = CString::from_raw(result)
              .into_string()
              .expect(format!("{} retType is not string", func_name).as_str());

            Either9::A(result_str)
          }
          DataType::I32 => {
            let mut result: i32 = 0;
            ffi_call(
              &mut cif,
              Some(*func),
              &mut result as *mut i32 as *mut c_void,
              arg_values_c_void.as_mut_ptr(),
            );
            Either9::B(result)
          }
          DataType::Void => {
            let mut result = ();
            ffi_call(
              &mut cif,
              Some(*func),
              &mut result as *mut () as *mut c_void,
              arg_values_c_void.as_mut_ptr(),
            );
            Either9::C(())
          }
          DataType::Double => {
            let mut result: f64 = 0.0;
            ffi_call(
              &mut cif,
              Some(*func),
              &mut result as *mut f64 as *mut c_void,
              arg_values_c_void.as_mut_ptr(),
            );
            Either9::D(result)
          }
          DataType::Boolean => {
            let mut result: u8 = 255;
            ffi_call(
              &mut cif,
              Some(*func),
              &mut result as *mut u8 as *mut c_void,
              arg_values_c_void.as_mut_ptr(),
            );
            if result != 0 && result != 1 {
              panic!("The returned type is not a boolean")
            }
            Either9::H(if result == 0 { false } else { true })
          }
          _ => {
            panic!(
              "{:?} is not currently avaiable as a return type",
              ret_data_type
            )
          }
        }
      }
      FFIJsValue::JsObject(ret_object) => {
        let ffi_tag = ret_object.has_named_property("ffiTypeTag").unwrap();
        if ffi_tag {
          let ffi_tag: &str = &js_string_to_string(
            ret_object
              .get_named_property::<JsString>("ffiTypeTag")
              .unwrap(),
          );
          match ffi_tag {
            "array" => {
              let array_len: usize =
                js_nunmber_to_i32(ret_object.get_named_property::<JsNumber>("length").unwrap())
                  as usize;

              let array_type: i32 =
                js_nunmber_to_i32(ret_object.get_named_property::<JsNumber>("type").unwrap());
              let array_type = number_to_data_type(array_type);
              match array_type {
                DataType::I32Array => {
                  let mut result: *mut c_int =
                    malloc(std::mem::size_of::<*mut c_int>()) as *mut c_int;
                  ffi_call(
                    &mut cif,
                    Some(*func),
                    &mut result as *mut _ as *mut c_void,
                    arg_values_c_void.as_mut_ptr(),
                  );
                  let arr = create_array_from_pointer(ArrayPointerType::I32(result), array_len);
                  if !result.is_null() {
                    libc::free(result as *mut c_void);
                  }
                  match arr {
                    ArrayType::I32(arr) => Either9::E(arr),
                    _ => panic!("some error"),
                  }
                }
                DataType::DoubleArray => {
                  let mut result: *mut c_double =
                    malloc(std::mem::size_of::<*mut c_double>()) as *mut c_double;
                  ffi_call(
                    &mut cif,
                    Some(*func),
                    &mut result as *mut _ as *mut c_void,
                    arg_values_c_void.as_mut_ptr(),
                  );
                  let arr = create_array_from_pointer(ArrayPointerType::Double(result), array_len);
                  if !result.is_null() {
                    libc::free(result as *mut c_void);
                  }
                  match arr {
                    ArrayType::Double(arr) => Either9::G(arr),
                    _ => panic!("some error"),
                  }
                }
                DataType::StringArray => {
                  let mut result: *mut *mut c_char =
                    malloc(std::mem::size_of::<*mut *mut c_char>()) as *mut *mut c_char;

                  ffi_call(
                    &mut cif,
                    Some(*func),
                    &mut result as *mut _ as *mut c_void,
                    arg_values_c_void.as_mut_ptr(),
                  );
                  let arr = create_array_from_pointer(ArrayPointerType::String(result), array_len);
                  if !result.is_null() {
                    libc::free(result as *mut c_void);
                  }
                  match arr {
                    ArrayType::String(arr) => Either9::F(arr),
                    _ => panic!("some error"),
                  }
                }
                _ => panic!("some error"),
              }
            }
            _ => panic!("some error"),
          }
        } else {
          let (ret_fields_size, align) =
            JsObject::keys(&ret_object)
              .unwrap()
              .into_iter()
              .fold((0, 0), |pre, current| {
                let (size, mut align) = pre;
                let val: JsUnknown = ret_object.get_named_property(&current).unwrap();
                let data_type = js_unknown_to_data_type(val);
                let (field_size, field_align) = get_data_type_size_align(data_type);
                align = align.max(field_align);
                (size + field_size, align)
              });
          let mut result: *mut c_void = malloc(ret_fields_size);
          ffi_call(
            &mut cif,
            Some(*func),
            &mut result as *mut _ as *mut c_void,
            arg_values_c_void.as_mut_ptr(),
          );
          let mut js_object = env.create_object().unwrap();
          let mut field_ptr = result;
          JsObject::keys(&ret_object)
            .unwrap()
            .into_iter()
            .for_each(|field| {
              let val: JsUnknown = ret_object.get_named_property(&field).unwrap();
              let data_type = js_unknown_to_data_type(val);
              let array_constructor: JsObject = ret_object.get_named_property(&field).unwrap();
              let array_len: usize = if array_constructor.has_named_property("length").unwrap() {
                js_nunmber_to_i32(
                  array_constructor
                    .get_named_property::<JsNumber>("length")
                    .unwrap(),
                ) as usize
              } else {
                0
              };
              match data_type {
                DataType::I32 => {
                  let type_field_ptr = field_ptr as *mut i32;
                  js_object
                    .set_property(
                      env.create_string(&field).unwrap(),
                      env.create_int32(*type_field_ptr).unwrap(),
                    )
                    .unwrap();
                  field_ptr =
                    field_ptr.offset(std::mem::size_of::<c_int>() as isize) as *mut c_void;
                  field_ptr = align_ptr(field_ptr, align);
                }
                DataType::Double => {
                  let type_field_ptr = field_ptr as *mut c_double;
                  js_object
                    .set_property(
                      env.create_string(&field).unwrap(),
                      env.create_double(*type_field_ptr).unwrap(),
                    )
                    .unwrap();
                  field_ptr =
                    field_ptr.offset(std::mem::size_of::<c_int>() as isize) as *mut c_void;
                  field_ptr = align_ptr(field_ptr, align);
                }
                DataType::String => {
                  let type_field_ptr = field_ptr as *mut *mut c_char;
                  let js_string = CString::from_raw(*type_field_ptr).into_string().unwrap();
                  js_object
                    .set_property(
                      env.create_string(&field).unwrap(),
                      env.create_string(&js_string).unwrap(),
                    )
                    .unwrap();
                  field_ptr =
                    field_ptr.offset(std::mem::size_of::<*const c_char>() as isize) as *mut c_void;
                  field_ptr = align_ptr(field_ptr, align);
                }
                DataType::StringArray => {
                  let type_field_ptr = field_ptr as *mut *mut *mut c_char;
                  let arr =
                    create_array_from_pointer(ArrayPointerType::String(*type_field_ptr), array_len);
                  match arr {
                    ArrayType::String(arr) => {
                      let js_array = rs_array_to_js_array(env, ArrayType::String(arr));
                      js_object
                        .set_property(env.create_string(&field).unwrap(), js_array)
                        .unwrap();
                    }
                    _ => panic!("some error"),
                  }
                  field_ptr = field_ptr.offset(std::mem::size_of::<*const *const c_char>() as isize)
                    as *mut c_void;
                  field_ptr = align_ptr(field_ptr, align);
                }
                DataType::DoubleArray => {
                  let type_field_ptr = field_ptr as *mut *mut c_double;
                  let arr =
                    create_array_from_pointer(ArrayPointerType::Double(*type_field_ptr), array_len);
                  match arr {
                    ArrayType::Double(arr) => {
                      let js_array = rs_array_to_js_array(env, ArrayType::Double(arr));
                      js_object
                        .set_property(env.create_string(&field).unwrap(), js_array)
                        .unwrap();
                    }
                    _ => panic!("some error"),
                  }
                  field_ptr = field_ptr.offset(std::mem::size_of::<*const c_double>() as isize)
                    as *mut c_void;
                  field_ptr = align_ptr(field_ptr, align);
                }
                DataType::I32Array => {
                  let type_field_ptr = field_ptr as *mut *mut c_int;
                  let arr =
                    create_array_from_pointer(ArrayPointerType::I32(*type_field_ptr), array_len);
                  match arr {
                    ArrayType::I32(arr) => {
                      let js_array = rs_array_to_js_array(env, ArrayType::I32(arr));
                      js_object
                        .set_property(env.create_string(&field).unwrap(), js_array)
                        .unwrap();
                    }
                    _ => panic!("some error"),
                  }
                  field_ptr =
                    field_ptr.offset(std::mem::size_of::<*const c_int>() as isize) as *mut c_void;
                  field_ptr = align_ptr(field_ptr, align);
                }

                _ => panic!(
                  "{:?} is not available as a field type at this time",
                  data_type
                ),
              }
            });
          Either9::I(js_object)
        }
      }

      FFIJsValue::Unknown => Either9::C(()),
    }
  }
}
