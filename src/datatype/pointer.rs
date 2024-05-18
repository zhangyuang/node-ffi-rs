use crate::utils::dataprocess::{get_array_desc, get_ffi_tag, get_func_desc};
use crate::utils::object_utils::get_size_align;
use indexmap::IndexMap;
use libc::{c_double, c_float, c_int, c_void, free};
use libffi::middle::Closure;
use std::ffi::{c_char, c_longlong, c_uchar, c_ulonglong, CStr, CString};

use crate::define::*;
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
impl_array_pointer!(*mut f32, f32);

impl ArrayPointer for *mut *mut c_char {
  type Output = String;
  unsafe fn get_and_advance(&mut self) -> Self::Output {
    let value = **self;
    *self = self.offset(1);
    CStr::from_ptr(value).to_string_lossy().to_string()
  }
}
pub fn create_array_from_pointer<P>(mut pointer: P, len: usize) -> Vec<P::Output>
where
  P: ArrayPointer,
{
  unsafe { (0..len).map(|_| pointer.get_and_advance()).collect() }
}

unsafe fn free_struct_memory(ptr: *mut c_void, struct_desc: IndexMap<String, RsArgsValue>) {
  println!("xx");
  let mut field_ptr = ptr;
  let mut offset = 0;
  let mut field_size = 0;
  for (_, val) in struct_desc {
    if let RsArgsValue::I32(number) = val {
      let data_type = number_to_basic_data_type(number);
      match data_type {
        BasicDataType::U8 => {
          let (size, align) = get_size_align::<c_uchar>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          offset += size + padding;
          field_size = size
        }
        BasicDataType::I32 => {
          let (size, align) = get_size_align::<c_int>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          offset += size + padding;
          field_size = size
        }
        BasicDataType::I64 => {
          let (size, align) = get_size_align::<c_longlong>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          offset += size + padding;
          field_size = size
        }
        BasicDataType::U64 => {
          let (size, align) = get_size_align::<c_ulonglong>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          offset += size + padding;
          field_size = size
        }
        BasicDataType::Float => {
          let (size, align) = get_size_align::<c_float>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          offset += size + padding;
          field_size = size
        }
        BasicDataType::Double => {
          let (size, align) = get_size_align::<c_double>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          offset += size + padding;
          field_size = size
        }
        BasicDataType::Boolean => {
          let (size, align) = get_size_align::<bool>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          offset += size + padding;
          field_size = size
        }
        BasicDataType::Void => {
          let (size, align) = (std::mem::size_of::<()>(), std::mem::align_of::<()>());
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          offset += size + padding;
          field_size = size
        }
        BasicDataType::String => {
          let (size, align) = get_size_align::<*const c_void>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut *mut c_char;
          let js_string = CStr::from_ptr(*type_field_ptr)
            .to_string_lossy()
            .to_string();
          offset += size + padding;
          field_size = size
        }
        BasicDataType::External => {
          let (size, align) = get_size_align::<*const c_void>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut *mut c_void;

          offset += size + padding;
          field_size = size
        }
      };
    }
    if let RsArgsValue::Object(obj) = val {
      if let FFITag::Array = get_ffi_tag(&obj) {
        let array_desc = get_array_desc(&obj);
        // array
        let FFIARRARYDESC {
          array_type,
          array_len,
          dynamic_array,
          ..
        } = &array_desc;
        match array_type {
          RefDataType::StringArray => {
            let (size, align) = get_size_align::<*const c_void>();
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            let type_field_ptr = field_ptr as *mut *mut *mut c_char;
            let arr = create_array_from_pointer(*type_field_ptr, *array_len);
            offset += size + padding;
            field_size = size
          }
          RefDataType::DoubleArray => {
            let (size, align) = if *dynamic_array {
              get_size_align::<*const c_void>()
            } else {
              let (size, align) = get_size_align::<c_double>();
              (size * array_len, align)
            };
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            if *dynamic_array {
              let type_field_ptr = field_ptr as *mut *mut c_double;
              // let arr = create_array_from_pointer(*type_field_ptr, *array_len);
            } else {
              // let arr =
              //   create_static_array_from_pointer(field_ptr as *mut c_void, &array_desc);
            }
            offset += size + padding;
            field_size = size
          }
          RefDataType::FloatArray => {
            let (size, align) = if *dynamic_array {
              get_size_align::<*const c_void>()
            } else {
              let (size, align) = get_size_align::<c_double>();
              (size * array_len, align)
            };
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            if *dynamic_array {
              let type_field_ptr = field_ptr as *mut *mut c_float;
              // let arr = create_array_from_pointer(*type_field_ptr, *array_len);
            } else {
              // let arr =
              //   create_static_array_from_pointer(field_ptr as *mut c_void, &array_desc);
            }
            offset += size + padding;
            field_size = size
          }
          RefDataType::I32Array => {
            let (size, align) = if *dynamic_array {
              get_size_align::<*const c_void>()
            } else {
              let (size, align) = get_size_align::<c_int>();
              (size * array_len, align)
            };
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            if *dynamic_array {
              let type_field_ptr = field_ptr as *mut *mut c_int;
              let arr = create_array_from_pointer(*type_field_ptr, *array_len);
            } else {
              // let arr =
              //   create_static_array_from_pointer(field_ptr as *mut c_void, &array_desc);
            }
            offset += size + padding;
            field_size = size
          }
          RefDataType::U8Array => {
            let (size, align) = if *dynamic_array {
              get_size_align::<*const c_void>()
            } else {
              let (size, align) = get_size_align::<u8>();
              (size * array_len, align)
            };
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            if *dynamic_array {
              let type_field_ptr = field_ptr as *mut *mut c_uchar;
              // let arr = create_array_from_pointer(*type_field_ptr, array_desc.array_len);
            } else {
              // let arr =
              //   create_static_array_from_pointer(field_ptr as *mut c_void, &array_desc);
            }
            offset += size + padding;
            field_size = size
          }
        };
      } else {
        // function | raw object
        let (size, align) = get_size_align::<*const c_void>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        let type_field_ptr = field_ptr as *mut *mut c_void;

        offset += size + padding;
        field_size = size
      };
    };
    field_ptr = field_ptr.offset(field_size as isize) as *mut c_void;
  }
}
pub unsafe fn free_rs_pointer_memory(ptr: *mut c_void, ptr_desc: RsArgsValue) {
  match ptr_desc {
    RsArgsValue::I32(number) => {
      let basic_data_type = number_to_basic_data_type(number);
      match basic_data_type {
        BasicDataType::String => {
          let _ = CString::from_raw(*(ptr as *mut *mut i8));
        }
        BasicDataType::U8 => {
          let _ = Box::from_raw(ptr);
        }
        BasicDataType::I32 => {
          let _ = Box::from_raw(ptr);
        }
        BasicDataType::I64 => {
          let _ = Box::from_raw(ptr);
        }
        BasicDataType::U64 => {
          let _ = Box::from_raw(ptr);
        }
        BasicDataType::Void => {
          let _ = Box::from_raw(ptr);
        }
        BasicDataType::Float => {
          let _ = Box::from_raw(ptr);
        }
        BasicDataType::Double => {
          let _ = Box::from_raw(ptr);
        }
        BasicDataType::Boolean => {
          let _ = Box::from_raw(ptr);
        }
        BasicDataType::External => {
          //
        }
      }
    }
    RsArgsValue::Object(obj) => {
      let ffi_tag = get_ffi_tag(&obj);
      if let FFITag::Array = ffi_tag {
        let array_desc = get_array_desc(&obj);
        // array
        let FFIARRARYDESC {
          array_type,
          array_len,
          ..
        } = array_desc;
        match array_type {
          RefDataType::U8Array => {
            let _ = Vec::from_raw_parts(*(ptr as *mut *mut c_char), array_len, array_len);
          }
          RefDataType::I32Array => {
            let _ = Vec::from_raw_parts(*(ptr as *mut *mut c_int), array_len, array_len);
          }
          RefDataType::DoubleArray => {
            let _ = Vec::from_raw_parts(*(ptr as *mut *mut c_double), array_len, array_len);
          }
          RefDataType::FloatArray => {
            let _ = Vec::from_raw_parts(*(ptr as *mut *mut c_float), array_len, array_len);
          }
          RefDataType::StringArray => {
            let v = Vec::from_raw_parts(*(ptr as *mut *mut *mut c_char), array_len, array_len);
            v.into_iter().for_each(|str_ptr| {
              let _ = CString::from_raw(str_ptr);
            });
          }
        }
      }
      if let FFITag::Function = ffi_tag {
        let func_desc = get_func_desc(&obj);
        if func_desc.need_free {
          let _ = Box::from_raw(*(ptr as *mut *mut Closure));
        }
      } else {
        free_struct_memory(ptr, obj)
      }
    }
    _ => panic!("free rust pointer memory error"),
  }
}

