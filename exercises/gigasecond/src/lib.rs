extern crate chrono;
use chrono::{DateTime, Utc, Duration};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
	let dur = Duration::seconds(1000000000);
	start + dur
}
