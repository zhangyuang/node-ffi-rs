use super::js_value::create_js_value_unchecked;
use crate::datatype::array::ToRsArray;
use crate::datatype::buffer::get_safe_buffer;
use crate::datatype::function::get_rs_value_from_pointer;
use crate::datatype::object_calculate::generate_c_struct;
use crate::datatype::object_generate::{create_rs_struct_from_pointer, rs_value_to_js_unknown};
use crate::datatype::pointer::*;
use crate::datatype::string::{js_string_to_string, string_to_c_string};
use crate::define::*;
use indexmap::IndexMap;
use libc::free;
use libc::{c_char, c_double, c_float, c_int, c_uchar, c_void};
use libffi_sys::{
  ffi_type, ffi_type_double, ffi_type_float, ffi_type_pointer, ffi_type_sint32, ffi_type_sint64,
  ffi_type_uint64, ffi_type_uint8, ffi_type_void,
};
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi::{
  bindgen_prelude::*, Env, JsBoolean, JsBuffer, JsExternal, JsNumber, JsObject, JsString,
  JsUnknown, NapiRaw,
};
use std::ffi::{CStr, CString};

pub unsafe fn get_js_external_wrap_data(env: &Env, js_external: JsExternal) -> Result<*mut c_void> {
  use std::any::TypeId;
  #[repr(C)]
  pub struct TaggedObject<T> {
    type_id: TypeId,
    pub(crate) object: Option<T>,
  }
  let mut unknown_tagged_object = std::ptr::null_mut();
  sys::napi_get_value_external(env.raw(), js_external.raw(), &mut unknown_tagged_object);
  let tagged_object = unknown_tagged_object as *mut TaggedObject<*mut c_void>;
  let p = (*tagged_object).object.as_mut().unwrap();
  Ok(*p)
}

pub fn get_ffi_tag(obj: &IndexMap<String, RsArgsValue>) -> FFITag {
  if obj.get(FFI_TAG_FIELD).is_none() {
    return FFITag::Unknown;
  }
  if let RsArgsValue::String(ffitypetag) = obj.get(FFI_TAG_FIELD).unwrap() {
    if ffitypetag == &ARRAY_FFI_TAG {
      return FFITag::Array;
    }
    if ffitypetag == &FUNCTION_FFI_TAG {
      return FFITag::Function;
    }
    FFITag::Unknown
  } else {
    FFITag::Unknown
  }
}
pub fn get_array_desc(obj: &IndexMap<String, RsArgsValue>) -> FFIARRARYDESC {
  let (mut array_len, mut array_type, mut array_dynamic) = (0, 0, true);
  if let RsArgsValue::I32(number) = obj.get(ARRAY_LENGTH_TAG).unwrap() {
    array_len = *number as usize
  }
  if let RsArgsValue::I32(number) = obj.get(ARRAY_TYPE_TAG).unwrap() {
    array_type = *number
  }
  if let RsArgsValue::Boolean(val) = obj.get(ARRAY_DYNAMIC_TAG).unwrap() {
    array_dynamic = *val
  }
  let array_type = number_to_ref_data_type(array_type);

  FFIARRARYDESC {
    array_len,
    dynamic_array: array_dynamic,
    array_type,
    array_value: obj.get(ARRAY_VALUE_TAG),
  }
}

pub fn js_number_to_i32(js_number: JsNumber) -> i32 {
  js_number.try_into().unwrap()
}

