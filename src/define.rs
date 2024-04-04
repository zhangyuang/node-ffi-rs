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
