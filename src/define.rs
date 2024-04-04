use indexmap::IndexMap;
use napi::bindgen_prelude::*;
use napi::{Env, JsObject};
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
  Function = 7,
  Void = 8,
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
    7 => DataType::Function,
    8 => DataType::Void,
    _ => panic!("unknow DataType"),
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
