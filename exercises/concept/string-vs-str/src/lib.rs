// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct CSVRecordBuilder {
    content: String
}

impl CSVRecordBuilder {
    // Create a new builder
    pub fn new() -> Self {
        unimplemented!()
    }

    /// Adds an item to the list separated by a space and a comma.
    pub fn add(&mut self, val: &str) {
        unimplemented!()
    }

    /// Consumes the builder and returns the comma separated list
    pub fn build(self) -> String {
        unimplemented!()
    }
}
