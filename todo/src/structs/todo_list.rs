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

    pub fn from_json_file(s: &str) -> TodoList {
        let path = Path::new(s);
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
            Ok(_) => return TodoList::from_json(s.as_str()),
        }
    }

    pub fn to_json_file(&self, s: &str) {
        let path = Path::new(s);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(self.to_json().as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => return,
        }
    }

    pub fn from_markdown_file(s: &str) -> TodoList {
        let path = Path::new(s);
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
            Ok(_) => return TodoList::from_markdown(s.as_str()),
        }
    }

    pub fn to_markdown_file(&self, s: &str) {
        let path = Path::new(s);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(self.to_markdown().as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => return,
        }
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
        let iter = l.into_iter();
        let mut tdl = TodoList::new();

        for e in iter {
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
