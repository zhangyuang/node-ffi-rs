use super::get_array_desc;
use super::js_value::create_js_value_unchecked;
use super::object_utils::calculate_struct_size;
use crate::datatype::array::ToRsArray;
use crate::datatype::buffer::get_safe_buffer;
use crate::datatype::create_struct::generate_c_struct;
use crate::datatype::function::get_rs_value_from_pointer;
use crate::datatype::pointer::*;
use crate::datatype::restore_struct::{create_rs_struct_from_pointer, rs_value_to_js_unknown};
use crate::datatype::string::{js_string_to_string, string_to_c_string, string_to_c_w_string};
use crate::define::*;
use indexmap::IndexMap;
use libc::{c_char, c_double, c_float, c_int, c_uchar, c_void};

use napi::threadsafe_function::{
  ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode,
};
use napi::{
  bindgen_prelude::*, Env, JsBigInt, JsBoolean, JsBuffer, JsExternal, JsNumber, JsObject, JsString,
  JsUnknown, NapiRaw,
};
use std::alloc::{alloc, Layout};
use std::collections::HashMap;
use std::ffi::CStr;
use std::rc::Rc;
use widestring::{WideCString, WideChar};

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

pub fn get_ffi_tag(obj: &IndexMap<String, RsArgsValue>) -> FFITypeTag {
  if obj.get(FFI_TAG_FIELD).is_none() {
    return FFITypeTag::Unknown;
  }
  if let Some(RsArgsValue::I32(ffitypetag)) = obj.get(FFI_TAG_FIELD) {
    if ffitypetag == &FFITypeTag::Array.into() {
      return FFITypeTag::Array;
    }
    if ffitypetag == &FFITypeTag::StackStruct.into() {
      return FFITypeTag::StackStruct;
    }
    if ffitypetag == &FFITypeTag::StackArray.into() {
      return FFITypeTag::StackArray;
    }
    if ffitypetag == &FFITypeTag::Function.into() {
      return FFITypeTag::Function;
    }
    FFITypeTag::Unknown
  } else {
    FFITypeTag::Unknown
  }
}

pub fn get_func_desc(obj: &IndexMap<String, RsArgsValue>) -> FFIFUNCDESC {
  let need_free = if let RsArgsValue::Boolean(val) = obj.get(FUNCTION_FREE_TAG).unwrap() {
    *val
  } else {
    false
  };
  FFIFUNCDESC { need_free }
}

