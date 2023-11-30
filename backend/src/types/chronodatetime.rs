use serde::Serialize;
use serde::Serializer;
//use chrono::NaiveDateTime;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct ChronoDateTime(chrono::DateTime<Utc>);

impl ChronoDateTime {
    pub fn new(time: chrono::DateTime<Utc>) -> Self {
        Self(time)
    }

    pub fn as_datetime(&self) -> DateTime<Utc> {
        self.0
    }

    pub fn from_unix_seconds_string(unix_time_seconds: &str) -> Option<Self> {
        if let Ok(seconds) = unix_time_seconds.parse::<i64>() {
            Some(Self(DateTime::from_timestamp(seconds, 0)?))
        } else {
            None
        }
    }
}

impl Serialize for ChronoDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = self.0.timestamp();
        serializer.serialize_i64(timestamp)
    }
}
