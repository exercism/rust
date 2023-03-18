// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct CsvRecordBuilder {
    content: String,
}

impl CsvRecordBuilder {
    // Create a new builder
    pub fn new() -> Self {
        todo!("implement the `CsvRecordBuilder::new` method")
    }

    /// Adds an item to the list separated by a space and a comma.
    pub fn add(&mut self, val: &str) {
        todo!("implement the `CsvRecordBuilder::add` method, adding {val}")
    }

    /// Consumes the builder and returns the comma separated list
    pub fn build(self) -> String {
        todo!("implement the `CsvRecordBuilder::build` method")
    }
}
