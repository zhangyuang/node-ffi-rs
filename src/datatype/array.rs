use super::string::js_string_to_string;
use napi::bindgen_prelude::*;
use napi::{Error, JsNumber, JsObject, JsString, JsUnknown, NapiValue};
pub trait ToRsArray<T, U> {
  fn to_rs_array(self) -> Result<Vec<T>>
  where
    U: TryFrom<JsUnknown> + NapiValue;
}

impl ToRsArray<String, JsString> for JsObject {
  fn to_rs_array(self) -> Result<Vec<String>>
  where
    JsString: TryFrom<JsUnknown> + NapiValue,
  {
    (0..self.get_array_length()?)
      .enumerate()
      .map(|(index, _)| {
        let js_unknown: JsUnknown = self.get_element(index as u32)?;
        Ok(js_string_to_string(JsString::try_from(js_unknown)?)?)
      })
      .collect()
  }
}
fn convert_number_array<T, U>(obj: JsObject) -> Result<Vec<T>>
where
  U: TryFrom<JsUnknown, Error = Error> + NapiValue,
  T: TryFrom<U, Error = Error>,
{
  (0..obj.get_array_length()?)
    .enumerate()
    .map(|(index, _)| {
      let js_unknown: JsUnknown = obj.get_element(index as u32)?;
      let js_number: U = js_unknown.try_into()?;
      Ok(js_number.try_into()?)
    })
    .collect()
}
impl ToRsArray<f64, JsNumber> for JsObject {
  fn to_rs_array(self) -> Result<Vec<f64>>
  where
    JsNumber: TryFrom<JsUnknown> + NapiValue,
  {
    convert_number_array::<f64, JsNumber>(self)
  }
}

impl ToRsArray<i32, JsNumber> for JsObject {
  fn to_rs_array(self) -> Result<Vec<i32>>
  where
    JsNumber: TryFrom<JsUnknown> + NapiValue,
  {
    convert_number_array::<i32, JsNumber>(self)
  }
}

impl ToRsArray<i16, JsNumber> for JsObject {
  fn to_rs_array(self) -> Result<Vec<i16>>
  where
    JsNumber: TryFrom<JsUnknown> + NapiValue,
  {
    Ok(
      convert_number_array::<i32, JsNumber>(self)?
        .into_iter()
        .map(|item| item as i16)
        .collect(),
    )
  }
}

pub trait ToJsArray {
  fn to_js_array(self, env: &Env) -> Result<JsObject>;
}
impl ToJsArray for Vec<String> {
  fn to_js_array(self, env: &Env) -> Result<JsObject> {
    let mut js_array = env.create_array_with_length(self.len())?;
    let _ = self
      .into_iter()
      .enumerate()
      .try_for_each(|(index, str)| js_array.set_element(index as u32, env.create_string(&str)?));
    Ok(js_array)
  }
}
impl ToJsArray for Vec<f64> {
  fn to_js_array(self, env: &Env) -> Result<JsObject> {
    let mut js_array = env.create_array_with_length(self.len())?;
    let _ = self
      .into_iter()
      .enumerate()
      .try_for_each(|(index, item)| js_array.set_element(index as u32, env.create_double(item)?));
    Ok(js_array)
  }
}
impl ToJsArray for Vec<f32> {
  fn to_js_array(self, env: &Env) -> Result<JsObject> {
    let mut js_array = env.create_array_with_length(self.len())?;
    let _ = self.into_iter().enumerate().try_for_each(|(index, item)| {
      js_array.set_element(index as u32, env.create_double(item.into())?)
    });
    Ok(js_array)
  }
}

impl ToJsArray for Vec<i32> {
  fn to_js_array(self, env: &Env) -> Result<JsObject> {
    let mut js_array = env.create_array_with_length(self.len())?;
    let _ = self
      .into_iter()
      .enumerate()
      .try_for_each(|(index, item)| js_array.set_element(index as u32, env.create_int32(item)?));
    Ok(js_array)
  }
}
impl ToJsArray for Vec<i16> {
  fn to_js_array(self, env: &Env) -> Result<JsObject> {
    let mut js_array = env.create_array_with_length(self.len())?;
    let _ = self.into_iter().enumerate().try_for_each(|(index, item)| {
      js_array.set_element(index as u32, env.create_int32(item as i32)?)
    });
    Ok(js_array)
  }
}
impl ToJsArray for Vec<u8> {
  fn to_js_array(self, env: &Env) -> Result<JsObject> {
    let mut js_array = env.create_array_with_length(self.len())?;
    let _ = self.into_iter().enumerate().try_for_each(|(index, item)| {
      js_array.set_element(index as u32, env.create_uint32(item as u32)?)
    });
    Ok(js_array)
  }
}
