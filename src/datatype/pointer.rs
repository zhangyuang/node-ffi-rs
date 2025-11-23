use crate::utils::{
  calculate_struct_size, get_array_desc, get_ffi_tag, get_func_desc, get_size_align,
};
use indexmap::IndexMap;
use libc::{c_double, c_float, c_int, c_short, c_void, free};
use std::alloc::{dealloc, Layout};
use std::ffi::{c_char, c_longlong, c_uchar, c_ulonglong, CStr, CString};
use widestring::{WideCString, WideChar};

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
impl_array_pointer!(*mut i16, i16);
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
  struct_desc: &IndexMap<String, RsArgsValue>,
  ptr_type: PointerType,
) {
  let mut field_ptr = ptr;
  let mut offset = 0;
  let mut field_size = 0;
  for (field, val) in struct_desc {
    if field == FFI_TAG_FIELD {
      continue;
    }
    if let RsArgsValue::I32(number) = val {
      let data_type = (*number).try_into().unwrap();
      match data_type {
        BasicDataType::U8 => {
          let (size, align) = get_size_align::<c_uchar>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          offset += size + padding;
          field_size = size;
        }
        BasicDataType::I16 => {
          let (size, align) = get_size_align::<c_short>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          offset += size + padding;
          field_size = size;
        }
        BasicDataType::I32 => {
          let (size, align) = get_size_align::<c_int>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          offset += size + padding;
          field_size = size;
        }
        BasicDataType::U32 => {
          let (size, align) = get_size_align::<u32>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          offset += size + padding;
          field_size = size;
        }
        BasicDataType::I64 | BasicDataType::BigInt => {
          let (size, align) = get_size_align::<c_longlong>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          offset += size + padding;
          field_size = size;
        }
        BasicDataType::U64 => {
          let (size, align) = get_size_align::<c_ulonglong>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          offset += size + padding;
          field_size = size;
        }
        BasicDataType::Float => {
          let (size, align) = get_size_align::<c_float>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          offset += size + padding;
          field_size = size;
        }
        BasicDataType::Double => {
          let (size, align) = get_size_align::<c_double>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          offset += size + padding;
          field_size = size;
        }
        BasicDataType::Boolean => {
          let (size, align) = get_size_align::<bool>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          offset += size + padding;
          field_size = size;
        }
        BasicDataType::Void => {
          let (size, align) = (std::mem::size_of::<()>(), std::mem::align_of::<()>());
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          offset += size + padding;
          field_size = size;
        }
        BasicDataType::String => {
          let (size, align) = get_size_align::<*const c_void>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut *mut c_char;
          match ptr_type {
            PointerType::CPointer => free(*type_field_ptr as *mut c_void),
            PointerType::RsPointer => {
              let _ = CString::from_raw(*(type_field_ptr as *mut *mut c_char));
            }
          }
          offset += size + padding;
          field_size = size;
        }
        BasicDataType::WString => {
          let (size, align) = get_size_align::<*const c_void>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          let type_field_ptr = field_ptr as *mut *mut WideChar;
          match ptr_type {
            PointerType::CPointer => free(*type_field_ptr as *mut c_void),
            PointerType::RsPointer => {
              let _ = WideCString::from_raw(*(type_field_ptr as *mut *mut WideChar));
            }
          }
          offset += size + padding;
          field_size = size;
        }
        BasicDataType::External => {
          let (size, align) = get_size_align::<*const c_void>();
          let padding = (align - (offset % align)) % align;
          field_ptr = field_ptr.offset(padding as isize);
          offset += size + padding;
          field_size = size;
        }
      }
    }
    if let RsArgsValue::Object(obj) = val {
      match get_ffi_tag(&obj) {
        FFITypeTag::Array | FFITypeTag::StackArray => {
          let array_desc = get_array_desc(&obj);
          // array
          let FFIARRARYDESC {
            array_type,
            array_len,
            struct_item_type,
            ..
          } = array_desc;
          let dynamic_array = get_ffi_tag(&obj) == FFITypeTag::Array;
          match array_type {
            RefDataType::StringArray => {
              let (size, align) = get_size_align::<*const c_void>();
              let padding = (align - (offset % align)) % align;
              field_ptr = field_ptr.offset(padding as isize);
              if dynamic_array {
                free_dynamic_string_array(field_ptr, array_len);
              }
              offset += size + padding;
              field_size = size;
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
                free_dynamic_array::<f64>(field_ptr, array_len);
              }
              offset += size + padding;
              field_size = size;
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
                free_dynamic_array::<f32>(field_ptr, array_len);
              }
              offset += size + padding;
              field_size = size;
            }
            RefDataType::I16Array => {
              let (size, align) = if dynamic_array {
                get_size_align::<*const c_void>()
              } else {
                let (size, align) = get_size_align::<i16>();
                (size * array_len, align)
              };
              let padding = (align - (offset % align)) % align;
              field_ptr = field_ptr.offset(padding as isize);
              if dynamic_array {
                free_dynamic_array::<i16>(field_ptr, array_len);
              }
              offset += size + padding;
              field_size = size;
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
                free_dynamic_array::<i32>(field_ptr, array_len);
              }
              offset += size + padding;
              field_size = size;
            }
            RefDataType::StructArray => {
              let (size, align) = if dynamic_array {
                get_size_align::<*const c_void>()
              } else {
                let (size, align) = calculate_struct_size(&struct_item_type.as_ref().unwrap());
                (size * array_len, align)
              };
              let padding = (align - (offset % align)) % align;
              field_ptr = field_ptr.offset(padding as isize);
              if dynamic_array {
                // need to review
                let (size, _) = calculate_struct_size(&struct_item_type.as_ref().unwrap());
                let mut target_ptr = *(field_ptr as *mut *mut c_void);
                (0..array_len).for_each(|_| {
                  free_struct_memory(target_ptr, struct_item_type.as_ref().unwrap(), ptr_type);
                  target_ptr = target_ptr.offset(size as isize);
                });
              }
              offset += size + padding;
              field_size = size;
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
                  free_dynamic_array::<u8>(field_ptr, array_len);
                }
              }
              offset += size + padding;
              field_size = size;
            }
          }
        }
        FFITypeTag::Function => {
          let func_desc = get_func_desc(&obj);
          if func_desc.need_free {
            match ptr_type {
              PointerType::CPointer => free(*(ptr as *mut *mut c_void)),
              PointerType::RsPointer => free_closure(ptr),
            }
          }
        }
        _ => {
          // struct
          if get_ffi_tag(obj) == FFITypeTag::StackStruct {
            let (size, align) = calculate_struct_size(&obj);
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            free_struct_memory(field_ptr, obj, ptr_type);
            offset += size + padding;
            field_size = size;
          } else {
            let (size, align) = get_size_align::<*const c_void>();
            let padding = (align - (offset % align)) % align;
            field_ptr = field_ptr.offset(padding as isize);
            free_struct_memory(*(field_ptr as *mut *mut c_void), obj, ptr_type);
            offset += size + padding;
            field_size = size;
          }
        }
      }
    }
    field_ptr = field_ptr.offset(field_size as isize) as *mut c_void;
  }
}
pub unsafe fn free_rs_pointer_memory(ptr: *mut c_void, ptr_desc: &RsArgsValue) {
  match ptr_desc {
    RsArgsValue::I32(number) => {
      let basic_data_type = (*number).try_into().unwrap();
      match basic_data_type {
        BasicDataType::String => {
          let _ = CString::from_raw(*(ptr as *mut *mut c_char));
          free(ptr);
        }
        BasicDataType::WString => {
          let _ = WideCString::from_raw(*(ptr as *mut *mut WideChar));
          free(ptr);
        }
        BasicDataType::U8
        | BasicDataType::I16
        | BasicDataType::I32
        | BasicDataType::U32
        | BasicDataType::I64
        | BasicDataType::BigInt
        | BasicDataType::U64
        | BasicDataType::Void
        | BasicDataType::Float
        | BasicDataType::Double
        | BasicDataType::Boolean
        | BasicDataType::External => {
          let _ = Box::from_raw(ptr);
        }
      }
    }
    RsArgsValue::Object(obj) => {
      let ffi_tag = get_ffi_tag(&obj);

      if let FFITypeTag::Array | FFITypeTag::StackArray = ffi_tag {
        let array_desc = get_array_desc(&obj);
        // array
        let FFIARRARYDESC {
          array_type,
          array_len,
          struct_item_type,
          ..
        } = array_desc;
        match array_type {
          RefDataType::U8Array => {}
          RefDataType::I16Array => {
            free_dynamic_array::<i16>(ptr, array_len);
            free(ptr);
          }
          RefDataType::I32Array => {
            free_dynamic_array::<i32>(ptr, array_len);
            free(ptr);
          }
          RefDataType::DoubleArray => {
            free_dynamic_array::<f64>(ptr, array_len);
            free(ptr);
          }
          RefDataType::FloatArray => {
            free_dynamic_array::<f32>(ptr, array_len);
            free(ptr);
          }
          RefDataType::StringArray => {
            free_dynamic_string_array(ptr, array_len);
            free(ptr);
          }
          RefDataType::StructArray => {
            let is_stack_struct =
              get_ffi_tag(struct_item_type.as_ref().unwrap()) == FFITypeTag::StackStruct;
            let (size, align) = calculate_struct_size(&struct_item_type.as_ref().unwrap());
            if size <= 0 {
              return;
            }
            if is_stack_struct {
              let arr_size = size * array_len;
              let layout = Layout::from_size_align(arr_size, align).unwrap();
              dealloc(*(ptr as *mut *mut u8), layout);
              free(ptr);
            } else {
              let layout = Layout::from_size_align(size, align).unwrap();
              let mut start_ptr = ptr;
              (0..array_len).for_each(|_| {
                free_struct_memory(
                  *(start_ptr as *mut *mut c_void),
                  struct_item_type.as_ref().unwrap(),
                  PointerType::RsPointer,
                );
                start_ptr = start_ptr.offset(size as isize);
              });
              dealloc(*(ptr as *mut *mut u8), layout);
              free(ptr);
            }
          }
        }
      } else if let FFITypeTag::Function = ffi_tag {
        let func_desc = get_func_desc(&obj);
        if func_desc.need_free {
          free_closure(ptr)
        }
      } else {
        let is_stack_struct = get_ffi_tag(&obj) == FFITypeTag::StackStruct;
        let (size, align) = calculate_struct_size(&obj);
        if size > 0 {
          let layout = Layout::from_size_align(size, align).unwrap();
          if !is_stack_struct {
            free_struct_memory(*(ptr as *mut *mut c_void), obj, PointerType::RsPointer);
            dealloc(*(ptr as *mut *mut u8), layout);
          } else {
            free_struct_memory(ptr, obj, PointerType::RsPointer);
          }
        }
      }
    }
    _ => panic!("free rust pointer memory error"),
  }
}

