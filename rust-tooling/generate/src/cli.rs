use anyhow::{Context, Result};
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

    /// do not update problem-specifications cache
    #[arg(long)]
    offline: bool,
}

pub struct FullAddArgs {
    pub slug: String,
    pub name: String,
    pub difficulty: track_config::Difficulty,
    pub offline: bool,
}

impl AddArgs {
    pub fn unwrap_args_or_prompt(self) -> Result<FullAddArgs> {
        let slug = match self.slug {
            Some(slug) => slug,
            _ => prompt_for_add_slug()?,
        };
        let name = match self.name {
            Some(name) => name,
            None => prompt_for_exercise_name(&slug)?,
        };
        let difficulty = match self.difficulty {
            Some(diff) => diff,
            None => prompt_for_difficulty()?,
        }
        .into();
        Ok(FullAddArgs {
            slug,
            name,
            difficulty,
            offline: self.offline,
        })
    }
}

#[derive(Parser)]
pub struct UpdateArgs {
    /// slug of the exercise to update
    #[arg(short, long)]
    slug: Option<String>,

    /// do not update problem-specifications cache
    #[arg(long)]
    pub offline: bool,
}

impl UpdateArgs {
    pub fn unwrap_slug_or_prompt(self) -> Result<String> {
        match self.slug {
            Some(slug) => Ok(slug),
            _ => prompt_for_update_slug(),
        }
    }
}

pub fn prompt_for_update_slug() -> Result<String> {
    let implemented_exercises = glob("exercises/practice/*")
        .map_err(anyhow::Error::from)?
        .filter_map(Result::ok)
        .flat_map(|path| path.file_name()?.to_str().map(|s| s.to_owned()))
        .collect::<Vec<_>>();

    Select::new(
        "Which exercise would you like to update?",
        implemented_exercises,
    )
    .prompt()
    .context("failed to select slug")
}

pub fn prompt_for_add_slug() -> Result<String> {
    let implemented_exercises = glob("exercises/concept/*")
        .map_err(anyhow::Error::from)?
        .chain(glob("exercises/practice/*").map_err(anyhow::Error::from)?)
        .filter_map(Result::ok)
        .flat_map(|path| path.file_name()?.to_str().map(|s| s.to_owned()))
        .collect::<Vec<_>>();

    let todo_with_spec = glob("problem-specifications/exercises/*")
        .map_err(anyhow::Error::from)?
        .filter_map(Result::ok)
        .flat_map(|path| path.file_name()?.to_str().map(|s| s.to_owned()))
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
        .context("failed to prompt for slug")
}

pub fn prompt_for_exercise_name(slug: &str) -> Result<String> {
    Text::new("What's the name of your exercise?")
        .with_initial_value(&slug.to_case(Case::Title))
        .prompt()
        .context("failed to prompt for exercise name")
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

pub fn prompt_for_difficulty() -> Result<Difficulty> {
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
    .context("failed to select difficulty")
}

pub fn prompt_for_template_generation() -> bool {
    inquire::Confirm::new("Would you like to add a template for test generation?")
        .with_default(false)
        .prompt()
        .unwrap_or_default()
}
