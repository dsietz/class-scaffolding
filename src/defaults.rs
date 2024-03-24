//! The defaults module provides the methods for creating deafult values
//! for the Scaffolding common attributes

use chrono::{DateTime, Duration, Months, Utc};
use uuid::Uuid;

/// generates a uuid v4 value
/// 
/// ```rust
/// use scaffolding_core::defaults::*;
/// 
/// assert_eq!(id().len(), "54324f57-9e6b-4142-b68d-1d4c86572d0a".len());
/// ```
pub fn id() -> String {
    Uuid::new_v4().to_string()
}

/// adds x days to the timestamp
/// 
/// ```rust
/// use scaffolding_core::defaults::*;
/// 
/// assert_eq!(add_days(1711295319, 1), 1711381719);
/// ```
pub fn add_days(dtm: i64, days: i64) -> i64 {
    let dt = DateTime::from_timestamp(dtm, 0).unwrap() + Duration::try_days(days).unwrap();
    dt.timestamp()
}

/// adds x months to the timestamp
/// 
/// ```rust
/// use scaffolding_core::defaults::*;
/// 
/// assert_eq!(add_months(1711295319, 1), 1713973719);
/// ```
pub fn add_months(dtm: i64, months: u32) -> i64 {
    let dt = DateTime::from_timestamp(dtm, 0).unwrap() + Months::new(months);
    dt.timestamp()
}

/// adds x years to the timestamp
/// 
/// ```rust
/// use scaffolding_core::defaults::*;
/// 
/// assert_eq!(add_years(1711295319, 1), 1742831319);
/// ```
pub fn add_years(dtm: i64, years: u32) -> i64 {
    let dt = DateTime::from_timestamp(dtm, 0).unwrap() + Months::new(years * 12);
    dt.timestamp()
}

/// provided the default unix epoch time (UTC) as seconds
/// for the timestamp: 9999-12-31 23:59:59
/// 
/// ```rust
/// use scaffolding_core::defaults::*;
/// 
/// assert_eq!(never(), 253402261199);
/// ```
pub fn never() -> i64 {
    253402261199
}

/// generate the current unix epoch time (UTC) as seconds
/// 
/// ```rust
/// use chrono::Utc;
/// use scaffolding_core::defaults::*;
/// 
/// assert_eq!(now(), Utc::now().timestamp());
/// ```
pub fn now() -> i64 {
    Utc::now().timestamp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        assert_eq!(id().len(), "54324f57-9e6b-4142-b68d-1d4c86572d0a".len());
    }

    #[test]
    fn test_add_days() {
        assert_eq!(add_days(1711295319, 1), 1711381719);
    }

    #[test]
    fn test_add_months() {
        assert_eq!(add_months(1711295319, 1), 1713973719);
        // test for a leap year
        // 2023-1-29 +1 = 2023-2-28
        assert_eq!(add_months(1674993600, 1), 1677585600); 
    }

    #[test]
    fn test_add_years() {
        assert_eq!(add_years(1711295319, 1), 1742831319);
        // test for a leap year
        // 2024-2-29 +1 = 2025-2-28
        assert_eq!(add_years(1709208000, 1), 1740744000);
    }

    #[test]
    fn test_never() {
        assert_eq!(never(), 253402261199);
    }

    #[test]
    fn test_now() {
        assert_eq!(now(), Utc::now().timestamp());
    }
}
