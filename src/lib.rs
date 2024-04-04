#[macro_use]
extern crate napi_derive;
use napi::bindgen_prelude::*;

use indexmap::IndexMap;
use libc::malloc;
use libc::{c_char, c_double, c_int};
use libffi_sys::{
  ffi_abi_FFI_DEFAULT_ABI, ffi_call, ffi_cif, ffi_prep_cif, ffi_type, ffi_type_double,
  ffi_type_pointer, ffi_type_sint32, ffi_type_uint8, ffi_type_void,
};
use libloading::{Library, Symbol};
use napi::{Env, JsNumber, JsObject, JsString, JsUnknown};
use std::alloc::{alloc, Layout};
use std::collections::HashMap;
use std::ffi::c_void;
use std::ffi::CString;

#[derive(Debug)]
pub enum RsArgsValue {
  String(String),
  I32(i32),
  Double(f64),
  I32Array(Vec<i32>),
  StringArray(Vec<String>),
  DoubleArray(Vec<f64>),
  Object(IndexMap<String, RsArgsValue>),
  Boolean(bool),
}

#[napi]
#[derive(Debug)]
pub enum DataType {
  String = 0,
  I32 = 1,
  Double = 2,
  I32Array = 3,
  StringArray = 4,
  DoubleArray = 5,
  Boolean = 6,
  Object = 7,
  // Function = 8,
  Void = 9,
}
pub fn number_to_data_type(value: i32) -> DataType {
  match value {
    0 => DataType::String,
    1 => DataType::I32,
    2 => DataType::Double,
    3 => DataType::I32Array,
    4 => DataType::StringArray,
    5 => DataType::DoubleArray,
    6 => DataType::Boolean,
    7 => DataType::Object,
    // 8 => DataType::Function,
    9 => DataType::Void,
    _ => panic!("unknow DataType"),
  }
}

