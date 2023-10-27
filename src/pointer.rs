use crate::define::*;
use crate::utils::*;
use napi::bindgen_prelude::*;
use napi::{JsBoolean, JsNumber, JsObject, JsString, JsUnknown};
use std::ffi::c_void;
use std::ffi::{c_char, c_double, c_int, CString};

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

pub unsafe fn create_object_from_pointer(
  env: &Env,
  ptr: *mut c_void,
  ret_object: JsObject,
) -> JsObject {
  let mut js_object = env.create_object().unwrap();
  let mut field_ptr = ptr;
  let mut offset = 0;
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
          let align = std::mem::align_of::<c_int>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut c_int;
          js_object
            .set_property(
              env.create_string(&field).unwrap(),
              env.create_int32(*type_field_ptr).unwrap(),
            )
            .unwrap();
          offset = std::mem::size_of::<c_int>();
        }
        DataType::Double => {
          let align = std::mem::align_of::<c_double>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut c_double;
          js_object
            .set_property(
              env.create_string(&field).unwrap(),
              env.create_double(*type_field_ptr).unwrap(),
            )
            .unwrap();
          offset = std::mem::size_of::<c_double>();
        }
        DataType::Boolean => {
          let align = std::mem::align_of::<bool>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut bool;
          js_object
            .set_property(
              env.create_string(&field).unwrap(),
              env.get_boolean(*type_field_ptr).unwrap(),
            )
            .unwrap();
          offset = std::mem::size_of::<bool>();
        }
        DataType::String => {
          let align = std::mem::align_of::<*const c_char>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut *mut c_char;
          let js_string = CString::from_raw(*type_field_ptr).into_string().unwrap();
          js_object
            .set_property(
              env.create_string(&field).unwrap(),
              env.create_string(&js_string).unwrap(),
            )
            .unwrap();
          offset = std::mem::size_of::<*const c_char>();
        }
        DataType::StringArray => {
          let align = std::mem::align_of::<*const *const c_char>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut *mut *mut c_char;
          let arr = create_array_from_pointer(*type_field_ptr, array_len);
          let js_array = rs_array_to_js_array(env, ArrayType::String(arr));
          js_object
            .set_property(env.create_string(&field).unwrap(), js_array)
            .unwrap();
          offset = std::mem::size_of::<*const *const c_char>();
        }
        DataType::DoubleArray => {
          let align = std::mem::align_of::<*const c_double>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut *mut c_double;
          let arr = create_array_from_pointer(*type_field_ptr, array_len);
          let js_array = rs_array_to_js_array(env, ArrayType::Double(arr));
          js_object
            .set_property(env.create_string(&field).unwrap(), js_array)
            .unwrap();
          offset = std::mem::size_of::<*const c_double>();
        }
        DataType::I32Array => {
          let align = std::mem::align_of::<*const c_int>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut *mut c_int;
          let arr = create_array_from_pointer(*type_field_ptr, array_len);
          let js_array = rs_array_to_js_array(env, ArrayType::I32(arr));
          js_object
            .set_property(env.create_string(&field).unwrap(), js_array)
            .unwrap();
          offset = std::mem::size_of::<*const c_int>();
        }

        _ => panic!(
          "{:?} is not available as a field type at this time",
          data_type
        ),
      }
      field_ptr = field_ptr.offset(offset as isize) as *mut c_void;
    });
  js_object
}
