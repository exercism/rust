use std::{cmp, fmt};

pub struct Clock {
    hours: u8,
    minutes: u8,
}

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

    pub fn to_string(&self) -> String {
        unimplemented!("Return String representation of a Clock");
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!("Return debug representation of a Clock");
    }
}

impl cmp::PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        unimplemented!("Implement comparison between two Clocks");
    }
}
