mod anagram;

extern crate js_sys;

#[macro_use]
extern crate serde_derive;

use std::collections::HashSet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_anagram(phrase: &str, dictionary: &JsValue) -> JsValue {
  let dict: Vec<String> = dictionary.into_serde().unwrap();
  let dict_hash: HashSet<&str> = dict.iter().map(|w| &w[..]).collect();
  let anagrams = anagram::Phrase::new(phrase).get_anagrams(&dict_hash);
  JsValue::from_serde(&anagrams).unwrap()
}
