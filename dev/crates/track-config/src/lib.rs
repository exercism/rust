//! This crate provides a data structure for the track configuration.
//! It is capable of serializing and deserializing the configuration,
//! for example with `serde_json`.
//! 
//! Some definitions are not yet perfectly precise,
//! because we don't anticipate they will be needed much.
//! Feel free to improve this if need be.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrackConfig {
    pub language: String,
    pub slug: String,
    pub active: bool,
    pub status: HashMap<String, bool>,
    pub blurb: String,
    pub version: u8,
    pub online_editor: OnlineEditorConfig,
    pub test_runner: HashMap<String, u8>,
    pub files: HashMap<String, Vec<String>>,
    pub exercises: ExercisesConfig,
    pub concepts: Vec<ConceptConfig>,
    pub key_features: Vec<KeyFeatureConfig>,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OnlineEditorConfig {
    pub indent_style: String,
    pub indent_size: u8,
    pub highlightjs_language: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExercisesConfig {
    pub concept: Vec<ConceptExerciseConfig>,
    pub practice: Vec<PracticeExerciseConfig>,
    pub foregone: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConceptExerciseStatus {
    Active,
    Wip,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConceptExerciseConfig {
    pub slug: String,
    pub uuid: String,
    pub name: String,
    pub difficulty: u8,
    pub concepts: Vec<String>,
    pub prerequisites: Vec<String>,
    pub status: ConceptExerciseStatus,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PracticeExerciseStatus {
    Deprecated,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PracticeExerciseConfig {
    pub slug: String,
    pub name: String,
    pub uuid: String,
    pub practices: Vec<String>,
    pub prerequisites: Vec<String>,
    pub difficulty: u8,
    pub topics: Vec<String>,
    #[serde(default)]
    pub status: Option<PracticeExerciseStatus>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConceptConfig {
    pub uuid: String,
    pub slug: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeyFeatureConfig {
    pub icon: String,
    pub title: String,
    pub content: String,
}
