use wasm_bindgen::prelude::*;
use std::collections::HashMap;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

macro_rules! err {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )* ).into());
    }
}

#[derive(Debug)]
struct Line {
  prefix: String,
  content: String
}

#[derive(Debug)]
struct Record {
  name: String,
  lines: Vec<Line>
}

fn unify_source_path(content: &str) -> String {
  let mut unified_components: Vec<String> = Vec::new();

  for comp in content.split("/") {
    if comp == ".." && unified_components.len() > 0 { unified_components.pop(); }
    else if comp != "." && comp != "" { unified_components.push(comp.to_string()); }
  }

  return unified_components.join("/")
}

fn get_records(filename: &str, content: &str, name_prefix: &str) -> Vec<Record> {
  let mut records: Vec<Record> = Vec::new();
  let mut raw_lines = content.lines();
  while let Some(mut line) = raw_lines.next() {
    let mut lines: Vec<Line> = Vec::new();
    let mut name: String = String::new();
    log!("{line:?}");
    while line != "end_of_record" {
      if line.is_empty() || line.starts_with("#") {
        match raw_lines.next() {
         Some(next_line) => line = next_line,
         None => { break }
        }
        continue
      }
      let separator_index = match line.find(":") {
        Some(index) => index,
        None => {
          err!("The {filename:?} file seems malformed, encountered line without any colon other than \"end_of_record\": {line:?}");
          line.len()
        },
      };
      if separator_index == line.len() {
        match raw_lines.next() {
         Some(next_line) => line = next_line,
         None => { break }
        }
        continue
      }

      let (prefix, content_with_colon) = line.split_at(separator_index);
      let content = &content_with_colon[1..];
      if prefix == name_prefix {
        name = unify_source_path(content);
        match raw_lines.next() {
         Some(next_line) => line = next_line,
         None => { break }
        }
        continue;
      }

      lines.push(Line{prefix: prefix.to_string(), content: content.to_string()});

      match raw_lines.next() {
       Some(next_line) => line = next_line,
       None => { break }
      }
    }
    records.push(Record{name, lines});
  }
  return records;
}

struct SubGroup {
  value: u32
}

impl SubGroup {
  fn new() -> SubGroup {
    SubGroup{value: 0}
  }
}

struct Group {
  sub_groups: HashMap<String, SubGroup>
}

impl Group {
  fn get_sub_group(&mut self, name: &str) -> &SubGroup {
    if !self.sub_groups.contains_key(name) {
      self.sub_groups.insert(name.to_string(), SubGroup::new());
    }
    self.sub_groups.get(name).unwrap()
  }

  fn stats(&self) -> (u32, u32) {
    let (mut hits, mut total) = (0, 0);
    for group in self.sub_groups.values() {
      if group.value > 0 { hits += 1 }
      total += 1;
    }
    (hits, total)
  }
}

struct SourceLine {
  value: u32,
  groups: HashMap<String, Group>,
}

/* TODO)) This function will also accept a third argument callled "records" in the future which will be a HashMap<str, Record>.
 * The Record struct will be created later along with the relevant impl. */
#[wasm_bindgen]
pub fn parse_info(filename: &str, content: &str) -> Vec<String> {
  let mut records: Vec<String> = Vec::new();

  for record in get_records(filename, content, "SF") {

  }

  records
}
