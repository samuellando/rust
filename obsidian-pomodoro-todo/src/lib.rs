mod obsidian;

use std::collections::HashMap;
use todo::TodoList;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

const DATABASE: &str = "pomodoro-todo-db";

#[wasm_bindgen]
pub fn parse_to_db(path: String, content: String) {
    let window = web_sys::window().expect("no global `window` exists");
    let storage = match window.session_storage() {
        Ok(Some(e)) => e,
        Ok(None) | Err(_) => panic!("Cant get storage"),
    };

    let js = match storage.get_item(DATABASE) {
        Ok(e) => e,
        Err(_) => panic!("Cant read value"),
    };

    let mut db: HashMap<String, TodoList> = match js {
        Some(e) => match serde_json::from_str(e.as_str()) {
            Ok(e) => e,
            Err(e) => panic!("Couldn't convert from json. {}", e),
        },
        None => HashMap::new(),
    };

    let tdl = TodoList::from_mixed_markdown(content.as_str());

    db.insert(path, tdl);

    let s = match serde_json::to_string(&db) {
        Ok(e) => e,
        Err(_) => panic!("Couldn't convert to json."),
    };

    match storage.set_item(DATABASE, s.as_str()) {
        Ok(_) => return,
        Err(_) => panic!("Cant save"),
    };
}
