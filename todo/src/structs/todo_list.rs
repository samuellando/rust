use crate::Todo;
use serde::{Deserialize, Serialize};
use std::ops::Index;
use std::ops::IndexMut;

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
