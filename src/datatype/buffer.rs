use napi::{bindgen_prelude::*, Env, JsBuffer, JsBufferValue};

pub fn create_buffer_val(env: &Env, data: Vec<u8>) -> JsBufferValue {
  env.create_buffer_with_data(data).unwrap()
}