pub unsafe fn get_arg_types_values(
  env: &Env,
  params_type: Vec<JsUnknown>,
  params_value: Vec<JsUnknown>,
) -> Result<(Vec<*mut ffi_type>, Vec<RsArgsValue>)> {
  if params_type.len() != params_value.len() {
    return Err(
      FFIError::Panic(format!(
        "params_type length is not equal with params_value length"
      ))
      .into(),
    );
  }
  params_type
    .into_iter()
    .zip(params_value.into_iter())
    .map(|(param, value)| {
      let value_type = param.get_type()?;
      let res = match value_type {
        ValueType::Number => {
          let param_data_type =
            number_to_data_type(create_js_value_unchecked::<JsNumber>(env, param).try_into()?);
          match param_data_type {
            DataType::I32 => {
              let arg_type = Box::into_raw(Box::new(ffi_type_sint32)) as *mut ffi_type;
              let arg_val: i32 = create_js_value_unchecked::<JsNumber>(env, value).try_into()?;
              (arg_type, RsArgsValue::I32(arg_val))
            }
            DataType::U8 => {
              let arg_type = Box::into_raw(Box::new(ffi_type_sint32)) as *mut ffi_type;
              let arg_val: u32 = create_js_value_unchecked::<JsNumber>(env, value).try_into()?;
              (arg_type, RsArgsValue::U8(arg_val as u8))
            }
            DataType::I64 => {
              let arg_type = Box::into_raw(Box::new(ffi_type_sint64)) as *mut ffi_type;
              let arg_val: i64 = create_js_value_unchecked::<JsNumber>(env, value).try_into()?;
              (arg_type, RsArgsValue::I64(arg_val))
            }
            DataType::U64 => {
              let arg_type = Box::into_raw(Box::new(ffi_type_uint64)) as *mut ffi_type;
              let arg_val: i64 = create_js_value_unchecked::<JsNumber>(env, value).try_into()?;
              (arg_type, RsArgsValue::U64(arg_val as u64))
            }
            DataType::Float => {
              let arg_type = Box::into_raw(Box::new(ffi_type_float)) as *mut ffi_type;
              let arg_val: f64 = create_js_value_unchecked::<JsNumber>(env, value).try_into()?;
              (arg_type, RsArgsValue::Float(arg_val as f32))
            }
            DataType::Double => {
              let arg_type = Box::into_raw(Box::new(ffi_type_double)) as *mut ffi_type;
              let arg_val: f64 = create_js_value_unchecked::<JsNumber>(env, value).try_into()?;
              (arg_type, RsArgsValue::Double(arg_val))
            }
            DataType::String => {
              let arg_type = Box::into_raw(Box::new(ffi_type_pointer)) as *mut ffi_type;
              let arg_val: String =
                js_string_to_string(create_js_value_unchecked::<JsString>(env, value))?;

              (arg_type, RsArgsValue::String(arg_val))
            }
            DataType::U8Array => {
              let arg_type = Box::into_raw(Box::new(ffi_type_pointer)) as *mut ffi_type;
              let js_buffer: JsBuffer = value.try_into()?;
              (
                arg_type,
                RsArgsValue::U8Array(Some(js_buffer.into_value()?), None),
              )
            }
            DataType::I32Array => {
              let arg_type = &mut ffi_type_pointer as *mut ffi_type;
              let js_object = create_js_value_unchecked::<JsObject>(env, value);
              let arg_val = vec![0; js_object.get_array_length()? as usize]
                .iter()
                .enumerate()
                .map(|(index, _)| {
                  let js_element: JsNumber = js_object.get_element(index as u32).unwrap();
                  js_element.get_int32().unwrap()
                })
                .collect::<Vec<i32>>();
              (arg_type, RsArgsValue::I32Array(arg_val))
            }
            DataType::DoubleArray => {
              let arg_type = &mut ffi_type_pointer as *mut ffi_type;
              let js_object = create_js_value_unchecked::<JsObject>(env, value);
              let arg_val = vec![0; js_object.get_array_length()? as usize]
                .iter()
                .enumerate()
                .map(|(index, _)| {
                  let js_element: JsNumber = js_object.get_element(index as u32).unwrap();
                  js_element.get_double().unwrap()
                })
                .collect::<Vec<f64>>();

              (arg_type, RsArgsValue::DoubleArray(arg_val))
            }
            DataType::FloatArray => {
              let arg_type = &mut ffi_type_pointer as *mut ffi_type;
              let js_object = create_js_value_unchecked::<JsObject>(env, value);
              let arg_val = vec![0; js_object.get_array_length()? as usize]
                .iter()
                .enumerate()
                .map(|(index, _)| {
                  let js_element: JsNumber = js_object.get_element(index as u32).unwrap();
                  js_element.get_double().unwrap() as f32
                })
                .collect::<Vec<f32>>();
              (arg_type, RsArgsValue::FloatArray(arg_val))
            }
            DataType::StringArray => {
              let arg_type = &mut ffi_type_pointer as *mut ffi_type;
              let js_object = create_js_value_unchecked::<JsObject>(env, value);
              let arg_val = js_object.to_rs_array()?;
              (arg_type, RsArgsValue::StringArray(arg_val))
            }
            DataType::Boolean => {
              let arg_type = &mut ffi_type_uint8 as *mut ffi_type;
              let arg_val: bool = create_js_value_unchecked::<JsBoolean>(env, value).get_value()?;
              (arg_type, RsArgsValue::Boolean(arg_val))
            }
            DataType::Void => {
              let arg_type = &mut ffi_type_void as *mut ffi_type;
              (arg_type, RsArgsValue::Void(()))
            }
            DataType::External => {
              let arg_type = &mut ffi_type_pointer as *mut ffi_type;
              let js_external: JsExternal = value.try_into()?;
              (arg_type, RsArgsValue::External(js_external))
            }
          }
        }
        ValueType::Object => {
          let params_type_object: JsObject = create_js_value_unchecked::<JsObject>(env, param);
          let params_type_object_rs = type_object_to_rs_struct(env, &params_type_object)?;
          let arg_type = &mut ffi_type_pointer as *mut ffi_type;
          if let FFITag::Function = get_ffi_tag(&params_type_object_rs) {
            let params_val_function: JsFunction = value.try_into()?;
            let arg_type = &mut ffi_type_pointer as *mut ffi_type;
            (
              arg_type,
              RsArgsValue::Function(params_type_object, params_val_function),
            )
          } else {
            let params_value_object = create_js_value_unchecked::<JsObject>(env, value);
            let index_map =
              get_params_value_rs_struct(&env, &params_type_object, &params_value_object);
            (arg_type, RsArgsValue::Object(index_map.unwrap()))
          }
        }
        _ => panic!("unsupported params type {:?}", value_type),
      };
      Ok(res)
    })
    .collect::<napi::Result<Vec<(*mut ffi_type, RsArgsValue)>>>()
    .map(|pairs| {
      let (arg_types, arg_values) = pairs.into_iter().unzip();
      (arg_types, arg_values)
    })
}

