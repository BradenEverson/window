//! Simple time construct that really only cares about the time of the day

use std::time::{SystemTime, UNIX_EPOCH};

pub const SECONDS_PER_DAY: i64 = 86400;
pub const SECONDS_PER_HOUR: i64 = 3600;
pub const MINUTES_PER_HOUR: i64 = 60;

pub struct SimpleTime {
    /// Hour of day
    hour: u32,
    /// Minute of hour
    minute: u32,
}

impl SimpleTime {
    /// Returns the current time of day in hours and minutes
    pub fn now() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before Unix epoch");

        Self::from_unix_timestamp(now.as_secs() as i64)
    }

    pub fn from_unix_timestamp(timestamp: i64) -> Self {
        let offset_seconds = Self::get_local_time_offset();

        let local_timestamp = timestamp + offset_seconds;

        let total_seconds = local_timestamp % SECONDS_PER_DAY;

        let hour = (total_seconds / SECONDS_PER_HOUR) as u32;
        let minute = ((total_seconds % SECONDS_PER_HOUR) / MINUTES_PER_HOUR) as u32;

        Self { hour, minute }
    }

    /// Get the local time zone offset in seconds
    fn get_local_time_offset() -> i64 {
        // Hardcoded to CDT for now because I'm the only user of this ever <3
        -5 * SECONDS_PER_HOUR
    }

    /// Get the hour
    pub fn hour(&self) -> u32 {
        self.hour
    }

    /// Get the minute
    pub fn minute(&self) -> u32 {
        self.minute
    }
}

impl std::fmt::Display for SimpleTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}
