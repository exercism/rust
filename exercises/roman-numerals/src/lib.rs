use std::fmt::{Display, Formatter, Result};

pub struct Roman;

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!("Return a roman-numeral string representation of the Roman object");
    }
}

impl From<i32> for Roman {
    fn from(num: i32) -> Self {
        unimplemented!("Construct a Roman object from the '{}' number", num);
    }
}