#[macro_export]
macro_rules! match_args_len {
 ($env:ident, $args_len:ident, $tsfn_ptr:expr, $func_args_type_rs_ptr:expr,  $($num:literal => $closure:ident, $($arg:ident),*),*) => {
        match $args_len {
            $(
                $num => {
                   let lambda = move |$($arg: *mut c_void),*| {
                            let func_args_type_rs = &*$func_args_type_rs_ptr;
                            let arg_arr = [$($arg),*];
                            let value: Vec<RsArgsValue> = (0..$num)
                                .map(|index| {
                                    let c_param = arg_arr[index as usize];
                                     let arg_type = func_args_type_rs.get(&index.to_string()).unwrap();
                                    let param = get_js_function_call_value(&$env,arg_type, c_param, true);
                                    param
                                })
                              .collect();
                            (&*$tsfn_ptr).call(value, ThreadsafeFunctionCallMode::NonBlocking);
                    };
                    let closure = Box::into_raw(Box::new($closure::new(&*Box::into_raw(Box::new(lambda)))));
                    std::mem::transmute((*closure).code_ptr())
                }
            )*
            _ => {
                std::ptr::null_mut() as *mut c_void
            },
        }
    };
}
pub unsafe fn get_value_pointer(
  env: Env,
  arg_values: Vec<RsArgsValue>,
) -> Result<(Vec<*mut c_void>, Vec<Box<dyn Fn()>>)> {
  let mut free_funcs = Vec::new();
  let res = arg_values
    .into_iter()
    .map(|val| match val {
      RsArgsValue::External(val) => {
        Ok(Box::into_raw(Box::new(get_js_external_wrap_data(&env, val)?)) as *mut c_void)
      }
      RsArgsValue::U8(val) => {
        let ptr = Box::into_raw(Box::new(val)) as *mut c_void;
        let free_func = Box::new(move || free(ptr)) as Box<dyn Fn()>;
        free_funcs.push(free_func);
        Ok(ptr)
      }
      RsArgsValue::I32(val) => {
        let ptr = Box::into_raw(Box::new(val)) as *mut c_void;
        let free_func = Box::new(move || free(ptr)) as Box<dyn Fn()>;
        free_funcs.push(free_func);
        Ok(ptr)
      }
      RsArgsValue::I64(val) => {
        let ptr = Box::into_raw(Box::new(val)) as *mut c_void;
        let free_func = Box::new(move || free(ptr)) as Box<dyn Fn()>;
        free_funcs.push(free_func);
        Ok(ptr)
      }
      RsArgsValue::U64(val) => {
        let ptr = Box::into_raw(Box::new(val)) as *mut c_void;
        let free_func = Box::new(move || Box::from_raw(ptr)) as Box<dyn Fn()>;
        free_funcs.push(free_func);
        Ok(ptr)
      }

      RsArgsValue::Float(val) => {
        let ptr = Box::into_raw(Box::new(val)) as *mut c_void;
        let free_func = Box::new(move || {
          Box::from_raw(ptr);
        }) as Box<dyn Fn()>;
        free_funcs.push(free_func);
        Ok(ptr)
      }
      RsArgsValue::Double(val) => {
        let ptr = Box::into_raw(Box::new(val)) as *mut c_void;
        let free_func = Box::new(move || {
          Box::from_raw(ptr);
        }) as Box<dyn Fn()>;
        free_funcs.push(free_func);
        Ok(ptr)
      }
      RsArgsValue::Boolean(val) => {
        let ptr = Box::into_raw(Box::new(val)) as *mut c_void;
        let free_func = Box::new(move || {
          Box::from_raw(ptr);
        }) as Box<dyn Fn()>;
        free_funcs.push(free_func);
        Ok(ptr)
      }
      RsArgsValue::Void(val) => {
        let ptr = Box::into_raw(Box::new(val)) as *mut c_void;
        let free_func = Box::new(move || {
          Box::from_raw(ptr);
        }) as Box<dyn Fn()>;
        free_funcs.push(free_func);
        Ok(ptr)
      }
      RsArgsValue::String(val) => {
        let c_string = string_to_c_string(val);
        let ptr = c_string.as_ptr();
        std::mem::forget(c_string);
        let free_func = Box::new(move || {
          CString::from_raw(ptr as *mut i8);
        }) as Box<dyn Fn()>;
        free_funcs.push(free_func);
        Ok(Box::into_raw(Box::new(ptr)) as *mut c_void)
      }
      RsArgsValue::U8Array(buffer, v) => {
        let buffer = buffer.unwrap();
        let ptr = buffer.as_ptr();
        std::mem::forget(buffer);
        Ok(Box::into_raw(Box::new(ptr)) as *mut c_void)
      }
      RsArgsValue::I32Array(val) => {
        let val_ptr = val.as_ptr();
        let val_len = val.len();
        let val_capacity = val.capacity();
        std::mem::forget(val);
        let ptr = Box::into_raw(Box::new(val_ptr)) as *mut c_void;
        let free_func = Box::new(move || {
          let _val = Vec::from_raw_parts(val_ptr as *mut i32, val_len, val_capacity);
        }) as Box<dyn Fn()>;
        free_funcs.push(free_func);
        Ok(ptr)
      }
      RsArgsValue::DoubleArray(val) => {
        let val_ptr = val.as_ptr();
        let val_len = val.len();
        let val_capacity = val.capacity();
        std::mem::forget(val);
        let ptr = Box::into_raw(Box::new(val_ptr)) as *mut c_void;
        let free_func = Box::new(move || {
          let _val = Vec::from_raw_parts(val_ptr as *mut f64, val_len, val_capacity);
        }) as Box<dyn Fn()>;
        free_funcs.push(free_func);
        Ok(ptr)
      }
      RsArgsValue::FloatArray(val) => {
        let val_ptr = val.as_ptr();
        let val_len = val.len();
        let val_capacity = val.capacity();
        std::mem::forget(val);
        let ptr = Box::into_raw(Box::new(val_ptr)) as *mut c_void;
        let free_func = Box::new(move || {
          let _val = Vec::from_raw_parts(val_ptr as *mut f32, val_len, val_capacity);
        }) as Box<dyn Fn()>;
        free_funcs.push(free_func);
        Ok(ptr)
      }
      RsArgsValue::StringArray(val) => {
        let c_char_vec: Vec<*const c_char> = val
          .into_iter()
          .map(|str| {
            let c_string = string_to_c_string(str);
            let ptr = c_string.as_ptr();
            std::mem::forget(c_string);
            ptr
          })
          .collect();
        let ptr = c_char_vec.as_ptr();
        let c_char_vec_clone = c_char_vec.clone();
        let free_func = Box::new(move || {
          c_char_vec_clone.iter().for_each(|ptr| {
            CString::from_raw(*ptr as *mut i8);
          })
        }) as Box<dyn Fn()>;
        free_funcs.push(free_func);
        std::mem::forget(c_char_vec);
        Ok(Box::into_raw(Box::new(ptr)) as *mut c_void)
      }
      RsArgsValue::Object(val) => {
        Ok(Box::into_raw(Box::new(generate_c_struct(&env, val)?)) as *mut c_void)
      }
      RsArgsValue::Function(func_desc, js_function) => {
        use libffi::low;
        use libffi::middle::*;
        let func_args_type: JsObject = func_desc.get_property(env.create_string("paramsType")?)?;
        let func_args_type_rs = type_object_to_rs_vector(&env, &func_args_type)?;
        let tsfn: ThreadsafeFunction<Vec<RsArgsValue>, ErrorStrategy::Fatal> = (&js_function)
          .create_threadsafe_function(0, |ctx| {
            let value: Vec<RsArgsValue> = ctx.value;
            let js_call_params: Vec<JsUnknown> = value
              .into_iter()
              .map(|rs_args| rs_value_to_js_unknown(&ctx.env, rs_args))
              .collect::<Result<Vec<JsUnknown>, _>>()?;

            Ok(js_call_params)
          })?;

        unsafe extern "C" fn lambda_callback<F: Fn(Vec<*mut c_void>)>(
          _cif: &low::ffi_cif,
          result: &mut *mut c_void,
          args: *const *const c_void,
          userdata: &F,
        ) {
          let params: Vec<*mut c_void> = (0.._cif.nargs)
            .map(|index| *args.offset(index as isize) as *mut c_void)
            .collect();
          userdata(params);
        }
        let cif = Cif::new(
          func_args_type_rs
            .iter()
            .map(|arg_type| rs_value_to_ffi_type(arg_type)),
          Type::void(),
        );
        let lambda = move |args: Vec<*mut c_void>| {
          let value: Vec<RsArgsValue> = args
            .into_iter()
            .enumerate()
            .map(|(index, c_param)| {
              let arg_type = &(func_args_type_rs)[index];
              let param = get_rs_value_from_pointer(&env, arg_type, c_param, true);
              param
            })
            .collect();
          tsfn.call(value, ThreadsafeFunctionCallMode::Blocking);
        };
        let lambda_ptr = Box::into_raw(Box::new(lambda));
        let closure = Box::into_raw(Box::new(Closure::new(cif, lambda_callback, &*lambda_ptr)));
        let free_func = Box::new(move || {
          let _ = Box::from_raw(closure);
        }) as Box<dyn Fn()>;
        free_funcs.push(free_func);
        Ok(std::mem::transmute((*closure).code_ptr()))

        // has been deprecated
        // Ok(
        //   match_args_len!(env, args_len, tsfn_ptr, func_args_type_rs_ptr,
        //       1 => Closure1, a
        //       ,2 => Closure2, a,b
        //       ,3 => Closure3, a,b,c
        //       ,4 => Closure4, a,b,c,d
        //       ,5 => Closure5, a,b,c,d,e
        //       ,6 => Closure6, a,b,c,d,e,f
        //       ,7 => Closure7, a,b,c,d,e,f,g
        //       ,8 => Closure8, a,b,c,d,e,f,g,h
        //       ,9 => Closure9, a,b,c,d,e,f,g,h,i
        //       ,10 => Closure10, a,b,c,d,e,f,g,h,i,j
        //   ),
        // )
      }
    })
    .collect::<Result<Vec<*mut c_void>>>();
  Ok((res?, free_funcs))
}

