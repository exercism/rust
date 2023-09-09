//! This module provides a data structure for the track configuration.
//! It is capable of serializing and deserializing the configuration,
//! for example with `serde_json`.
//!
//! Some definitions may not be perfectly precise.
//! Feel free to improve this if need be.
//! (e.g. replace `String` with an enum of possible values)

use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub static TRACK_CONFIG: Lazy<TrackConfig> = Lazy::new(|| {
    let config = include_str!("../../config.json");
    serde_json::from_str(config).expect("should deserialize the track config")
});

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrackConfig {
    pub language: String,
    pub slug: String,
    pub active: bool,
    pub status: Status,
    pub blurb: String,
    pub version: u8,
    pub online_editor: OnlineEditor,
    pub test_runner: HashMap<String, u8>,
    pub files: Files,
    pub exercises: Exercises,
    pub concepts: Vec<ConceptConfig>,
    pub key_features: Vec<KeyFeature>,
    pub tags: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Status {
    pub concept_exercises: bool,
    pub test_runner: bool,
    pub representer: bool,
    pub analyzer: bool,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OnlineEditor {
    pub indent_style: String,
    pub indent_size: u8,
    pub highlightjs_language: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Files {
    pub solution: Vec<String>,
    pub test: Vec<String>,
    pub example: Vec<String>,
    pub exemplar: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Exercises {
    pub concept: Vec<ConceptExercise>,
    pub practice: Vec<PracticeExercise>,
    pub foregone: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConceptExerciseStatus {
    Active,
    Wip,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConceptExercise {
    pub slug: String,
    pub uuid: String,
    pub name: String,
    pub difficulty: u8,
    pub concepts: Vec<String>,
    pub prerequisites: Vec<String>,
    pub status: ConceptExerciseStatus,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PracticeExerciseStatus {
    Deprecated,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PracticeExercise {
    pub slug: String,
    pub name: String,
    pub uuid: String,
    pub practices: Vec<String>,
    pub prerequisites: Vec<String>,
    pub difficulty: u8,
    pub topics: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PracticeExerciseStatus>,
}

impl PracticeExercise {
    pub fn new(slug: String, name: String, difficulty: u8) -> Self {
        Self {
            slug,
            name,
            uuid: uuid::Uuid::new_v4().to_string(),
            practices: Vec::new(),
            prerequisites: Vec::new(),
            difficulty,
            topics: Vec::new(),
            status: None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConceptConfig {
    pub uuid: String,
    pub slug: String,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeyFeature {
    pub icon: String,
    pub title: String,
    pub content: String,
}

#[test]
fn test_deserialize() {
    // force deserialization of lazy static
    assert!(TRACK_CONFIG.active, "should deserialize track config");
}
