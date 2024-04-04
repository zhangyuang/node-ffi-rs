use indexmap::IndexMap;
use napi::bindgen_prelude::*;
use napi::sys::napi_value__;
use napi::{Env, JsObject};
use std::hash::Hash;

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

// #[napi]
// pub enum RetType {
//   String,
//   I32,
//   Void,
//   Double,
//   I32Array,
//   StringArray,
//   DoubleArray,
//   Object,
// }
// #[derive(Debug)]
// pub enum RsArgsValue {
//   String(String),
//   I32(i32),
//   Double(f64),
//   I32Array(Vec<i32>),
//   StringArray(Vec<String>),
//   DoubleArray(Vec<f64>),
//   IndexMap(IndexMap<String, RsArgsValue>),
// }

// #[napi]
// pub enum ParamsType {
//   String,
//   I32,
//   Double,
//   I32Array,
//   StringArray,
//   DoubleArray,
//   Object,
// }

// #[napi(object)]
// struct FFIParams {
//   pub params_type: Vec<ParamsOrMap>,
// }

// impl FromNapiValue for ParamsOrMap {
//   unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> Result<Self> {
//     let obj = unsafe { JsObject::from_napi_value(env, napi_val)? };
//     let keys = JsObject::keys(&obj)?;

//     if keys.contains(&"params".to_string()) {
//       let params_val = obj.get("params")?;
//       let params = ParamsType::from_napi_value(env, &params_val as *mut napi_value__)?;
//       Ok(ParamsOrMap {
//         params: Some(params),
//         map: None,
//       })
//     } else if keys.contains(&"map".to_string()) {
//       let map_val = obj.get("map")?;
//       let map = NapiIndexMap::from_napi_value(env, map_val as *mut napi_value__)?;
//       Ok(ParamsOrMap {
//         params: None,
//         map: Some(map),
//       })
//     } else {
//       panic!()
//     }
//   }
// }

// impl ToNapiValue for ParamsOrMap {
//   unsafe fn to_napi_value(raw_env: sys::napi_env, val: Self) -> Result<sys::napi_value> {
//     let env = Env::from(raw_env);
//     let mut obj = env.create_object()?;

//     if let Some(params) = val.params {
//       obj.set("params", params)?;
//     } else if let Some(map) = val.map {
//       obj.set("map", map)?;
//     }

//     unsafe { JsObject::to_napi_value(raw_env, obj) }
//   }
// }

// #[napi]
// pub struct ParamsOrMap {
//   params: Option<ParamsType>,
//   map: Option<NapiIndexMap<String, ParamsType>>,
// }
