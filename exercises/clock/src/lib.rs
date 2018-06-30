use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        unimplemented!(
            "Construct a new Clock from {} hours and {} minutes",
            hours,
            minutes
        );
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!("Implement 'hh:mm' display format for the Clock.");
    }
}
