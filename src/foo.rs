#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[macro_use]
extern crate napi_derive;
#[macro_use]
mod ffi_macro {}
mod define {
    use indexmap::IndexMap;
    use napi::bindgen_prelude::*;
    use napi::{Env, JsObject, JsUnknown};
    use std::hash::Hash;
    pub struct NapiIndexMap<K, V>(IndexMap<K, V>);
    #[automatically_derived]
    impl<K: ::core::clone::Clone, V: ::core::clone::Clone> ::core::clone::Clone
    for NapiIndexMap<K, V> {
        #[inline]
        fn clone(&self) -> NapiIndexMap<K, V> {
            NapiIndexMap(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<K, V> NapiIndexMap<K, V> {
        pub fn get_inner_map(&self) -> &IndexMap<K, V> {
            &self.0
        }
    }
    impl<K, V> FromNapiValue for NapiIndexMap<K, V>
    where
        K: From<String> + Eq + Hash,
        V: FromNapiValue,
    {
        unsafe fn from_napi_value(
            env: sys::napi_env,
            napi_val: sys::napi_value,
        ) -> Result<Self> {
            let obj = unsafe { JsObject::from_napi_value(env, napi_val)? };
            let mut map = IndexMap::new();
            for key in JsObject::keys(&obj)?.into_iter() {
                if let Some(val) = obj.get(&key)? {
                    map.insert(K::from(key), val);
                }
            }
            Ok(NapiIndexMap(map))
        }
    }
    impl<K, V> ToNapiValue for NapiIndexMap<K, V>
    where
        K: AsRef<str>,
        V: ToNapiValue,
    {
        unsafe fn to_napi_value(
            raw_env: sys::napi_env,
            val: Self,
        ) -> Result<sys::napi_value> {
            let env = Env::from(raw_env);
            let mut obj = env.create_object()?;
            let map = val.0;
            for (k, v) in map.into_iter() {
                obj.set(k.as_ref(), v)?;
            }
            unsafe { JsObject::to_napi_value(raw_env, obj) }
        }
    }
    pub enum DataType {
        String = 0,
        I32 = 1,
        Double = 2,
        I32Array = 3,
        StringArray = 4,
        DoubleArray = 5,
        Boolean = 6,
        Void = 7,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for DataType {}
    #[automatically_derived]
    impl ::core::clone::Clone for DataType {
        #[inline]
        fn clone(&self) -> DataType {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DataType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    DataType::String => "String",
                    DataType::I32 => "I32",
                    DataType::Double => "Double",
                    DataType::I32Array => "I32Array",
                    DataType::StringArray => "StringArray",
                    DataType::DoubleArray => "DoubleArray",
                    DataType::Boolean => "Boolean",
                    DataType::Void => "Void",
                },
            )
        }
    }
    impl napi::bindgen_prelude::TypeName for DataType {
        fn type_name() -> &'static str {
            "DataType"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for DataType {
        unsafe fn validate(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<napi::sys::napi_value> {
            {
                let mut value_type = 0;
                #[allow(unused_unsafe)]
                {
                    let c = unsafe {
                        ::napi::sys::napi_typeof(env, napi_val, &mut value_type)
                    };
                    match c {
                        ::napi::sys::Status::napi_ok => Ok(()),
                        _ => {
                            Err(
                                ::napi::Error::new(::napi::Status::from(c), "".to_owned()),
                            )
                        }
                    }
                }
                    .and_then(|_| Ok(::napi::ValueType::from(value_type)))
            }
                .and_then(|received_type| {
                    if received_type == napi::bindgen_prelude::ValueType::Number {
                        Ok(())
                    } else {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::InvalidArg,
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Expect value to be {0}, but received {1}",
                                            napi::bindgen_prelude::ValueType::Number,
                                            received_type,
                                        ),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                })?;
            Ok(std::ptr::null_mut())
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for DataType {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let val = FromNapiValue::from_napi_value(env, napi_val)
                .map_err(|e| {
                    ::napi::Error::new(
                        e.status,
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Failed to convert napi value into enum `{0}`. {1}",
                                    "DataType",
                                    e,
                                ),
                            );
                            res
                        },
                    )
                })?;
            match val {
                0 => Ok(DataType::String),
                1 => Ok(DataType::I32),
                2 => Ok(DataType::Double),
                3 => Ok(DataType::I32Array),
                4 => Ok(DataType::StringArray),
                5 => Ok(DataType::DoubleArray),
                6 => Ok(DataType::Boolean),
                7 => Ok(DataType::Void),
                _ => {
                    Err(
                        ::napi::Error::new(
                            napi::bindgen_prelude::Status::InvalidArg,
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "value `{0:?}` does not match any variant of enum `{1}`",
                                        val,
                                        "DataType",
                                    ),
                                );
                                res
                            },
                        ),
                    )
                }
            }
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for DataType {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: Self,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let val = match val {
                DataType::String => 0,
                DataType::I32 => 1,
                DataType::Double => 2,
                DataType::I32Array => 3,
                DataType::StringArray => 4,
                DataType::DoubleArray => 5,
                DataType::Boolean => 6,
                DataType::Void => 7,
            };
            ToNapiValue::to_napi_value(env, val)
        }
    }
    #[allow(non_snake_case)]
    #[allow(clippy::all)]
    unsafe fn __register__enum__DataType_callback__(
        env: napi::bindgen_prelude::sys::napi_env,
    ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
        use std::ffi::CString;
        use std::ptr;
        let mut obj_ptr = ptr::null_mut();
        {
            let c = napi::bindgen_prelude::sys::napi_create_object(env, &mut obj_ptr);
            match c {
                ::napi::sys::Status::napi_ok => Ok(()),
                _ => {
                    Err(
                        ::napi::Error::new(
                            ::napi::Status::from(c),
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!("Failed to create napi object"),
                                );
                                res
                            },
                        ),
                    )
                }
            }
        }?;
        {
            let name = std::ffi::CStr::from_bytes_with_nul_unchecked(
                "String\0".as_bytes(),
            );
            {
                let c = napi::bindgen_prelude::sys::napi_set_named_property(
                    env,
                    obj_ptr,
                    name.as_ptr(),
                    ToNapiValue::to_napi_value(env, 0)?,
                );
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!("Failed to defined enum `{0}`", "DataType\0"),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
        };
        {
            let name = std::ffi::CStr::from_bytes_with_nul_unchecked("I32\0".as_bytes());
            {
                let c = napi::bindgen_prelude::sys::napi_set_named_property(
                    env,
                    obj_ptr,
                    name.as_ptr(),
                    ToNapiValue::to_napi_value(env, 1)?,
                );
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!("Failed to defined enum `{0}`", "DataType\0"),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
        };
        {
            let name = std::ffi::CStr::from_bytes_with_nul_unchecked(
                "Double\0".as_bytes(),
            );
            {
                let c = napi::bindgen_prelude::sys::napi_set_named_property(
                    env,
                    obj_ptr,
                    name.as_ptr(),
                    ToNapiValue::to_napi_value(env, 2)?,
                );
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!("Failed to defined enum `{0}`", "DataType\0"),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
        };
        {
            let name = std::ffi::CStr::from_bytes_with_nul_unchecked(
                "I32Array\0".as_bytes(),
            );
            {
                let c = napi::bindgen_prelude::sys::napi_set_named_property(
                    env,
                    obj_ptr,
                    name.as_ptr(),
                    ToNapiValue::to_napi_value(env, 3)?,
                );
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!("Failed to defined enum `{0}`", "DataType\0"),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
        };
        {
            let name = std::ffi::CStr::from_bytes_with_nul_unchecked(
                "StringArray\0".as_bytes(),
            );
            {
                let c = napi::bindgen_prelude::sys::napi_set_named_property(
                    env,
                    obj_ptr,
                    name.as_ptr(),
                    ToNapiValue::to_napi_value(env, 4)?,
                );
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!("Failed to defined enum `{0}`", "DataType\0"),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
        };
        {
            let name = std::ffi::CStr::from_bytes_with_nul_unchecked(
                "DoubleArray\0".as_bytes(),
            );
            {
                let c = napi::bindgen_prelude::sys::napi_set_named_property(
                    env,
                    obj_ptr,
                    name.as_ptr(),
                    ToNapiValue::to_napi_value(env, 5)?,
                );
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!("Failed to defined enum `{0}`", "DataType\0"),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
        };
        {
            let name = std::ffi::CStr::from_bytes_with_nul_unchecked(
                "Boolean\0".as_bytes(),
            );
            {
                let c = napi::bindgen_prelude::sys::napi_set_named_property(
                    env,
                    obj_ptr,
                    name.as_ptr(),
                    ToNapiValue::to_napi_value(env, 6)?,
                );
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!("Failed to defined enum `{0}`", "DataType\0"),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
        };
        {
            let name = std::ffi::CStr::from_bytes_with_nul_unchecked(
                "Void\0".as_bytes(),
            );
            {
                let c = napi::bindgen_prelude::sys::napi_set_named_property(
                    env,
                    obj_ptr,
                    name.as_ptr(),
                    ToNapiValue::to_napi_value(env, 7)?,
                );
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => {
                        Err(
                            ::napi::Error::new(
                                ::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!("Failed to defined enum `{0}`", "DataType\0"),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
        };
        Ok(obj_ptr)
    }
    #[allow(non_snake_case)]
    #[allow(clippy::all)]
    #[cfg(all(not(test), not(feature = "noop"), not(target_arch = "wasm32")))]
    extern fn __napi_register__DataType_0() {
        napi::bindgen_prelude::register_module_export(
            None,
            "DataType\0",
            __register__enum__DataType_callback__,
        );
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = "__DATA,__mod_init_func"]
    static __napi_register__DataType_0___rust_ctor___ctor: unsafe extern "C" fn() = {
        unsafe extern "C" fn __napi_register__DataType_0___rust_ctor___ctor() {
            __napi_register__DataType_0()
        }
        __napi_register__DataType_0___rust_ctor___ctor
    };
    pub fn number_to_data_type(value: i32) -> DataType {
        match value {
            0 => DataType::String,
            1 => DataType::I32,
            2 => DataType::Double,
            3 => DataType::I32Array,
            4 => DataType::StringArray,
            5 => DataType::DoubleArray,
            6 => DataType::Boolean,
            7 => DataType::Void,
            _ => {
                ::core::panicking::panic_fmt(format_args!("unknow DataType"));
            }
        }
    }
    pub enum RsArgsValue {
        String(String),
        I32(i32),
        Double(f64),
        I32Array(Vec<i32>),
        StringArray(Vec<String>),
        DoubleArray(Vec<f64>),
        Object(IndexMap<String, RsArgsValue>),
        Boolean(bool),
        Void(()),
        Function(JsFunction, JsFunction),
    }
    pub struct FFIParams {
        pub library: String,
        pub func_name: String,
        pub ret_type: JsUnknown,
        pub params_type: Vec<JsUnknown>,
        pub params_value: Vec<JsUnknown>,
    }
    impl napi::bindgen_prelude::TypeName for FFIParams {
        fn type_name() -> &'static str {
            "FFIParams"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for FFIParams {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: FFIParams,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self {
                library: library_,
                func_name: func_name_,
                ret_type: ret_type_,
                params_type: params_type_,
                params_value: params_value_,
            } = val;
            obj.set("library", library_)?;
            obj.set("funcName", func_name_)?;
            obj.set("retType", ret_type_)?;
            obj.set("paramsType", params_type_)?;
            obj.set("paramsValue", params_value_)?;
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for FFIParams {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let library_: String = obj
                .get("library")?
                .ok_or_else(|| napi::bindgen_prelude::Error::new(
                    napi::bindgen_prelude::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Missing field `{0}`", "library"),
                        );
                        res
                    },
                ))?;
            let func_name_: String = obj
                .get("funcName")?
                .ok_or_else(|| napi::bindgen_prelude::Error::new(
                    napi::bindgen_prelude::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Missing field `{0}`", "funcName"),
                        );
                        res
                    },
                ))?;
            let ret_type_: JsUnknown = obj
                .get("retType")?
                .ok_or_else(|| napi::bindgen_prelude::Error::new(
                    napi::bindgen_prelude::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Missing field `{0}`", "retType"),
                        );
                        res
                    },
                ))?;
            let params_type_: Vec<JsUnknown> = obj
                .get("paramsType")?
                .ok_or_else(|| napi::bindgen_prelude::Error::new(
                    napi::bindgen_prelude::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Missing field `{0}`", "paramsType"),
                        );
                        res
                    },
                ))?;
            let params_value_: Vec<JsUnknown> = obj
                .get("paramsValue")?
                .ok_or_else(|| napi::bindgen_prelude::Error::new(
                    napi::bindgen_prelude::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Missing field `{0}`", "paramsValue"),
                        );
                        res
                    },
                ))?;
            let val = Self {
                library: library_,
                func_name: func_name_,
                ret_type: ret_type_,
                params_type: params_type_,
                params_value: params_value_,
            };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for FFIParams {}
    pub struct OpenParams {
        pub library: String,
        pub path: String,
    }
    impl napi::bindgen_prelude::TypeName for OpenParams {
        fn type_name() -> &'static str {
            "OpenParams"
        }
        fn value_type() -> napi::ValueType {
            napi::ValueType::Object
        }
    }
    impl napi::bindgen_prelude::ToNapiValue for OpenParams {
        unsafe fn to_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            val: OpenParams,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = env_wrapper.create_object()?;
            let Self { library: library_, path: path_ } = val;
            obj.set("library", library_)?;
            obj.set("path", path_)?;
            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }
    impl napi::bindgen_prelude::FromNapiValue for OpenParams {
        unsafe fn from_napi_value(
            env: napi::bindgen_prelude::sys::napi_env,
            napi_val: napi::bindgen_prelude::sys::napi_value,
        ) -> napi::bindgen_prelude::Result<Self> {
            let env_wrapper = napi::bindgen_prelude::Env::from(env);
            let mut obj = napi::bindgen_prelude::Object::from_napi_value(env, napi_val)?;
            let library_: String = obj
                .get("library")?
                .ok_or_else(|| napi::bindgen_prelude::Error::new(
                    napi::bindgen_prelude::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Missing field `{0}`", "library"),
                        );
                        res
                    },
                ))?;
            let path_: String = obj
                .get("path")?
                .ok_or_else(|| napi::bindgen_prelude::Error::new(
                    napi::bindgen_prelude::Status::InvalidArg,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Missing field `{0}`", "path"),
                        );
                        res
                    },
                ))?;
            let val = Self {
                library: library_,
                path: path_,
            };
            Ok(val)
        }
    }
    impl napi::bindgen_prelude::ValidateNapiValue for OpenParams {}
}
mod pointer {
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
    impl ArrayPointer for *mut i32 {
        type Output = i32;
        unsafe fn get_and_advance(&mut self) -> Self::Output {
            let value = **self;
            *self = self.offset(1);
            value
        }
    }
    impl ArrayPointer for *mut f64 {
        type Output = f64;
        unsafe fn get_and_advance(&mut self) -> Self::Output {
            let value = **self;
            *self = self.offset(1);
            value
        }
    }
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
                let array_constructor: JsObject = ret_object
                    .get_named_property(&field)
                    .unwrap();
                let array_len: usize = if array_constructor
                    .has_named_property("length")
                    .unwrap()
                {
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
                        let js_string = CString::from_raw(*type_field_ptr)
                            .into_string()
                            .unwrap();
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
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "{0:?} is not available as a field type at this time",
                                data_type,
                            ),
                        );
                    }
                }
                field_ptr = field_ptr.offset(offset as isize) as *mut c_void;
            });
        js_object
    }
}
mod utils {
    use crate::define::{number_to_data_type, DataType, RsArgsValue};
    use crate::pointer::{create_array_from_pointer, create_object_from_pointer};
    use indexmap::IndexMap;
    use napi::bindgen_prelude::*;
    use napi::{JsBoolean, JsNumber, JsObject, JsString, JsUnknown};
    use std::ffi::c_void;
    use std::ffi::{c_char, c_double, c_int, CString};
    pub unsafe fn get_js_function_call_value(
        env: &Env,
        func_arg_type: JsUnknown,
        func_arg_ptr: *mut c_void,
    ) -> JsUnknown {
        return match func_arg_type.get_type().unwrap() {
            ValueType::Number => {
                let data_type: DataType = number_to_data_type(
                    func_arg_type.coerce_to_number().unwrap().try_into().unwrap(),
                );
                let data = match data_type {
                    DataType::I32 => {
                        env.create_int32(func_arg_ptr as i32).unwrap().into_unknown()
                    }
                    DataType::Boolean => {
                        env.get_boolean(
                                if func_arg_ptr as i32 == 0 { false } else { true },
                            )
                            .unwrap()
                            .into_unknown()
                    }
                    DataType::String => {
                        return env
                            .create_string(
                                &CString::from_raw(func_arg_ptr as *mut c_char)
                                    .into_string()
                                    .unwrap(),
                            )
                            .unwrap()
                            .into_unknown();
                    }
                    DataType::Double => {
                        {
                            ::std::io::_print(format_args!("{0:?}\n", func_arg_ptr));
                        };
                        return env.create_double(1.1).unwrap().into_unknown();
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "{0:?} data_type as function args is unsupported at this time",
                                data_type,
                            ),
                        );
                    }
                };
                data
            }
            ValueType::Object => {
                let args_type = func_arg_type.coerce_to_object().unwrap();
                let ffi_tag = args_type.has_named_property("ffiTypeTag").unwrap();
                if ffi_tag {
                    let array_len: usize = js_nunmber_to_i32(
                        args_type.get_named_property::<JsNumber>("length").unwrap(),
                    ) as usize;
                    let array_type: i32 = js_nunmber_to_i32(
                        args_type.get_named_property::<JsNumber>("type").unwrap(),
                    );
                    let array_type = number_to_data_type(array_type);
                    match array_type {
                        DataType::StringArray => {
                            let arr = create_array_from_pointer(
                                func_arg_ptr as *mut *mut c_char,
                                array_len,
                            );
                            rs_array_to_js_array(env, ArrayType::String(arr))
                                .into_unknown()
                        }
                        DataType::I32Array => {
                            let arr = create_array_from_pointer(
                                func_arg_ptr as *mut c_int,
                                array_len,
                            );
                            rs_array_to_js_array(env, ArrayType::I32(arr)).into_unknown()
                        }
                        DataType::DoubleArray => {
                            let arr = create_array_from_pointer(
                                func_arg_ptr as *mut c_double,
                                array_len,
                            );
                            rs_array_to_js_array(env, ArrayType::Double(arr))
                                .into_unknown()
                        }
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "{0:?} as function args is unsupported ",
                                    array_type,
                                ),
                            );
                        }
                    }
                } else {
                    create_object_from_pointer(env, func_arg_ptr, args_type)
                        .into_unknown()
                }
            }
            _ => {
                ::core::panicking::panic_fmt(
                    format_args!("get_js_function_call_value err "),
                );
            }
        };
    }
    pub fn js_array_to_string_array(js_array: JsObject) -> Vec<String> {
        (0..js_array.get_array_length().unwrap())
            .enumerate()
            .map(|(index, _)| {
                let js_element: JsString = js_array.get_element(index as u32).unwrap();
                return js_element.into_utf8().unwrap().try_into().unwrap();
            })
            .collect::<Vec<String>>()
    }
    pub fn js_array_to_number_array<T>(js_array: JsObject) -> Vec<T>
    where
        T: TryFrom<JsNumber>,
        <T as TryFrom<JsNumber>>::Error: std::fmt::Debug,
    {
        (0..js_array.get_array_length().unwrap())
            .enumerate()
            .map(|(index, _)| {
                let js_element: JsNumber = js_array.get_element(index as u32).unwrap();
                return js_element.try_into().unwrap();
            })
            .collect::<Vec<T>>()
    }
    fn calculate_i32(size: usize, align: usize) -> (usize, usize) {
        let align = align.max(std::mem::align_of::<c_int>());
        let size = size + std::mem::size_of::<c_int>();
        (size, align)
    }
    fn calculate_double(size: usize, align: usize) -> (usize, usize) {
        let align = align.max(std::mem::align_of::<c_double>());
        let size = size + std::mem::size_of::<c_double>();
        (size, align)
    }
    fn calculate_boolean(size: usize, align: usize) -> (usize, usize) {
        let align = align.max(std::mem::align_of::<bool>());
        let size = size + std::mem::size_of::<bool>();
        (size, align)
    }
    fn calculate_string(size: usize, align: usize) -> (usize, usize) {
        let align = align.max(std::mem::align_of::<*const c_char>());
        let size = size + std::mem::size_of::<*const c_char>();
        (size, align)
    }
    fn calculate_string_array(size: usize, align: usize) -> (usize, usize) {
        let align = align.max(std::mem::align_of::<*const *const c_char>());
        let size = size + std::mem::size_of::<*const *const c_char>();
        (size, align)
    }
    fn calculate_double_array(size: usize, align: usize) -> (usize, usize) {
        let align = align.max(std::mem::align_of::<*const c_double>());
        let size = size + std::mem::size_of::<*const c_double>();
        (size, align)
    }
    fn calculate_i32_array(size: usize, align: usize) -> (usize, usize) {
        let align = align.max(std::mem::align_of::<*const c_int>());
        let size = size + std::mem::size_of::<*const c_int>();
        (size, align)
    }
    pub fn calculate_layout(map: &IndexMap<String, RsArgsValue>) -> (usize, usize) {
        let (size, align) = map
            .iter()
            .fold(
                (0, 0),
                |(size, align), (_, field_val)| match field_val {
                    RsArgsValue::I32(_) => calculate_i32(size, align),
                    RsArgsValue::Double(_) => calculate_double(size, align),
                    RsArgsValue::String(_) => calculate_string(size, align),
                    RsArgsValue::Boolean(_) => calculate_boolean(size, align),
                    RsArgsValue::Object(val) => {
                        let (obj_size, obj_align) = calculate_layout(val);
                        let align = align.max(obj_align);
                        let size = size + obj_size;
                        (size, align)
                    }
                    RsArgsValue::StringArray(_) => calculate_string_array(size, align),
                    RsArgsValue::DoubleArray(_) => calculate_double_array(size, align),
                    RsArgsValue::I32Array(_) => calculate_i32_array(size, align),
                    _ => {
                        ::core::panicking::panic_fmt(format_args!("calculate_layout"));
                    }
                },
            );
        (size, align)
    }
    pub fn get_rs_value_size_align(val: &RsArgsValue) -> (usize, usize) {
        return match val {
            RsArgsValue::I32(_) => {
                (std::mem::size_of::<i32>(), std::mem::align_of::<i32>())
            }
            RsArgsValue::Boolean(_) => {
                (std::mem::size_of::<bool>(), std::mem::align_of::<bool>())
            }
            RsArgsValue::String(_) => {
                (
                    std::mem::size_of::<*const c_char>(),
                    std::mem::align_of::<*const c_char>(),
                )
            }
            RsArgsValue::Double(_) => {
                (std::mem::size_of::<c_double>(), std::mem::align_of::<c_double>())
            }
            RsArgsValue::StringArray(_) => {
                (
                    std::mem::size_of::<*const *const c_char>(),
                    std::mem::align_of::<*const *const c_char>(),
                )
            }
            RsArgsValue::DoubleArray(_) => {
                (
                    std::mem::size_of::<*const c_double>(),
                    std::mem::align_of::<*const c_double>(),
                )
            }
            RsArgsValue::I32Array(_) => {
                (
                    std::mem::size_of::<*const c_int>(),
                    std::mem::align_of::<*const c_int>(),
                )
            }
            _ => {
                ::core::panicking::panic_fmt(
                    format_args!("get_rs_value_size_align error"),
                );
            }
        };
    }
    pub fn get_data_type_size_align(data_type: DataType) -> (usize, usize) {
        return match data_type {
            DataType::I32 => (std::mem::size_of::<i32>(), std::mem::align_of::<i32>()),
            DataType::Boolean => {
                (std::mem::size_of::<bool>(), std::mem::align_of::<bool>())
            }
            DataType::String => {
                (
                    std::mem::size_of::<*const c_char>(),
                    std::mem::align_of::<*const c_char>(),
                )
            }
            DataType::Double => {
                (std::mem::size_of::<c_double>(), std::mem::align_of::<c_double>())
            }
            DataType::StringArray => {
                (
                    std::mem::size_of::<*const *const c_char>(),
                    std::mem::align_of::<*const *const c_char>(),
                )
            }
            DataType::DoubleArray => {
                (
                    std::mem::size_of::<*const c_double>(),
                    std::mem::align_of::<*const c_double>(),
                )
            }
            DataType::I32Array => {
                (
                    std::mem::size_of::<*const c_int>(),
                    std::mem::align_of::<*const c_int>(),
                )
            }
            _ => {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "{0:?} Not available as a field type at this time",
                        data_type,
                    ),
                );
            }
        };
    }
    pub enum ArrayType {
        I32(Vec<i32>),
        Double(Vec<f64>),
        String(Vec<String>),
    }
    pub fn js_string_to_string(js_string: JsString) -> String {
        js_string.into_utf8().unwrap().try_into().unwrap()
    }
    pub fn js_nunmber_to_i32(js_number: JsNumber) -> i32 {
        js_number.try_into().unwrap()
    }
    pub fn js_unknown_to_data_type(val: JsUnknown) -> DataType {
        match val.get_type().unwrap() {
            ValueType::Number => {
                let val = val.coerce_to_number().unwrap();
                number_to_data_type(val.try_into().unwrap())
            }
            ValueType::Object => {
                let val = val.coerce_to_object().unwrap();
                let ffi_tag = val.has_named_property("ffiTypeTag").unwrap();
                if ffi_tag {
                    number_to_data_type(
                        js_nunmber_to_i32(
                            val.get_named_property::<JsNumber>("type").unwrap(),
                        ),
                    )
                } else {
                    {
                        ::core::panicking::panic_fmt(format_args!("some error"));
                    }
                }
            }
            _ => {
                ::core::panicking::panic_fmt(format_args!("some error"));
            }
        }
    }
    pub fn rs_array_to_js_array(env: &Env, val: ArrayType) -> JsObject {
        match val {
            ArrayType::String(arr) => {
                let mut js_array = env.create_array_with_length(arr.len()).unwrap();
                arr.into_iter()
                    .enumerate()
                    .for_each(|(index, str)| {
                        js_array
                            .set_element(index as u32, env.create_string(&str).unwrap())
                            .unwrap();
                    });
                js_array
            }
            ArrayType::Double(arr) => {
                let mut js_array = env.create_array_with_length(arr.len()).unwrap();
                arr.into_iter()
                    .enumerate()
                    .for_each(|(index, item)| {
                        js_array
                            .set_element(index as u32, env.create_double(item).unwrap())
                            .unwrap();
                    });
                js_array
            }
            ArrayType::I32(arr) => {
                let mut js_array = env.create_array_with_length(arr.len()).unwrap();
                arr.into_iter()
                    .enumerate()
                    .for_each(|(index, item)| {
                        js_array
                            .set_element(index as u32, env.create_int32(item).unwrap())
                            .unwrap();
                    });
                js_array
            }
        }
    }
    pub fn jsobject_to_rs_struct(
        params_type_object: JsObject,
        params_value_object: JsObject,
    ) -> IndexMap<String, RsArgsValue> {
        let mut index_map = IndexMap::new();
        JsObject::keys(&params_value_object)
            .unwrap()
            .into_iter()
            .for_each(|field| {
                let field_type: DataType = params_type_object
                    .get_named_property(&field)
                    .unwrap();
                let val = match field_type {
                    DataType::String => {
                        let val: JsString = params_value_object
                            .get_named_property(&field)
                            .unwrap();
                        let val: String = val.into_utf8().unwrap().try_into().unwrap();
                        RsArgsValue::String(val)
                    }
                    DataType::I32 => {
                        let val: JsNumber = params_value_object
                            .get_named_property(&field)
                            .unwrap();
                        let val: i32 = val.try_into().unwrap();
                        RsArgsValue::I32(val)
                    }
                    DataType::Boolean => {
                        let val: JsBoolean = params_value_object
                            .get_named_property(&field)
                            .unwrap();
                        let val: bool = val.get_value().unwrap();
                        RsArgsValue::Boolean(val)
                    }
                    DataType::Double => {
                        let val: JsNumber = params_value_object
                            .get_named_property(&field)
                            .unwrap();
                        let val: f64 = val.try_into().unwrap();
                        RsArgsValue::Double(val)
                    }
                    DataType::StringArray => {
                        let js_array: JsObject = params_value_object
                            .get_named_property(&field)
                            .unwrap();
                        let arg_val = js_array_to_string_array(js_array);
                        RsArgsValue::StringArray(arg_val)
                    }
                    DataType::DoubleArray => {
                        let js_array: JsObject = params_value_object
                            .get_named_property(&field)
                            .unwrap();
                        let arg_val = js_array_to_number_array(js_array);
                        RsArgsValue::DoubleArray(arg_val)
                    }
                    DataType::I32Array => {
                        let js_array: JsObject = params_value_object
                            .get_named_property(&field)
                            .unwrap();
                        let arg_val = js_array_to_number_array(js_array);
                        RsArgsValue::I32Array(arg_val)
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!("jsobject_to_rs_struct"),
                        );
                    }
                };
                index_map.insert(field, val);
            });
        index_map
    }
}
use define::*;
use indexmap::IndexMap;
use libc::malloc;
use libc::{c_char, c_double, c_int};
use libffi_sys::{
    ffi_abi_FFI_DEFAULT_ABI, ffi_call, ffi_cif, ffi_prep_cif, ffi_type, ffi_type_double,
    ffi_type_pointer, ffi_type_sint32, ffi_type_uint8, ffi_type_void,
};
use libloading::{Library, Symbol};
use napi::bindgen_prelude::*;
use napi::{Env, JsFunction, JsNumber, JsObject, JsString, JsUnknown};
use pointer::*;
use std::alloc::{alloc, Layout};
use std::collections::HashMap;
use std::ffi::c_void;
use std::ffi::CString;
use utils::*;
enum FFIJsValue {
    I32(i32),
    JsObject(JsObject),
    Unknown,
}
static mut LibraryMap: Option<HashMap<String, Library>> = None;
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
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(clippy::all)]
extern "C" fn __napi__open(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
        napi::bindgen_prelude::CallbackInfo::<1usize>::new(env, cb, None)
            .and_then(|mut cb| {
                let arg0 = {
                    <OpenParams as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                        env,
                        cb.get_arg(0usize),
                    )?
                };
                napi::bindgen_prelude::within_runtime_if_available(move || {
                    let _ret = { open(arg0) };
                    <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
                })
            })
            .unwrap_or_else(|e| {
                napi::bindgen_prelude::JsError::from(e).throw_into(env);
                std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
            })
    }
}
#[allow(non_snake_case)]
#[allow(clippy::all)]
unsafe fn open_js_function(
    env: napi::bindgen_prelude::sys::napi_env,
) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
    let mut fn_ptr = std::ptr::null_mut();
    {
        let c = napi::bindgen_prelude::sys::napi_create_function(
            env,
            "open\0".as_ptr().cast(),
            4usize,
            Some(__napi__open),
            std::ptr::null_mut(),
            &mut fn_ptr,
        );
        match c {
            ::napi::sys::Status::napi_ok => Ok(()),
            _ => {
                Err(
                    ::napi::Error::new(
                        ::napi::Status::from(c),
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("Failed to register function `{0}`", "open"),
                            );
                            res
                        },
                    ),
                )
            }
        }
    }?;
    napi::bindgen_prelude::register_js_function(
        "open\0",
        open_js_function,
        Some(__napi__open),
    );
    Ok(fn_ptr)
}
#[allow(clippy::all)]
#[allow(non_snake_case)]
#[cfg(all(not(test), not(feature = "noop"), not(target_arch = "wasm32")))]
extern fn __napi_register__open_1() {
    napi::bindgen_prelude::register_module_export(None, "open\0", open_js_function);
}
#[used]
#[allow(non_upper_case_globals)]
#[doc(hidden)]
#[link_section = "__DATA,__mod_init_func"]
static __napi_register__open_1___rust_ctor___ctor: unsafe extern "C" fn() = {
    unsafe extern "C" fn __napi_register__open_1___rust_ctor___ctor() {
        __napi_register__open_1()
    }
    __napi_register__open_1___rust_ctor___ctor
};
fn close(library: String) {
    unsafe {
        if LibraryMap.is_none() {
            return;
        }
        let map = LibraryMap.as_mut().unwrap();
        map.remove(&library);
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(clippy::all)]
extern "C" fn __napi__close(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
        napi::bindgen_prelude::CallbackInfo::<1usize>::new(env, cb, None)
            .and_then(|mut cb| {
                let arg0 = {
                    <String as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                        env,
                        cb.get_arg(0usize),
                    )?
                };
                napi::bindgen_prelude::within_runtime_if_available(move || {
                    let _ret = { close(arg0) };
                    <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
                })
            })
            .unwrap_or_else(|e| {
                napi::bindgen_prelude::JsError::from(e).throw_into(env);
                std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
            })
    }
}
#[allow(non_snake_case)]
#[allow(clippy::all)]
unsafe fn close_js_function(
    env: napi::bindgen_prelude::sys::napi_env,
) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
    let mut fn_ptr = std::ptr::null_mut();
    {
        let c = napi::bindgen_prelude::sys::napi_create_function(
            env,
            "close\0".as_ptr().cast(),
            5usize,
            Some(__napi__close),
            std::ptr::null_mut(),
            &mut fn_ptr,
        );
        match c {
            ::napi::sys::Status::napi_ok => Ok(()),
            _ => {
                Err(
                    ::napi::Error::new(
                        ::napi::Status::from(c),
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("Failed to register function `{0}`", "close"),
                            );
                            res
                        },
                    ),
                )
            }
        }
    }?;
    napi::bindgen_prelude::register_js_function(
        "close\0",
        close_js_function,
        Some(__napi__close),
    );
    Ok(fn_ptr)
}
#[allow(clippy::all)]
#[allow(non_snake_case)]
#[cfg(all(not(test), not(feature = "noop"), not(target_arch = "wasm32")))]
extern fn __napi_register__close_2() {
    napi::bindgen_prelude::register_module_export(None, "close\0", close_js_function);
}
#[used]
#[allow(non_upper_case_globals)]
#[doc(hidden)]
#[link_section = "__DATA,__mod_init_func"]
static __napi_register__close_2___rust_ctor___ctor: unsafe extern "C" fn() = {
    unsafe extern "C" fn __napi_register__close_2___rust_ctor___ctor() {
        __napi_register__close_2()
    }
    __napi_register__close_2___rust_ctor___ctor
};
unsafe fn load(
    env: Env,
    params: FFIParams,
) -> Either9<String, i32, (), f64, Vec<i32>, Vec<String>, Vec<f64>, bool, JsObject> {
    let FFIParams { library, func_name, ret_type, params_type, params_value } = params;
    let lib = LibraryMap.as_ref().unwrap();
    let lib = lib.get(&library).unwrap();
    let func: Symbol<unsafe extern "C" fn()> = lib
        .get(func_name.as_str().as_bytes())
        .unwrap();
    let params_type_len = params_type.len();
    let (mut arg_types, arg_values): (Vec<*mut ffi_type>, Vec<RsArgsValue>) = params_type
        .into_iter()
        .zip(params_value.into_iter())
        .map(|(param, value)| {
            let value_type = param.get_type().unwrap();
            match value_type {
                ValueType::Number => {
                    let param_data_type = number_to_data_type(
                        param.coerce_to_number().unwrap().try_into().unwrap(),
                    );
                    match param_data_type {
                        DataType::I32 => {
                            let arg_type = &mut ffi_type_sint32 as *mut ffi_type;
                            let arg_val: i32 = value
                                .coerce_to_number()
                                .unwrap()
                                .try_into()
                                .unwrap();
                            (arg_type, RsArgsValue::I32(arg_val))
                        }
                        DataType::Double => {
                            let arg_type = &mut ffi_type_double as *mut ffi_type;
                            let arg_val: f64 = value
                                .coerce_to_number()
                                .unwrap()
                                .try_into()
                                .unwrap();
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
                            let arg_val = ::alloc::vec::from_elem(
                                    0,
                                    js_object.get_array_length().unwrap() as usize,
                                )
                                .iter()
                                .enumerate()
                                .map(|(index, _)| {
                                    let js_element: JsNumber = js_object
                                        .get_element(index as u32)
                                        .unwrap();
                                    return js_element.get_int32().unwrap();
                                })
                                .collect::<Vec<i32>>();
                            (arg_type, RsArgsValue::I32Array(arg_val))
                        }
                        DataType::DoubleArray => {
                            let arg_type = &mut ffi_type_pointer as *mut ffi_type;
                            let js_object = value.coerce_to_object().unwrap();
                            let arg_val = ::alloc::vec::from_elem(
                                    0,
                                    js_object.get_array_length().unwrap() as usize,
                                )
                                .iter()
                                .enumerate()
                                .map(|(index, _)| {
                                    let js_element: JsNumber = js_object
                                        .get_element(index as u32)
                                        .unwrap();
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
                            let arg_val: bool = value
                                .coerce_to_bool()
                                .unwrap()
                                .get_value()
                                .unwrap();
                            (arg_type, RsArgsValue::Boolean(arg_val))
                        }
                        DataType::Void => {
                            let arg_type = &mut ffi_type_void as *mut ffi_type;
                            (arg_type, RsArgsValue::Void(()))
                        }
                    }
                }
                ValueType::Object => {
                    let params_type_object: JsObject = param.coerce_to_object().unwrap();
                    let arg_type = &mut ffi_type_pointer as *mut ffi_type;
                    let params_value_object = value.coerce_to_object().unwrap();
                    let index_map = jsobject_to_rs_struct(
                        params_type_object,
                        params_value_object,
                    );
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
                _ => {
                    ::core::panicking::panic_fmt(format_args!("unknow params type"));
                }
            }
        })
        .unzip();
    let mut func_type_map = HashMap::new();
    let mut func_value_map = HashMap::new();
    (&arg_values)
        .iter()
        .enumerate()
        .for_each(|(index, item)| match item {
            RsArgsValue::Function(func_desc, js_function) => {
                let func_desc_obj = func_desc
                    .call_without_args(None)
                    .unwrap()
                    .coerce_to_object()
                    .unwrap();
                let func_args_type: JsObject = func_desc_obj
                    .get_property(env.create_string("paramsType").unwrap())
                    .unwrap();
                func_type_map.insert(index, func_args_type);
            }
            _ => {}
        });
    let mut arg_values_c_void: Vec<*mut c_void> = arg_values
        .into_iter()
        .enumerate()
        .map(|(index, val)| {
            match val {
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
                RsArgsValue::Void(_) => Box::into_raw(Box::new(())) as *mut c_void,
                RsArgsValue::Function(_, js_function) => {
                    use libffi::high::*;
                    let func_args_type = func_type_map.get(&index).unwrap();
                    let args_len = func_args_type.get_array_length().unwrap();
                    func_value_map.insert(index, js_function);
                    let js_function = func_value_map.get(&index).unwrap();
                    let res = match args_len {
                        1 => {
                            let lambda = |a: *mut c_void| {
                                let arg_arr = [a];
                                let value: Vec<JsUnknown> = (0..1)
                                    .map(|index| {
                                        let c_param = arg_arr[index as usize];
                                        let arg_type = (func_args_type)
                                            .get_element::<JsUnknown>(index)
                                            .unwrap();
                                        let param = get_js_function_call_value(
                                            &env,
                                            arg_type,
                                            c_param,
                                        );
                                        param
                                    })
                                    .collect();
                                (&js_function).call(None, &value).unwrap();
                            };
                            let closure = Box::into_raw(
                                Box::new(Closure1::new(&lambda)),
                            );
                            return std::mem::transmute((*closure).code_ptr());
                        }
                        2 => {
                            let lambda = |a: *mut c_void, b: *mut c_void| {
                                let arg_arr = [a, b];
                                let value: Vec<JsUnknown> = (0..2)
                                    .map(|index| {
                                        let c_param = arg_arr[index as usize];
                                        let arg_type = (func_args_type)
                                            .get_element::<JsUnknown>(index)
                                            .unwrap();
                                        let param = get_js_function_call_value(
                                            &env,
                                            arg_type,
                                            c_param,
                                        );
                                        param
                                    })
                                    .collect();
                                (&js_function).call(None, &value).unwrap();
                            };
                            let closure = Box::into_raw(
                                Box::new(Closure2::new(&lambda)),
                            );
                            return std::mem::transmute((*closure).code_ptr());
                        }
                        3 => {
                            let lambda = |
                                a: *mut c_void,
                                b: *mut c_void,
                                c: *mut c_void|
                            {
                                let arg_arr = [a, b, c];
                                let value: Vec<JsUnknown> = (0..3)
                                    .map(|index| {
                                        let c_param = arg_arr[index as usize];
                                        let arg_type = (func_args_type)
                                            .get_element::<JsUnknown>(index)
                                            .unwrap();
                                        let param = get_js_function_call_value(
                                            &env,
                                            arg_type,
                                            c_param,
                                        );
                                        param
                                    })
                                    .collect();
                                (&js_function).call(None, &value).unwrap();
                            };
                            let closure = Box::into_raw(
                                Box::new(Closure3::new(&lambda)),
                            );
                            return std::mem::transmute((*closure).code_ptr());
                        }
                        4 => {
                            let lambda = |
                                a: *mut c_void,
                                b: *mut c_void,
                                c: *mut c_void,
                                d: *mut c_void|
                            {
                                let arg_arr = [a, b, c, d];
                                let value: Vec<JsUnknown> = (0..4)
                                    .map(|index| {
                                        let c_param = arg_arr[index as usize];
                                        let arg_type = (func_args_type)
                                            .get_element::<JsUnknown>(index)
                                            .unwrap();
                                        let param = get_js_function_call_value(
                                            &env,
                                            arg_type,
                                            c_param,
                                        );
                                        param
                                    })
                                    .collect();
                                (&js_function).call(None, &value).unwrap();
                            };
                            let closure = Box::into_raw(
                                Box::new(Closure4::new(&lambda)),
                            );
                            return std::mem::transmute((*closure).code_ptr());
                        }
                        5 => {
                            let lambda = |
                                a: *mut c_void,
                                b: *mut c_void,
                                c: *mut c_void,
                                d: *mut c_void,
                                e: *mut c_void|
                            {
                                let arg_arr = [a, b, c, d, e];
                                let value: Vec<JsUnknown> = (0..5)
                                    .map(|index| {
                                        let c_param = arg_arr[index as usize];
                                        let arg_type = (func_args_type)
                                            .get_element::<JsUnknown>(index)
                                            .unwrap();
                                        let param = get_js_function_call_value(
                                            &env,
                                            arg_type,
                                            c_param,
                                        );
                                        param
                                    })
                                    .collect();
                                (&js_function).call(None, &value).unwrap();
                            };
                            let closure = Box::into_raw(
                                Box::new(Closure5::new(&lambda)),
                            );
                            return std::mem::transmute((*closure).code_ptr());
                        }
                        6 => {
                            let lambda = |
                                a: *mut c_void,
                                b: *mut c_void,
                                c: *mut c_void,
                                d: *mut c_void,
                                e: *mut c_void,
                                f: *mut c_void|
                            {
                                let arg_arr = [a, b, c, d, e, f];
                                let value: Vec<JsUnknown> = (0..6)
                                    .map(|index| {
                                        let c_param = arg_arr[index as usize];
                                        let arg_type = (func_args_type)
                                            .get_element::<JsUnknown>(index)
                                            .unwrap();
                                        let param = get_js_function_call_value(
                                            &env,
                                            arg_type,
                                            c_param,
                                        );
                                        param
                                    })
                                    .collect();
                                (&js_function).call(None, &value).unwrap();
                            };
                            let closure = Box::into_raw(
                                Box::new(Closure6::new(&lambda)),
                            );
                            return std::mem::transmute((*closure).code_ptr());
                        }
                        7 => {
                            let lambda = |
                                a: *mut c_void,
                                b: *mut c_void,
                                c: *mut c_void,
                                d: *mut c_void,
                                e: *mut c_void,
                                f: *mut c_void,
                                g: *mut c_void|
                            {
                                let arg_arr = [a, b, c, d, e, f, g];
                                let value: Vec<JsUnknown> = (0..7)
                                    .map(|index| {
                                        let c_param = arg_arr[index as usize];
                                        let arg_type = (func_args_type)
                                            .get_element::<JsUnknown>(index)
                                            .unwrap();
                                        let param = get_js_function_call_value(
                                            &env,
                                            arg_type,
                                            c_param,
                                        );
                                        param
                                    })
                                    .collect();
                                (&js_function).call(None, &value).unwrap();
                            };
                            let closure = Box::into_raw(
                                Box::new(Closure7::new(&lambda)),
                            );
                            return std::mem::transmute((*closure).code_ptr());
                        }
                        8 => {
                            let lambda = |
                                a: *mut c_void,
                                b: *mut c_void,
                                c: *mut c_void,
                                d: *mut c_void,
                                e: *mut c_void,
                                f: *mut c_void,
                                g: *mut c_void,
                                h: *mut c_void|
                            {
                                let arg_arr = [a, b, c, d, e, f, g, h];
                                let value: Vec<JsUnknown> = (0..8)
                                    .map(|index| {
                                        let c_param = arg_arr[index as usize];
                                        let arg_type = (func_args_type)
                                            .get_element::<JsUnknown>(index)
                                            .unwrap();
                                        let param = get_js_function_call_value(
                                            &env,
                                            arg_type,
                                            c_param,
                                        );
                                        param
                                    })
                                    .collect();
                                (&js_function).call(None, &value).unwrap();
                            };
                            let closure = Box::into_raw(
                                Box::new(Closure8::new(&lambda)),
                            );
                            return std::mem::transmute((*closure).code_ptr());
                        }
                        9 => {
                            let lambda = |
                                a: *mut c_void,
                                b: *mut c_void,
                                c: *mut c_void,
                                d: *mut c_void,
                                e: *mut c_void,
                                f: *mut c_void,
                                g: *mut c_void,
                                h: *mut c_void,
                                i: *mut c_void|
                            {
                                let arg_arr = [a, b, c, d, e, f, g, h, i];
                                let value: Vec<JsUnknown> = (0..9)
                                    .map(|index| {
                                        let c_param = arg_arr[index as usize];
                                        let arg_type = (func_args_type)
                                            .get_element::<JsUnknown>(index)
                                            .unwrap();
                                        let param = get_js_function_call_value(
                                            &env,
                                            arg_type,
                                            c_param,
                                        );
                                        param
                                    })
                                    .collect();
                                (&js_function).call(None, &value).unwrap();
                            };
                            let closure = Box::into_raw(
                                Box::new(Closure9::new(&lambda)),
                            );
                            return std::mem::transmute((*closure).code_ptr());
                        }
                        _ => std::ptr::null_mut() as *mut c_void,
                    };
                    return res;
                }
                RsArgsValue::Object(val) => {
                    let (size, _) = calculate_layout(&val);
                    let layout = if size > 0 {
                        let (_, first_field) = val.get_index(0).unwrap();
                        let (_, align) = get_rs_value_size_align(first_field);
                        Layout::from_size_align(size, align).unwrap()
                    } else {
                        Layout::new::<i32>()
                    };
                    let ptr = alloc(layout) as *mut c_void;
                    let field_ptr = ptr;
                    unsafe fn write_data(
                        map: IndexMap<String, RsArgsValue>,
                        mut field_ptr: *mut c_void,
                    ) {
                        let mut offset = 0;
                        for (_, field_val) in map {
                            match field_val {
                                RsArgsValue::I32(number) => {
                                    let align = std::mem::align_of::<c_int>();
                                    let padding = (align - (offset % align)) % align;
                                    field_ptr = field_ptr.offset(padding as isize);
                                    (field_ptr as *mut c_int).write(number);
                                    offset = std::mem::size_of::<c_int>();
                                }
                                RsArgsValue::Double(double_number) => {
                                    let align = std::mem::align_of::<c_double>();
                                    let padding = (align - (offset % align)) % align;
                                    field_ptr = field_ptr.offset(padding as isize);
                                    (field_ptr as *mut c_double).write(double_number);
                                    offset = std::mem::size_of::<c_double>();
                                }
                                RsArgsValue::Boolean(val) => {
                                    let align = std::mem::align_of::<bool>();
                                    let padding = (align - (offset % align)) % align;
                                    field_ptr = field_ptr.offset(padding as isize);
                                    (field_ptr as *mut bool).write(val);
                                    offset = std::mem::size_of::<bool>();
                                }
                                RsArgsValue::String(str) => {
                                    let align = std::mem::align_of::<*const c_char>();
                                    let padding = (align - (offset % align)) % align;
                                    field_ptr = field_ptr.offset(padding as isize);
                                    let c_string = CString::new(str).unwrap();
                                    (field_ptr as *mut *const c_char).write(c_string.as_ptr());
                                    std::mem::forget(c_string);
                                    offset = std::mem::size_of::<*const c_char>();
                                }
                                RsArgsValue::StringArray(str_arr) => {
                                    let align = std::mem::align_of::<*const *const c_char>();
                                    let padding = (align - (offset % align)) % align;
                                    field_ptr = field_ptr.offset(padding as isize);
                                    let c_char_vec: Vec<*const c_char> = str_arr
                                        .into_iter()
                                        .map(|str| {
                                            let c_string = CString::new(str).unwrap();
                                            let ptr = c_string.as_ptr();
                                            std::mem::forget(c_string);
                                            return ptr;
                                        })
                                        .collect();
                                    (field_ptr as *mut *const *const c_char)
                                        .write(c_char_vec.as_ptr());
                                    std::mem::forget(c_char_vec);
                                    offset = std::mem::size_of::<*const *const c_char>();
                                }
                                RsArgsValue::DoubleArray(arr) => {
                                    let align = std::mem::align_of::<*const c_double>();
                                    let padding = (align - (offset % align)) % align;
                                    field_ptr = field_ptr.offset(padding as isize);
                                    (field_ptr as *mut *const c_double).write(arr.as_ptr());
                                    std::mem::forget(arr);
                                    offset = std::mem::size_of::<*const c_double>();
                                }
                                RsArgsValue::I32Array(arr) => {
                                    let align = std::mem::align_of::<*const c_int>();
                                    let padding = (align - (offset % align)) % align;
                                    field_ptr = field_ptr.offset(padding as isize);
                                    (field_ptr as *mut *const c_int).write(arr.as_ptr());
                                    std::mem::forget(arr);
                                    offset = std::mem::size_of::<*const c_int>();
                                }
                                RsArgsValue::Object(val) => {
                                    let (size, _) = calculate_layout(&val);
                                    let align = std::mem::align_of::<usize>();
                                    let padding = (align - (offset % align)) % align;
                                    field_ptr = field_ptr.offset(padding as isize);
                                    write_data(val, field_ptr);
                                    offset = size;
                                }
                                _ => {
                                    ::core::panicking::panic_fmt(format_args!("write_data"));
                                }
                            }
                            field_ptr = field_ptr.offset(offset as isize);
                        }
                    }
                    write_data(val, field_ptr);
                    return Box::into_raw(Box::new(ptr)) as *mut c_void;
                }
            }
        })
        .collect();
    let ret_value_type = ret_type.get_type().unwrap();
    let ret_value: FFIJsValue = match ret_value_type {
        ValueType::Number => {
            FFIJsValue::I32(ret_type.coerce_to_number().unwrap().try_into().unwrap())
        }
        ValueType::Object => FFIJsValue::JsObject(ret_type.coerce_to_object().unwrap()),
        _ => FFIJsValue::Unknown,
    };
    let r_type: *mut ffi_type = match ret_value {
        FFIJsValue::I32(number) => {
            let ret_data_type = number_to_data_type(number);
            match ret_data_type {
                DataType::I32 => &mut ffi_type_sint32 as *mut ffi_type,
                DataType::String => &mut ffi_type_pointer as *mut ffi_type,
                DataType::Void => &mut ffi_type_void as *mut ffi_type,
                DataType::Double => &mut ffi_type_double as *mut ffi_type,
                DataType::I32Array => &mut ffi_type_pointer as *mut ffi_type,
                DataType::StringArray => &mut ffi_type_pointer as *mut ffi_type,
                DataType::DoubleArray => &mut ffi_type_pointer as *mut ffi_type,
                DataType::Boolean => &mut ffi_type_uint8 as *mut ffi_type,
            }
        }
        FFIJsValue::JsObject(_) => &mut ffi_type_pointer as *mut ffi_type,
        FFIJsValue::Unknown => &mut ffi_type_void as *mut ffi_type,
    };
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
    match ret_value {
        FFIJsValue::I32(number) => {
            let ret_data_type = number_to_data_type(number);
            match ret_data_type {
                DataType::String => {
                    let mut result: *mut c_char = malloc(
                        std::mem::size_of::<*mut c_char>(),
                    ) as *mut c_char;
                    ffi_call(
                        &mut cif,
                        Some(*func),
                        &mut result as *mut *mut c_char as *mut c_void,
                        arg_values_c_void.as_mut_ptr(),
                    );
                    let result_str = CString::from_raw(result)
                        .into_string()
                        .expect(
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!("{0} retType is not string", func_name),
                                );
                                res
                            }
                                .as_str(),
                        );
                    Either9::A(result_str)
                }
                DataType::I32 => {
                    let mut result: i32 = 0;
                    ffi_call(
                        &mut cif,
                        Some(*func),
                        &mut result as *mut i32 as *mut c_void,
                        arg_values_c_void.as_mut_ptr(),
                    );
                    Either9::B(result)
                }
                DataType::Void => {
                    let mut result = ();
                    ffi_call(
                        &mut cif,
                        Some(*func),
                        &mut result as *mut () as *mut c_void,
                        arg_values_c_void.as_mut_ptr(),
                    );
                    Either9::C(())
                }
                DataType::Double => {
                    let mut result: f64 = 0.0;
                    ffi_call(
                        &mut cif,
                        Some(*func),
                        &mut result as *mut f64 as *mut c_void,
                        arg_values_c_void.as_mut_ptr(),
                    );
                    Either9::D(result)
                }
                DataType::Boolean => {
                    let mut result: bool = false;
                    ffi_call(
                        &mut cif,
                        Some(*func),
                        &mut result as *mut bool as *mut c_void,
                        arg_values_c_void.as_mut_ptr(),
                    );
                    Either9::H(result)
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "{0:?} is not currently avaiable as a return type",
                            ret_data_type,
                        ),
                    );
                }
            }
        }
        FFIJsValue::JsObject(ret_object) => {
            let ffi_tag = ret_object.has_named_property("ffiTypeTag").unwrap();
            if ffi_tag {
                let ffi_tag: &str = &js_string_to_string(
                    ret_object.get_named_property::<JsString>("ffiTypeTag").unwrap(),
                );
                match ffi_tag {
                    "array" => {
                        let array_len: usize = js_nunmber_to_i32(
                            ret_object.get_named_property::<JsNumber>("length").unwrap(),
                        ) as usize;
                        let array_type: i32 = js_nunmber_to_i32(
                            ret_object.get_named_property::<JsNumber>("type").unwrap(),
                        );
                        let array_type = number_to_data_type(array_type);
                        match array_type {
                            DataType::I32Array => {
                                let mut result: *mut c_int = malloc(
                                    std::mem::size_of::<*mut c_int>(),
                                ) as *mut c_int;
                                ffi_call(
                                    &mut cif,
                                    Some(*func),
                                    &mut result as *mut _ as *mut c_void,
                                    arg_values_c_void.as_mut_ptr(),
                                );
                                let arr = create_array_from_pointer(result, array_len);
                                if !result.is_null() {
                                    libc::free(result as *mut c_void);
                                }
                                Either9::E(arr)
                            }
                            DataType::DoubleArray => {
                                let mut result: *mut c_double = malloc(
                                    std::mem::size_of::<*mut c_double>(),
                                ) as *mut c_double;
                                ffi_call(
                                    &mut cif,
                                    Some(*func),
                                    &mut result as *mut _ as *mut c_void,
                                    arg_values_c_void.as_mut_ptr(),
                                );
                                let arr = create_array_from_pointer(result, array_len);
                                if !result.is_null() {
                                    libc::free(result as *mut c_void);
                                }
                                Either9::G(arr)
                            }
                            DataType::StringArray => {
                                let mut result: *mut *mut c_char = malloc(
                                    std::mem::size_of::<*mut *mut c_char>(),
                                ) as *mut *mut c_char;
                                ffi_call(
                                    &mut cif,
                                    Some(*func),
                                    &mut result as *mut _ as *mut c_void,
                                    arg_values_c_void.as_mut_ptr(),
                                );
                                let arr = create_array_from_pointer(result, array_len);
                                if !result.is_null() {
                                    libc::free(result as *mut c_void);
                                }
                                Either9::F(arr)
                            }
                            _ => {
                                ::core::panicking::panic_fmt(format_args!("some error"));
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic_fmt(format_args!("some error"));
                    }
                }
            } else {
                let ret_fields_size = JsObject::keys(&ret_object)
                    .unwrap()
                    .into_iter()
                    .fold(
                        0,
                        |pre, current| {
                            let size = pre;
                            let val: JsUnknown = ret_object
                                .get_named_property(&current)
                                .unwrap();
                            let data_type = js_unknown_to_data_type(val);
                            let (field_size, _) = get_data_type_size_align(data_type);
                            size + field_size
                        },
                    );
                let mut result: *mut c_void = malloc(ret_fields_size);
                ffi_call(
                    &mut cif,
                    Some(*func),
                    &mut result as *mut _ as *mut c_void,
                    arg_values_c_void.as_mut_ptr(),
                );
                let js_object = create_object_from_pointer(&env, result, ret_object);
                Either9::I(js_object)
            }
        }
        FFIJsValue::Unknown => Either9::C(()),
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(clippy::all)]
extern "C" fn __napi__load(
    env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
        napi::bindgen_prelude::CallbackInfo::<2usize>::new(env, cb, None)
            .and_then(|mut cb| {
                let arg0 = {
                    <FFIParams as napi::bindgen_prelude::FromNapiValue>::from_napi_value(
                        env,
                        cb.get_arg(0usize),
                    )?
                };
                napi::bindgen_prelude::within_runtime_if_available(move || {
                    let _ret = { load(napi::bindgen_prelude::Env::from(env), arg0) };
                    <Either9<
                        String,
                        i32,
                        (),
                        f64,
                        Vec<i32>,
                        Vec<String>,
                        Vec<f64>,
                        bool,
                        JsObject,
                    > as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, _ret)
                })
            })
            .unwrap_or_else(|e| {
                napi::bindgen_prelude::JsError::from(e).throw_into(env);
                std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
            })
    }
}
#[allow(non_snake_case)]
#[allow(clippy::all)]
unsafe fn load_js_function(
    env: napi::bindgen_prelude::sys::napi_env,
) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
    let mut fn_ptr = std::ptr::null_mut();
    {
        let c = napi::bindgen_prelude::sys::napi_create_function(
            env,
            "load\0".as_ptr().cast(),
            4usize,
            Some(__napi__load),
            std::ptr::null_mut(),
            &mut fn_ptr,
        );
        match c {
            ::napi::sys::Status::napi_ok => Ok(()),
            _ => {
                Err(
                    ::napi::Error::new(
                        ::napi::Status::from(c),
                        {
                            let res = ::alloc::fmt::format(
                                format_args!("Failed to register function `{0}`", "load"),
                            );
                            res
                        },
                    ),
                )
            }
        }
    }?;
    napi::bindgen_prelude::register_js_function(
        "load\0",
        load_js_function,
        Some(__napi__load),
    );
    Ok(fn_ptr)
}
#[allow(clippy::all)]
#[allow(non_snake_case)]
#[cfg(all(not(test), not(feature = "noop"), not(target_arch = "wasm32")))]
extern fn __napi_register__load_3() {
    napi::bindgen_prelude::register_module_export(None, "load\0", load_js_function);
}
#[used]
#[allow(non_upper_case_globals)]
#[doc(hidden)]
#[link_section = "__DATA,__mod_init_func"]
static __napi_register__load_3___rust_ctor___ctor: unsafe extern "C" fn() = {
    unsafe extern "C" fn __napi_register__load_3___rust_ctor___ctor() {
        __napi_register__load_3()
    }
    __napi_register__load_3___rust_ctor___ctor
};