pub unsafe fn free_c_pointer_memory(ptr: *mut c_void, ptr_desc: RsArgsValue, skip_basic: bool) {
  match ptr_desc {
    RsArgsValue::I32(number) => {
      let basic_data_type = number_to_basic_data_type(number);
      if skip_basic {
        match basic_data_type {
          BasicDataType::String => {
            free(*(ptr as *mut *mut i8) as *mut c_void);
          }

          BasicDataType::External => {
            //
          }
          _ => {
            //
          }
        }
      } else {
        match basic_data_type {
          BasicDataType::String => {
            free(*(ptr as *mut *mut i8) as *mut c_void);
          }
          BasicDataType::U8 => free(ptr),
          BasicDataType::I32 => {
            free(ptr);
          }
          BasicDataType::I64 => free(ptr),
          BasicDataType::U64 => free(ptr),
          BasicDataType::Void => free(ptr),
          BasicDataType::Float => free(ptr),
          BasicDataType::Double => free(ptr),
          BasicDataType::Boolean => free(ptr),
          BasicDataType::External => {
            //
          }
        }
      }
    }
    RsArgsValue::Object(obj) => {
      let ffi_tag = get_ffi_tag(&obj);
      if let FFITag::Array = ffi_tag {
        let array_desc = get_array_desc(&obj);
        // array
        let FFIARRARYDESC {
          array_type,
          array_len,
          ..
        } = array_desc;
        match array_type {
          RefDataType::U8Array => {
            let _ = Vec::from_raw_parts(*(ptr as *mut *mut c_char), array_len, array_len);
          }
          RefDataType::I32Array => {
            let _ = Vec::from_raw_parts(*(ptr as *mut *mut c_int), array_len, array_len);
          }
          RefDataType::DoubleArray => {
            let _ = Vec::from_raw_parts(*(ptr as *mut *mut c_double), array_len, array_len);
          }
          RefDataType::FloatArray => {
            let _ = Vec::from_raw_parts(*(ptr as *mut *mut c_float), array_len, array_len);
          }
          RefDataType::StringArray => {
            let v = Vec::from_raw_parts(*(ptr as *mut *mut *mut c_char), array_len, array_len);
            v.into_iter().for_each(|str_ptr| {
              let _ = CString::from_raw(str_ptr);
            });
          }
        }
      }
      if let FFITag::Function = ffi_tag {
        let func_desc = get_func_desc(&obj);
        if func_desc.need_free {
          let _ = free(*(ptr as *mut *mut Closure) as *mut c_void);
        }
      } else {
        // raw object
      }
    }
    _ => panic!("free c pointer memory error"),
  }
}
