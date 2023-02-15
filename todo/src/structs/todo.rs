use crate::TodoList;
use chrono::{offset::TimeZone, Duration, Local, LocalResult, NaiveDateTime};
use core::str::FromStr;
use core::time::Duration as StdDuration;
use cron::Schedule;
use duration_human::DurationHuman;
use std::cmp::max;

#[derive(Clone)]
enum Repeat {
    FromCompleted(Duration),
    FromDue(Duration),
    Every(Schedule),
}

impl ToString for Repeat {
    fn to_string(&self) -> String {
        match self {
            Repeat::FromCompleted(d) => match d.to_std() {
                Ok(d) => format!("{} after completed", DurationHuman::from(d)),
                Err(_) => panic!("Unexpected"),
            },
            Repeat::FromDue(d) => match d.to_std() {
                Ok(d) => format!("after {}", DurationHuman::from(d)),
                Err(_) => panic!("Unexpected"),
            },
            Repeat::Every(s) => format!("every {}", s.to_string()),
        }
    }
}

#[derive(Clone)]
pub struct Todo {
    completed: Option<NaiveDateTime>,
    title: String,
    due: Option<NaiveDateTime>,
    start: Option<NaiveDateTime>,
    repeat: Option<Repeat>,
    tags: Vec<String>,
    sub_tasks: TodoList,
    dependencies: TodoList,
    // For the next repetion.
    next_sub_tasks: TodoList,
    next_dependencies: TodoList,
    duration: Option<Duration>,
}

impl Todo {
    pub fn from_title(title: String) -> Todo {
        return Todo {
            completed: None,
            title,
            due: None,
            start: None,
            repeat: None,
            tags: Vec::new(),
            sub_tasks: TodoList::new(),
            dependencies: TodoList::new(),
            next_sub_tasks: TodoList::new(),
            next_dependencies: TodoList::new(),
            duration: None,
        };
    }

    pub fn set_title(&mut self, t: String) {
        self.title = t;
    }

    pub fn set_due_iso8601(&mut self, s: String) {
        let fmt = "%Y-%m-%d %H:%M:%S";
        self.due = match NaiveDateTime::parse_from_str(s.as_str(), fmt) {
            Ok(e) => Some(e),
            Err(_) => match NaiveDateTime::parse_from_str(&format!("{} 11:59:59", s), fmt) {
                Ok(e) => Some(e),
                Err(_) => None,
            },
        };
    }

    pub fn set_start_iso8601(&mut self, s: String) {
        let fmt = "%Y-%m-%d %H:%M:%S";
        self.start = match NaiveDateTime::parse_from_str(s.as_str(), fmt) {
            Ok(e) => Some(e),
            Err(_) => match NaiveDateTime::parse_from_str(&format!("{} 11:59:59", s), fmt) {
                Ok(e) => Some(e),
                Err(_) => None,
            },
        };
    }

    pub fn set_repeat(&mut self, rule: String) {
        self.repeat = match Schedule::from_str(&rule) {
            Ok(d) => Some(Repeat::Every(d)),
            Err(_) => {
                let rulel = rule.to_lowercase();
                let v = rulel.split("from").collect::<Vec<&str>>();

                let dur = match DurationHuman::try_from(v[0]) {
                    Ok(e) => match Duration::from_std(StdDuration::from(&e)) {
                        Ok(e) => e,
                        Err(_) => return,
                    },
                    Err(_) => return,
                };

                if v.len() > 1 && String::from(v[1]).contains("c") {
                    Some(Repeat::FromCompleted(dur))
                } else {
                    Some(Repeat::FromDue(dur))
                }
            }
        }
    }

    pub fn set_duration(&mut self, rule: String) {
        self.duration = match DurationHuman::try_from(rule.as_str()) {
            Ok(d) => match Duration::from_std(StdDuration::from(&d)) {
                Ok(d) => Some(d),
                Err(error) => panic!("Could not parse duration {}", error),
            },
            Err(error) => panic!("Could not parse duration {}", error),
        };
    }

    pub fn complete(&mut self) -> Option<Todo> {
        // Because we can't recomplete tasks.
        if self.completed.is_some() {
            return None;
        }

        if self.dependencies.filter(|t| t.completed == None).len() > 0 {
            return None;
        }

        let mut t = self.clone();
        let dt = Local::now();
        let d = dt.naive_local();
        self.completed = Some(d);

        match (self.due, &self.repeat) {
            (_, None) => return None,
            (Some(due), Some(Repeat::FromDue(d))) => {
                t.due = Some(due + d.clone());
            }
            (_, Some(Repeat::FromDue(dur))) | (_, Some(Repeat::FromCompleted(dur))) => {
                t.due = Some(d + dur.clone());
            }
            (_, Some(Repeat::Every(e))) => {
                let after = match self.due {
                    Some(d) => match Local.from_local_datetime(&d) {
                        LocalResult::None => dt,
                        LocalResult::Single(e) => max(e, dt),
                        LocalResult::Ambiguous(_, _) => dt,
                    },
                    None => dt,
                };
                t.due = match e.after(&after).next() {
                    Some(e) => Some(e.naive_local()),
                    None => None,
                };
            }
        };

        t.dependencies = t.next_dependencies;
        t.next_dependencies = TodoList::new();

        return Some(t);
    }

    pub fn add_dependency(&mut self, indexes: Vec<usize>, t: Todo) {
        let mut parent = self;

        parent.completed = None;
        for i in indexes {
            parent = &mut parent.dependencies[i];
            parent.completed = None;
        }

        parent.dependencies.add(t);
    }

    pub fn complete_dependency(&mut self, indexes: Vec<usize>) {
        let mut parent = self;
        let mut t: &mut Todo;

        if indexes.len() == 0 {
            return;
        }

        t = &mut parent.dependencies[indexes[0]];

        for i in &indexes[1..] {
            parent = t;
            t = &mut parent.dependencies[*i];
        }

        match t.complete() {
            Some(e) => parent.next_dependencies.add(e),
            None => return,
        }
    }

    pub fn set_completed_iso8601(&mut self, s: String) {
        self.completed = match NaiveDateTime::parse_from_str(s.as_str(), "%Y-%m-%d") {
            Ok(e) => Some(e),
            Err(_) => None,
        };
    }

    pub fn add_tag(&mut self, t: String) {
        self.tags.push(t);
    }

    pub fn remove_tag(&mut self, t: String) {
        match self.tags.binary_search(&t) {
            Ok(i) => {
                self.tags.remove(i);
                return;
            }
            Err(_) => return,
        }
    }
}

impl ToString for Todo {
    fn to_string(&self) -> String {
        let mut s = format!("{}", self.title);
        s = match self.duration {
            Some(e) => format!("{} 🕒 {}", s, format!("{} minutes", e.num_minutes())),
            None => s,
        };
        s = match self.start {
            Some(e) => format!("{} ✈️ {}", s, e.to_string()),
            None => s,
        };
        s = match self.due {
            Some(e) => format!("{} 📅 {}", s, e.to_string()),
            None => s,
        };
        s = match &self.repeat {
            Some(e) => format!("{} 🔁 {}", s, e.to_string()),
            None => s,
        };
        s = match self.completed {
            Some(e) => format!("[x] {} ✅ {}", s, e.to_string()),
            None => format!("[ ] {}", s),
        };

        for t in &self.tags {
            s = format!("{} #{}", s, t);
        }

        let deps = self.dependencies.clone();
        for (i, t) in deps.into_iter().enumerate() {
            let ds = t.to_string().replace("\n", "\n<<");
            s = format!("{}\n<< {i} {}", s, ds);
        }

        return s;
    }
}