pub unsafe fn get_params_value_rs_struct(
  env: &Env,
  params_type_object: &JsObject,
  params_value_object: &JsObject,
) -> Result<IndexMap<String, RsArgsValue>> {
  let mut index_map = IndexMap::new();
  let parse_result: Result<()> = JsObject::keys(&params_value_object)?
    .into_iter()
    .try_for_each(|field| {
      let field_type: JsUnknown = params_type_object.get_named_property(&field)?;
      match field_type.get_type()? {
        ValueType::Number => {
          let data_type: DataType =
            number_to_data_type(create_js_value_unchecked::<JsNumber>(env, field_type).try_into()?);
          let val = match data_type {
            DataType::String => {
              let val: JsString = params_value_object.get_named_property(&field)?;
              let val: String = js_string_to_string(val)?;
              RsArgsValue::String(val)
            }
            DataType::U8 => {
              let val: JsNumber = params_value_object.get_named_property(&field)?;
              let val: u32 = val.try_into()?;
              RsArgsValue::U8(val as u8)
            }
            DataType::I32 => {
              let val: JsNumber = params_value_object.get_named_property(&field)?;
              let val: i32 = val.try_into()?;
              RsArgsValue::I32(val)
            }
            DataType::I64 => {
              let val: JsNumber = params_value_object.get_named_property(&field)?;
              let val: i64 = val.try_into()?;
              RsArgsValue::I64(val)
            }
            DataType::U64 => {
              let val: JsNumber = params_value_object.get_named_property(&field)?;
              let val: i64 = val.try_into()?;
              RsArgsValue::U64(val as u64)
            }
            DataType::Boolean => {
              let val: JsBoolean = params_value_object.get_named_property(&field)?;
              let val: bool = val.get_value()?;
              RsArgsValue::Boolean(val)
            }
            DataType::Float => {
              let val: JsNumber = params_value_object.get_named_property(&field)?;
              let val: f64 = val.try_into()?;
              RsArgsValue::Float(val as f32)
            }
            DataType::Double => {
              let val: JsNumber = params_value_object.get_named_property(&field)?;
              let val: f64 = val.try_into()?;
              RsArgsValue::Double(val)
            }
            DataType::StringArray => {
              let js_array: JsObject = params_value_object.get_named_property(&field)?;
              let arg_val = js_array.to_rs_array()?;
              RsArgsValue::StringArray(arg_val)
            }
            DataType::DoubleArray => {
              let js_array: JsObject = params_value_object.get_named_property(&field)?;
              let arg_val: Vec<f64> = js_array.to_rs_array()?;
              RsArgsValue::DoubleArray(arg_val)
            }
            DataType::FloatArray => {
              let js_array: JsObject = params_value_object.get_named_property(&field)?;
              let arg_val: Vec<f32> = js_array
                .to_rs_array()?
                .into_iter()
                .map(|item: f64| item as f32)
                .collect();
              RsArgsValue::FloatArray(arg_val)
            }
            DataType::I32Array => {
              let js_array: JsObject = params_value_object.get_named_property(&field)?;
              let arg_val = js_array.to_rs_array()?;
              RsArgsValue::I32Array(arg_val)
            }
            DataType::U8Array => {
              let js_buffer: JsBuffer = params_value_object.get_named_property(&field)?;
              RsArgsValue::U8Array(Some(js_buffer.into_value()?), None)
            }
            DataType::External => {
              let val: JsExternal = params_value_object.get_named_property(&field)?;
              RsArgsValue::External(val)
            }
            DataType::Void => RsArgsValue::Void(()),
          };
          index_map.insert(field, val);
        }

        ValueType::Object => {
          let params_type = create_js_value_unchecked::<JsObject>(env, field_type);
          let params_value: JsObject = params_value_object.get_named_property(&field)?;
          let mut params_type_rs_value = type_object_to_rs_struct(env, &params_type)?;
          if let FFITag::Array = get_ffi_tag(&params_type_rs_value) {
            let array_desc = get_array_desc(&params_type_rs_value);
            let FFIARRARYDESC { array_type, .. } = array_desc;
            let array_value = match array_type {
              RefDataType::U8Array => {
                let js_buffer: JsBuffer = params_value_object.get_named_property(&field)?;
                RsArgsValue::U8Array(Some(js_buffer.into_value()?), None)
              }
              RefDataType::I32Array => {
                let js_array: JsObject = params_value_object.get_named_property(&field)?;
                let arg_val = js_array.to_rs_array()?;
                RsArgsValue::I32Array(arg_val)
              }
              RefDataType::DoubleArray => {
                let js_array: JsObject = params_value_object.get_named_property(&field)?;
                let arg_val = js_array.to_rs_array()?;
                RsArgsValue::DoubleArray(arg_val)
              }
              RefDataType::FloatArray => {
                let js_array: JsObject = params_value_object.get_named_property(&field)?;
                let arg_val: Vec<f32> = js_array
                  .to_rs_array()?
                  .into_iter()
                  .map(|item: f64| item as f32)
                  .collect();
                RsArgsValue::FloatArray(arg_val)
              }
              RefDataType::StringArray => {
                let js_array: JsObject = params_value_object.get_named_property(&field)?;
                let arg_val = js_array.to_rs_array()?;
                RsArgsValue::StringArray(arg_val)
              }
            };
            params_type_rs_value.insert(ARRAY_VALUE_TAG.to_string(), array_value);
            index_map.insert(field, RsArgsValue::Object(params_type_rs_value));
          } else {
            let map = get_params_value_rs_struct(env, &params_type, &params_value);
            index_map.insert(field, RsArgsValue::Object(map?));
          }
        }
        _ => {
          return Err(
            FFIError::UnsupportedValueType(format!(
              "Get field {:?} received {:?} but params type only supported number or object ",
              field,
              field_type.get_type().unwrap()
            ))
            .into(),
          )
        }
      };
      Ok(())
    });
  match parse_result {
    Ok(_) => Ok(index_map),
    Err(e) => Err(e),
  }
}

