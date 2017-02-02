#[allow(unused_variables)]

pub struct School {
}

impl School {
    pub fn new() -> School {
        unimplemented!()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        unimplemented!()
    }

    pub fn grades(&self) -> Vec<u32> {
        unimplemented!()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        unimplemented!()
    }
}
