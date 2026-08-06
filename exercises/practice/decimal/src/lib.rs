use std::str::FromStr;

/// Type implementing arbitrary-precision decimal arithmetic
pub struct Decimal {
    // implement your type here
}

impl FromStr for Decimal {
    // implement your error type here
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        todo!("Create a new decimal with a value of {input}")
    }
}
