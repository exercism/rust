pub struct Brackets;

impl From<&'static str> for Brackets {
    fn from(input: &str) -> Self {
        unimplemented!("From the '{}' input construct a new Brackets struct", input);
    }
}

impl Brackets {
    pub fn are_balanced(&self) -> bool {
        unimplemented!("Check if your Brackets struct contains balanced brackets");
    }
}