pub unsafe fn type_object_to_rs_struct(
  env: &Env,
  params_type: &JsObject,
) -> Result<IndexMap<String, RsArgsValue>> {
  let mut index_map = IndexMap::new();
  let parse_result: Result<()> = JsObject::keys(params_type)
    .unwrap()
    .into_iter()
    .try_for_each(|field| {
      let field_type: JsUnknown = params_type.get_named_property(&field).unwrap();
      match field_type.get_type().unwrap() {
        ValueType::Number => {
          let number: JsNumber = field_type.try_into()?;
          let val: i32 = number.try_into()?;
          index_map.insert(field, RsArgsValue::I32(val));
        }
        ValueType::Boolean => {
          let val: JsBoolean = field_type.try_into()?;
          index_map.insert(field, RsArgsValue::Boolean(val.try_into()?));
        }
        ValueType::Object => {
          // maybe jsobject or jsarray
          let args_type = create_js_value_unchecked::<JsObject>(env, field_type);
          let map = type_object_to_rs_struct(env, &args_type)?;
          index_map.insert(field, RsArgsValue::Object(map));
        }
        ValueType::String => {
          let str: JsString = field_type.try_into()?;
          let str = js_string_to_string(str)?;
          index_map.insert(field, RsArgsValue::String(str));
        }
        _ => {
          return Err(
            FFIError::UnsupportedValueType(format!(
              "Receive {:?} but params type can only be number or object ",
              field_type.get_type()?
            ))
            .into(),
          );
        }
      };
      Ok(())
    });
  match parse_result {
    Ok(_) => Ok(index_map),
    Err(e) => Err(e),
  }
}

