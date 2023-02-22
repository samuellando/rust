mod obsidian;

use js_sys::{Array, JsString};
use todo::{Todo, TodoList};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_all_markdown(a: Array) -> String {
    let v = a.to_vec();

    let mut contents: Vec<String> = Vec::new();
    for jsval in v {
        match jsval.as_string() {
            Some(e) => contents.push(e),
            None => return String::from("NONE"),
        }
    }

    let mut tdl = TodoList::new();
    for content in contents {
        let tdl2 = TodoList::from_mixed_markdown(content.as_str());
        for t in tdl2 {
            let t = t.clone();
            tdl.add(t);
        }
    }

    return tdl.to_markdown();
}

#[wasm_bindgen]
pub fn parse_mixed_md(s: String) -> String {
    let tdl = TodoList::from_mixed_markdown(s.as_str());

    return tdl.to_markdown();
}
