#[macro_use]
extern crate napi_derive;


use libloading::{Library, Symbol};
use napi::JsString;
use std::any::{Any, TypeId};
use libffi_sys::{ffi_cif, ffi_type_uint32, ffi_prep_cif, ffi_type, ffi_call, ffi_abi_FFI_DEFAULT_ABI};




fn test() {
  unsafe {

    let mut arg_types: [*mut ffi_type; 2] = [&mut ffi_type_uint32 as *mut ffi_type, &mut ffi_type_uint32 as *mut ffi_type];
    let rtype: *mut ffi_type = &mut ffi_type_uint32 as *mut ffi_type;
  
    let mut cif: ffi_cif = ffi_cif {
        abi: ffi_abi_FFI_DEFAULT_ABI,
        nargs: 2,
        arg_types: arg_types.as_ptr(),
        rtype: rtype,
        bytes: 0,
        flags: 0,
    };
  }
}

// #[napi]
// fn sum(a:i32) -> i32 {
//   unsafe {
//     let lib = Library::new("/Users/yuuang/Desktop/github/node-ffi-rs/libsum.so").unwrap();
//     let func: Symbol<unsafe extern "C" fn(i32, i32) -> i32> = lib.get(b"sum").unwrap();
//     func(1, 2)
//   }
// }

