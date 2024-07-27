use crate::define::*;
use napi::{Env, JsBufferValue};

pub fn create_buffer_val(env: &Env, data: Vec<u8>) -> JsBufferValue {
  env.create_buffer_with_data(data).unwrap()
}

pub fn get_safe_buffer(env: &Env, arr: Vec<u8>, need_thread_safe: bool) -> RsArgsValue {
  if need_thread_safe {
    RsArgsValue::U8Array(None, Some(arr))
  } else {
    RsArgsValue::U8Array(Some(create_buffer_val(env, arr)), None)
  }
}
