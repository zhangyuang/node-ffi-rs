use indexmap::IndexMap;
use napi::bindgen_prelude::*;
use napi::{Env, JsObject, JsUnknown};
use std::hash::Hash;

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
}

#[derive(Debug)]
pub enum RefDataType {
  I32Array = 3,
  StringArray = 4,
  DoubleArray = 5,
}

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
    8 => DataType::I64,
    9 => DataType::U8,
    _ => panic!("unknow DataType"),
  }
}

pub fn number_to_basic_data_type(value: i32) -> BasicDataType {
  match value {
    0 => BasicDataType::String,
    1 => BasicDataType::I32,
    2 => BasicDataType::Double,
    6 => BasicDataType::Boolean,
    7 => BasicDataType::Void,
    8 => BasicDataType::I64,
    9 => BasicDataType::U8,
    _ => panic!("unknow DataType"),
  }
}
pub fn number_to_ref_data_type(value: i32) -> RefDataType {
  match value {
    3 => RefDataType::I32Array,
    4 => RefDataType::StringArray,
    5 => RefDataType::DoubleArray,
    _ => panic!("unknow DataType"),
  }
}

pub enum RsArgsValue {
  String(String),
  U8(u8),
  I32(i32),
  I64(i64),
  Double(f64),
  I32Array(Vec<i32>),
  StringArray(Vec<String>),
  DoubleArray(Vec<f64>),
  Object(IndexMap<String, RsArgsValue>),
  Boolean(bool),
  Void(()),
  Function(JsFunction, JsFunction),
}
impl std::fmt::Debug for RsArgsValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      RsArgsValue::String(s) => write!(f, "String({})", s),
      RsArgsValue::U8(i) => write!(f, "U8({})", i),
      RsArgsValue::I32(i) => write!(f, "I32({})", i),
      RsArgsValue::I64(i) => write!(f, "I64({})", i),
      RsArgsValue::Double(d) => write!(f, "Double({})", d),
      RsArgsValue::I32Array(arr) => write!(f, "I32Array({:?})", arr),
      RsArgsValue::StringArray(arr) => write!(f, "StringArray({:?})", arr),
      RsArgsValue::DoubleArray(arr) => write!(f, "DoubleArray({:?})", arr),
      RsArgsValue::Object(obj) => write!(f, "Object({:?})", obj),
      RsArgsValue::Boolean(b) => write!(f, "Boolean({})", b),
      RsArgsValue::Void(_) => write!(f, "Void"),
      RsArgsValue::Function(_, _) => write!(f, "Function(<JsFunction>, <JsFunction>)"),
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
}
#[napi(object)]
pub struct OpenParams {
  pub library: String,
  pub path: String,
}

pub const ARRAY_LENGTH_TAG: &str = "length";
pub const ARRAY_TYPE_TAG: &str = "type";
