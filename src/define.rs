use indexmap::IndexMap;
use libc::c_void;
use libffi_sys::{ffi_cif, ffi_type};
use napi::bindgen_prelude::{Error, Result, Status as NapiStatus};
use napi::{bindgen_prelude::*, JsBufferValue};
use napi::{Env, JsExternal, JsObject, JsUnknown};
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

pub enum FFIError {
  NapiError(Error<NapiStatus>),
  UnExpectedError,
  Panic(String),
  LibraryNotFound(String),
  FunctionNotFound(String),
  UnsupportedValueType(String),
}
impl AsRef<str> for FFIError {
  fn as_ref(&self) -> &str {
    match self {
      FFIError::UnExpectedError => "UnexpectedError",
      FFIError::NapiError(e) => e.status.as_ref(),
      FFIError::Panic(desc) => desc,
      FFIError::LibraryNotFound(desc) | FFIError::FunctionNotFound(desc) => desc,
      FFIError::UnsupportedValueType(desc) => desc,
    }
  }
}
impl From<FFIError> for Error {
  fn from(err: FFIError) -> Self {
    Error::new(napi::Status::Unknown, format!("{}", err.as_ref()))
  }
}

#[derive(Clone)]
pub struct NapiIndexMap<K, V>(IndexMap<K, V>);

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
  unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> Result<Self> {
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
  unsafe fn to_napi_value(raw_env: sys::napi_env, val: Self) -> Result<sys::napi_value> {
    let env = Env::from(raw_env);
    let mut obj = env.create_object()?;
    let map = val.0;
    for (k, v) in map.into_iter() {
      obj.set(k.as_ref(), v)?;
    }

    unsafe { JsObject::to_napi_value(raw_env, obj) }
  }
}
#[derive(Debug)]
pub struct FFIARRARYDESC {
  pub dynamic_array: bool,
  pub array_type: RefDataType,
  pub array_len: usize,
}

pub struct FFIFUNCDESC {
  pub need_free: bool,
}

#[napi]
#[derive(Debug)]
pub enum DataType {
  String = 0,
  I32 = 1,
  Double = 2,
  I32Array = 3,
  StringArray = 4,
  DoubleArray = 5,
  Boolean = 6,
  Void = 7,
  I64 = 8,
  U8 = 9,
  U8Array = 10,
  External = 11,
  U64 = 12,
  FloatArray = 13,
  Float = 14,
  WString = 15,
  BigInt = 16,
}

pub enum ReserveDataType {
  StackStruct = 999,
}
impl ReserveDataType {
  pub fn to_i32(&self) -> i32 {
    match self {
      ReserveDataType::StackStruct => 999,
    }
  }
}
#[derive(Debug)]
pub enum BasicDataType {
  String = 0,
  I32 = 1,
  Double = 2,
  Boolean = 6,
  Void = 7,
  I64 = 8,
  U8 = 9,
  External = 11,
  U64 = 12,
  Float = 14,
  WString = 15,
  BigInt = 16,
}

#[derive(Debug)]
pub enum RefDataType {
  I32Array = 3,
  StringArray = 4,
  DoubleArray = 5,
  U8Array = 10,
  FloatArray = 13,
}

