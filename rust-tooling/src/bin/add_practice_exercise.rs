use std::fmt::Display;

use convert_case::{Case, Casing};
use exercism_tooling::{
    fs_utils,
    track_config::{self, TRACK_CONFIG},
};
use glob::glob;
use inquire::{validator::Validation, Select, Text};
use tap::prelude::*;

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

impl Display for Difficulty {
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

    update_problem_spec_cache();

    let implemented_exercises = glob("exercises/concept/*")
        .unwrap()
        .chain(glob("exercises/practice/*").unwrap())
        .filter_map(Result::ok)
        .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
        .collect::<Vec<_>>();

    let unimplemented_with_spec = glob("problem-specifications/exercises/*")
        .unwrap()
        .filter_map(Result::ok)
        .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
        .filter(|e| !implemented_exercises.contains(e))
        .collect::<Vec<_>>();

    let slug = Text::new("What's the slug of your exercise?")
        .with_autocomplete(move |input: &_| {
            Ok(unimplemented_with_spec
                .clone()
                .tap_ref_mut(|v: &mut Vec<_>| v.retain(|e| e.starts_with(input))))
        })
        .with_validator(|input: &str| {
            if input.is_case(Case::Kebab) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(
                    "The slug must be in kebab-case.".into(),
                ))
            }
        })
        .prompt()
        .unwrap();

    let name = Text::new("What's the name of your exercise?")
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

    let config = track_config::PracticeExercise::new(slug, name, difficulty);

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
}

/// Populates ~/.cache/exercism/configlet/problem-specifications
///
/// configlet manages a cache of the problem specifications repository.
/// So, instead of fetching the problem specs every time over the network,
/// we can just reuse configlet's cache.
/// We just need to make sure that the cache is up-to-date.
fn update_problem_spec_cache() {
    std::process::Command::new("./bin/configlet")
        .args(["sync"])
        .output()
        .unwrap();
}
