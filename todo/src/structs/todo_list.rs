use crate::Todo;

#[derive(Clone)]
pub struct TodoList {
    items: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> TodoList {
        let v = Vec::new();
        return TodoList { items: v };
    }

    pub fn complete(&mut self, i: usize) {
        match self.items[i].complete() {
            Some(e) => self.add(e),
            None => (),
        };
    }

    pub fn set_due_ymd(&mut self, i: usize, y: i32, m: u32, d: u32) {
        self.items[i].set_due_ymd(y, m, d);
    }

    pub fn set_title(&mut self, i: usize, t: String) {
        self.items[i].set_title(t);
    }

    pub fn set_due_iso8601(&mut self, i: usize, s: String) {
        self.items[i].set_due_iso8601(s);
    }

    pub fn set_repeat(&mut self, i: usize, s: String) {
        self.items[i].set_repeat(s);
    }

    pub fn set_repeat_type(&mut self, i: usize, from_completed: bool) {
        self.items[i].set_repeat_type(from_completed);
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
