use chrono::{Duration, Local, NaiveDate};
use duration_str;
use std::io;

#[derive(Clone)]
enum RepeatType {
    FromCompleted,
    FromDue,
}

#[derive(Clone)]
struct Todo {
    completed: Option<NaiveDate>,
    title: String,
    due: Option<NaiveDate>,
    start: Option<NaiveDate>,
    repeat: Option<Duration>,
    repeat_type: RepeatType,
    tags: Vec<String>,
    sub_tasks: Vec<Todo>,
    dependencies: Vec<Todo>,
}

impl Todo {
    fn from_title(title: String) -> Todo {
        return Todo {
            completed: None,
            title,
            due: None,
            start: None,
            repeat: None,
            repeat_type: RepeatType::FromCompleted,
            tags: Vec::new(),
            sub_tasks: Vec::new(),
            dependencies: Vec::new(),
        };
    }

    fn set_due_ymd(&mut self, year: i32, month: u32, day: u32) {
        self.due = NaiveDate::from_ymd_opt(year, month, day)
    }

    fn set_due_iso8601(&mut self, s: String) {
        self.due = match NaiveDate::parse_from_str(s.as_str(), "%Y-%m-%d") {
            Ok(e) => Some(e),
            Err(_) => None,
        };
    }

    fn set_repeat(&mut self, rule: String) {
        self.repeat = match duration_str::parse_std(rule) {
            Ok(d) => match Duration::from_std(d) {
                Ok(d) => Some(d),
                Err(error) => panic!("Could not parse duration {}", error),
            },
            Err(error) => panic!("Could not parse duration {}", error),
        };
    }

    fn set_repeat_type(&mut self, from_completed: bool) {
        self.repeat_type = match from_completed {
            true => RepeatType::FromCompleted,
            false => RepeatType::FromDue,
        };
    }

    fn complete(&mut self) -> Option<Todo> {
        let mut t = self.clone();
        let dt = Local::now();
        let d = dt.date_naive();
        self.completed = Some(d);

        return match (self.due, self.repeat, &self.repeat_type) {
            (_, None, _) => None,
            (Some(due), Some(e), RepeatType::FromDue) => {
                t.due = Some(due + e);
                Some(t)
            }
            (_, Some(e), _) => {
                t.due = Some(d + e);
                Some(t)
            }
        };
    }
}

impl ToString for Todo {
    fn to_string(&self) -> String {
        let mut s = format!("{}", self.title);
        s = match self.due {
            Some(e) => format!("{} 📅 {}", s, e.to_string()),
            None => s,
        };
        s = match self.repeat {
            Some(e) => format!("{} 🔁 {}", s, format!("{} days", e.num_days())),
            None => s,
        };
        s = match (self.due, self.repeat, &self.repeat_type) {
            (Some(_), Some(_), RepeatType::FromCompleted) => format!("{} {}", s, "after completed"),
            (_, _, _) => s,
        };
        s = match self.completed {
            Some(e) => format!("[x] {} ✅ {}", s, e.to_string()),
            None => format!("[ ] {}", s),
        };

        return s;
    }
}

#[derive(Clone)]
struct TodoList {
    items: Vec<Todo>,
}

impl TodoList {
    fn new() -> TodoList {
        let v = Vec::new();
        return TodoList { items: v };
    }

    fn complete(&mut self, i: usize) {
        match self.items[i].complete() {
            Some(e) => self.add(e),
            None => (),
        };
    }

    fn set_due_ymd(&mut self, i: usize, y: i32, m: u32, d: u32) {
        self.items[i].set_due_ymd(y, m, d);
    }

    fn set_due_iso8601(&mut self, i: usize, s: String) {
        self.items[i].set_due_iso8601(s);
    }

    fn set_repeat(&mut self, i: usize, s: String) {
        self.items[i].set_repeat(s);
    }

    fn set_repeat_type(&mut self, i: usize, from_completed: bool) {
        self.items[i].set_repeat_type(from_completed);
    }

    fn add(&mut self, e: Todo) {
        self.items.push(e);
    }

    fn filter(&self, predicate: for<'a> fn(&'a Todo) -> bool) -> TodoList {
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

fn main() {
    let l = [
        Todo::from_title(String::from("Clean")),
        Todo::from_title(String::from("Laundry")),
        Todo::from_title(String::from("Cook")),
    ];

    let mut tdl = TodoList::from_iter(l);

    loop {
        for (i, e) in tdl.items.iter_mut().enumerate() {
            // Todo: add to_string() for todolist.
            e.set_repeat(String::from("4d"));
            println!("{i} {}", e.to_string());
        }

        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read line.");

        let (action, n) = match inp.find(' ') {
            Some(e) => (String::from(&inp[..e]), String::from(&inp[e + 1..])),
            None => (String::from(inp.trim()), String::from("")),
        };

        let n: Option<usize> = match n.trim().parse() {
            Ok(e) => Some(e),
            Err(_) => None,
        };

        match (action.as_str(), n) {
            ("c", Some(e)) => tdl.complete(e),
            ("d", Some(e)) => {
                println!("Enter yyyy-mm-dd: ");
                inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                inp = String::from(inp.trim());
                tdl.set_due_iso8601(e, inp)
            }
            ("r", Some(e)) => {
                println!("Enter yyyy-mm-dd: ");
                inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                inp = String::from(inp.trim());
                tdl.set_repeat(e, inp)
            }
            ("t", Some(e)) => println!("{} {}", "title", e),
            ("rt", Some(e)) => {
                println!("d: from due c: from completed");
                inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                inp = String::from(inp.trim());
                match inp.as_str() {
                    "c" => tdl.set_repeat_type(e, true),
                    "d" => tdl.set_repeat_type(e, false),
                    _ => continue,
                }
            }
            ("q", _) => break,
            (_, _) => continue,
        }
    }
}
