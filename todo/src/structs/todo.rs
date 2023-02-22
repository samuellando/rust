use crate::structs::Duration;
use crate::structs::NaiveDateTime;
use crate::structs::Schedule;
use crate::TodoList;
use chrono::{offset::TimeZone, Local, LocalResult};
use core::time::Duration as StdDuration;
use duration_human::DurationHuman;
use serde::{Deserialize, Serialize};
use std::cmp::max;
use std::str::FromStr;

#[derive(Clone, Serialize, Deserialize)]
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Todo {
    completed: Option<NaiveDateTime>,
    title: String,
    due: Option<NaiveDateTime>,
    start: Option<NaiveDateTime>,
    repeat: Option<Repeat>,
    tags: Vec<String>,
    sub_tasks: TodoList,
    dependencies: TodoList,
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
            duration: None,
        };
    }

    pub fn set_title(&mut self, t: String) {
        self.title = t;
    }

    fn _parse_iso8601(s: String) -> Option<NaiveDateTime> {
        let fmt = "%Y-%m-%d %H:%M:%S";
        match NaiveDateTime::parse_from_str(s.as_str(), fmt) {
            Ok(e) => Some(e),
            Err(_) => match NaiveDateTime::parse_from_str(&format!("{} 11:59:59", s), fmt) {
                Ok(e) => Some(e),
                Err(_) => None,
            },
        }
    }

    pub fn set_due_iso8601(&mut self, s: String) {
        self.due = Todo::_parse_iso8601(s)
    }

    pub fn set_start_iso8601(&mut self, s: String) {
        self.start = Todo::_parse_iso8601(s)
    }

    pub fn set_completed_iso8601(&mut self, s: String) {
        self.completed = Todo::_parse_iso8601(s)
    }

    pub fn set_repeat(&mut self, rule: String) {
        // First try to read the cron expression.
        self.repeat = match Schedule::from_str(&rule) {
            Ok(d) => Some(Repeat::Every(d)),
            // If that does not work, parse as plaintext.
            Err(_) => {
                let rulel = rule.to_lowercase();
                // We can say from completed, ot from due.
                let v = rulel.split("from").collect::<Vec<&str>>();

                // Use hman duration to parse the duration.
                let dur = match DurationHuman::try_from(v[0]) {
                    Ok(e) => match Duration::from_std(StdDuration::from(&e)) {
                        Ok(e) => e,
                        Err(_) => return,
                    },
                    Err(_) => return,
                };
                // Finally check if it's from completed.
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
                Err(_) => return,
            },
            Err(_) => return,
        };
    }

    pub fn complete(&mut self) -> Option<Todo> {
        // Because we can't recomplete tasks.
        if self.completed.is_some() {
            return None;
        }
        // Because we can't complete a task with uncompleted dependencies.
        if self.dependencies.filter(|t| t.completed == None).len() > 0 {
            return None;
        }

        let mut t = self.clone();
        let dt = Local::now();
        let d = NaiveDateTime(dt.naive_local());
        self.completed = Some(d.clone());

        match (&self.due, &self.repeat) {
            (_, None) => return None,
            (Some(due), Some(Repeat::FromDue(d))) => {
                t.due = Some(due.clone() + d.clone());
            }
            (_, Some(Repeat::FromDue(dur))) | (_, Some(Repeat::FromCompleted(dur))) => {
                t.due = Some(d + dur.clone());
            }
            (_, Some(Repeat::Every(e))) => {
                let after = match &self.due {
                    Some(d) => match Local.from_local_datetime(&d.0) {
                        LocalResult::None => dt,
                        LocalResult::Single(e) => max(e, dt),
                        LocalResult::Ambiguous(_, _) => dt,
                    },
                    None => dt,
                };
                t.due = match e.after(&after).next() {
                    Some(e) => Some(NaiveDateTime(e.naive_local())),
                    None => None,
                };
            }
        };

        for i in 0..t.dependencies.len() {
            t.dependencies[i].completed = None;
        }
        for i in 0..t.sub_tasks.len() {
            t.sub_tasks[i].completed = None;
        }

        return Some(t);
    }

    pub fn get_dependencies(&mut self) -> &mut TodoList {
        &mut self.dependencies
    }

    pub fn get_sub_tasks(&mut self) -> &mut TodoList {
        &mut self.sub_tasks
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

    pub fn to_markdown(&self) -> String {
        let mut s = format!("{}", self.title);
        s = match &self.duration {
            Some(e) => format!("{} 🕒 {}", s, format!("{} minutes", e.num_minutes())),
            None => s,
        };
        s = match &self.start {
            Some(e) => format!("{} ✈️ {}", s, e.to_string()),
            None => s,
        };
        s = match &self.due {
            Some(e) => format!("{} 📅 {}", s, e.to_string()),
            None => s,
        };
        s = match &self.repeat {
            Some(e) => format!("{} 🔁 {}", s, e.to_string()),
            None => s,
        };
        s = match &self.completed {
            Some(e) => format!("- [x] {} ✅ {}", s, e.to_string()),
            None => format!("- [ ] {}", s),
        };

        for t in &self.tags {
            s = format!("{} #{}", s, t);
        }

        let deps = self.dependencies.clone();
        if deps.len() > 0 {
            s = format!("{}\n  - Dependencies:", s);
        }
        for t in deps {
            let ds = t.to_string().replace("\n", "\n    ");
            s = format!("{}\n    {}", s, ds);
        }

        let subs = self.sub_tasks.clone();
        if subs.len() > 0 {
            s = format!("{}\n  - Sub Tasks:", s);
        }
        for t in subs.into_iter() {
            let ds = t.to_string().replace("\n", "\n    ");
            s = format!("{}\n    {}", s, ds);
        }

        return s;
    }

    pub fn from_markdown(s: &str) -> Todo {
        let s = String::from(s);

        // Start by parsing the sub lists.
        let mut lists = Vec::from(["\n\t- Dependencies:", "\n\t- Sub Tasks:"]);
        lists.sort_by_key(|x| s.find(x));
        let found: Vec<&str> = lists.into_iter().filter(|x| s.find(*x) != None).collect();

        let mut parts: Vec<&str> = s.split("\n\t- Dependencies:\n").collect();
        if parts.len() == 1 {
            parts = s.split("\n\t- Sub Tasks:").collect();
        } else {
            let next_parts: Vec<&str> = parts[1].split("\n\t- Sub Tasks:\n").collect();
            parts[1] = next_parts[0];
            if next_parts.len() > 1 {
                parts.push(next_parts[1]);
            }
        }

        let mut dependencies = TodoList::new();
        let mut sub_tasks = TodoList::new();

        for i in 1..parts.len() {
            match found[i - 1] {
                "\n\t- Dependencies:" => {
                    dependencies = TodoList::from_markdown(&parts[i].replace("\n\t\t", "\n")[2..])
                }
                "\n\t- Sub Tasks:" => {
                    sub_tasks = TodoList::from_markdown(&parts[i].replace("\n\t\t", "\n")[2..])
                }
                _ => panic!("Unreachable"),
            }
        }

        // Since the airplane emoji is 2 characters and we need singles.
        let s = parts[0].replace("✈️", "✝");

        let mut symbols = Vec::from(['🕒', '✝', '📅', '🔁', '✅']);

        symbols.sort_by_cached_key(|x| s.find(*x));

        let task_parts: Vec<String> = s
            .split(|c| match c {
                e if symbols.contains(&e) => true,
                _ => false,
            })
            .map(|x| String::from(x.trim()))
            .collect();

        let mut title = task_parts[0].replace("- [ ]", "");
        title = title.replace("- [x]", "");
        title = String::from(title.trim());

        let mut task = Todo::from_title(title);

        let syms = symbols.clone();
        let found = syms.into_iter().filter(|x| s.find(*x) != None);
        for (i, sym) in found.into_iter().enumerate() {
            match sym {
                '🕒' => task.set_duration(task_parts[i + 1].replace("minutes", "min")),
                '✝' => task.set_start_iso8601(task_parts[i + 1].clone()),
                '📅' => task.set_due_iso8601(task_parts[i + 1].clone()),
                '🔁' => task.set_repeat(
                    task_parts[i + 1]
                        .replace("after ", "")
                        .replace("every ", ""),
                ),
                '✅' => task.set_completed_iso8601(task_parts[i + 1].clone()),
                _ => panic!("Unreachable"),
            }
        }

        task.dependencies = dependencies;
        task.sub_tasks = sub_tasks;

        return task;
    }
}

impl ToString for Todo {
    fn to_string(&self) -> String {
        return self.to_markdown();
    }
}
