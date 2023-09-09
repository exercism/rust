//! This module provides a data structure for exercise configuration stored in
//! `.meta/config`. It is capable of serializing and deserializing th
//! configuration, for example with `serde_json`.

use serde::{Deserialize, Serialize};
use tera::Tera;

use crate::track_config::TRACK_CONFIG;

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConceptExercise {
    pub authors: Vec<String>,
    pub contributors: Option<Vec<String>>,
    pub files: ConceptFiles,
    pub icon: Option<String>,
    pub blurb: String,
    pub source: Option<String>,
    pub source_url: Option<String>,
    pub test_runner: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConceptFiles {
    pub solution: Vec<String>,
    pub test: Vec<String>,
    pub exemplar: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PracticeExercise {
    pub authors: Vec<String>,
    pub contributors: Option<Vec<String>>,
    pub files: PracticeFiles,
    pub icon: Option<String>,
    pub blurb: String,
    pub source: Option<String>,
    pub source_url: Option<String>,
    pub test_runner: Option<bool>,
    pub custom: Option<Custom>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PracticeFiles {
    pub solution: Vec<String>,
    pub test: Vec<String>,
    pub example: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Custom {
    #[serde(rename = "allowed-to-not-compile")]
    pub allowed_to_not_compile: Option<String>,
    #[serde(rename = "test-in-release-mode")]
    pub test_in_release_mode: Option<bool>,
    #[serde(rename = "ignore-count-ignores")]
    pub ignore_count_ignores: Option<bool>,
}

pub fn get_all_concept_exercise_paths() -> impl Iterator<Item = String> {
    let crate_dir = env!("CARGO_MANIFEST_DIR");

    TRACK_CONFIG
        .exercises
        .concept
        .iter()
        .map(move |e| format!("{crate_dir}/../exercises/concept/{}", e.slug))
}

pub fn get_all_practice_exercise_paths() -> impl Iterator<Item = String> {
    let crate_dir = env!("CARGO_MANIFEST_DIR");

    TRACK_CONFIG
        .exercises
        .practice
        .iter()
        .map(move |e| format!("{crate_dir}/../exercises/practice/{}", e.slug))
}

pub fn get_all_exercise_paths() -> impl Iterator<Item = String> {
    get_all_concept_exercise_paths().chain(get_all_practice_exercise_paths())
}

#[test]
fn test_deserialize_all() {
    for path in get_all_concept_exercise_paths() {
        let config_path = format!("{path}/.meta/config.json");
        let config_contents = std::fs::read_to_string(config_path).unwrap();
        let _: ConceptExercise = serde_json::from_str(config_contents.as_str())
            .expect("should deserialize concept exercise config");
    }
    for path in get_all_practice_exercise_paths() {
        let config_path = format!("{path}/.meta/config.json");
        let config_contents = std::fs::read_to_string(config_path).unwrap();
        let _: PracticeExercise = serde_json::from_str(config_contents.as_str())
            .expect("should deserialize practice exercise config");
    }
}

/// Returns the uuids of the tests excluded in .meta/tests.toml
pub fn get_excluded_tests(slug: &str) -> Vec<String> {
    let path = std::path::PathBuf::from("exercises/practice")
        .join(slug)
        .join(".meta/tests.toml");
    let contents = std::fs::read_to_string(&path).unwrap();

    let mut excluded_tests = Vec::new();

    // shitty toml parser
    for case in contents.split("\n[").skip(1) {
        let (uuid, rest) = case.split_once(']').unwrap();
        if rest.contains("include = false") {
            excluded_tests.push(uuid.to_string());
        }
    }

    excluded_tests
}

/// Returns the uuids of the tests excluded in .meta/tests.toml
pub fn get_test_emplate(slug: &str) -> Option<Tera> {
    Some(Tera::new(format!("exercises/practice/{slug}/.meta/*.tera").as_str()).unwrap())
}
