//! Rust struct for canonical test data.
//!
//! See https://github.com/exercism/problem-specifications/blob/master/canonical-schema.json
//! for more details on the JSON schema, which makes it possible to implement
//! `serde::Deserialize`.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

macro_rules! pub_struct_serde_getters {
    (
        pub struct $name:ident {
            $( $fname:ident : $ftype:ty ),* $(,)*
        }
    ) => {
        #[derive(Serialize, Deserialize, Debug)]
        pub struct $name {
            $($fname : $ftype),*
        }

        impl $name {
            $(
            pub fn $fname(&self) -> &$ftype {
                &self.$fname
            }
            )*
        }
    };
}

pub_struct_serde_getters! {
    pub struct CanonicalData {
        exercise: Exercise,
        version: Version,
        comments: Option<Comments>,
        cases: TestGroup,
    }
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

pub_struct_serde_getters! {
    pub struct LabeledTest {
        description: Description,
        optional: Option<Optional>,
        comments: Option<Comments>,
        property: Property,
        input: Input,
        expected: Expected,
    }
}

pub_struct_serde_getters! {
    pub struct LabeledTestGroup {
        description: Description,
        optional: Option<Optional>,
        comments: Option<Comments>,
        cases: TestGroup,
    }
}

type Description = String;
type Optional = String;
type Property = String;
type Input = serde_json::Value;
type Expected = serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Data {
    None,
    Bool(bool),
    Integer(i32),
    Float(f64),
    String(String),
    Array(Vec<Data>),
    Map(IndexMap<String, Data>),
}