#[napi(object)]
struct FFIParams {
  pub library: String,
  pub func_name: String,
  pub ret_type: JsUnknown,
  pub ret_type_len: Option<u32>,
  pub params_type: Vec<JsUnknown>,
  pub params_value: Vec<JsUnknown>,
}
#[napi(object)]
struct OpenParams {
  pub library: String,
  pub path: String,
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
    ret_type_len,
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
              DataType::Boolean => {
                let arg_type = &mut ffi_type_uint8 as *mut ffi_type;
                let arg_val: bool = value.coerce_to_bool().unwrap().try_into().unwrap();
                (arg_type, RsArgsValue::Boolean(arg_val))
              }
              // DataType::Function => {
              //   let arg_type = &mut ffi_type_pointer as *mut ffi_type;
              //   let arg_val: JsFunction = value.try_into().unwrap();
              //   (arg_type, RsArgsValue::Boolean(true))
              // }
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
          _ => panic!("unknow params type"),
        }
      })
      .unzip();

    let mut arg_values_cvoid: Vec<*mut c_void> = arg_values
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
        RsArgsValue::Object(val) => {
          fn calculate_layout(map: &IndexMap<String, RsArgsValue>) -> (usize, usize) {
            let (size, align) =
              map
                .iter()
                .fold((0, 0), |(size, align), (_, field_val)| match field_val {
                  RsArgsValue::I32(_) => {
                    let align = align.max(std::mem::align_of::<c_int>());
                    let size = size + std::mem::size_of::<c_int>();
                    (size, align)
                  }
                  RsArgsValue::String(_) => {
                    let align = align.max(std::mem::align_of::<*const c_char>());
                    let size = size + std::mem::size_of::<*const c_char>();
                    (size, align)
                  }
                  RsArgsValue::Object(val) => {
                    let (obj_size, obj_align) = calculate_layout(val);
                    let align = align.max(obj_align);
                    let size = size + obj_size;
                    (size, align)
                  }
                  _ => panic!("calculate_layout"),
                });

            (size, align)
          }
          let (size, align) = calculate_layout(&val);
          let layout = Layout::from_size_align(size, align).unwrap();
          let ptr = alloc(layout) as *mut c_void;
          let field_ptr = ptr;
          unsafe fn write_data(map: IndexMap<String, RsArgsValue>, mut field_ptr: *mut c_void) {
            for (_, field_val) in map {
              match field_val {
                RsArgsValue::I32(number) => {
                  (field_ptr as *mut c_int).write(number);
                  field_ptr =
                    field_ptr.offset(std::mem::size_of::<c_int>() as isize) as *mut c_void;
                }
                RsArgsValue::String(str) => {
                  let c_string = CString::new(str).unwrap();
                  (field_ptr as *mut *const c_char).write(c_string.as_ptr());
                  std::mem::forget(c_string);
                  field_ptr =
                    field_ptr.offset(std::mem::size_of::<*const c_char>() as isize) as *mut c_void;
                }
                RsArgsValue::Object(val) => {
                  let (size, _) = calculate_layout(&val);
                  write_data(val, field_ptr);
                  field_ptr = field_ptr.offset(size as isize) as *mut c_void;
                }
                _ => panic!("write_data"),
              }
            }
          }
          write_data(val, field_ptr);
          return Box::into_raw(Box::new(ptr)) as *mut c_void;
        }
      })
      .collect();
    let ret_value_type = ret_type.get_type().unwrap();
    let (ret_number, ret_object): (Option<i32>, Option<JsObject>) = match ret_value_type {
      ValueType::Number => (
        Some(ret_type.coerce_to_number().unwrap().try_into().unwrap()),
        None,
      ),
      ValueType::Object => {
        let params_type_object: JsObject = ret_type.coerce_to_object().unwrap();
        (None, Some(params_type_object))
      }
      _ => (None, None),
    };
    let r_type: Option<*mut ffi_type> = if ret_number.is_some() {
      let ret_number = number_to_data_type(ret_number.unwrap());
      let res = match ret_number {
        DataType::I32 => &mut ffi_type_sint32 as *mut ffi_type,
        DataType::String => &mut ffi_type_pointer as *mut ffi_type,
        DataType::Void => &mut ffi_type_void as *mut ffi_type,
        DataType::Double => &mut ffi_type_double as *mut ffi_type,
        DataType::I32Array => &mut ffi_type_pointer as *mut ffi_type,
        DataType::StringArray => &mut ffi_type_pointer as *mut ffi_type,
        DataType::DoubleArray => &mut ffi_type_pointer as *mut ffi_type,
        DataType::Object => &mut ffi_type_pointer as *mut ffi_type,
        DataType::Boolean => &mut ffi_type_uint8 as *mut ffi_type,
      };
      Some(res)
    } else if ret_object.is_some() {
      Some(&mut ffi_type_pointer as *mut ffi_type)
    } else {
      None
    };
    if r_type.is_none() {
      panic!("current DataType is not currently supported")
    }
    let r_type = r_type.unwrap();
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

    if ret_number.is_some() {
      let ret_number = number_to_data_type(ret_number.unwrap());
      match ret_number {
        DataType::String => {
          let mut result: *mut c_char = malloc(std::mem::size_of::<*mut c_char>()) as *mut c_char;
          ffi_call(
            &mut cif,
            Some(*func),
            &mut result as *mut *mut c_char as *mut c_void,
            arg_values_cvoid.as_mut_ptr(),
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
            arg_values_cvoid.as_mut_ptr(),
          );
          Either9::B(result)
        }
        DataType::Void => {
          let mut result = ();
          ffi_call(
            &mut cif,
            Some(*func),
            &mut result as *mut () as *mut c_void,
            arg_values_cvoid.as_mut_ptr(),
          );
          Either9::C(())
        }
        DataType::Double => {
          let mut result: f64 = 0.0;
          ffi_call(
            &mut cif,
            Some(*func),
            &mut result as *mut f64 as *mut c_void,
            arg_values_cvoid.as_mut_ptr(),
          );
          Either9::D(result)
        }
        DataType::I32Array => {
          let mut result: *mut c_int = malloc(std::mem::size_of::<*mut c_int>()) as *mut c_int;
          ffi_call(
            &mut cif,
            Some(*func),
            &mut result as *mut _ as *mut c_void,
            arg_values_cvoid.as_mut_ptr(),
          );

          let result_slice = std::slice::from_raw_parts(result, ret_type_len.unwrap() as usize);
          let result_vec = result_slice.to_vec();
          if !result.is_null() {
            libc::free(result as *mut c_void);
          }
          Either9::E(result_vec)
        }
        DataType::StringArray => {
          let mut result: *mut *mut c_char =
            malloc(std::mem::size_of::<*mut *mut c_char>()) as *mut *mut c_char;

          ffi_call(
            &mut cif,
            Some(*func),
            &mut result as *mut _ as *mut c_void,
            arg_values_cvoid.as_mut_ptr(),
          );
          let output_vec = vec![0; ret_type_len.unwrap() as usize]
            .iter()
            .enumerate()
            .map(|(index, _)| {
              CString::from_raw(*result.offset(index as isize))
                .into_string()
                .unwrap()
            })
            .collect();
          if !result.is_null() {
            libc::free(result as *mut c_void);
          }
          Either9::F(output_vec)
        }
        DataType::DoubleArray => {
          let mut result: *mut c_double =
            malloc(std::mem::size_of::<*mut c_double>()) as *mut c_double;
          ffi_call(
            &mut cif,
            Some(*func),
            &mut result as *mut _ as *mut c_void,
            arg_values_cvoid.as_mut_ptr(),
          );

          let result_slice = std::slice::from_raw_parts(result, ret_type_len.unwrap() as usize);
          let result_vec = result_slice.to_vec();
          if !result.is_null() {
            libc::free(result as *mut c_void);
          }
          Either9::G(result_vec)
        }
        DataType::Boolean => {
          let mut result: u8 = 255;
          ffi_call(
            &mut cif,
            Some(*func),
            &mut result as *mut u8 as *mut c_void,
            arg_values_cvoid.as_mut_ptr(),
          );
          if result != 0 && result != 1 {
            panic!("The returned type is not a boolean")
          }
          Either9::H(if result == 0 { false } else { true })
        }
        _ => Either9::C(()),
      }
    } else if ret_object.is_some() {
      let ret_object = ret_object.unwrap();
      let (ret_fields_size, field_vec) =
        JsObject::keys(&ret_object)
          .unwrap()
          .into_iter()
          .fold((0, vec![]), |pre, current| {
            let val: JsNumber = ret_object.get_named_property(&current).unwrap();
            let data_type = number_to_data_type(val.try_into().unwrap());
            let field_size = match data_type {
              DataType::I32 => std::mem::size_of::<c_int>(),
              DataType::String => std::mem::size_of::<*const c_char>(),
              _ => {
                panic!("")
              }
            };
            let (size, mut field_vec) = pre;
            field_vec.push(current);
            (size + field_size, field_vec)
          });
      let mut result: *mut c_void = malloc(ret_fields_size);
      ffi_call(
        &mut cif,
        Some(*func),
        &mut result as *mut _ as *mut c_void,
        arg_values_cvoid.as_mut_ptr(),
      );
      let mut js_object = env.create_object().unwrap();
      let mut offset = 0;
      field_vec.into_iter().for_each(|field| {
        let val: JsNumber = ret_object.get_named_property(&field).unwrap();
        let data_type = number_to_data_type(val.try_into().unwrap());
        match data_type {
          DataType::I32 => {
            let field_ptr = result.offset(offset as isize) as *mut i32;
            js_object
              .set_property(
                env.create_string(&field).unwrap(),
                env.create_int32(*field_ptr).unwrap(),
              )
              .unwrap();
            offset += std::mem::size_of::<c_int>();
          }
          DataType::String => {
            let field_ptr = result.offset(offset as isize) as *mut *mut c_char;
            let js_string = CString::from_raw(*field_ptr).into_string().unwrap();
            js_object
              .set_property(
                env.create_string(&field).unwrap(),
                env.create_string(&js_string).unwrap(),
              )
              .unwrap();
            offset += std::mem::size_of::<*const c_char>();
          }
          _ => {}
        }
      });
      Either9::I(js_object)
    } else {
      Either9::C(())
    }
  }
}
