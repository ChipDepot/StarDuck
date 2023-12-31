use std::env;

use chrono::FixedOffset;
use chrono::{NaiveDateTime, Utc};

use log::warn;

// Offset for GMT-5 ('cause Colombia)
const DEFAULT_TIMEZONE_OFFSET: i32 = -5;
const HOUR_IN_SECS: i32 = 3600;
const TIMEZONE_OFFSET_EAST: &str = "TIMEZONE_OFFSET_EAST";

pub trait WithOffset {
    fn now_with_offset() -> NaiveDateTime;
}

impl WithOffset for Utc {
    fn now_with_offset() -> NaiveDateTime {
        let offset = env::var(TIMEZONE_OFFSET_EAST)
            .map(|tz| tz.parse().unwrap())
            .unwrap_or(DEFAULT_TIMEZONE_OFFSET);

        if let Some(tz_offset) = FixedOffset::east_opt(offset * HOUR_IN_SECS) {
            return Utc::now().with_timezone(&tz_offset).naive_local();
        };

        warn!("Invalid offset {offset}, defaulting to GMT-5");
        return Utc::now()
            .with_timezone(&FixedOffset::east_opt(DEFAULT_TIMEZONE_OFFSET * HOUR_IN_SECS).unwrap())
            .naive_local();
    }
}
