use libc::c_void;
use napi::{bindgen_prelude::*, Env, JsExternal, JsUnknown};
use std::ffi::{c_char, CString};

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

pub unsafe fn create_js_external_from_jsunknown(env: &Env, js_val: JsUnknown) {
  //
}
// pub unsafe  () -> {
//     let (mut arg_types, arg_values): (Vec<*mut ffi_type>, Vec<RsArgsValue>) = params_type
//       .into_iter()
//       .zip(params_value.into_iter())
//       .map(|(param, value)| {
//         let value_type = param.get_type().unwrap();
//         match value_type {
//           ValueType::Number => {
//             let param_data_type =
//               number_to_data_type(param.coerce_to_number().unwrap().try_into().unwrap());
//             match param_data_type {
//               DataType::I32 => {
//                 let arg_type = &mut ffi_type_sint32 as *mut ffi_type;
//                 let arg_val: i32 = value.coerce_to_number().unwrap().try_into().unwrap();
//                 (arg_type, RsArgsValue::I32(arg_val))
//               }
//               DataType::U8 => {
//                 let arg_type = &mut ffi_type_sint32 as *mut ffi_type;
//                 let arg_val: u32 = value.coerce_to_number().unwrap().try_into().unwrap();
//                 (arg_type, RsArgsValue::U8(arg_val as u8))
//               }
//               DataType::I64 => {
//                 let arg_type = &mut ffi_type_sint32 as *mut ffi_type;
//                 let arg_val: i64 = value.coerce_to_number().unwrap().try_into().unwrap();
//                 (arg_type, RsArgsValue::I64(arg_val))
//               }
//               DataType::Double => {
//                 let arg_type = &mut ffi_type_double as *mut ffi_type;
//                 let arg_val: f64 = value.coerce_to_number().unwrap().try_into().unwrap();
//                 (arg_type, RsArgsValue::Double(arg_val))
//               }
//               DataType::String => {
//                 let arg_type = &mut ffi_type_pointer as *mut ffi_type;
//                 let arg_val: String = value
//                   .coerce_to_string()
//                   .unwrap()
//                   .into_utf8()
//                   .unwrap()
//                   .try_into()
//                   .unwrap();
//                 (arg_type, RsArgsValue::String(arg_val))
//               }
//               DataType::U8Array => {
//                 let arg_type = &mut ffi_type_pointer as *mut ffi_type;
//                 let js_object = value.coerce_to_object().unwrap();
//                 let arg_val = vec![0; js_object.get_array_length().unwrap() as usize]
//                   .iter()
//                   .enumerate()
//                   .map(|(index, _)| {
//                     let js_element: JsNumber = js_object.get_element(index as u32).unwrap();
//                     return js_element.get_uint32().unwrap() as u8;
//                   })
//                   .collect::<Vec<u8>>();

//                 (arg_type, RsArgsValue::U8Array(arg_val))
//               }
//               DataType::I32Array => {
//                 let arg_type = &mut ffi_type_pointer as *mut ffi_type;
//                 let js_object = value.coerce_to_object().unwrap();
//                 let arg_val = vec![0; js_object.get_array_length().unwrap() as usize]
//                   .iter()
//                   .enumerate()
//                   .map(|(index, _)| {
//                     let js_element: JsNumber = js_object.get_element(index as u32).unwrap();
//                     return js_element.get_int32().unwrap();
//                   })
//                   .collect::<Vec<i32>>();

//                 (arg_type, RsArgsValue::I32Array(arg_val))
//               }
//               DataType::DoubleArray => {
//                 let arg_type = &mut ffi_type_pointer as *mut ffi_type;
//                 let js_object = value.coerce_to_object().unwrap();
//                 let arg_val = vec![0; js_object.get_array_length().unwrap() as usize]
//                   .iter()
//                   .enumerate()
//                   .map(|(index, _)| {
//                     let js_element: JsNumber = js_object.get_element(index as u32).unwrap();
//                     return js_element.get_double().unwrap();
//                   })
//                   .collect::<Vec<f64>>();

//                 (arg_type, RsArgsValue::DoubleArray(arg_val))
//               }
//               DataType::StringArray => {
//                 let arg_type = &mut ffi_type_pointer as *mut ffi_type;
//                 let js_object = value.coerce_to_object().unwrap();
//                 let arg_val = js_array_to_string_array(js_object);
//                 (arg_type, RsArgsValue::StringArray(arg_val))
//               }
//               DataType::Boolean => {
//                 let arg_type = &mut ffi_type_uint8 as *mut ffi_type;
//                 let arg_val: bool = value.coerce_to_bool().unwrap().get_value().unwrap();
//                 (arg_type, RsArgsValue::Boolean(arg_val))
//               }
//               DataType::Void => {
//                 let arg_type = &mut ffi_type_void as *mut ffi_type;
//                 (arg_type, RsArgsValue::Void(()))
//               }
//               DataType::External => {
//                 let arg_type = &mut ffi_type_pointer as *mut ffi_type;
//                 let js_external: JsExternal = value.try_into().unwrap();
//                 (arg_type, RsArgsValue::External(js_external))
//               }
//             }
//           }
//           ValueType::Object => {
//             let params_type_object: JsObject = param.coerce_to_object().unwrap();
//             let arg_type = &mut ffi_type_pointer as *mut ffi_type;
//             let params_value_object = value.coerce_to_object().unwrap();
//             let index_map =
//               get_params_value_rs_struct(&env, &params_type_object, &params_value_object);
//             (arg_type, RsArgsValue::Object(index_map))
//           }
//           ValueType::Function => {
//             let params_type_function: JsFunction = param.try_into().unwrap();
//             let params_val_function: JsFunction = value.try_into().unwrap();
//             let arg_type = &mut ffi_type_pointer as *mut ffi_type;
//             (
//               arg_type,
//               RsArgsValue::Function(params_type_function, params_val_function),
//             )
//           }
//           _ => panic!("unsupported params type {:?}", value_type),
//         }
//       })
//       .unzip();
// }
