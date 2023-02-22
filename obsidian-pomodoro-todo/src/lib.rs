mod obsidian;

use js_sys::JsString;
use todo::{Todo, TodoList};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_mixed_md(s: String) -> String {
    let tdl = TodoList::from_mixed_markdown(s.as_str());

    return tdl.to_markdown();
}
