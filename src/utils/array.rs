use crate::define::{
  RsArgsValue, ARRAY_LENGTH_TAG, ARRAY_STRUCT_ITEM_TYPE_TAG, ARRAY_TYPE_TAG, FFIARRARYDESC,
};
use indexmap::IndexMap;

pub fn get_array_desc(obj: &IndexMap<String, RsArgsValue>) -> FFIARRARYDESC {
  let (mut array_len, mut array_type, mut struct_item_type) = (0, 0, None);
  if let RsArgsValue::I32(number) = obj.get(ARRAY_LENGTH_TAG).unwrap() {
    array_len = *number as usize
  }
  if let RsArgsValue::I32(number) = obj.get(ARRAY_TYPE_TAG).unwrap() {
    array_type = *number
  }
  if let Some(RsArgsValue::Object(item_type)) = obj.get(ARRAY_STRUCT_ITEM_TYPE_TAG) {
    struct_item_type = Some(item_type.clone());
  }

  let array_type = array_type.try_into().unwrap();
  FFIARRARYDESC {
    array_len,
    array_type,
    struct_item_type,
  }
}