pub unsafe fn get_arg_values(
  params_type: Rc<Vec<RsArgsValue>>,
  params_value: Vec<JsUnknown>,
) -> Result<Vec<RsArgsValue>> {
  if params_type.len() != params_value.len() {
    return Err(
      FFIError::Panic(format!(
        "params_type length is not equal with params_value length"
      ))
      .into(),
    );
  }
  params_type
    .iter()
    .zip(params_value.into_iter())
    .map(|(param, value)| {
      let res = match param {
        RsArgsValue::I32(number) => {
          let param_data_type = (*number).try_into()?;
          match param_data_type {
            BasicDataType::U8 => {
              let arg_val: u32 = create_js_value_unchecked::<JsNumber>(value)?.try_into()?;
              RsArgsValue::U8(arg_val as u8)
            }
            BasicDataType::I16 => {
              let arg_val: i32 = create_js_value_unchecked::<JsNumber>(value)?.try_into()?;
              RsArgsValue::I16(arg_val as i16)
            }
            BasicDataType::I32 => {
              let arg_val: i32 = create_js_value_unchecked::<JsNumber>(value)?.try_into()?;
              RsArgsValue::I32(arg_val)
            }
            BasicDataType::U32 => {
              let arg_val: u32 = create_js_value_unchecked::<JsNumber>(value)?.try_into()?;
              RsArgsValue::U32(arg_val)
            }
            BasicDataType::I64 => {
              let arg_val: i64 = create_js_value_unchecked::<JsNumber>(value)?.try_into()?;
              RsArgsValue::I64(arg_val)
            }
            BasicDataType::BigInt => {
              let arg_val: i64 = create_js_value_unchecked::<JsBigInt>(value)?.try_into()?;
              RsArgsValue::I64(arg_val)
            }
            BasicDataType::U64 => {
              let arg_val: i64 = create_js_value_unchecked::<JsNumber>(value)?.try_into()?;
              RsArgsValue::U64(arg_val as u64)
            }
            BasicDataType::Float => {
              let arg_val: f64 = create_js_value_unchecked::<JsNumber>(value)?.try_into()?;
              RsArgsValue::Float(arg_val as f32)
            }
            BasicDataType::Double => {
              let arg_val: f64 = create_js_value_unchecked::<JsNumber>(value)?.try_into()?;
              RsArgsValue::Double(arg_val)
            }
            BasicDataType::String => {
              let arg_val: String =
                js_string_to_string(create_js_value_unchecked::<JsString>(value)?)?;
              RsArgsValue::String(arg_val)
            }
            BasicDataType::WString => {
              let arg_val: String =
                js_string_to_string(create_js_value_unchecked::<JsString>(value)?)?;
              RsArgsValue::WString(arg_val)
            }
            BasicDataType::Boolean => {
              let arg_val: bool = create_js_value_unchecked::<JsBoolean>(value)?.get_value()?;
              RsArgsValue::Boolean(arg_val)
            }
            BasicDataType::Void => RsArgsValue::Void(()),
            BasicDataType::External => {
              let js_external: JsExternal = value.try_into()?;
              RsArgsValue::External(js_external)
            }
          }
        }
        RsArgsValue::Object(params_type_object_rs) => {
          if let FFITypeTag::Array | FFITypeTag::StackArray = get_ffi_tag(&params_type_object_rs) {
            let array_desc = get_array_desc(&params_type_object_rs);
            let FFIARRARYDESC {
              array_type,
              struct_item_type,
              ..
            } = array_desc;
            match array_type {
              RefDataType::U8Array => {
                let js_buffer: JsBuffer = value.try_into()?;
                RsArgsValue::U8Array(Some(js_buffer.into_value()?), None)
              }
              RefDataType::I16Array => {
                let js_object = create_js_value_unchecked::<JsObject>(value)?;
                let arg_val = vec![0; js_object.get_array_length()? as usize]
                  .iter()
                  .enumerate()
                  .map(|(index, _)| {
                    let js_element: JsNumber = js_object.get_element(index as u32).unwrap();
                    js_element.get_int32().unwrap() as i16
                  })
                  .collect::<Vec<i16>>();
                RsArgsValue::I16Array(arg_val)
              }
              RefDataType::I32Array => {
                let js_object = create_js_value_unchecked::<JsObject>(value)?;
                let arg_val = vec![0; js_object.get_array_length()? as usize]
                  .iter()
                  .enumerate()
                  .map(|(index, _)| {
                    let js_element: JsNumber = js_object.get_element(index as u32).unwrap();
                    js_element.get_int32().unwrap()
                  })
                  .collect::<Vec<i32>>();
                RsArgsValue::I32Array(arg_val)
              }
              RefDataType::FloatArray => {
                let js_object = create_js_value_unchecked::<JsObject>(value)?;
                let arg_val = vec![0; js_object.get_array_length()? as usize]
                  .iter()
                  .enumerate()
                  .map(|(index, _)| {
                    let js_element: JsNumber = js_object.get_element(index as u32).unwrap();
                    js_element.get_double().unwrap() as f32
                  })
                  .collect::<Vec<f32>>();
                RsArgsValue::FloatArray(arg_val)
              }
              RefDataType::DoubleArray => {
                let js_object = create_js_value_unchecked::<JsObject>(value)?;
                let arg_val = vec![0; js_object.get_array_length()? as usize]
                  .iter()
                  .enumerate()
                  .map(|(index, _)| {
                    let js_element: JsNumber = js_object.get_element(index as u32).unwrap();
                    js_element.get_double().unwrap()
                  })
                  .collect::<Vec<f64>>();
                RsArgsValue::DoubleArray(arg_val)
              }
              RefDataType::StringArray => {
                let js_object = create_js_value_unchecked::<JsObject>(value)?;
                let arg_val = js_object.to_rs_array()?;
                RsArgsValue::StringArray(arg_val)
              }
              RefDataType::StructArray => {
                let js_object = create_js_value_unchecked::<JsObject>(value)?;
                let arg_values = vec![0; js_object.get_array_length()? as usize]
                  .iter()
                  .enumerate()
                  .map(|(index, _)| {
                    let js_element: JsObject = js_object.get_element(index as u32).unwrap();
                    let struct_item_type = struct_item_type.as_ref().unwrap();
                    let index_map = get_params_value_rs_struct(struct_item_type, &js_element);
                    index_map.unwrap()
                  })
                  .collect();
                RsArgsValue::StructArray(arg_values)
              }
            }
          } else if let FFITypeTag::Function = get_ffi_tag(&params_type_object_rs) {
            let params_val_function: JsFunction = value.try_into()?;
            RsArgsValue::Function(params_type_object_rs.clone(), params_val_function)
          } else {
            // struct
            let params_value_object = create_js_value_unchecked::<JsObject>(value)?;
            let index_map =
              get_params_value_rs_struct(params_type_object_rs, &params_value_object)?;
            RsArgsValue::Object(index_map)
          }
        }
        _ => panic!("unsupported params type {:?}", param),
      };
      Ok(res)
    })
    .collect()
}

