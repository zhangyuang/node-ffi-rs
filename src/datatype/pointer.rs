use crate::datatype::array::*;
use crate::datatype::function::*;
use crate::datatype::object_calculate::*;
use crate::datatype::object_generate::*;
use crate::define::*;
use libc::c_void;
use libffi_sys::{
  ffi_type, ffi_type_double, ffi_type_pointer, ffi_type_sint32, ffi_type_uint8, ffi_type_void,
};
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi::{bindgen_prelude::*, Env, JsBuffer, JsExternal, JsNumber, JsObject, JsUnknown};
use std::ffi::{c_char, CString};

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
                    return std::mem::transmute((*closure).code_ptr());
                }
            )*
            _ => {
                std::ptr::null_mut() as *mut c_void
            },
        }
    };
}
pub trait ArrayPointer {
  type Output;
  unsafe fn get_and_advance(&mut self) -> Self::Output;
}

macro_rules! impl_array_pointer {
  ($type:ty, $output:ty) => {
    impl ArrayPointer for $type {
      type Output = $output;
      unsafe fn get_and_advance(&mut self) -> Self::Output {
        let value = **self;
        *self = self.offset(1);
        value
      }
    }
  };
}
impl_array_pointer!(*mut u8, u8);
impl_array_pointer!(*mut i32, i32);
impl_array_pointer!(*mut f64, f64);

impl ArrayPointer for *mut *mut c_char {
  type Output = String;
  unsafe fn get_and_advance(&mut self) -> Self::Output {
    let value = **self;
    *self = self.offset(1);
    CString::from_raw(value).into_string().unwrap()
  }
}
pub fn create_array_from_pointer<P>(mut pointer: P, len: usize) -> Vec<P::Output>
where
  P: ArrayPointer,
{
  unsafe { (0..len).map(|_| pointer.get_and_advance()).collect() }
}

pub unsafe fn get_js_external_wrap_Data(env: &Env, js_external: JsExternal) -> *mut c_void {
  let js_external_raw = JsExternal::to_napi_value(env.raw(), js_external).unwrap();
  let external: External<*mut c_void> =
    External::from_napi_value(env.raw(), js_external_raw).unwrap();
  *external
}

pub unsafe fn get_arg_types_values(
  env: &Env,
  params_type: Vec<JsUnknown>,
  params_value: Vec<JsUnknown>,
) -> (Vec<*mut ffi_type>, Vec<RsArgsValue>) {
  let (arg_types, arg_values): (Vec<*mut ffi_type>, Vec<RsArgsValue>) = params_type
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
            DataType::U8 => {
              let arg_type = &mut ffi_type_sint32 as *mut ffi_type;
              let arg_val: u32 = value.coerce_to_number().unwrap().try_into().unwrap();
              (arg_type, RsArgsValue::U8(arg_val as u8))
            }
            DataType::I64 => {
              let arg_type = &mut ffi_type_sint32 as *mut ffi_type;
              let arg_val: i64 = value.coerce_to_number().unwrap().try_into().unwrap();
              (arg_type, RsArgsValue::I64(arg_val))
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
            DataType::U8Array => {
              let arg_type = &mut ffi_type_pointer as *mut ffi_type;
              let js_buffer: JsBuffer = value.try_into().unwrap();
              (
                arg_type,
                RsArgsValue::U8Array(Some(js_buffer.into_value().unwrap()), None),
              )
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
            DataType::External => {
              let arg_type = &mut ffi_type_pointer as *mut ffi_type;
              let js_external: JsExternal = value.try_into().unwrap();
              (arg_type, RsArgsValue::External(js_external))
            }
          }
        }
        ValueType::Object => {
          let params_type_object: JsObject = param.coerce_to_object().unwrap();
          let arg_type = &mut ffi_type_pointer as *mut ffi_type;
          let params_value_object = value.coerce_to_object().unwrap();
          let index_map =
            get_params_value_rs_struct(&env, &params_type_object, &params_value_object);
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
  (arg_types, arg_values)
}
pub unsafe fn get_value_pointer(env: &Env, arg_values: Vec<RsArgsValue>) -> Vec<*mut c_void> {
  return arg_values
    .into_iter()
    .map(|val| match val {
      RsArgsValue::External(val) => {
        Box::into_raw(Box::new(get_js_external_wrap_Data(&env, val))) as *mut c_void
      }
      RsArgsValue::U8(val) => {
        let c_num = Box::new(val);
        Box::into_raw(c_num) as *mut c_void
      }
      RsArgsValue::I32(val) => {
        let c_num = Box::new(val);
        Box::into_raw(c_num) as *mut c_void
      }
      RsArgsValue::I64(val) => {
        let c_num = Box::new(val);
        Box::into_raw(c_num) as *mut c_void
      }
      RsArgsValue::String(val) => {
        let c_string = CString::new(val).unwrap();
        let ptr = c_string.as_ptr();
        let boxed_ptr = Box::new(ptr);
        let raw_ptr = Box::into_raw(boxed_ptr);
        std::mem::forget(c_string);
        println!("cstringptr{:?}", ptr);

        return raw_ptr as *mut c_void;
      }
      RsArgsValue::Double(val) => {
        let c_double = Box::new(val);
        Box::into_raw(c_double) as *mut c_void
      }
      RsArgsValue::U8Array(buffer, v) => {
        let buffer = buffer.unwrap();
        let ptr = buffer.as_ptr();
        let boxed_ptr = Box::new(ptr);
        let raw_ptr = Box::into_raw(boxed_ptr);
        std::mem::forget(buffer);
        return raw_ptr as *mut c_void;
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
        return match_args_len!(env, args_len, tsfn_ptr, func_args_type_rs_ptr,
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
        Box::into_raw(Box::new(generate_c_struct(&env, val))) as *mut c_void
      }
    })
    .collect();
}
