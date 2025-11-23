use super::utils::get_ffi_tag;
use indexmap::IndexMap;
use libc::c_void;
use libffi::middle::Closure;
use libffi_sys::{ffi_cif, ffi_type};
use napi::bindgen_prelude::{Error, Result, Status as NapiStatus};
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use napi::{bindgen_prelude::*, JsBufferValue};
use napi::{Env, JsExternal, JsObject, JsUnknown};
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use strum_macros::FromRepr;

type StandardResult<T, E> = std::result::Result<T, E>;

#[derive(Debug)]
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

pub struct TsFnCallContext<'a> {
  pub tsfn: ThreadsafeFunction<Vec<RsArgsValue>, ErrorStrategy::Fatal>,
  pub lambda: Option<Box<dyn Fn((Vec<*mut c_void>, *mut c_void)) + 'a>>,
  pub closure: Option<Closure<'a>>,
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
  pub array_type: RefDataType,
  pub array_len: usize,
  pub struct_item_type: Option<IndexMap<String, RsArgsValue>>,
}

pub struct FFIFUNCDESC {
  pub need_free: bool,
}

#[napi]
#[derive(Debug, FromRepr)]
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
  I16 = 17,
  StructArray = 18,
  I16Array = 19,
  U32 = 20,
}
#[derive(Debug, FromRepr)]
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
  I16 = 17,
  U32 = 20,
}

#[derive(Debug, FromRepr)]
pub enum RefDataType {
  I32Array = 3,
  StringArray = 4,
  DoubleArray = 5,
  U8Array = 10,
  FloatArray = 13,
  StructArray = 18,
  I16Array = 19,
}

impl TryFrom<i32> for DataType {
  type Error = FFIError;
  fn try_from(value: i32) -> StandardResult<Self, Self::Error> {
    DataType::from_repr(value as usize).ok_or(FFIError::UnsupportedValueType(format!(
      "Invalid DataType value: {}",
      value
    )))
  }
}

impl TryFrom<i32> for BasicDataType {
  type Error = FFIError;
  fn try_from(value: i32) -> StandardResult<Self, Self::Error> {
    if let Ok(_) = value.try_into() as StandardResult<RefDataType, FFIError> {
      return Err(FFIError::UnsupportedValueType(format!(
        "In the latest ffi-rs version, please use ffi-rs.arrayConstrutor to describe array type"
      )));
    }
    BasicDataType::from_repr(value as usize).ok_or(FFIError::UnsupportedValueType(format!(
      "Invalid BasicDataType value: {}",
      value
    )))
  }
}
impl TryFrom<i32> for RefDataType {
  type Error = FFIError;
  fn try_from(value: i32) -> StandardResult<Self, Self::Error> {
    RefDataType::from_repr(value as usize).ok_or(FFIError::UnsupportedValueType(format!(
      "Invalid RefDataType value: {}",
      value
    )))
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
        let data_type = (*number).try_into().unwrap();
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
          BasicDataType::I16 => Type::i16(),
          BasicDataType::U32 => Type::u32(),
        }
      }
      RsArgsValue::Object(obj_type) => {
        let is_stack_struct = get_ffi_tag(&obj_type) == FFITypeTag::StackStruct;
        if is_stack_struct {
          Type::structure(
            obj_type
              .iter()
              .filter(|(k, _)| k.as_str() != FFI_TAG_FIELD)
              .map(|(_, v)| v.to_ffi_type())
              .collect::<Vec<Type>>(),
          )
        } else {
          Type::pointer()
        }
      }
      _ => panic!("parse function params type err {:?}", self),
    }
  }
}

pub enum RsArgsValue {
  String(String),
  WString(String),
  U8(u8),
  I16(i16),
  I32(i32),
  I64(i64),
  BigInt(i64),
  U64(u64),
  U32(u32),
  Float(f32),
  Double(f64),
  U8Array(Option<JsBufferValue>, Option<Vec<u8>>),
  I16Array(Vec<i16>),
  I32Array(Vec<i32>),
  StringArray(Vec<String>),
  DoubleArray(Vec<f64>),
  FloatArray(Vec<f32>),
  StructArray(Vec<IndexMap<String, RsArgsValue>>),
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
      RsArgsValue::I16(i) => RsArgsValue::I16(*i),
      RsArgsValue::I32(i) => RsArgsValue::I32(*i),
      RsArgsValue::I64(i) => RsArgsValue::I64(*i),
      RsArgsValue::BigInt(u) => RsArgsValue::BigInt(*u),
      RsArgsValue::U64(u) => RsArgsValue::U64(*u),
      RsArgsValue::U32(u) => RsArgsValue::U32(*u),
      RsArgsValue::Float(f) => RsArgsValue::Float(*f),
      RsArgsValue::Double(d) => RsArgsValue::Double(*d),
      RsArgsValue::I16Array(vec) => RsArgsValue::I16Array(vec.clone()),
      RsArgsValue::I32Array(vec) => RsArgsValue::I32Array(vec.clone()),
      RsArgsValue::StringArray(vec) => RsArgsValue::StringArray(vec.clone()),
      RsArgsValue::DoubleArray(vec) => RsArgsValue::DoubleArray(vec.clone()),
      RsArgsValue::FloatArray(vec) => RsArgsValue::FloatArray(vec.clone()),
      RsArgsValue::StructArray(vec) => RsArgsValue::StructArray(vec.clone()),
      RsArgsValue::Object(map) => RsArgsValue::Object(map.clone()),
      RsArgsValue::Boolean(b) => RsArgsValue::Boolean(*b),
      RsArgsValue::Void(()) => RsArgsValue::Void(()),
      RsArgsValue::U8Array(_, _) => panic!("U8Array is buffer cannot be cloned"),
      RsArgsValue::Function(_, _) => panic!("Function cannot be cloned"),
      RsArgsValue::External(_) => panic!("External cannot be cloned"),
    }
  }
}