pub trait ToDataType {
  fn to_data_type(self) -> DataType;
  fn to_basic_data_type(self) -> BasicDataType;
  fn to_ref_data_type(self) -> RefDataType;
}
impl ToDataType for i32 {
  fn to_data_type(self) -> DataType {
    match self {
      0 => DataType::String,
      1 => DataType::I32,
      2 => DataType::Double,
      3 => DataType::I32Array,
      4 => DataType::StringArray,
      5 => DataType::DoubleArray,
      6 => DataType::Boolean,
      7 => DataType::Void,
      8 => DataType::I64,
      9 => DataType::U8,
      10 => DataType::U8Array,
      11 => DataType::External,
      12 => DataType::U64,
      13 => DataType::FloatArray,
      14 => DataType::Float,
      15 => DataType::WString,
      16 => DataType::BigInt,
      _ => panic!("unknow DataType"),
    }
  }
  fn to_basic_data_type(self) -> BasicDataType {
    if is_array_type(&self) {
      panic!(
        "In the latest ffi-rs version, please use ffi-rs.arrayConstrutor to describe array type"
      )
    }
    match self {
      0 => BasicDataType::String,
      1 => BasicDataType::I32,
      2 => BasicDataType::Double,
      6 => BasicDataType::Boolean,
      7 => BasicDataType::Void,
      8 => BasicDataType::I64,
      9 => BasicDataType::U8,
      11 => BasicDataType::External,
      12 => BasicDataType::U64,
      14 => BasicDataType::Float,
      15 => BasicDataType::WString,
      16 => BasicDataType::BigInt,
      _ => panic!("unknow DataType"),
    }
  }
  fn to_ref_data_type(self) -> RefDataType {
    match self {
      3 => RefDataType::I32Array,
      4 => RefDataType::StringArray,
      5 => RefDataType::DoubleArray,
      10 => RefDataType::U8Array,
      13 => RefDataType::FloatArray,
      _ => panic!("unknow DataType"),
    }
  }
}
use libffi::middle::Type;

pub trait RsArgsTrait {
  fn to_ffi_type(&self) -> Type;
}
impl RsArgsTrait for RsArgsValue {
  fn to_ffi_type(&self) -> Type {
    match self {
      RsArgsValue::I32(number) => {
        let data_type = number.to_basic_data_type();
        match data_type {
          BasicDataType::String => Type::pointer(),
          BasicDataType::WString => Type::pointer(),
          BasicDataType::U8 | BasicDataType::Boolean => Type::u8(),
          BasicDataType::I32 => Type::i32(),
          BasicDataType::I64 | BasicDataType::BigInt => Type::i64(),
          BasicDataType::U64 => Type::u64(),
          BasicDataType::Float => Type::f32(),
          BasicDataType::Double => Type::f64(),
          BasicDataType::Void => Type::void(),
          BasicDataType::External => Type::pointer(),
        }
      }
      RsArgsValue::Object(_) => Type::pointer(),
      _ => panic!("parse function params type err {:?}", self),
    }
  }
}

pub fn is_array_type(value: &i32) -> bool {
  match value {
    3 | 4 | 5 | 10 | 13 => true,
    _ => false,
  }
}

pub enum RsArgsValue {
  String(String),
  WString(String),
  U8(u8),
  I32(i32),
  I64(i64),
  BigInt(i64),
  U64(u64),
  Float(f32),
  Double(f64),
  U8Array(Option<JsBufferValue>, Option<Vec<u8>>),
  I32Array(Vec<i32>),
  StringArray(Vec<String>),
  DoubleArray(Vec<f64>),
  FloatArray(Vec<f32>),
  Object(IndexMap<String, RsArgsValue>),
  Boolean(bool),
  Void(()),
  Function(IndexMap<String, RsArgsValue>, JsFunction),
  External(JsExternal),
}
impl Clone for RsArgsValue {
  fn clone(&self) -> Self {
    match self {
      RsArgsValue::String(s) => RsArgsValue::String(s.clone()),
      RsArgsValue::WString(s) => RsArgsValue::WString(s.clone()),
      RsArgsValue::U8(u) => RsArgsValue::U8(*u),
      RsArgsValue::I32(i) => RsArgsValue::I32(*i),
      RsArgsValue::I64(i) => RsArgsValue::I64(*i),
      RsArgsValue::BigInt(u) => RsArgsValue::BigInt(*u),
      RsArgsValue::U64(u) => RsArgsValue::U64(*u),
      RsArgsValue::Float(f) => RsArgsValue::Float(*f),
      RsArgsValue::Double(d) => RsArgsValue::Double(*d),
      RsArgsValue::I32Array(vec) => RsArgsValue::I32Array(vec.clone()),
      RsArgsValue::StringArray(vec) => RsArgsValue::StringArray(vec.clone()),
      RsArgsValue::DoubleArray(vec) => RsArgsValue::DoubleArray(vec.clone()),
      RsArgsValue::FloatArray(vec) => RsArgsValue::FloatArray(vec.clone()),
      RsArgsValue::Object(map) => RsArgsValue::Object(map.clone()),
      RsArgsValue::Boolean(b) => RsArgsValue::Boolean(*b),
      RsArgsValue::Void(()) => RsArgsValue::Void(()),
      RsArgsValue::U8Array(_, _) => panic!("U8Array is buffer cannot be cloned"),
      RsArgsValue::Function(_, _) => panic!("Function cannot be cloned"),
      RsArgsValue::External(_) => panic!("External cannot be cloned"),
    }
  }
}
use std::cmp::PartialEq;

