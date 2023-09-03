//! This crate provides a data structure for exercise configuration stored
//! in `.meta/config`. It is capable of serializing and deserializing th
//! configuration, for example with `serde_json`.

use serde::{Deserialize, Serialize};
use track_config::TRACK_CONFIG;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConceptExerciseConfig {
    pub authors: Vec<String>,
    pub contributors: Option<Vec<String>>,
    pub files: ConceptFilesConfig,
    pub icon: Option<String>,
    pub blurb: String,
    pub source: Option<String>,
    pub source_url: Option<String>,
    pub test_runner: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConceptFilesConfig {
    pub solution: Vec<String>,
    pub test: Vec<String>,
    pub exemplar: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PracticeExerciseConfig {
    pub authors: Vec<String>,
    pub contributors: Option<Vec<String>>,
    pub files: PracticeFilesConfig,
    pub icon: Option<String>,
    pub blurb: String,
    pub source: Option<String>,
    pub source_url: Option<String>,
    pub test_runner: Option<bool>,
    pub custom: Option<CustomConfig>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PracticeFilesConfig {
    pub solution: Vec<String>,
    pub test: Vec<String>,
    pub example: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomConfig {
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
        .map(move |e| format!("{crate_dir}/../../../exercises/concept/{}", e.slug))
}

pub fn get_all_practice_exercise_paths() -> impl Iterator<Item = String> {
    let crate_dir = env!("CARGO_MANIFEST_DIR");

    TRACK_CONFIG
        .exercises
        .practice
        .iter()
        .map(move |e| format!("{crate_dir}/../../../exercises/practice/{}", e.slug))
}

pub fn get_all_exercise_paths() -> impl Iterator<Item = String> {
    get_all_concept_exercise_paths().chain(get_all_practice_exercise_paths())
}
