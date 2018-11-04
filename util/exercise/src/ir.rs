//! The IR is the intermediate representation of the test data.
//!
//! These structs are constructed from canonical data and existing tests,
//! and are used to emit a well-structured test file.

#[derive(Serialize, Deserialize, Debug)]
pub struct Property {
    name: String,
    inputs: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    name: String,
    literal: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewTest {
    property: String,
    comment: Option<String>,
    description: String,
    inputs: Vec<Input>,
    expect_literal: String,
}

pub enum Test {
    New(NewTest),
    Existing(String),
}

pub enum Case {
    Test(Test),
    Cases { comment: String, case: Vec<Case> },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tests {
    header: bool,
    exercise: String,
    use_stmts: Vec<String>,
    existing_data: String,
    properties: Vec<String>,
    tests: Vec<String>,
}