impl PartialEq for RsArgsValue {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (RsArgsValue::String(a), RsArgsValue::String(b)) => a == b,
      (RsArgsValue::WString(a), RsArgsValue::WString(b)) => a == b,
      (RsArgsValue::U8(a), RsArgsValue::U8(b)) => a == b,
      (RsArgsValue::I32(a), RsArgsValue::I32(b)) => a == b,
      (RsArgsValue::I64(a), RsArgsValue::I64(b)) => a == b,
      (RsArgsValue::U64(a), RsArgsValue::U64(b)) => a == b,
      (RsArgsValue::BigInt(a), RsArgsValue::BigInt(b)) => a == b,
      (RsArgsValue::Float(a), RsArgsValue::Float(b)) => a == b,
      (RsArgsValue::Double(a), RsArgsValue::Double(b)) => a == b,
      (RsArgsValue::I32Array(a), RsArgsValue::I32Array(b)) => a == b,
      (RsArgsValue::StringArray(a), RsArgsValue::StringArray(b)) => a == b,
      (RsArgsValue::DoubleArray(a), RsArgsValue::DoubleArray(b)) => a == b,
      (RsArgsValue::FloatArray(a), RsArgsValue::FloatArray(b)) => a == b,
      (RsArgsValue::Object(a), RsArgsValue::Object(b)) => a == b,
      (RsArgsValue::Boolean(a), RsArgsValue::Boolean(b)) => a == b,
      (RsArgsValue::Void(a), RsArgsValue::Void(b)) => a == b,
      (RsArgsValue::U8Array(a1, a2), RsArgsValue::U8Array(b1, b2)) => false,
      (RsArgsValue::Function(..), _) | (_, RsArgsValue::Function(..)) => false,
      (RsArgsValue::External(..), _) | (_, RsArgsValue::External(..)) => false,
      _ => false,
    }
  }
}

impl Eq for RsArgsValue {}
unsafe impl Send for RsArgsValue {}
unsafe impl Sync for RsArgsValue {}

