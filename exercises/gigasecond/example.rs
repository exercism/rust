extern crate chrono;
use chrono::*;

pub fn after(start: DateTime<UTC>) -> DateTime<UTC> {
    start + Duration::seconds(1_000_000_000)
}
