use napi::{Env, JsUnknown, NapiRaw, NapiValue};

pub unsafe fn create_js_value_unchecked<T: NapiValue>(env: &Env, js_known: JsUnknown) -> T {
  T::from_raw_unchecked(env.raw(), js_known.raw())
}
