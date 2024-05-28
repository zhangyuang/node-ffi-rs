use crate::utils::{
  calculate_struct_size, get_array_desc, get_ffi_tag, get_func_desc, get_size_align,
};
use indexmap::IndexMap;
use libc::{c_double, c_float, c_int, c_void, free};
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

unsafe fn free_struct_memory(
  ptr: *mut c_void,
  struct_desc: IndexMap<String, RsArgsValue>,
  ptr_type: PointerType,
) {
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
          match ptr_type {
            PointerType::CPointer => free((*type_field_ptr) as *mut c_void),
            PointerType::RsPointer => {
              let _ = CString::from_raw(*(type_field_ptr as *mut *mut c_char));
            }
          }
          offset += size + padding;
          field_size = size
        }
        BasicDataType::External => {
          let (size, align) = get_size_align::<*const c_void>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
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
        } = array_desc;
        match array_type {
          RefDataType::StringArray => {
            let (size, align) = get_size_align::<*const c_void>();
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            if dynamic_array {
              free_dynamic_string_array(field_ptr, array_len);
            } else {
              //
            }
            offset += size + padding;
            field_size = size
          }
          RefDataType::DoubleArray => {
            let (size, align) = if dynamic_array {
              get_size_align::<*const c_void>()
            } else {
              let (size, align) = get_size_align::<c_double>();
              (size * array_len, align)
            };
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            if dynamic_array {
              free_dynamic_double_array(field_ptr, array_len);
            } else {
              //
            }
            offset += size + padding;
            field_size = size
          }
          RefDataType::FloatArray => {
            let (size, align) = if dynamic_array {
              get_size_align::<*const c_void>()
            } else {
              let (size, align) = get_size_align::<c_double>();
              (size * array_len, align)
            };
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            if dynamic_array {
              free_dynamic_float_array(field_ptr, array_len);
            } else {
              //
            }
            offset += size + padding;
            field_size = size
          }
          RefDataType::I32Array => {
            let (size, align) = if dynamic_array {
              get_size_align::<*const c_void>()
            } else {
              let (size, align) = get_size_align::<c_int>();
              (size * array_len, align)
            };
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            if dynamic_array {
              free_dynamic_i32_array(field_ptr, array_len)
            } else {
              //
            }
            offset += size + padding;
            field_size = size
          }
          RefDataType::U8Array => {
            let (size, align) = if dynamic_array {
              get_size_align::<*const c_void>()
            } else {
              let (size, align) = get_size_align::<u8>();
              (size * array_len, align)
            };
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            if dynamic_array {
              if let PointerType::CPointer = ptr_type {
                // only free u8 pointer data when the pointer is allocated in c
                // rust u8 pointer memory is buffer
                free_dynamic_u8_array(field_ptr, array_len)
              }
            } else {
              //
            }
            offset += size + padding;
            field_size = size
          }
        };
      } else if let FFITag::Function = get_ffi_tag(&obj) {
        let func_desc = get_func_desc(&obj);
        if func_desc.need_free {
          match ptr_type {
            PointerType::CPointer => free(*(ptr as *mut *mut c_void)),
            PointerType::RsPointer => free_closure(ptr),
          }
        }
      } else {
        // struct
        let (size, align) = get_size_align::<*const c_void>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        free_struct_memory(*(field_ptr as *mut *mut c_void), obj, ptr_type);
        offset += size + padding;
        field_size = size
      };
    };
    field_ptr = field_ptr.offset(field_size as isize) as *mut c_void;
  }
}
pub unsafe fn free_rs_pointer_memory(
  ptr: *mut c_void,
  ptr_desc: RsArgsValue,
  need_free_external: bool,
) {
  match ptr_desc {
    RsArgsValue::I32(number) => {
      let basic_data_type = number_to_basic_data_type(number);
      match basic_data_type {
        BasicDataType::String => {
          let _ = CString::from_raw(*(ptr as *mut *mut c_char));
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
          if need_free_external {
            let _ = Box::from_raw(ptr);
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
          RefDataType::U8Array => free_dynamic_u8_array(ptr, array_len),
          RefDataType::I32Array => free_dynamic_i32_array(ptr, array_len),
          RefDataType::DoubleArray => free_dynamic_double_array(ptr, array_len),
          RefDataType::FloatArray => free_dynamic_float_array(ptr, array_len),
          RefDataType::StringArray => free_dynamic_string_array(ptr, array_len),
        }
      } else if let FFITag::Function = ffi_tag {
        let func_desc = get_func_desc(&obj);
        if func_desc.need_free {
          free_closure(ptr)
        }
      } else {
        use std::alloc::{dealloc, Layout};
        let (size, align) = calculate_struct_size(&obj);
        let layout = if size > 0 {
          Layout::from_size_align(size, align).unwrap()
        } else {
          Layout::new::<i32>()
        };
        free_struct_memory(*(ptr as *mut *mut c_void), obj, PointerType::RsPointer);
        dealloc(*(ptr as *mut *mut u8), layout);
      }
    }
    _ => panic!("free rust pointer memory error"),
  }
}

unsafe fn free_closure(ptr: *mut c_void) {
  CLOSURE_MAP
    .as_ref()
    .unwrap()
    .get(&ptr)
    .unwrap()
    .iter()
    .enumerate()
    .for_each(|(index, p)| {
      if index == 0 {
        use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
        let _ =
          Box::from_raw((*p) as *mut ThreadsafeFunction<Vec<RsArgsValue>, ErrorStrategy::Fatal>)
            .abort();
      } else {
        let _ = Box::from_raw(*p);
      }
    });
}
pub unsafe fn free_c_pointer_memory(
  ptr: *mut c_void,
  ptr_desc: RsArgsValue,
  need_free_external: bool,
) {
  match ptr_desc {
    RsArgsValue::I32(number) => {
      let basic_data_type = number_to_basic_data_type(number);
      match basic_data_type {
        BasicDataType::String => {
          free(*(ptr as *mut *mut i8) as *mut c_void);
        }

        BasicDataType::External => {
          if need_free_external {
            free(ptr);
          }
        }
        _ => {
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
          RefDataType::U8Array => free_dynamic_u8_array(ptr, array_len),
          RefDataType::I32Array => free_dynamic_i32_array(ptr, array_len),
          RefDataType::DoubleArray => free_dynamic_double_array(ptr, array_len),
          RefDataType::FloatArray => free_dynamic_float_array(ptr, array_len),
          RefDataType::StringArray => free_dynamic_string_array(ptr, array_len),
        }
      } else if let FFITag::Function = ffi_tag {
        let func_desc = get_func_desc(&obj);
        if func_desc.need_free {
          free(*(ptr as *mut *mut c_void))
        }
      } else {
        // struct
        free_struct_memory(*(ptr as *mut *mut c_void), obj, PointerType::CPointer);
        free(*(ptr as *mut *mut c_void))
      }
    }
    _ => panic!("free c pointer memory error"),
  }
}

unsafe fn free_dynamic_string_array(ptr: *mut c_void, array_len: usize) {
  let v = Vec::from_raw_parts(*(ptr as *mut *mut *mut c_char), array_len, array_len);
  v.into_iter().for_each(|str_ptr| {
    let _ = CString::from_raw(str_ptr);
  });
}
unsafe fn free_dynamic_i32_array(ptr: *mut c_void, array_len: usize) {
  let _ = Vec::from_raw_parts(*(ptr as *mut *mut c_int), array_len, array_len);
}
unsafe fn free_dynamic_double_array(ptr: *mut c_void, array_len: usize) {
  let _ = Vec::from_raw_parts(*(ptr as *mut *mut c_double), array_len, array_len);
}
unsafe fn free_dynamic_float_array(ptr: *mut c_void, array_len: usize) {
  let _ = Vec::from_raw_parts(*(ptr as *mut *mut c_float), array_len, array_len);
}
unsafe fn free_dynamic_u8_array(ptr: *mut c_void, array_len: usize) {
  let _ = Vec::from_raw_parts(*(ptr as *mut *mut c_char), array_len, array_len);
}
