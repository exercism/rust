pub struct Clock;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        unimplemented!(
            "Construct a new Clock from {} hours and {} minutes",
            hours,
            minutes
        );
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }
}
