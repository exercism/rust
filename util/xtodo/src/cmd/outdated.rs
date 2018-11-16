use serde_json::Value as JsonValue;
use std::{fs, path::Path};
use toml::Value as TomlValue;

#[derive(Debug)]
struct ExerciseInfo {
    name: String,
    local_version: Option<String>,
    canonical_version: Option<String>,
}

impl ExerciseInfo {
    fn new(name: &str) -> Self {
        ExerciseInfo {
            name: name.to_string(),
            local_version: None,
            canonical_version: None,
        }
    }
}

fn get_local_version(exercise_name: &str) -> Option<String> {
    let track_root = xtodo::get_track_root();

    let cargo_toml_path = Path::new(&track_root)
        .join("exercises")
        .join(exercise_name)
        .join("Cargo.toml");

    let cargo_toml_content = fs::read_to_string(cargo_toml_path).unwrap();

    let cargo_toml: TomlValue = cargo_toml_content.parse().unwrap();

    cargo_toml["package"]
        .get("version")
        .map(|version| version.as_str().unwrap().to_string())
}

fn get_canonical_version(exercise_name: &str) -> Option<String> {
    let  mut canonical_data = reqwest::get(&format!("https://raw.githubusercontent.com/exercism/problem-specifications/master/exercises/{}/canonical-data.json", exercise_name))
        .unwrap();

    let canonical_data_json: JsonValue = canonical_data.json().unwrap_or_else(|_| {
        println!(
            "Could not retrieve canonical-data for the {} exercise: {}",
            exercise_name,
            canonical_data
                .text()
                .unwrap_or_else(|_| "Failed to make HTTP request.".to_string())
                .trim()
        );

        JsonValue::Null
    });

    canonical_data_json
        .get("version")
        .map(|version| version.as_str().unwrap().to_string())
}

pub fn list_outdated_exercises() {
    let config = xtodo::get_config_value();

    let track_name = config.get("language").unwrap().as_str().unwrap();

    let mut exercises: Vec<ExerciseInfo> = config
        .get("exercises")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .filter(|exercise| {
            !(exercise.get("deprecated").is_some() && exercise["deprecated"].as_bool().unwrap())
        }).map(|exercise_value| {
            ExerciseInfo::new(exercise_value.get("slug").unwrap().as_str().unwrap())
        }).collect();

    for exercise in &mut exercises {
        let name = &exercise.name;

        exercise.local_version = get_local_version(name);

        exercise.canonical_version = get_canonical_version(name);
    }

    let outdated_exercises: Vec<&ExerciseInfo> = exercises
        .iter()
        .filter(|exercise| {
            exercise.local_version.is_some()
                && exercise.canonical_version.is_some()
                && exercise.local_version != exercise.canonical_version
        }).collect();

    println!(
        "\nOutdated exercises for the {} track:\n{}",
        track_name,
        outdated_exercises
            .iter()
            .enumerate()
            .map(|(idx, exercise)| format!(
                "{:2}) {:25} local version: {:6} canonical version: {:6}",
                idx + 1,
                exercise.name,
                exercise.local_version.as_ref().unwrap(),
                exercise.canonical_version.as_ref().unwrap()
            )).collect::<Vec<String>>()
            .join("\n")
    );
}
