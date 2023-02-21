mod obsidian;

use js_sys::JsString;
use todo::{Todo, TodoList};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ExampleCommand {
    id: JsString,
    name: JsString,
}

#[wasm_bindgen]
impl ExampleCommand {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> JsString {
        self.id.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_id(&mut self, id: &str) {
        self.id = JsString::from(id)
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> JsString {
        self.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: &str) {
        self.name = JsString::from(name)
    }

    pub fn callback(&self) {
        let mut tdl = TodoList::new();
        tdl.add(Todo::from_title(String::from("test1")));
        tdl.add(Todo::from_title(String::from("test2")));
        obsidian::Notice::new(tdl.to_string().as_str());
    }
}

#[wasm_bindgen]
pub fn onload(plugin: &obsidian::Plugin) {
    let cmd = ExampleCommand {
        id: JsString::from("todo"),
        name: JsString::from("todo"),
    };
    plugin.addCommand(JsValue::from(cmd))
}
