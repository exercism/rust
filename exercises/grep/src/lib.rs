extern crate failure;

use failure::Error;

/// While using raw slice of str to handle flags is convenient,
/// in the real-world projects it is customary to use a struct,
/// that contains flags-related logic. So in this exercise
/// we ask you to implement a custom struct.
///
/// If you are curious about real-world implementation, refer to the `clap-rs` crate:
/// https://github.com/kbknapp/clap-rs/blob/master/src/args/arg_matches.rs
#[derive(Debug)]
pub struct Flags;

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        unimplemented!(
            "Given the flags {:?} implement your own 'Flags' struct to handle flags-related logic",
            flags
        );
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    unimplemented!(
        "Search the files '{:?}' for '{}' pattern and save the matches in a vector. Your search logic should be aware of the given flags '{:?}'",
        files,
        pattern,
        flags
    );
}
