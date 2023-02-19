use chrono::{
    offset::TimeZone, DateTime, Duration as ChronoDuration, NaiveDateTime as ChronoNaiveDateTime,
    OutOfRangeError,
};
use core::str::FromStr;
use core::time::Duration as StdDuration;
use cron::Schedule as CronSchedule;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Add;

#[derive(Clone)]
pub struct Duration(pub ChronoDuration);

impl Duration {
    pub fn num_milliseconds(&self) -> i64 {
        self.0.num_milliseconds()
    }

    pub fn milliseconds(v: i64) -> Self {
        Duration(ChronoDuration::milliseconds(v))
    }

    pub fn num_minutes(&self) -> i64 {
        self.0.num_milliseconds()
    }

    pub fn from_std(s: StdDuration) -> Result<Duration, OutOfRangeError> {
        match ChronoDuration::from_std(s) {
            Ok(e) => Ok(Duration(e)),
            Err(e) => Err(e),
        }
    }

    pub fn to_std(&self) -> Result<StdDuration, OutOfRangeError> {
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
pub struct Schedule(pub CronSchedule);

impl Schedule {
    pub fn after<Z: TimeZone>(&self, d: &DateTime<Z>) -> cron::ScheduleIterator<'_, Z> {
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
pub struct NaiveDateTime(pub ChronoNaiveDateTime);

impl NaiveDateTime {
    pub fn parse_from_str(s: &str, fmt: &str) -> chrono::ParseResult<NaiveDateTime> {
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