impl PartialEq for RsArgsValue {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (RsArgsValue::String(a), RsArgsValue::String(b)) => a == b,
      (RsArgsValue::WString(a), RsArgsValue::WString(b)) => a == b,
      (RsArgsValue::U8(a), RsArgsValue::U8(b)) => a == b,
      (RsArgsValue::I32(a), RsArgsValue::I32(b)) => a == b,
      (RsArgsValue::I64(a), RsArgsValue::I64(b)) => a == b,
      (RsArgsValue::U64(a), RsArgsValue::U64(b)) => a == b,
      (RsArgsValue::U32(a), RsArgsValue::U32(b)) => a == b,
      (RsArgsValue::BigInt(a), RsArgsValue::BigInt(b)) => a == b,
      (RsArgsValue::Float(a), RsArgsValue::Float(b)) => a == b,
      (RsArgsValue::Double(a), RsArgsValue::Double(b)) => a == b,
      (RsArgsValue::I16Array(a), RsArgsValue::I16Array(b)) => a == b,
      (RsArgsValue::I32Array(a), RsArgsValue::I32Array(b)) => a == b,
      (RsArgsValue::StringArray(a), RsArgsValue::StringArray(b)) => a == b,
      (RsArgsValue::DoubleArray(a), RsArgsValue::DoubleArray(b)) => a == b,
      (RsArgsValue::FloatArray(a), RsArgsValue::FloatArray(b)) => a == b,
      (RsArgsValue::Object(a), RsArgsValue::Object(b)) => a == b,
      (RsArgsValue::Boolean(a), RsArgsValue::Boolean(b)) => a == b,
      (RsArgsValue::Void(a), RsArgsValue::Void(b)) => a == b,
      (RsArgsValue::U8Array(_, _), RsArgsValue::U8Array(_, _)) => false,
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
      RsArgsValue::I16(i) => write!(f, "I16({})", i),
      RsArgsValue::I32(i) => write!(f, "I32({})", i),
      RsArgsValue::I64(i) => write!(f, "I64({})", i),
      RsArgsValue::U64(i) => write!(f, "U64({})", i),
      RsArgsValue::U32(i) => write!(f, "U32({})", i),
      RsArgsValue::BigInt(i) => write!(f, "BigInt({})", i),
      RsArgsValue::Float(d) => write!(f, "Float({})", d),
      RsArgsValue::Double(d) => write!(f, "Double({})", d),
      RsArgsValue::U8Array(buffer, v) => {
        write!(
          f,
          "U8Array({:?}, {:?})",
          buffer.as_ref().unwrap().as_ref(),
          v
        )
      }
      RsArgsValue::I16Array(arr) => write!(f, "I16Array({:?})", arr),
      RsArgsValue::I32Array(arr) => write!(f, "I32Array({:?})", arr),
      RsArgsValue::StringArray(arr) => write!(f, "StringArray({:?})", arr),
      RsArgsValue::DoubleArray(arr) => write!(f, "DoubleArray({:?})", arr),
      RsArgsValue::FloatArray(arr) => write!(f, "FloatArray({:?})", arr),
      RsArgsValue::StructArray(arr) => write!(f, "StructArray({:?})", arr),
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
  pub cif: ffi_cif,
  pub fn_pointer: unsafe extern "C" fn(),
  pub arg_types: Vec<*mut ffi_type>,
  pub arg_values_c_void: Vec<*mut c_void>,
  pub ret_type_rs: RsArgsValue,
  pub errno: Option<bool>,
  pub free_result_memory: bool,
  pub params_type_rs: Rc<Vec<RsArgsValue>>,
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
pub const ARRAY_STRUCT_ITEM_TYPE_TAG: &str = "structItemType";
pub const ARRAY_VALUE_TAG: &str = "value";

pub const FFI_TAG_FIELD: &str = "ffiTypeTag";
pub const FUNCTION_FREE_TAG: &str = "needFree";

pub const PARAMS_TYPE: &str = "paramsType";
pub const RET_TYPE: &str = "retType";
pub const FREE_FUNCTION_TAG: &str = "freeCFuncParamsMemory";

#[napi]
#[derive(PartialEq, Eq)]
pub enum FFITypeTag {
  Unknown = 0,
  StackArray = 996,
  Array = 997,
  Function = 998,
  StackStruct = 999,
}
impl From<FFITypeTag> for i32 {
  fn from(tag: FFITypeTag) -> i32 {
    match tag {
      FFITypeTag::Unknown => 0,
      FFITypeTag::StackArray => 996,
      FFITypeTag::Array => 997,
      FFITypeTag::Function => 998,
      FFITypeTag::StackStruct => 999,
    }
  }
}

pub static mut CLOSURE_MAP: Option<HashMap<*mut c_void, *mut c_void>> = None;
