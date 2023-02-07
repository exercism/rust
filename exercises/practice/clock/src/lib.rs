pub struct Clock;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        unimplemented!("Construct a new Clock from {hours} hours and {minutes} minutes");
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {minutes} minutes to existing Clock time");
    }
}
