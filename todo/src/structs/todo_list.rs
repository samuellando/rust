use crate::Todo;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::ops::Index;
use std::ops::IndexMut;
use std::path::Path;

#[derive(Clone, Serialize, Deserialize)]
pub struct TodoList {
    items: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> TodoList {
        let v = Vec::new();
        return TodoList { items: v };
    }

    pub fn add(&mut self, e: Todo) {
        self.items.push(e);
    }

    pub fn remove(&mut self, i: usize) -> Todo {
        return self.items.remove(i);
    }

    pub fn filter(&self, predicate: for<'a> fn(&'a Todo) -> bool) -> TodoList {
        let tdl = self.clone();
        return tdl.into_iter().filter(predicate).collect();
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn to_json(&self) -> String {
        match serde_json::to_string_pretty(self) {
            Ok(e) => e,
            Err(_) => panic!("Couldn't convert to json."),
        }
    }

    pub fn from_json(s: &str) -> Self {
        match serde_json::from_str(s) {
            Ok(e) => e,
            Err(e) => panic!("Couldn't convert from json. {}", e),
        }
    }

    pub fn to_markdown(&self) -> String {
        let mut s = String::new();
        let tdl = self.clone();
        for (i, todo) in tdl.into_iter().enumerate() {
            if i > 0 {
                s.push_str("\n");
            }
            s.push_str(todo.to_markdown().as_str());
        }

        return s;
    }

    pub fn from_markdown(s: &str) -> Self {
        let mut tasks: Vec<&str> = s.split("\n- [").collect();
        tasks[0] = &tasks[0][3..];

        let mut tdl = TodoList::new();

        for task in tasks {
            let mut s = task.replace("x] ", "");
            s = s.replace(" ] ", "");
            tdl.add(Todo::from_markdown(s.as_str()));
        }

        return tdl;
    }

    pub fn from_mixed_markdown(s: &str) -> Self {
        let mut md = String::from("");
        let lines = s.split("\n");

        for line in lines {
            if line.trim().starts_with("- [ ]")
                || line.trim().starts_with("- [X]")
                || line.trim().starts_with("- Dependencies:")
                || line.trim().starts_with("- Sub Tasks:")
            {
                md += line;
                md += "\n";
            }
        }
        md = String::from(&md[0..md.len() - 1]);

        return Self::from_markdown(md.as_str());
    }

    fn _from_file(file_name: &str, f: fn(&str) -> Self) -> Self {
        let path = Path::new(file_name);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => return f(s.as_str()),
        }
    }

    pub fn from_json_file(s: &str) -> Self {
        TodoList::_from_file(s, TodoList::from_json)
    }

    pub fn from_markdown_file(s: &str) -> Self {
        TodoList::_from_file(s, TodoList::from_markdown)
    }

    pub fn from_mixed_markdown_file(s: &str) -> Self {
        TodoList::_from_file(s, TodoList::from_mixed_markdown)
    }

    fn _to_file<'a>(file_name: &str, s: String) {
        let path = Path::new(file_name);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(s.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => return,
        }
    }

    pub fn to_json_file(&self, s: &str) {
        TodoList::_to_file(s, self.to_json())
    }

    pub fn to_markdown_file(&self, s: &str) {
        TodoList::_to_file(s, self.to_markdown())
    }
}

impl ToString for TodoList {
    fn to_string(&self) -> String {
        self.to_markdown()
    }
}

impl IntoIterator for TodoList {
    type Item = Todo;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl FromIterator<Todo> for TodoList {
    fn from_iter<I: IntoIterator<Item = Todo>>(l: I) -> Self {
        let mut tdl = TodoList::new();

        for e in l.into_iter() {
            tdl.add(e);
        }
        return tdl;
    }
}

impl Index<usize> for TodoList {
    type Output = Todo;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.items[index];
    }
}

impl IndexMut<usize> for TodoList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.items[index];
    }
}
