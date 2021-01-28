//! Rust struct for canonical test data.
//!
//! See https://github.com/exercism/problem-specifications/blob/main/canonical-schema.json
//! for more details on the JSON schema, which makes it possible to implement
//! `serde::Deserialize`.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;

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

impl CanonicalData {
    pub fn properties(&self) -> HashSet<&str> {
        self.cases
            .iter()
            .flat_map(LabeledTestItem::iter)
            .map(|case| case.property.as_str())
            .collect()
    }
}

impl LabeledTestItem {
    fn iter(&self) -> Box<dyn Iterator<Item = &LabeledTest> + '_> {
        match self {
            LabeledTestItem::Single(case) => Box::new(case.iter()),
            LabeledTestItem::Array(cases) => Box::new(cases.iter()),
        }
    }
}

impl LabeledTest {
    fn iter(&self) -> impl Iterator<Item = &LabeledTest> {
        std::iter::once(self)
    }
}

impl LabeledTestGroup {
    fn iter(&self) -> impl Iterator<Item = &LabeledTest> {
        self.cases.iter().flat_map(LabeledTestItem::iter)
    }
}
