use clap::{Parser, Subcommand};
use convert_case::{Case, Casing};
use glob::glob;
use inquire::{validator::Validation, Select, Text};
use models::track_config;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Add(AddArgs),
    Update(UpdateArgs),
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Add(_) => write!(f, "Add"),
            Command::Update(_) => write!(f, "Update"),
        }
    }
}

#[derive(Parser)]
pub struct AddArgs {
    #[arg(short, long)]
    slug: Option<String>,

    #[arg(short, long)]
    name: Option<String>,

    #[arg(short, long)]
    difficulty: Option<Difficulty>,
}

pub struct FullAddArgs {
    pub slug: String,
    pub name: String,
    pub difficulty: track_config::Difficulty,
}

impl AddArgs {
    pub fn unwrap_args_or_prompt(self) -> FullAddArgs {
        let slug = self.slug.unwrap_or_else(prompt_for_add_slug);
        let name = self.name.unwrap_or_else(|| prompt_for_exercise_name(&slug));
        let difficulty = self.difficulty.unwrap_or_else(prompt_for_difficulty).into();
        FullAddArgs {
            slug,
            name,
            difficulty,
        }
    }
}

#[derive(Parser)]
pub struct UpdateArgs {
    /// slug of the exercise to update
    #[arg(short, long)]
    slug: Option<String>,
}

impl UpdateArgs {
    pub fn unwrap_slug_or_prompt(self) -> String {
        self.slug.unwrap_or_else(prompt_for_update_slug)
    }
}

pub fn prompt_for_update_slug() -> String {
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

pub fn prompt_for_add_slug() -> String {
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
    Text::new("What's the slug of your exercise?")
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
        .unwrap()
}

pub fn prompt_for_exercise_name(slug: &str) -> String {
    Text::new("What's the name of your exercise?")
        .with_initial_value(&slug.to_case(Case::Title))
        .prompt()
        .unwrap()
}

/// Mostly a clone of the `Difficulty` enum from `models::track_config`.
/// The purpose of this is that we can implement cli-specific traits in this crate.
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
#[repr(u8)]
pub enum Difficulty {
    Easy = 1,
    Medium = 4,
    // I'm not sure why there are two medium difficulties
    Medium2 = 7,
    Hard = 10,
}

impl From<Difficulty> for track_config::Difficulty {
    fn from(value: Difficulty) -> Self {
        match value {
            Difficulty::Easy => track_config::Difficulty::Easy,
            Difficulty::Medium => track_config::Difficulty::Medium,
            Difficulty::Medium2 => track_config::Difficulty::Medium2,
            Difficulty::Hard => track_config::Difficulty::Hard,
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

pub fn prompt_for_difficulty() -> Difficulty {
    Select::new(
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
}
