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
    repeat: Option<Duration>,
    repeat_type: RepeatType,
}

struct TodoList {
    items: Vec<Todo>,
}

impl Todo {
    fn new(
        title: String,
        due: Option<NaiveDate>,
        repeat: Option<Duration>,
        repeat_type: Option<RepeatType>,
    ) -> Todo {
        let rt = match repeat_type {
            Some(e) => e,
            None => RepeatType::FromCompleted,
        };
        Todo {
            completed: None,
            title,
            due,
            repeat,
            repeat_type: rt,
        }
    }

    fn from_title(title: String) -> Todo {
        return Todo {
            completed: None,
            title,
            due: None,
            repeat: None,
            repeat_type: RepeatType::FromCompleted,
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

    fn set_repeat(&mut self, rule: String, from_completed: bool) {
        self.repeat = match duration_str::parse_std(rule) {
            Ok(d) => match Duration::from_std(d) {
                Ok(d) => Some(d),
                Err(error) => panic!("Could not parse duration {}", error),
            },
            Err(error) => panic!("Could not parse duration {}", error),
        };
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

impl TodoList {
    fn new() -> TodoList {
        let v = Vec::new();
        return TodoList { items: v };
    }

    fn from_into_iter(l: impl IntoIterator<Item = String>) -> TodoList {
        let iter = l.into_iter();
        let mut tdl = TodoList::new();

        for e in iter {
            tdl.add(Todo::from_title(e));
        }
        return tdl;
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

    fn add(&mut self, e: Todo) {
        self.items.push(e);
    }
}

fn main() {
    let l = [
        String::from("Clean"),
        String::from("Laundry"),
        String::from("Cook"),
    ];

    let mut tdl = TodoList::from_into_iter(l);

    loop {
        for (i, e) in tdl.items.iter_mut().enumerate() {
            // Todo: add to_string() for todolist.
            e.set_repeat(String::from("4d"), false);
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
            // TODO: Add mode due date.
            ("c", Some(e)) => tdl.complete(e),
            ("d", Some(e)) => println!("{} {}", "due", e),
            ("r", Some(e)) => println!("{} {}", "repeat", e),
            ("t", Some(e)) => println!("{} {}", "title", e),
            ("rt", Some(e)) => println!("{} {}", "repeat type", e),
            ("q", _) => break,
            (_, _) => continue,
        }
    }
}