pub unsafe fn type_object_to_rs_vector(
  env: &Env,
  params_type: &JsObject,
) -> Result<Vec<RsArgsValue>> {
  JsObject::keys(params_type)
    .unwrap()
    .into_iter()
    .map(|field| {
      let field_type: JsUnknown = params_type.get_named_property(&field)?;
      Ok(match field_type.get_type().unwrap() {
        ValueType::Number => {
          let number: JsNumber = field_type.try_into()?;
          let val: i32 = number.try_into()?;
          RsArgsValue::I32(val)
        }
        ValueType::Object => {
          // maybe jsobject or jsarray
          let args_type = create_js_value_unchecked::<JsObject>(&env, field_type);
          let map = type_object_to_rs_struct(env, &args_type)?;
          RsArgsValue::Object(map)
        }
        ValueType::String => {
          let str: JsString = field_type.try_into()?;
          let str = js_string_to_string(str)?;
          RsArgsValue::String(str)
        }
        _ => {
          return Err(
            FFIError::UnsupportedValueType(format!(
              "Receive {:?} but params type can only be number or object ",
              field_type.get_type().unwrap()
            ))
            .into(),
          )
        }
      })
    })
    .collect()
}

// describe paramsType or retType, field can only be number or object
pub unsafe fn type_define_to_rs_args(env: &Env, type_define: JsUnknown) -> Result<RsArgsValue> {
  let ret_value_type = type_define.get_type()?;
  let ret_value = match ret_value_type {
    ValueType::Number => RsArgsValue::I32(js_number_to_i32(create_js_value_unchecked::<JsNumber>(
      env,
      type_define,
    ))),
    ValueType::Object => RsArgsValue::Object(type_object_to_rs_struct(
      env,
      &create_js_value_unchecked::<JsObject>(env, type_define),
    )?),
    _ => {
      return Err(
        FFIError::UnsupportedValueType(format!(
          "ret_value_type can only be number or object but receive {}",
          ret_value_type
        ))
        .into(),
      )
    }
  };
  Ok(ret_value)
}

