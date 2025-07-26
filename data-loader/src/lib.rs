use wasm_bindgen::prelude::*;
// use std::collections::HashMap;

fn get_records(filename: &str, content: &str, name_prefix: &str) -> Vec<String> {
  let mut lines: Vec<String> = Vec::new();
  let name: &str;
  for line in content.lines() {
    lines.push(line.to_string());
  }
  return lines;
}

/* TODO)) This function will also accept a third argument callled "records" in the future which will be a HashMap<str, Record>.
 * The Record struct will be created later along with the relevant impl. */  
#[wasm_bindgen]
pub fn parse_info(filename: &str, content: &str) -> Vec<String> {
  return get_records(filename, content, "SF");
}
