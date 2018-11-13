use serde_json::Value;
use std::{collections::HashSet, fs, path::Path};

fn get_existing_exercises() -> HashSet<String> {
    let existing_exercise_dirs: Value = reqwest::get(
        "https://api.github.com/repos/exercism/problem-specifications/contents/exercises/",
    ).unwrap()
    .json()
    .unwrap();

    existing_exercise_dirs
        .as_array()
        .unwrap()
        .iter()
        .map(|exercise_dir| {
            exercise_dir
                .get("name")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
        }).collect()
}

fn get_config_value() -> Value {
    let track_root = xtodo::get_track_root();

    let config_path = Path::new(&track_root).join("config.json");

    let config_content = fs::read_to_string(config_path).unwrap();

    serde_json::from_str(&config_content).unwrap()
}

fn get_implemented_exercises(config: &Value) -> HashSet<String> {
    config
        .get("exercises")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|exercise| exercise.get("slug").unwrap().as_str().unwrap().to_string())
        .collect()
}

pub fn list_missing_exercises() {
    let existing_exercises = get_existing_exercises();

    let config = get_config_value();

    let track_name = config.get("language").unwrap().as_str().unwrap();

    let implemented_exercises = get_implemented_exercises(&config);

    let unimplemented_exercises: Vec<String> = existing_exercises
        .difference(&implemented_exercises)
        .map(|unimplemented_exercise| unimplemented_exercise.to_owned())
        .collect();

    println!(
        "Unimplemented exercises for the {} track:\n{}",
        track_name,
        unimplemented_exercises
            .iter()
            .enumerate()
            .map(|(idx, unimplemented_exercise)| format!("{}) {}", idx + 1, unimplemented_exercise))
            .collect::<Vec<String>>()
            .join("\n")
    );
}