pub unsafe fn get_js_unknown_from_pointer(
  env: &Env,
  ret_type_rs: &RsArgsValue,
  ptr: *mut c_void,
) -> Result<JsUnknown> {
  match ret_type_rs {
    RsArgsValue::I32(number) => {
      let ret_data_type = number_to_basic_data_type(*number);
      match ret_data_type {
        BasicDataType::String => {
          let ptr_str = CStr::from_ptr(*(ptr as *mut *const c_char))
            .to_string_lossy()
            .to_string();
          rs_value_to_js_unknown(&env, RsArgsValue::String(ptr_str))
        }
        BasicDataType::U8 => rs_value_to_js_unknown(env, RsArgsValue::U8(*(ptr as *mut u8))),
        BasicDataType::I32 => rs_value_to_js_unknown(env, RsArgsValue::I32(*(ptr as *mut i32))),
        BasicDataType::I64 => rs_value_to_js_unknown(env, RsArgsValue::I64(*(ptr as *mut i64))),
        BasicDataType::U64 => rs_value_to_js_unknown(env, RsArgsValue::U64(*(ptr as *mut u64))),
        BasicDataType::Void => rs_value_to_js_unknown(env, RsArgsValue::Void(())),
        BasicDataType::Float => rs_value_to_js_unknown(env, RsArgsValue::Float(*(ptr as *mut f32))),
        BasicDataType::Double => {
          rs_value_to_js_unknown(env, RsArgsValue::Double(*(ptr as *mut f64)))
        }
        BasicDataType::Boolean => {
          rs_value_to_js_unknown(env, RsArgsValue::Boolean(*(ptr as *mut bool)))
        }
        BasicDataType::External => {
          let js_external = env.create_external(*(ptr as *mut *mut c_void), None)?;
          rs_value_to_js_unknown(env, RsArgsValue::External(js_external))
        }
      }
    }
    RsArgsValue::Object(obj) => {
      if let FFITag::Array = get_ffi_tag(&obj) {
        let array_desc = get_array_desc(&obj);
        // array
        let FFIARRARYDESC {
          array_type,
          array_len,
          ..
        } = array_desc;
        match array_type {
          RefDataType::U8Array => {
            let arr = create_array_from_pointer(*(ptr as *mut *mut c_uchar), array_len);
            rs_value_to_js_unknown(env, get_safe_buffer(env, arr, false))
          }
          RefDataType::I32Array => {
            let arr = create_array_from_pointer(*(ptr as *mut *mut c_int), array_len);
            rs_value_to_js_unknown(env, RsArgsValue::I32Array(arr))
          }
          RefDataType::DoubleArray => {
            let arr = create_array_from_pointer(*(ptr as *mut *mut c_double), array_len);
            rs_value_to_js_unknown(env, RsArgsValue::DoubleArray(arr))
          }
          RefDataType::FloatArray => {
            let arr = create_array_from_pointer(*(ptr as *mut *mut c_float), array_len);
            rs_value_to_js_unknown(env, RsArgsValue::FloatArray(arr))
          }
          RefDataType::StringArray => {
            let arr = create_array_from_pointer(*(ptr as *mut *mut *mut c_char), array_len);
            rs_value_to_js_unknown(env, RsArgsValue::StringArray(arr))
          }
        }
      } else {
        // raw object
        let rs_struct = create_rs_struct_from_pointer(env, *(ptr as *mut *mut c_void), &obj, false);
        rs_value_to_js_unknown(env, RsArgsValue::Object(rs_struct))
      }
    }
    _ => Err(FFIError::Panic(format!("ret_type err {:?}", ret_type_rs)).into()),
  }
}
