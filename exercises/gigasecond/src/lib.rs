extern crate chrono;
use chrono::{DateTime, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    unimplemented!("What time is a gigasecond later than {}", start);
}