impl std::fmt::Debug for RsArgsValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      RsArgsValue::String(s) => write!(f, "String({})", s),
      RsArgsValue::WString(s) => write!(f, "WString({})", s),
      RsArgsValue::U8(i) => write!(f, "U8({})", i),
      RsArgsValue::I32(i) => write!(f, "I32({})", i),
      RsArgsValue::I64(i) => write!(f, "I64({})", i),
      RsArgsValue::U64(i) => write!(f, "U64({})", i),
      RsArgsValue::BigInt(i) => write!(f, "BigInt({})", i),
      RsArgsValue::Float(d) => write!(f, "Float({})", d),
      RsArgsValue::Double(d) => write!(f, "Double({})", d),
      RsArgsValue::U8Array(buffer, v) => {
        if buffer.is_some() {
          write!(f, "U8Array({:?})", buffer.as_ref().unwrap().as_ref())
        } else {
          write!(f, "U8Array({:?})", v.as_ref().unwrap())
        }
      }
      RsArgsValue::I32Array(arr) => write!(f, "I32Array({:?})", arr),
      RsArgsValue::StringArray(arr) => write!(f, "StringArray({:?})", arr),
      RsArgsValue::DoubleArray(arr) => write!(f, "DoubleArray({:?})", arr),
      RsArgsValue::FloatArray(arr) => write!(f, "FloatArray({:?})", arr),
      RsArgsValue::Object(obj) => write!(f, "Object({:?})", obj),
      RsArgsValue::Boolean(b) => write!(f, "Boolean({})", b),
      RsArgsValue::Void(_) => write!(f, "Void"),
      RsArgsValue::External(_) => write!(f, "JsExternal"),
      RsArgsValue::Function(_, _) => write!(f, "JsFunction"),
    }
  }
}
#[napi(object)]
pub struct FFIParams {
  pub library: String,
  pub func_name: String,
  pub ret_type: JsUnknown,
  pub params_type: Vec<JsUnknown>,
  pub params_value: Vec<JsUnknown>,
  pub errno: Option<bool>,
  pub run_in_new_thread: Option<bool>,
  pub free_result_memory: bool,
}

pub struct FFICALLPARAMS {
  pub cif: *mut ffi_cif,
  pub fn_pointer: unsafe extern "C" fn(),
  pub arg_values_c_void: Vec<*mut c_void>,
  pub ret_type_rs: RsArgsValue,
  pub errno: Option<bool>,
  pub free_result_memory: bool,
  pub params_type_rs: Rc<Vec<RsArgsValue>>,
  pub r_type_p: *mut *mut ffi_type,
  pub arg_types_p: *mut Vec<*mut ffi_type>,
}
pub struct BarePointerWrap(pub *mut c_void);
unsafe impl Send for FFICALL {}
unsafe impl Send for BarePointerWrap {}

pub struct FFICALL {
  pub data: FFICALLPARAMS,
}

impl FFICALL {
  pub fn new(data: FFICALLPARAMS) -> Self {
    Self { data }
  }
}

#[napi(object)]
pub struct CreatePointerParams {
  pub params_type: Vec<JsUnknown>,
  pub params_value: Vec<JsUnknown>,
}
#[derive(Debug)]
#[napi]
pub enum PointerType {
  RsPointer,
  CPointer,
}

#[napi(object)]
pub struct FreePointerParams {
  pub params_type: Vec<JsUnknown>,
  pub params_value: Vec<JsExternal>,
  pub pointer_type: PointerType,
}

#[napi(object)]
pub struct StorePointerParams {
  pub ret_type: Vec<JsUnknown>,
  pub params_value: Vec<JsExternal>,
}

#[napi(object)]
pub struct OpenParams {
  pub library: String,
  pub path: String,
}

pub const ARRAY_LENGTH_TAG: &str = "length";
pub const ARRAY_TYPE_TAG: &str = "type";
pub const ARRAY_DYNAMIC_TAG: &str = "dynamicArray";
pub const ARRAY_VALUE_TAG: &str = "value";

pub const FFI_TAG_FIELD: &str = "ffiTypeTag";
pub const ARRAY_FFI_TAG: &str = "array";
pub const FUNCTION_FFI_TAG: &str = "function";
pub const FUNCTION_FREE_TAG: &str = "needFree";

pub const PARAMS_TYPE: &str = "paramsType";
pub const RET_TYPE: &str = "retType";
pub const FREE_FUNCTION_TAG: &str = "freeCFuncParamsMemory";

pub static mut CLOSURE_MAP: Option<HashMap<*mut c_void, Vec<*mut c_void>>> = None;

#[derive(Debug)]
pub enum FFITag {
  Array,
  Function,
  Unknown,
}