unsafe fn free_closure(ptr: *mut c_void) {
  let p = CLOSURE_MAP.as_ref().unwrap().get(&ptr).unwrap();
  let _ = Box::from_raw(*p as *mut TsFnCallContext);
}
pub unsafe fn free_c_pointer_memory(ptr: *mut c_void, ptr_desc: &RsArgsValue) {
  match ptr_desc {
    RsArgsValue::I32(number) => {
      let basic_data_type = (*number).try_into().unwrap();
      match basic_data_type {
        BasicDataType::String => {
          free(*(ptr as *mut *mut i8) as *mut c_void);
          free(ptr);
        }
        _ => {
          free(ptr);
        }
      }
    }
    RsArgsValue::Object(obj) => {
      let ffi_tag = get_ffi_tag(&obj);
      if let FFITypeTag::Array = ffi_tag {
        let array_desc = get_array_desc(&obj);
        // array
        let FFIARRARYDESC {
          array_type,
          array_len,
          struct_item_type,
          ..
        } = array_desc;
        match array_type {
          RefDataType::U8Array => {
            free_dynamic_array::<u8>(ptr, array_len);
            free(ptr);
          }
          RefDataType::FloatArray => {
            free_dynamic_array::<f32>(ptr, array_len);
            free(ptr);
          }
          RefDataType::I16Array => {
            free_dynamic_array::<i16>(ptr, array_len);
            free(ptr);
          }
          RefDataType::I32Array => {
            free_dynamic_array::<i32>(ptr, array_len);
            free(ptr);
          }
          RefDataType::DoubleArray => {
            free_dynamic_array::<f64>(ptr, array_len);
            free(ptr);
          }
          RefDataType::StringArray => free_dynamic_string_array(ptr, array_len),
          RefDataType::StructArray => {
            let mut target_ptr = *(ptr as *mut *mut c_void);
            let (size, _) = calculate_struct_size(struct_item_type.as_ref().unwrap());
            (0..array_len).for_each(|_| {
              free_struct_memory(
                target_ptr,
                struct_item_type.as_ref().unwrap(),
                PointerType::CPointer,
              );
              target_ptr = target_ptr.offset(size as isize);
            });
          }
        }
      } else if let FFITypeTag::Function = ffi_tag {
        let func_desc = get_func_desc(&obj);
        if func_desc.need_free {
          free(*(ptr as *mut *mut c_void));
          free(ptr);
        }
      } else {
        // struct
        let is_stack_struct = get_ffi_tag(&obj) == FFITypeTag::StackStruct;
        let target_ptr = if is_stack_struct {
          ptr
        } else {
          *(ptr as *mut *mut c_void)
        };
        free_struct_memory(target_ptr, obj, PointerType::CPointer);
        free(*(ptr as *mut *mut c_void));
        free(ptr);
      }
    }
    _ => panic!("free c pointer memory error"),
  }
}

pub unsafe fn free_dynamic_string_array(ptr: *mut c_void, array_len: usize) {
  let v = Vec::from_raw_parts(*(ptr as *mut *mut *mut c_char), array_len, array_len);
  v.into_iter().for_each(|str_ptr| {
    let _ = CString::from_raw(str_ptr);
  });
}

pub unsafe fn free_dynamic_array<T>(ptr: *mut c_void, array_len: usize) {
  let _ = Vec::from_raw_parts(*(ptr as *mut *mut T), array_len, array_len);
}
