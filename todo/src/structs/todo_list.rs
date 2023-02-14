use crate::Todo;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Clone)]
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

    pub fn filter(&self, predicate: for<'a> fn(&'a Todo) -> bool) -> TodoList {
        let tdl = self.clone();
        return tdl.into_iter().filter(predicate).collect();
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
