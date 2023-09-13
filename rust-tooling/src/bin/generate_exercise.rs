use std::path::PathBuf;

use convert_case::{Case, Casing};
use exercism_tooling::{
    exercise_generation, fs_utils,
    track_config::{self, TRACK_CONFIG},
};
use glob::glob;
use inquire::{validator::Validation, Select, Text};

enum Difficulty {
    Easy,
    Medium,
    // I'm not sure why there are two medium difficulties
    Medium2,
    Hard,
}

impl From<Difficulty> for u8 {
    fn from(difficulty: Difficulty) -> Self {
        match difficulty {
            Difficulty::Easy => 1,
            Difficulty::Medium => 4,
            Difficulty::Medium2 => 7,
            Difficulty::Hard => 10,
        }
    }
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "Easy (1)"),
            Difficulty::Medium => write!(f, "Medium (4)"),
            Difficulty::Medium2 => write!(f, "Medium (7)"),
            Difficulty::Hard => write!(f, "Hard (10)"),
        }
    }
}

fn main() {
    fs_utils::cd_into_repo_root();

    let is_update = std::env::args().any(|arg| arg == "update");

    let slug = if is_update {
        ask_for_exercise_to_update()
    } else {
        add_entry_to_track_config()
    };

    make_configlet_generate_what_it_can(&slug);

    generate_exercise_files(&slug, is_update);
}

fn ask_for_exercise_to_update() -> String {
    let implemented_exercises = glob("exercises/practice/*")
        .unwrap()
        .filter_map(Result::ok)
        .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
        .collect::<Vec<_>>();

    Select::new(
        "Which exercise would you like to update?",
        implemented_exercises,
    )
    .prompt()
    .unwrap()
}

/// Interactively prompts the user for required fields in the track config
/// and writes the answers to config.json.
/// Returns slug.
fn add_entry_to_track_config() -> String {
    let implemented_exercises = glob("exercises/concept/*")
        .unwrap()
        .chain(glob("exercises/practice/*").unwrap())
        .filter_map(Result::ok)
        .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
        .collect::<Vec<_>>();

    let todo_with_spec = glob("problem-specifications/exercises/*")
        .unwrap()
        .filter_map(Result::ok)
        .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
        .filter(|e| !implemented_exercises.contains(e))
        .collect::<Vec<_>>();

    println!("(suggestions are from problem-specifications)");
    let slug = Text::new("What's the slug of your exercise?")
        .with_autocomplete(move |input: &_| {
            let mut slugs = todo_with_spec.clone();
            slugs.retain(|e| e.starts_with(input));
            Ok(slugs)
        })
        .with_validator(|input: &str| {
            if input.is_empty() {
                Ok(Validation::Invalid("The slug must not be empty.".into()))
            } else if !input.is_case(Case::Kebab) {
                Ok(Validation::Invalid(
                    "The slug must be in kebab-case.".into(),
                ))
            } else {
                Ok(Validation::Valid)
            }
        })
        .with_validator(move |input: &str| {
            if !implemented_exercises.contains(&input.to_string()) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(
                    "An exercise with this slug already exists.".into(),
                ))
            }
        })
        .prompt()
        .unwrap();

    let name = Text::new("What's the name of your exercise?")
        .with_initial_value(&slug.to_case(Case::Title))
        .prompt()
        .unwrap();

    let difficulty = Select::<Difficulty>::new(
        "What's the difficulty of your exercise?",
        vec![
            Difficulty::Easy,
            Difficulty::Medium,
            Difficulty::Medium2,
            Difficulty::Hard,
        ],
    )
    .prompt()
    .unwrap()
    .into();

    let config = track_config::PracticeExercise::new(slug.clone(), name, difficulty);

    let mut track_config = TRACK_CONFIG.clone();
    track_config.exercises.practice.push(config);
    let mut new_config = serde_json::to_string_pretty(&track_config)
        .unwrap()
        .to_string();
    new_config += "\n";
    std::fs::write("config.json", new_config).unwrap();

    println!(
        "\
Added your exercise to config.json.
You can add practices, prerequisites and topics if you like."
    );

    slug
}

fn make_configlet_generate_what_it_can(slug: &str) {
    let status = std::process::Command::new("just")
        .args([
            "configlet",
            "sync",
            "--update",
            "--yes",
            "--docs",
            "--metadata",
            "--tests",
            "include",
            "--exercise",
            slug,
        ])
        .status()
        .unwrap();
    if !status.success() {
        panic!("configlet sync failed");
    }
}

fn generate_exercise_files(slug: &str, is_update: bool) {
    let fn_names = if is_update {
        read_fn_names_from_lib_rs(slug)
    } else {
        vec!["TODO".to_string()]
    };

    let exercise = exercise_generation::new(slug, fn_names);

    let exercise_path = PathBuf::from("exercises/practice").join(slug);

    if !is_update {
        std::fs::write(exercise_path.join(".gitignore"), exercise.gitignore).unwrap();
        std::fs::write(exercise_path.join("Cargo.toml"), exercise.manifest).unwrap();
        std::fs::create_dir(exercise_path.join("src")).ok();
        std::fs::write(exercise_path.join("src/lib.rs"), exercise.lib_rs).unwrap();
        std::fs::write(exercise_path.join(".meta/example.rs"), exercise.example).unwrap();
    }

    let template_path = exercise_path.join(".meta/test_template.tera");
    if std::fs::metadata(&template_path).is_err() {
        std::fs::write(template_path, exercise.test_template).unwrap();
    }

    std::fs::create_dir(exercise_path.join("tests")).ok();
    std::fs::write(
        exercise_path.join(format!("tests/{slug}.rs")),
        exercise.tests,
    )
    .unwrap();
}

fn read_fn_names_from_lib_rs(slug: &str) -> Vec<String> {
    let lib_rs =
        std::fs::read_to_string(format!("exercises/practice/{}/src/lib.rs", slug)).unwrap();

    lib_rs
        .split("fn ")
        .skip(1)
        .map(|f| f.split_once('(').unwrap().0.to_string())
        .collect()
}
