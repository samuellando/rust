use chrono::{Duration, Local, NaiveDate};
use duration_str;

#[derive(Clone)]
enum RepeatType {
    FromCompleted,
    FromDue,
}

#[derive(Clone)]
pub struct Todo {
    completed: Option<NaiveDate>,
    title: String,
    due: Option<NaiveDate>,
    start: Option<NaiveDate>,
    repeat: Option<Duration>,
    repeat_type: RepeatType,
    tags: Vec<String>,
    sub_tasks: Vec<Todo>,
    dependencies: Vec<Todo>,
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
            repeat_type: RepeatType::FromCompleted,
            tags: Vec::new(),
            sub_tasks: Vec::new(),
            dependencies: Vec::new(),
            duration: None,
        };
    }

    pub fn set_due_ymd(&mut self, year: i32, month: u32, day: u32) {
        self.due = NaiveDate::from_ymd_opt(year, month, day)
    }

    pub fn set_title(&mut self, t: String) {
        self.title = t;
    }

    pub fn set_due_iso8601(&mut self, s: String) {
        self.due = match NaiveDate::parse_from_str(s.as_str(), "%Y-%m-%d") {
            Ok(e) => Some(e),
            Err(_) => None,
        };
    }

    pub fn set_start_iso8601(&mut self, s: String) {
        self.start = match NaiveDate::parse_from_str(s.as_str(), "%Y-%m-%d") {
            Ok(e) => Some(e),
            Err(_) => None,
        };
    }

    pub fn set_repeat(&mut self, rule: String) {
        self.repeat = match duration_str::parse_std(rule) {
            Ok(d) => match Duration::from_std(d) {
                Ok(d) => Some(d),
                Err(error) => panic!("Could not parse duration {}", error),
            },
            Err(error) => panic!("Could not parse duration {}", error),
        };
    }

    pub fn set_duration(&mut self, rule: String) {
        self.duration = match duration_str::parse_std(rule) {
            Ok(d) => match Duration::from_std(d) {
                Ok(d) => Some(d),
                Err(error) => panic!("Could not parse duration {}", error),
            },
            Err(error) => panic!("Could not parse duration {}", error),
        };
    }

    pub fn set_repeat_type(&mut self, from_completed: bool) {
        self.repeat_type = match from_completed {
            true => RepeatType::FromCompleted,
            false => RepeatType::FromDue,
        };
    }

    pub fn complete(&mut self) -> Option<Todo> {
        // Because we can't recomplete tasks.
        if self.completed.is_some() {
            return None;
        }

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

    pub fn set_completed_iso8601(&mut self, s: String) {
        self.completed = match NaiveDate::parse_from_str(s.as_str(), "%Y-%m-%d") {
            Ok(e) => Some(e),
            Err(_) => None,
        };
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
