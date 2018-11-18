use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::{fs, path::Path};
use toml::Value as TomlValue;

struct RustVersionReader;
struct HaskellVersionReader;

trait GetLocalVersion {
    fn get_local_version(&self, exercise_name: &str) -> xtodo::Result<String>;
}

impl GetLocalVersion for RustVersionReader {
    fn get_local_version(&self, exercise_name: &str) -> xtodo::Result<String> {
        let track_root = xtodo::get_track_root()?;

        let cargo_toml_path = Path::new(&track_root)
            .join("exercises")
            .join(exercise_name)
            .join("Cargo.toml");

        let cargo_toml_content = fs::read_to_string(cargo_toml_path)?;

        let cargo_toml: TomlValue = cargo_toml_content.parse()?;

        Ok(cargo_toml["package"]
            .get("version")
            .map(|version| version.as_str().unwrap().to_string())
            .unwrap())
    }
}

impl GetLocalVersion for HaskellVersionReader {
    fn get_local_version(&self, exercise_name: &str) -> xtodo::Result<String> {
        let track_root = xtodo::get_track_root()?;

        let package_yaml_path = Path::new(&track_root)
            .join("exercises")
            .join(exercise_name)
            .join("package.yaml");

        let package_yaml_content = fs::read_to_string(package_yaml_path)?;

        let package_yaml: YamlValue = serde_yaml::from_str(&package_yaml_content)?;

        Ok(package_yaml["version"]
            .as_str()
            .unwrap()
            .split('.')
            .take(3)
            .collect::<Vec<&str>>()
            .join("."))
    }
}

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

fn get_canonical_version(exercise_name: &str) -> xtodo::Result<String> {
    let canonical_data: JsonValue = reqwest::get(&format!("https://raw.githubusercontent.com/exercism/problem-specifications/master/exercises/{}/canonical-data.json", exercise_name))?.json()?;

    Ok(canonical_data
        .get("version")
        .map(|version| version.as_str().unwrap().to_string())
        .unwrap())
}

fn get_version_reader(track_name: &str) -> Option<Box<GetLocalVersion>> {
    match track_name.to_lowercase().as_ref() {
        "rust" => Some(Box::new(RustVersionReader)),
        "haskell" => Some(Box::new(HaskellVersionReader)),
        _ => None,
    }
}

pub fn list_outdated_exercises() -> xtodo::Result<()> {
    let config = xtodo::get_config_value()?;

    let track_name = config.get("language").unwrap().as_str().unwrap();

    let version_reader = get_version_reader(track_name);

    if version_reader.is_none() {
        println!(
            "Reading the local exercise version is not implemented for the {} track. Aborting.",
            track_name
        );

        return Ok(());
    }

    let version_reader = version_reader.unwrap();

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

        exercise.local_version = match version_reader.get_local_version(name) {
            Ok(local_version) => Some(local_version),
            Err(err) => {
                println!(
                    "Failed to get local version for the {} exercise: {}",
                    name, err
                );
                None
            }
        };

        exercise.canonical_version = match get_canonical_version(name) {
            Ok(canonical_version) => Some(canonical_version),
            Err(err) => {
                println!(
                    "Could not retrieve canonical-data for the {} exercise: {}",
                    name, err
                );
                None
            }
        };
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

    Ok(())
}
