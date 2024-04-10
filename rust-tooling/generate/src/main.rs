use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{AddArgs, FullAddArgs, UpdateArgs};
use models::track_config::{self, TRACK_CONFIG};

mod cli;

fn main() -> Result<()> {
    utils::fs::cd_into_repo_root();

    let cli_args = cli::CliArgs::parse();

    match cli_args.command {
        cli::Command::Add(args) => add_exercise(args),
        cli::Command::Update(args) => update_exercise(args),
    }
}

fn add_exercise(args: AddArgs) -> Result<()> {
    let FullAddArgs {
        slug,
        name,
        difficulty,
        offline,
    } = args.unwrap_args_or_prompt()?;

    let config = track_config::PracticeExercise::new(slug.clone(), name, difficulty);

    let mut track_config = TRACK_CONFIG.clone();
    track_config.exercises.practice.push(config);
    let mut new_config = serde_json::to_string_pretty(&track_config)
        .context("failed to deserialize track config")?
        .to_string();
    new_config += "\n";
    std::fs::write("config.json", new_config)?;

    println!(
        "\
Added your exercise to config.json.
You can add practices, prerequisites and topics if you like."
    );

    make_configlet_generate_what_it_can(&slug, offline)?;

    let is_update = false;
    generate_exercise_files(&slug, is_update)
}

fn update_exercise(args: UpdateArgs) -> Result<()> {
    let offline = args.offline;
    let slug = args.unwrap_slug_or_prompt()?;

    make_configlet_generate_what_it_can(&slug, offline)?;

    let is_update = true;
    generate_exercise_files(&slug, is_update)
}

fn make_configlet_generate_what_it_can(slug: &str, offline: bool) -> Result<()> {
    let status = std::process::Command::new("just")
        .args(
            [
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
            ]
            .into_iter()
            .chain(offline.then_some("--offline")),
        )
        .status()
        .context("failed to run configlet sync")?;
    if !status.success() {
        anyhow::bail!("configlet sync failed");
    }
    Ok(())
}

fn generate_exercise_files(slug: &str, is_update: bool) -> Result<()> {
    let exercise = generate::new(slug).context("failed to generate exercise")?;

    let exercise_path = PathBuf::from("exercises/practice").join(slug);

    if !is_update {
        std::fs::write(exercise_path.join(".gitignore"), exercise.gitignore)?;
        std::fs::write(exercise_path.join("Cargo.toml"), exercise.manifest)?;
        std::fs::create_dir(exercise_path.join("src")).ok();
        std::fs::write(exercise_path.join("src/lib.rs"), exercise.lib_rs)?;
        std::fs::write(exercise_path.join(".meta/example.rs"), exercise.example)?;
    }

    let template_path = exercise_path.join(".meta/test_template.tera");
    if std::fs::metadata(&template_path).is_err() {
        std::fs::write(template_path, exercise.test_template)?;
    }

    std::fs::create_dir(exercise_path.join("tests")).ok();
    std::fs::write(
        exercise_path.join(format!("tests/{slug}.rs")),
        exercise.tests,
    )
    .map_err(Into::into)
}
