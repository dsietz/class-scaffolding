// #[macro_use]
// use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Duration, Months, Utc};
use uuid::Uuid;

pub enum Intervals {
    Year,
    Month,
    Day,
}

pub trait Scaffolding {
    fn hello(&self) {}

    /// generates a uuid v4 value
    fn id() -> String {
        Uuid::new_v4().to_string()
    }

    /// adds x days to the timestamp
    fn add_days(dtm: i64, days: i64) -> i64 {
        let dt = DateTime::from_timestamp(dtm, 0).unwrap() + Duration::try_days(days).unwrap();
        dt.timestamp()
    }

    /// adds x years to the timestamp
    fn add_months(dtm: i64, months: u32) -> i64 {
        let dt = DateTime::from_timestamp(dtm, 0).unwrap() + Months::new(months);
        dt.timestamp()
    }

    /// adds x years to the timestamp
    fn add_years(dtm: i64, years: u32) -> i64 {
        let dt = DateTime::from_timestamp(dtm, 0).unwrap() + Months::new(years * 12);
        dt.timestamp()
    }

    /// provided the default unix epoch time (UTC) as seconds
    /// for the timestamp: 9999-12-31 23:59:59
    fn never() -> i64 {
        253402261199
    }

    /// generate the current unix epoch time (UTC) as seconds
    fn now() -> i64 {
        Utc::now().timestamp()
    }
}
