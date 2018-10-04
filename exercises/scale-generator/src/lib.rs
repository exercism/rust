extern crate failure;
use failure::Error;

pub struct Scale;

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        unimplemented!(
            "Construct a new scale with tonic {} and intervals {}",
            tonic,
            intervals
        )
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        unimplemented!("Construct a new chromatic scale with tonic {}", tonic)
    }

    pub fn enumerate(&self) -> Vec<String> {
        unimplemented!()
    }
}