#[macro_export]
macro_rules! match_args_len {
    (
        $env:ident,
        $args_len:ident,
        $tsfn_ptr:expr,
        $func_args_type_rs_ptr:expr,
        $($num:literal => $closure:ident, $($arg:ident),*),*
    ) => {
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
  env: &Env,
  params_type: Rc<Vec<RsArgsValue>>,
  arg_values: Vec<RsArgsValue>,
) -> Result<Vec<*mut c_void>> {
  params_type
    .iter()
    .zip(arg_values.into_iter())
    .map(|(arg_type, val)| {
      match val {
        RsArgsValue::External(val) => {
          Ok(Box::into_raw(Box::new(get_js_external_wrap_data(&env, val)?)) as *mut c_void)
        }
        RsArgsValue::U8(val) => Ok(Box::into_raw(Box::new(val)) as *mut c_void),
        RsArgsValue::I16(val) => Ok(Box::into_raw(Box::new(val)) as *mut c_void),
        RsArgsValue::I32(val) => Ok(Box::into_raw(Box::new(val)) as *mut c_void),
        RsArgsValue::I64(val) | RsArgsValue::BigInt(val) => {
          Ok(Box::into_raw(Box::new(val)) as *mut c_void)
        }
        RsArgsValue::U64(val) => Ok(Box::into_raw(Box::new(val)) as *mut c_void),
        RsArgsValue::U32(val) => Ok(Box::into_raw(Box::new(val)) as *mut c_void),
        RsArgsValue::String(val) => {
          let c_string = string_to_c_string(val);
          let ptr = c_string.as_ptr();
          std::mem::forget(c_string);
          Ok(Box::into_raw(Box::new(ptr)) as *mut c_void)
        }
        RsArgsValue::WString(val) => {
          let c_w_string = string_to_c_w_string(val);
          let ptr = c_w_string.as_ptr();
          std::mem::forget(c_w_string);
          Ok(Box::into_raw(Box::new(ptr)) as *mut c_void)
        }
        RsArgsValue::Float(val) => {
          let c_float = Box::new(val);
          Ok(Box::into_raw(c_float) as *mut c_void)
        }
        RsArgsValue::Double(val) => {
          let c_double = Box::new(val);
          Ok(Box::into_raw(c_double) as *mut c_void)
        }
        RsArgsValue::U8Array(buffer, _) => {
          let buffer = buffer.unwrap();
          let ptr = buffer.as_ptr();
          std::mem::forget(buffer);
          Ok(Box::into_raw(Box::new(ptr)) as *mut c_void)
        }
        RsArgsValue::I16Array(val) => {
          let ptr = val.as_ptr();
          std::mem::forget(val);
          Ok(Box::into_raw(Box::new(ptr)) as *mut c_void)
        }
        RsArgsValue::I32Array(val) => {
          let ptr = val.as_ptr();
          std::mem::forget(val);
          Ok(Box::into_raw(Box::new(ptr)) as *mut c_void)
        }
        RsArgsValue::DoubleArray(val) => {
          let ptr = val.as_ptr();
          std::mem::forget(val);
          Ok(Box::into_raw(Box::new(ptr)) as *mut c_void)
        }
        RsArgsValue::FloatArray(val) => {
          let ptr = val.as_ptr();
          std::mem::forget(val);
          Ok(Box::into_raw(Box::new(ptr)) as *mut c_void)
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
          std::mem::forget(c_char_vec);
          Ok(Box::into_raw(Box::new(ptr)) as *mut c_void)
        }
        RsArgsValue::StructArray(val) => {
          if let RsArgsValue::Object(arg_type) = arg_type {
            let array_desc = get_array_desc(arg_type);
            let FFIARRARYDESC {
              struct_item_type,
              array_len,
              ..
            } = array_desc;
            let struct_item_type = struct_item_type
              .as_ref()
              .ok_or_else(|| FFIError::Panic("Missing struct item type".to_string()))?;

            let is_stack_struct = get_ffi_tag(struct_item_type) == FFITypeTag::StackStruct;

            if is_stack_struct {
              let (struct_size, align) = calculate_struct_size(struct_item_type);
              let mut head_ptr = None;
              let mut current_ptr =
                alloc(Layout::from_size_align(struct_size * array_len, align).unwrap())
                  as *mut c_void;

              for item in val {
                let struct_ptr =
                  generate_c_struct(&env, struct_item_type, item, Some(current_ptr))?;
                if head_ptr.is_none() {
                  head_ptr = Some(struct_ptr);
                }
                current_ptr = struct_ptr.offset(struct_size as isize);
              }
              Ok(Box::into_raw(Box::new(head_ptr.unwrap())) as *mut c_void)
            } else {
              let struct_ptrs: Vec<_> = val
                .into_iter()
                .map(|item| generate_c_struct(&env, struct_item_type, item, None))
                .collect::<Result<Vec<_>>>()?;
              let ptr = struct_ptrs.as_ptr();
              std::mem::forget(struct_ptrs);

              Ok(Box::into_raw(Box::new(ptr as *mut c_void)) as *mut c_void)
            }
          } else {
            Err(FFIError::Panic(format!("uncorrect params type {:?}", arg_type)).into())
          }
        }
        RsArgsValue::Boolean(val) => {
          let c_bool = Box::new(val);
          Ok(Box::into_raw(c_bool) as *mut c_void)
        }
        RsArgsValue::Void(_) => {
          Ok(Box::into_raw(Box::new(std::ptr::null_mut() as *mut c_void)) as *mut c_void)
        }
        RsArgsValue::Object(val) => {
          if let RsArgsValue::Object(arg_type_rs) = arg_type {
            let is_stack_struct = get_ffi_tag(arg_type_rs) == FFITypeTag::StackStruct;
            Ok(if is_stack_struct {
              generate_c_struct(&env, &arg_type_rs, val, None)?
            } else {
              Box::into_raw(Box::new(generate_c_struct(&env, &arg_type_rs, val, None)?))
                as *mut c_void
            })
          } else {
            Err(FFIError::Panic(format!("uncorrect params type {:?}", arg_type)).into())
          }
        }
        RsArgsValue::Function(func_desc, js_function) => {
          use libffi::low;
          use libffi::middle::*;
          let func_args_type = func_desc.get(PARAMS_TYPE).unwrap().clone();

          let func_ret_type = if func_desc.get(RET_TYPE).is_some() {
            func_desc.get(RET_TYPE).unwrap().clone()
          } else {
            RsArgsValue::I32(DataType::Void as i32)
          };
          let free_c_params_memory = func_desc.get(FREE_FUNCTION_TAG).unwrap().clone();
          let tsfn: ThreadsafeFunction<Vec<RsArgsValue>, ErrorStrategy::Fatal> = (&js_function)
            .create_threadsafe_function(
              0,
              move |ctx: ThreadSafeCallContext<Vec<RsArgsValue>>| {
                let js_call_params: Vec<JsUnknown> = ctx
                  .value
                  .into_iter()
                  .map(|rs_args| rs_value_to_js_unknown(&ctx.env, rs_args))
                  .collect::<Result<Vec<JsUnknown>, _>>()?;
                Ok(js_call_params)
              },
            )?;

          unsafe extern "C" fn lambda_callback<F: Fn((Vec<*mut c_void>, *mut c_void))>(
            _cif: &low::ffi_cif,
            result: &mut c_void,
            args: *const *const c_void,
            userdata: &F,
          ) {
            let params: Vec<*mut c_void> = (0.._cif.nargs)
              .map(|index| *args.offset(index as isize) as *mut c_void)
              .collect();

            userdata((params, result));
          }

          let tsfn_call_context = TsFnCallContext {
            tsfn,
            lambda: None,
            closure: None,
          };

          let tsfn_call_context_ptr = Box::into_raw(Box::new(tsfn_call_context));

          let (cif, lambda) = if let RsArgsValue::Object(func_args_type_rs) = func_args_type {
            let cif = Cif::new(
              func_args_type_rs
                .values()
                .into_iter()
                .map(|val| val.to_ffi_type()),
              func_ret_type.to_ffi_type(),
            );
            let main_thread_id = std::thread::current().id();
            let env_clone = env.clone();

            let lambda = move |args: (Vec<*mut c_void>, *mut c_void)| {
              let (params, result) = args;
              let value: Vec<RsArgsValue> = params
                .into_iter()
                .enumerate()
                .map(|(index, c_param)| {
                  let arg_type = func_args_type_rs.get(&index.to_string()).unwrap();
                  let param = get_rs_value_from_pointer(env, arg_type, c_param, true);
                  if free_c_params_memory == RsArgsValue::Boolean(true) {
                    free_c_pointer_memory(c_param, arg_type);
                  }

                  param
                })
                .collect();
              let func_ret_type_rc = Rc::new(vec![func_ret_type.clone()]);
              if std::thread::current().id() != main_thread_id
                && func_ret_type != RsArgsValue::I32(7)
              {
                let (se, re) = std::sync::mpsc::channel();
                (*tsfn_call_context_ptr).tsfn.call_with_return_value(
                  value,
                  ThreadsafeFunctionCallMode::Blocking,
                  move |js_return_value: JsUnknown| {
                    let js_return_value_rs =
                      get_arg_values(Rc::clone(&func_ret_type_rc), vec![js_return_value]).unwrap();
                    let js_return_value_rs_ptr = get_value_pointer(
                      &env_clone,
                      Rc::clone(&func_ret_type_rc),
                      js_return_value_rs,
                    )
                    .unwrap()[0];
                    write_rs_ptr_to_c(
                      &Rc::clone(&func_ret_type_rc)[0],
                      js_return_value_rs_ptr,
                      result,
                    );
                    se.send(()).unwrap();
                    Ok(())
                  },
                );
                re.recv().unwrap();
              } else {
                if func_ret_type != RsArgsValue::I32(DataType::Void as i32) {
                  println!(
                    "\x1b[33m{}\x1b[0m",
                    "warning: set runInNewThread to true to get the return value in c environment"
                  );
                }
                (*tsfn_call_context_ptr)
                  .tsfn
                  .call(value, ThreadsafeFunctionCallMode::Blocking);
              }
            };
            (cif, lambda)
          } else {
            return Err(FFIError::Panic(format!("generate cif error")).into());
          };
          (*tsfn_call_context_ptr).lambda = Some(Box::new(lambda));
          let closure = Closure::new(
            cif,
            lambda_callback,
            (*tsfn_call_context_ptr).lambda.as_ref().unwrap(),
          );
          (*tsfn_call_context_ptr).closure = Some(closure);
          if CLOSURE_MAP.is_none() {
            CLOSURE_MAP = Some(HashMap::new());
          }
          let code_ptr = std::mem::transmute(
            (*tsfn_call_context_ptr)
              .closure
              .as_ref()
              .unwrap()
              .code_ptr(),
          );
          CLOSURE_MAP
            .as_mut()
            .unwrap()
            .insert(code_ptr, tsfn_call_context_ptr as *mut c_void);
          Ok(code_ptr)

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
      }
    })
    .collect::<Result<Vec<*mut c_void>>>()
}

pub unsafe fn get_params_value_rs_struct(
  params_type_object: &IndexMap<String, RsArgsValue>,
  params_value_object: &JsObject,
) -> Result<IndexMap<String, RsArgsValue>> {
  let mut index_map = IndexMap::new();
  let parse_result: Result<()> =
    params_type_object
      .into_iter()
      .try_for_each(|(field, field_type)| {
        if field == FFI_TAG_FIELD {
          return Ok(());
        }
        let field = field.clone();
        match field_type.clone() {
          RsArgsValue::I32(data_type_number) => {
            let data_type: DataType = data_type_number.try_into()?;
            let val = match data_type {
              DataType::String => {
                let val: JsString = params_value_object.get_named_property(&field)?;
                let val: String = js_string_to_string(val)?;
                RsArgsValue::String(val)
              }
              DataType::WString => {
                let val: JsString = params_value_object.get_named_property(&field)?;
                let val: String = js_string_to_string(val)?;
                RsArgsValue::WString(val)
              }
              DataType::U8 => {
                let val: JsNumber = params_value_object.get_named_property(&field)?;
                let val: u32 = val.try_into()?;
                RsArgsValue::U8(val as u8)
              }
              DataType::I16 => {
                let val: JsNumber = params_value_object.get_named_property(&field)?;
                let val: i32 = val.try_into()?;
                RsArgsValue::I16(val as i16)
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
              DataType::BigInt => {
                let val: JsNumber = params_value_object.get_named_property(&field)?;
                let val: i64 = val.try_into()?;
                RsArgsValue::BigInt(val)
              }
              DataType::U64 => {
                let val: JsNumber = params_value_object.get_named_property(&field)?;
                let val: i64 = val.try_into()?;
                RsArgsValue::U64(val as u64)
              }
              DataType::U32 => {
                let val: JsNumber = params_value_object.get_named_property(&field)?;
                let val: i64 = val.try_into()?;
                RsArgsValue::U32(val as u32)
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
              DataType::External => {
                let val: JsExternal = params_value_object.get_named_property(&field)?;
                RsArgsValue::External(val)
              }
              DataType::Void => RsArgsValue::Void(()),
              _ => panic!("unsupport data type: {:?}", data_type),
            };
            index_map.insert(field, val);
          }

          RsArgsValue::Object(mut params_type_rs_value) => {
            let params_value: JsObject = params_value_object.get_named_property(&field)?;
            if let FFITypeTag::Array | FFITypeTag::StackArray = get_ffi_tag(&params_type_rs_value) {
              let array_desc = get_array_desc(&params_type_rs_value);
              let FFIARRARYDESC {
                array_type,
                struct_item_type,
                ..
              } = array_desc;
              let array_value = match array_type {
                RefDataType::U8Array => {
                  let js_buffer: JsBuffer = params_value_object.get_named_property(&field)?;
                  RsArgsValue::U8Array(Some(js_buffer.into_value()?), None)
                }
                RefDataType::I16Array => {
                  let js_array: JsObject = params_value_object.get_named_property(&field)?;
                  let arg_val = js_array.to_rs_array()?;
                  RsArgsValue::I16Array(arg_val)
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
                RefDataType::StructArray => {
                  let js_array: JsObject = params_value_object.get_named_property(&field)?;
                  let arg_val = vec![0; js_array.get_array_length()? as usize]
                    .iter()
                    .enumerate()
                    .map(|(index, _)| {
                      let js_element: JsObject = js_array.get_element(index as u32).unwrap();
                      let struct_item_type = struct_item_type.as_ref().unwrap();
                      let index_map = get_params_value_rs_struct(struct_item_type, &js_element);
                      index_map.unwrap()
                    })
                    .collect();
                  RsArgsValue::StructArray(arg_val)
                }
              };
              params_type_rs_value.insert(ARRAY_VALUE_TAG.to_string(), array_value);
              index_map.insert(field, RsArgsValue::Object(params_type_rs_value));
            } else {
              let map = get_params_value_rs_struct(&params_type_rs_value, &params_value);
              index_map.insert(field, RsArgsValue::Object(map?));
            }
          }
          _ => {
            return Err(
              FFIError::UnsupportedValueType(format!(
                "Get field {:?} received {:?} but params type only supported number or object ",
                field, field_type
              ))
              .into(),
            );
          }
        }
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
  let parse_result: Result<()> = JsObject::keys(params_type)?
    .into_iter()
    .try_for_each(|field| {
      let field_type: JsUnknown = params_type.get_named_property(&field)?;
      match field_type.get_type()? {
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
          let args_type = create_js_value_unchecked::<JsObject>(field_type)?;
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
      }
      Ok(())
    });
  match parse_result {
    Ok(_) => Ok(index_map),
    Err(e) => Err(e),
  }
}

// describe paramsType or retType, field can only be number or object
pub unsafe fn type_define_to_rs_args(env: &Env, type_define: JsUnknown) -> Result<RsArgsValue> {
  let params_type_value_type = type_define.get_type()?;
  let ret_value = match params_type_value_type {
    ValueType::Number => {
      RsArgsValue::I32(create_js_value_unchecked::<JsNumber>(type_define)?.try_into()?)
    }
    ValueType::Object => RsArgsValue::Object(type_object_to_rs_struct(
      env,
      &create_js_value_unchecked::<JsObject>(type_define)?,
    )?),
    _ => {
      return Err(
        FFIError::UnsupportedValueType(format!(
          "ret_value_type can only be number or object but receive {}",
          params_type_value_type
        ))
        .into(),
      );
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
      let ret_data_type = (*number).try_into()?;
      match ret_data_type {
        BasicDataType::String => {
          let ptr_str = CStr::from_ptr(*(ptr as *mut *const c_char))
            .to_string_lossy()
            .to_string();
          rs_value_to_js_unknown(&env, RsArgsValue::String(ptr_str))
        }
        BasicDataType::WString => {
          let ptr_str = WideCString::from_ptr_str(*(ptr as *mut *const WideChar)).to_string_lossy();
          rs_value_to_js_unknown(&env, RsArgsValue::WString(ptr_str))
        }
        BasicDataType::U8 => rs_value_to_js_unknown(env, RsArgsValue::U8(*(ptr as *mut u8))),
        BasicDataType::I16 => rs_value_to_js_unknown(env, RsArgsValue::I16(*(ptr as *mut i16))),
        BasicDataType::I32 => rs_value_to_js_unknown(env, RsArgsValue::I32(*(ptr as *mut i32))),
        BasicDataType::I64 => rs_value_to_js_unknown(env, RsArgsValue::I64(*(ptr as *mut i64))),
        BasicDataType::U64 => rs_value_to_js_unknown(env, RsArgsValue::U64(*(ptr as *mut u64))),
        BasicDataType::U32 => rs_value_to_js_unknown(env, RsArgsValue::U32(*(ptr as *mut u32))),
        BasicDataType::BigInt => {
          rs_value_to_js_unknown(env, RsArgsValue::BigInt(*(ptr as *mut i64)))
        }
        BasicDataType::Void => rs_value_to_js_unknown(env, RsArgsValue::Void(())),
        BasicDataType::Float => rs_value_to_js_unknown(env, RsArgsValue::Float(*(ptr as *mut f32))),
        BasicDataType::Double => {
          rs_value_to_js_unknown(env, RsArgsValue::Double(*(ptr as *mut f64)))
        }
        BasicDataType::Boolean => {
          rs_value_to_js_unknown(env, RsArgsValue::Boolean(*(ptr as *mut bool)))
        }
        BasicDataType::External => {
          let js_external = env.create_external(
            *(ptr as *mut *mut c_void),
            Some(std::mem::size_of::<*mut c_void>() as i64),
          )?;
          rs_value_to_js_unknown(env, RsArgsValue::External(js_external))
        }
      }
    }
    RsArgsValue::Object(sub_obj_type) => {
      if let FFITypeTag::Array | FFITypeTag::StackArray = get_ffi_tag(&sub_obj_type) {
        let array_desc = get_array_desc(&sub_obj_type);
        // array
        let FFIARRARYDESC {
          array_type,
          array_len,
          struct_item_type,
          ..
        } = array_desc;
        match array_type {
          RefDataType::U8Array => {
            let arr = create_array_from_pointer(*(ptr as *mut *mut c_uchar), array_len);
            rs_value_to_js_unknown(env, get_safe_buffer(env, arr, false))
          }
          RefDataType::I16Array => {
            let arr = create_array_from_pointer(*(ptr as *mut *mut i16), array_len);
            rs_value_to_js_unknown(env, RsArgsValue::I16Array(arr))
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
          RefDataType::StructArray => {
            let mut safe_ptr = std::ptr::read(ptr as *const *mut c_void);
            let is_stack_struct =
              get_ffi_tag(struct_item_type.as_ref().unwrap()) == FFITypeTag::StackStruct;
            let v = (0..array_len)
              .map(|_| {
                let rs_struct = create_rs_struct_from_pointer(
                  env,
                  safe_ptr,
                  struct_item_type.as_ref().unwrap(),
                  false,
                );
                let (struct_size, _) = calculate_struct_size(&struct_item_type.as_ref().unwrap());
                if is_stack_struct {
                  safe_ptr = safe_ptr.offset(struct_size as isize);
                } else {
                  safe_ptr = safe_ptr.offset(1);
                }
                rs_struct
              })
              .collect();
            rs_value_to_js_unknown(env, RsArgsValue::StructArray(v))
          }
        }
      } else {
        // raw object
        let is_stack_struct = get_ffi_tag(&sub_obj_type) == FFITypeTag::StackStruct;
        let rs_struct = create_rs_struct_from_pointer(
          env,
          if is_stack_struct {
            ptr
          } else {
            *(ptr as *mut *mut c_void)
          },
          &sub_obj_type,
          false,
        );
        rs_value_to_js_unknown(env, RsArgsValue::Object(rs_struct))
      }
    }
    _ => Err(FFIError::Panic(format!("ret_type err {:?}", ret_type_rs)).into()),
  }
}

unsafe fn write_rs_ptr_to_c(ret_type: &RsArgsValue, src: *mut c_void, dst: *mut c_void) {
  match &ret_type {
    RsArgsValue::I32(number) => {
      let ret_data_type = (*number).try_into().unwrap();
      match ret_data_type {
        BasicDataType::U8 => std::ptr::copy(src, dst, std::mem::size_of::<u8>()),
        BasicDataType::I16 => std::ptr::copy(src, dst, std::mem::size_of::<i16>()),
        BasicDataType::I32 => std::ptr::copy(src, dst, std::mem::size_of::<i32>()),
        BasicDataType::I64 | BasicDataType::BigInt => {
          std::ptr::copy(src, dst, std::mem::size_of::<i64>());
        }
        BasicDataType::U64 => std::ptr::copy(src, dst, std::mem::size_of::<u64>()),
        BasicDataType::U32 => std::ptr::copy(src, dst, std::mem::size_of::<u32>()),
        BasicDataType::Float => std::ptr::copy(src, dst, std::mem::size_of::<f32>()),
        BasicDataType::Double => std::ptr::copy(src, dst, std::mem::size_of::<f64>()),
        BasicDataType::Boolean => std::ptr::copy(src, dst, std::mem::size_of::<bool>()),
        BasicDataType::String => std::ptr::copy(src, dst, std::mem::size_of::<*const c_char>()),
        BasicDataType::WString => std::ptr::copy(src, dst, std::mem::size_of::<*const WideChar>()),
        BasicDataType::External => std::ptr::copy(src, dst, std::mem::size_of::<*mut c_void>()),
        BasicDataType::Void => {}
      }
      match ret_data_type {
        BasicDataType::U8
        | BasicDataType::I32
        | BasicDataType::I64
        | BasicDataType::BigInt
        | BasicDataType::U64
        | BasicDataType::U32
        | BasicDataType::Float
        | BasicDataType::Double
        | BasicDataType::Boolean => {
          let _ = Box::from_raw(src);
        }
        _ => {}
      }
    }
    RsArgsValue::Object(_) => std::ptr::copy(src, dst, std::mem::size_of::<*const *const c_void>()),
    _ => {}
  }
}
