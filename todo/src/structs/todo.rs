use crate::TodoList;
use chrono::{
    offset::TimeZone, DateTime, Duration as ChronoDuration, Local, LocalResult,
    NaiveDateTime as ChronoNaiveDateTime, OutOfRangeError,
};
use core::str::FromStr;
use core::time::Duration as StdDuration;
use cron::Schedule as CronSchedule;
use duration_human::DurationHuman;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use serde_json;
use std::cmp::max;
use std::ops::Add;

#[derive(Clone)]
struct Duration(ChronoDuration);

impl Duration {
    fn num_milliseconds(&self) -> i64 {
        self.0.num_milliseconds()
    }

    fn milliseconds(v: i64) -> Self {
        Duration(ChronoDuration::milliseconds(v))
    }

    fn num_minutes(&self) -> i64 {
        self.0.num_milliseconds()
    }

    fn from_std(s: StdDuration) -> Result<Duration, OutOfRangeError> {
        match ChronoDuration::from_std(s) {
            Ok(e) => Ok(Duration(e)),
            Err(e) => Err(e),
        }
    }

    fn to_std(&self) -> Result<StdDuration, OutOfRangeError> {
        self.0.to_std()
    }
}

impl Add<Duration> for NaiveDateTime {
    type Output = NaiveDateTime;

    fn add(self, rhs: Duration) -> NaiveDateTime {
        NaiveDateTime(self.0.add(rhs.0))
    }
}

impl Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.num_milliseconds())
    }
}

struct DurationVisitor;

impl<'de> Visitor<'de> for DurationVisitor {
    type Value = Duration;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Duration in milliseconds")
    }

    fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(Duration::milliseconds(v))
    }

    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(Duration::milliseconds(v as i64))
    }
}

impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i64(DurationVisitor)
    }
}

#[derive(Clone)]
struct Schedule(CronSchedule);

impl Schedule {
    fn after<Z: TimeZone>(&self, d: &DateTime<Z>) -> cron::ScheduleIterator<'_, Z> {
        self.0.after(d)
    }
}

impl ToString for Schedule {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for Schedule {
    type Err = cron::error::Error;

    fn from_str(s: &str) -> Result<Schedule, Self::Err> {
        match CronSchedule::from_str(s) {
            Ok(e) => Ok(Schedule(e)),
            Err(e) => Err(e),
        }
    }
}

impl Serialize for Schedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

struct ScheduleVisitor;

impl<'de> Visitor<'de> for ScheduleVisitor {
    type Value = Schedule;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "A cron expression")
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        match Schedule::from_str(v) {
            Ok(e) => Ok(e),
            Err(_) => Err(serde::de::Error::custom(format!(
                "Could not parse cron expression {}",
                v
            ))),
        }
    }
}

impl<'de> Deserialize<'de> for Schedule {
    fn deserialize<D>(deserializer: D) -> Result<Schedule, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ScheduleVisitor)
    }
}

#[derive(Clone)]
struct NaiveDateTime(ChronoNaiveDateTime);

impl NaiveDateTime {
    fn parse_from_str(s: &str, fmt: &str) -> chrono::ParseResult<NaiveDateTime> {
        match ChronoNaiveDateTime::parse_from_str(s, fmt) {
            Ok(e) => Ok(NaiveDateTime(e)),
            Err(e) => Err(e),
        }
    }
}

impl ToString for NaiveDateTime {
    fn to_string(&self) -> String {
        self.0.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

impl PartialEq<NaiveDateTime> for NaiveDateTime {
    fn eq(&self, other: &NaiveDateTime) -> bool {
        self.0.eq(&other.0)
    }

    fn ne(&self, other: &NaiveDateTime) -> bool {
        self.0.ne(&other.0)
    }
}

impl Serialize for NaiveDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

struct NaiveDateTimeVisitor;

impl<'de> Visitor<'de> for NaiveDateTimeVisitor {
    type Value = NaiveDateTime;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let fmt = "%Y-%m-%d %H:%M:%S";
        write!(formatter, "Datetime {}", fmt)
    }

    fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
        let fmt = "%Y-%m-%d %H:%M:%S";
        match NaiveDateTime::parse_from_str(s, fmt) {
            Ok(e) => Ok(e),
            Err(_) => Err(serde::de::Error::custom(format!(
                "Could not parse datetime {}",
                s
            ))),
        }
    }
}

impl<'de> Deserialize<'de> for NaiveDateTime {
    fn deserialize<D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(NaiveDateTimeVisitor)
    }
}

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

        t.dependencies = t.next_dependencies;
        t.next_dependencies = TodoList::new();

        // All subtasks should complete automatically, so that their next itteration is loaded.
        for i in 0..t.sub_tasks.len() {
            match t.sub_tasks[i].complete() {
                Some(e) => t.next_sub_tasks.add(e),
                None => continue,
            };
        }
        t.sub_tasks = t.next_sub_tasks;
        t.next_sub_tasks = TodoList::new();

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

    pub fn add_sub_task(&mut self, indexes: Vec<usize>, t: Todo) {
        let mut parent = self;

        for i in indexes {
            parent = &mut parent.sub_tasks[i];
        }

        parent.sub_tasks.add(t);
    }

    pub fn complete_sub_task(&mut self, indexes: Vec<usize>) {
        let mut parent = self;
        let mut t: &mut Todo;

        if indexes.len() == 0 {
            return;
        }

        t = &mut parent.sub_tasks[indexes[0]];

        for i in &indexes[1..] {
            parent = t;
            t = &mut parent.sub_tasks[*i];
        }

        match t.complete() {
            Some(e) => parent.next_sub_tasks.add(e),
            None => return,
        }
    }

    pub fn set_completed_iso8601(&mut self, s: String) {
        let fmt = "%Y-%m-%d %H:%M:%S";
        self.completed = match NaiveDateTime::parse_from_str(s.as_str(), fmt) {
            Ok(e) => Some(e),
            Err(_) => match NaiveDateTime::parse_from_str(&format!("{} 11:59:59", s), fmt) {
                Ok(e) => Some(e),
                Err(_) => None,
            },
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

    pub fn to_json(&self) -> String {
        match serde_json::to_string_pretty(self) {
            Ok(e) => e,
            Err(_) => panic!("Couldn't convert to json."),
        }
    }

    pub fn from_json(s: &str) -> Self {
        match serde_json::from_str(s) {
            Ok(e) => e,
            Err(_) => panic!("Couldn't convert from json."),
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
        let mut lists = Vec::from(["\n  - Dependencies:", "\n  - Sub Tasks:"]);
        lists.sort_by_key(|x| s.find(x));
        let found: Vec<&str> = lists.into_iter().filter(|x| s.find(*x) != None).collect();

        let mut parts: Vec<&str> = s.split("\n  - Dependencies:\n").collect();
        if parts.len() == 1 {
            parts = s.split("\n  - Sub Tasks:").collect();
        } else {
            let next_parts: Vec<&str> = parts[1].split("\n  - Sub Tasks:\n").collect();
            parts[1] = next_parts[0];
            if next_parts.len() > 1 {
                parts.push(next_parts[1]);
            }
        }

        let mut dependencies = TodoList::new();
        let mut sub_tasks = TodoList::new();

        for i in 1..parts.len() {
            match found[i - 1] {
                "\n  - Dependencies:" => {
                    dependencies = TodoList::from_markdown(&parts[i].replace("\n    ", "\n")[4..])
                }
                "\n  - Sub Tasks:" => {
                    sub_tasks = TodoList::from_markdown(&parts[i].replace("\n    ", "\n")[4..])
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
