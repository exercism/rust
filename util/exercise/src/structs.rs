//! Rust struct for canonical test data.
//!
//! See https://github.com/exercism/problem-specifications/blob/master/canonical-schema.json
//! for more details on the JSON schema, which makes it possible to implement
//! `serde::Deserialize`.

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct CanonicalData {
    pub exercise: Exercise,
    pub version: Version,
    pub comments: Option<Comments>,
    pub cases: TestGroup,
}

type Exercise = String;
type Version = String;
type Comments = Vec<String>;
type TestGroup = Vec<LabeledTestItem>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum LabeledTestItem {
    Single(LabeledTest),
    Array(LabeledTestGroup),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LabeledTest {
    pub description: Description,
    pub optional: Option<Optional>,
    pub comments: Option<Comments>,
    pub property: Property,
    pub input: Input,
    pub expected: Expected,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LabeledTestGroup {
    pub description: Description,
    pub optional: Option<Optional>,
    pub comments: Option<Comments>,
    pub cases: TestGroup,
}

type Description = String;
type Optional = String;
type Property = String;
type Input = Value;
type Expected = Value;
