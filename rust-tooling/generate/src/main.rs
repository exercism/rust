use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{AddArgs, FullAddArgs, UpdateArgs, prompt_for_template_generation};
use models::{
    exercise_config::PracticeExercise,
    track_config::{self, TRACK_CONFIG},
};

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
        author,
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

    make_configlet_generate_what_it_can(&slug)?;

    if let Err(e) = ensure_exercise_files_are_filled(&slug, (!author.is_empty()).then_some(&author))
    {
        eprintln!("WARNING: failed to ensure exercise files are filled:\n{e:?}");
    }

    let is_update = false;
    generate_exercise_files(&slug, is_update)
}

fn update_exercise(args: UpdateArgs) -> Result<()> {
    let slug = args.unwrap_slug_or_prompt()?;

    make_configlet_generate_what_it_can(&slug)?;

    let author = None;
    if let Err(e) = ensure_exercise_files_are_filled(&slug, author) {
        eprintln!("WARNING: failed to ensure exercise files are filled:\n{e:?}");
    }

    let is_update = true;
    generate_exercise_files(&slug, is_update)
}

fn make_configlet_generate_what_it_can(slug: &str) -> Result<()> {
    let status = std::process::Command::new("mise")
        .args([
            "run",
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
        .context("failed to run configlet sync")?;
    if !status.success() {
        anyhow::bail!("configlet sync failed");
    }

    Ok(())
}

fn ensure_exercise_files_are_filled(slug: &str, author: Option<&str>) -> Result<()> {
    let config_path = format!("exercises/practice/{slug}/.meta/config.json");
    let config = std::fs::read_to_string(&config_path).context("failed to read exercise config")?;
    let mut config: PracticeExercise =
        serde_json::from_str(&config).context("failed to deserialize exercise config")?;

    let snake_slug = slug.replace('-', "_");

    let ensure_filled = |list: &mut Vec<String>, content: &str| {
        if !list.iter().any(|s| s == content) {
            list.push(content.into())
        }
    };
    ensure_filled(&mut config.files.solution, "src/lib.rs");
    ensure_filled(&mut config.files.solution, "Cargo.toml");
    ensure_filled(&mut config.files.test, &format!("tests/{snake_slug}.rs"));
    ensure_filled(&mut config.files.example, ".meta/example.rs");

    if let Some(author) = author {
        ensure_filled(&mut config.authors, author);
    }

    let mut config =
        serde_json::to_string_pretty(&config).context("failed to deserialize config")?;
    config.push('\n'); // trailing newline
    std::fs::write(config_path, config).context("failed to write config")?;

    Ok(())
}

fn generate_exercise_files(slug: &str, is_update: bool) -> Result<()> {
    let exercise = generate::new(slug);

    let exercise_path = PathBuf::from("exercises/practice").join(slug);

    if !is_update {
        std::fs::write(exercise_path.join(".gitignore"), exercise.gitignore)?;
        std::fs::write(exercise_path.join("Cargo.toml"), exercise.manifest)?;
        std::fs::create_dir(exercise_path.join("src")).ok();
        std::fs::write(exercise_path.join("src/lib.rs"), exercise.lib_rs)?;
        std::fs::write(exercise_path.join(".meta/example.rs"), exercise.example)?;
    }

    let template_path = exercise_path.join(".meta/test_template.tera");
    if !template_path.exists() && (!is_update || prompt_for_template_generation()) {
        // Some exercises have existing test cases that are difficult to migrate
        // to the test generator. That's why we only generate a template for
        // existing exercises if the users requests it.
        std::fs::write(&template_path, exercise.test_template)?;
    }

    let snake_slug = slug.replace('-', "_");

    std::fs::create_dir(exercise_path.join("tests")).ok();
    if template_path.exists() {
        std::fs::write(
            exercise_path.join(format!("tests/{snake_slug}.rs")),
            exercise.tests,
        )?;
    }
    Ok(())
}
